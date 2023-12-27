import machine
import time


green = machine.Pin(14, machine.Pin.OUT)
red = machine.Pin(25, machine.Pin.OUT)

while True:
    green.value(0)

    time.sleep(1)
    red.value(0)

    time.sleep(1)
    green.value(1)

    time.sleep(1)
    red.value(1)
