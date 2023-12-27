from machine import Pin
import time

# Dip switches, right to left.
dip_switches = [Pin(n, Pin.IN, Pin.PULL_DOWN) for n in [2, 3, 4, 5, 6]]
# LED segments on segmented light component, right to left.
segments = [Pin(n, Pin.OUT) for n in [9, 10, 11, 12, 13]]

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
