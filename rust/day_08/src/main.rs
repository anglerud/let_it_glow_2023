//! This example shows powerful PIO module in the RP2040 chip to communicate with WS2812 LED modules.
//! See (https://www.sparkfun.com/categories/tags/ws2812)

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::dma::{AnyChannel, Channel};
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{
    Common, Config, FifoJoin, Instance, InterruptHandler, Pio, PioPin, ShiftConfig, ShiftDirection, StateMachine,
};
use embassy_rp::{bind_interrupts, clocks, into_ref, Peripheral, PeripheralRef};
use embassy_time::Timer;
use fixed::types::U24F8;
use fixed_macro::fixed;
use palette::{self, FromColor, IntoColor, Hsl, Lch, Lighten, Darken, ShiftHue, Srgb, rgb::Rgb};
use smart_leds::RGB8;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

pub struct Ws2812<'d, P: Instance, const S: usize, const N: usize> {
    dma: PeripheralRef<'d, AnyChannel>,
    sm: StateMachine<'d, P, S>,
}

impl<'d, P: Instance, const S: usize, const N: usize> Ws2812<'d, P, S, N> {
    pub fn new(
        pio: &mut Common<'d, P>,
        mut sm: StateMachine<'d, P, S>,
        dma: impl Peripheral<P = impl Channel> + 'd,
        pin: impl PioPin,
    ) -> Self {
        into_ref!(dma);

        // Setup sm0

        // prepare the PIO program
        let side_set = pio::SideSet::new(false, 1, false);
        let mut a: pio::Assembler<32> = pio::Assembler::new_with_side_set(side_set);

        const T1: u8 = 2; // start bit
        const T2: u8 = 5; // data bit
        const T3: u8 = 3; // stop bit
        const CYCLES_PER_BIT: u32 = (T1 + T2 + T3) as u32;

        let mut wrap_target = a.label();
        let mut wrap_source = a.label();
        let mut do_zero = a.label();
        a.set_with_side_set(pio::SetDestination::PINDIRS, 1, 0);
        a.bind(&mut wrap_target);
        // Do stop bit
        a.out_with_delay_and_side_set(pio::OutDestination::X, 1, T3 - 1, 0);
        // Do start bit
        a.jmp_with_delay_and_side_set(pio::JmpCondition::XIsZero, &mut do_zero, T1 - 1, 1);
        // Do data bit = 1
        a.jmp_with_delay_and_side_set(pio::JmpCondition::Always, &mut wrap_target, T2 - 1, 1);
        a.bind(&mut do_zero);
        // Do data bit = 0
        a.nop_with_delay_and_side_set(T2 - 1, 0);
        a.bind(&mut wrap_source);

        let prg = a.assemble_with_wrap(wrap_source, wrap_target);
        let mut cfg = Config::default();

        // Pin config
        let out_pin = pio.make_pio_pin(pin);
        cfg.set_out_pins(&[&out_pin]);
        cfg.set_set_pins(&[&out_pin]);

        cfg.use_program(&pio.load_program(&prg), &[&out_pin]);

        // Clock config, measured in kHz to avoid overflows
        // TODO CLOCK_FREQ should come from embassy_rp
        let clock_freq = U24F8::from_num(clocks::clk_sys_freq() / 1000);
        let ws2812_freq = fixed!(800: U24F8);
        let bit_freq = ws2812_freq * CYCLES_PER_BIT;
        cfg.clock_divider = clock_freq / bit_freq;

        // FIFO config
        cfg.fifo_join = FifoJoin::TxOnly;
        cfg.shift_out = ShiftConfig {
            auto_fill: true,
            threshold: 24,
            direction: ShiftDirection::Left,
        };

        sm.set_config(&cfg);
        sm.set_enable(true);

        Self {
            dma: dma.map_into(),
            sm,
        }
    }

    pub async fn write(&mut self, colors: &[RGB8; N]) {
        // Precompute the word bytes from the colors
        let mut words = [0u32; N];
        for i in 0..N {
            let word = (u32::from(colors[i].g) << 24) | (u32::from(colors[i].r) << 16) | (u32::from(colors[i].b) << 8);
            words[i] = word;
        }

        // DMA transfer
        self.sm.tx().dma_push(self.dma.reborrow(), &words).await;
    }
}


// TODO:
// * Break  this into different effects we can sub in
// * Animate teh shift_hue one.
// * Animate different aspects - like saturation, and luminance around wheel
// * Use gradient creation in pallette to generate color schemes
// * Use triples etc to generate color  schemes from single color - look up
//   online color creation things
// * motion stuff - bounces?
// * motion stuff - water drop simulation?
// * motion stuff - look up other ideas. Chases, bounces, shakes, blinks...
// * MOVE into let_it_glow_2023, or make own project?

// async fn static_rainbow(mut ws2812: Ws2812<'_, PIO0, 0, 12>, mut leds: [smart_leds::RGB<u8>; 12]){
// }

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Start");
    let p = embassy_rp::init(Default::default());

    let Pio { mut common, sm0, .. } = Pio::new(p.PIO0, Irqs);
    let mut ws2812 = Ws2812::new(&mut common, sm0, p.DMA_CH0, p.PIN_2);

    // This is the number of leds in the LED ring.
    const NUM_LEDS: usize = 12;
    let mut leds = [RGB8::default(); NUM_LEDS];

    let angle_per_led = 360.0 / 12.0;
    let original_color = Srgb::new(0.8, 0.3, 0.3);
    let original_color_lch = Lch::from_color(original_color).darken(0.9);

    // XXX: Don't need to name all of these - can do a loop and array...
    let c1 = original_color_lch.shift_hue(0.0);
    let c2 = original_color_lch.shift_hue(angle_per_led);
    let c3 = original_color_lch.shift_hue(angle_per_led * 2.0);
    let c4 = original_color_lch.shift_hue(angle_per_led * 3.0);
    let c5 = original_color_lch.shift_hue(angle_per_led * 4.0);
    let c6 = original_color_lch.shift_hue(angle_per_led * 5.0);
    let c7 = original_color_lch.shift_hue(angle_per_led * 6.0);
    let c8 = original_color_lch.shift_hue(angle_per_led * 7.0);
    let c9 = original_color_lch.shift_hue(angle_per_led * 8.0);
    let c10 = original_color_lch.shift_hue(angle_per_led * 9.0);
    let c11 = original_color_lch.shift_hue(angle_per_led * 10.0);
    let c12 = original_color_lch.shift_hue(angle_per_led * 11.0);


    // Loop forever making RGB values and pushing them out to the WS2812.
    loop {
        // TODO: break out the list to the above.
        for (i, color ) in [c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12].iter().enumerate() {

            // Convert back into Srgb, then into three bytes, and finally into the smart LED RGB type.
            let srgb_color: palette::Srgb<u8> = Srgb::from_linear((*color).into_color());
            let arr_color: [u8; 3] = srgb_color.into_format().into();
            let led_color: smart_leds::RGB<u8> = arr_color.into();

            debug!("beep");
            leds[i] = led_color;
            // debug!("R: {} G: {} B: {}", leds[i].r, leds[i].g, leds[i].b);
        }

        ws2812.write(&leds).await;
        Timer::after_millis(10).await;
        // let mut new_color = green.clone();
        // for j in 0..(359) {
        //     new_color = new_color.shift_hue(j as f32);
        //     for i in 0..NUM_LEDS {
        //         let new_color = new_color.darken((i as f32) * 0.083);

        //         let arr_color: [u8; 3] = new_color.into_format().into();
        //         let led_color: smart_leds::RGB<u8> = arr_color.into();

        //         data[i] = led_color;
        //         debug!("Rotating: {}, darken: {}", j, (i as f32) * 0.083);
        //         debug!("R: {} G: {} B: {}", data[i].r, data[i].g, data[i].b);
        //     }
        //     ws2812.write(&data).await;

        //     Timer::after_millis(10).await;
        // }
    }
}
