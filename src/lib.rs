#![no_std]

use embedded_hal as hal;

use crate::hal::digital::OutputPin;
use crate::hal::timer::{CountDown, Periodic};
use smart_leds_trait::{Color, SmartLedsWrite};

use nb;
use nb::block;

pub struct Ws2812<'a, TIMER, PIN> {
    timer: TIMER,
    pin: &'a mut PIN,
}

impl<'a, TIMER, PIN> Ws2812<'a, TIMER, PIN>
where
    TIMER: CountDown + Periodic,
    PIN: OutputPin,
{
    /// The timer has to already run at with a frequency of 3 MHz
    pub fn new(timer: TIMER, pin: &'a mut PIN) -> Ws2812<'a, TIMER, PIN> {
        pin.set_low();
        Self { timer, pin }
    }
    /// Write a single color for ws2812 devices
    fn write_color(&mut self, data: Color) {
        let mut serial_bits = (data.g as u32) << 16 | (data.r as u32) << 8 | (data.b as u32) << 0;
        for _ in 0..24 {
            if (serial_bits & 0x00800000) != 0 {
                block!(self.timer.wait()).ok();
                self.pin.set_high();
                block!(self.timer.wait()).ok();
                block!(self.timer.wait()).ok();
                self.pin.set_low();
            } else {
                block!(self.timer.wait()).ok();
                self.pin.set_high();
                block!(self.timer.wait()).ok();
                self.pin.set_low();
                block!(self.timer.wait()).ok();
            }
            serial_bits <<= 1;
        }
    }
}

impl<TIMER, PIN> SmartLedsWrite for Ws2812<'_, TIMER, PIN>
where
    TIMER: CountDown + Periodic,
    PIN: OutputPin,
{
    type Error = ();

    /// Write all the items of an iterator to a ws2812 strip
    fn write<T>(&mut self, iterator: T) -> Result<(), ()>
    where
        T: Iterator<Item = Color>,
    {
        for item in iterator {
            self.write_color(item);
        }
        // Get a timeout period of 300 ns
        for _ in 0..900 {
            block!(self.timer.wait()).ok();
        }
        Ok(())
    }
}
