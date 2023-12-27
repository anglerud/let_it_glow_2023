from machine import Pin
import time

# Set up input pins
red_button = Pin(2, Pin.IN, Pin.PULL_DOWN)
green_button = Pin(3, Pin.IN, Pin.PULL_DOWN)

# Set up the LED pins
seg1 = Pin(13, Pin.OUT)
seg2 = Pin(12, Pin.OUT)
seg3 = Pin(11, Pin.OUT)
seg4 = Pin(10, Pin.OUT)
seg5 = Pin(9, Pin.OUT)
segments = [seg1, seg2, seg3, seg4, seg5]

MIN_COUNTER, MAX_COUNTER = 1, 50
DEBOUNCE_THRESHOLD_MS = 50

class TimerState:
    def __init__(self, delay_multiplier=0.01, counter=25):
        self.delay_multiplier = delay_multiplier
        self.counter = counter
        self.last_trigger = 0
    
    def debounce(self):
        # debounce:
        now = time.ticks_ms()
        last = self.last_trigger
        self.last_trigger = now
        return (now - last) < DEBOUNCE_THRESHOLD_MS
            
    def dec(self, pin):
        if self.debounce():
            return
        
        if pin.value() == 1:
            if self.counter > MIN_COUNTER:
                self.counter -= 1
    
    def inc(self, pin):
        if self.debounce():
            return
        
        if pin.value() == 1:
            if self.counter < MAX_COUNTER:
                self.counter += 1
            
    def value(self):
        return self.counter * self.delay_multiplier

timer_state = TimerState()
red_button.irq(lambda p: timer_state.inc(p))
green_button.irq(lambda p: timer_state.dec(p))

is_reversed = False
while True:
    prev_seg = None
    segments_iter = reversed(segments) if is_reversed else segments
    for seg in segments_iter:
        if prev_seg:
            prev_seg.off()
        seg.on()
        prev_seg = seg
        
        time.sleep(timer_state.value())
    
    is_reversed = not is_reversed