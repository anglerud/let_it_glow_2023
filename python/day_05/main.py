from machine import Pin
import time

# Set up switch input pins
dip5 = Pin(6, Pin.IN, Pin.PULL_DOWN)
dip4 = Pin(5, Pin.IN, Pin.PULL_DOWN)
dip3 = Pin(4, Pin.IN, Pin.PULL_DOWN)
dip2 = Pin(3, Pin.IN, Pin.PULL_DOWN)
dip1 = Pin(2, Pin.IN, Pin.PULL_DOWN)
dip_switches = [dip1, dip2, dip3, dip4, dip5]  # right to left

seg5 = Pin(13, Pin.OUT)
seg4 = Pin(12, Pin.OUT)
seg3 = Pin(11, Pin.OUT)
seg2 = Pin(10, Pin.OUT)
seg1 = Pin(9, Pin.OUT)
segments = [seg1, seg2, seg3, seg4, seg5]  # right to left

prev_value = None
while True:
    value = 0b0
    rc = 0b1
    for dip_switch, segment in zip(dip_switches, segments):
        if dip_switch.value() == 1:
            value |= rc
            segment.on()
        else:
            segment.off()
        
        rc = rc << 1
        
    if prev_value != value:
        print(f"Updated value: {value}")
        prev_value = value
    time.sleep(0.1)
