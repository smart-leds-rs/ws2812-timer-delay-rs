# Ws2812 lib with timer based delays

If your timer/micro is to slow (e.g. all leds are white), you may wish to enable
the `slow` feature. It will remove any delay for the high part of the zero bits.
This may be too short for some led strips, which may display wrong data. In that
case, you might want to clock higher or use another driver.
