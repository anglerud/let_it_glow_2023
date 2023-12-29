"""Day 7 of Let it Glow.

Slider to fade intensity between two LEDS.
"""
import time
from machine import ADC, Pin
from neopixel import NeoPixel

# Gamma correction to try to make the light intensities look like
# they're smoothly transitioning beween the two LEDs.
GAMMA         = 2.5
MAX_INTENSITY = 255

def generate_gamma_table():
    return [min(255, round((n / MAX_INTENSITY)**GAMMA * MAX_INTENSITY + 0.5))
            for n in range(256)]

GAMMA_TABLE = generate_gamma_table()
SLIDER_MIN, SLIDER_MAX = 224, 65535  # Measured on board - not very accurate

# Define the RGB LEDs and controls
grb_led_1 = NeoPixel(Pin(2), 1)
grb_led_2 = NeoPixel(Pin(5), 1)
slider = ADC(Pin(28))


def normalize_to_8bit(value, min_range=SLIDER_MIN, max_range=SLIDER_MAX):
    # Ensure the input value is within the specified range
    value = max(min(value, max_range), min_range)

    # Calculate the normalized value in the 8-bit range
    normalized_val = int(((value - min_range) / (max_range - min_range)) * 255)
    # Cap the value as the slider min and max seems a bit tempramental
    return max(min(normalized_val, 255), 0)


while True:
    value = normalize_to_8bit(slider.read_u16())
    led_1_intensity = GAMMA_TABLE[value]
    led_2_intensity = GAMMA_TABLE[-value]
    
    grb_led_1.fill((GAMMA_TABLE[value],0,0))
    grb_led_2.fill((GAMMA_TABLE[255-value],0,0))
    grb_led_1.write()
    grb_led_2.write()
    time.sleep(0.005)

