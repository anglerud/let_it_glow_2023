# Enable emergency stack buffer so that one exception can be retained
import micropython
micropython.alloc_emergency_exception_buf(100)

from machine import Pin

# Set up input pins
red_button = Pin(2, Pin.IN, Pin.PULL_DOWN)
green_button = Pin(3, Pin.IN, Pin.PULL_DOWN)

# Set up output pins
red_led = Pin(14, Pin.OUT)
green_led = Pin(25, Pin.OUT)

class LedButton:
    def __init__(self, led_pin):
        self.is_on = False
        self.led_pin = led_pin
    
    def trigger(self, pin):
        if pin.value() == 1:
            # Only trigger on button down
            self.is_on = not self.is_on
        if self.is_on:
            self.led_pin.on()
        else:
            self.led_pin.off()

red_button_state = LedButton(red_led)
green_button_state = LedButton(green_led)

red_button.irq(lambda pin: red_button_state.trigger(pin))
green_button.irq(lambda pin: green_button_state.trigger(pin))

# Note, no `while: True` needed. The IRQ functions stay resident it seems.