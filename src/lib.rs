//! # Use ws2812 leds with timers
//!
//! - For usage with `smart-leds`
//! - Implements the `SmartLedsWrite` trait
//!
//! The `new` method needs a periodic timer running at 3 MHz
//!
//! If it's too slow (e.g.  e.g. all/some leds are white or display the wrong color)
//! you may want to try the `slow` feature.

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
    #[cfg(feature = "slow")]
    fn write_byte(&mut self, mut data: u8) {
        for _ in 0..8 {
            if (data & 0x80) != 0 {
                block!(self.timer.wait()).ok();
                self.pin.set_high();
                block!(self.timer.wait()).ok();
                block!(self.timer.wait()).ok();
                self.pin.set_low();
            } else {
                block!(self.timer.wait()).ok();
                self.pin.set_high();
                self.pin.set_low();
                block!(self.timer.wait()).ok();
                block!(self.timer.wait()).ok();
            }
            data <<= 1;
        }
    }

    /// Write a single color for ws2812 devices
    #[cfg(not(feature = "slow"))]
    fn write_byte(&mut self, mut data: u8) {
        for _ in 0..8 {
            if (data & 0x80) != 0 {
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
            data <<= 1;
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
            self.write_byte(item.g);
            self.write_byte(item.r);
            self.write_byte(item.b);
        }
        // Get a timeout period of 300 ns
        for _ in 0..900 {
            block!(self.timer.wait()).ok();
        }
        Ok(())
    }
}
