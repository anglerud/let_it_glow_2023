"""Day 6 of Let it Glow.

Attempt to "flow" light between two LEDs.
"""
import time
from machine import Pin
from neopixel import NeoPixel

# Gamma correction to try to make the light intensities look like
# they're smoothly transitioning beween the two LEDs.
GAMMA         = 2.5
MAX_INTENSITY = 255

def generate_gamma_table():
    return [round((n / MAX_INTENSITY)**GAMMA * MAX_INTENSITY + 0.5)
            for n in range(256)]

GAMMA_TABLE = generate_gamma_table()

# Define the RGB LEDs and controls
grb_led_1 = NeoPixel(Pin(2), 1)
grb_led_2 = NeoPixel(Pin(5), 1)


while True:
    # Fade left to right
    for g1, g2 in zip(range(255), reversed(range(255))):
        grb_led_1.fill((GAMMA_TABLE[g1],0,0))
        grb_led_2.fill((GAMMA_TABLE[g2],0,0))
        grb_led_1.write()
        grb_led_2.write()
        
        time.sleep(0.005)
        
    # Fade right to left
    for g1, g2 in zip(reversed(range(255)), range(255)):
        grb_led_1.fill((GAMMA_TABLE[g1],0,0))
        grb_led_2.fill((GAMMA_TABLE[g2],0,0))
        grb_led_1.write()
        grb_led_2.write()

        time.sleep(0.005)

