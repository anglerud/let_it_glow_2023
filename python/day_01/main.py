import machine

onboardLED = machine.Pin(25, machine.Pin.OUT)
onboardLED.value(1)