//! # Use ws2812 leds with timers
//!
//! - For usage with `smart-leds`
//! - Implements the `SmartLedsWrite` trait
//!
//! The `new` method needs a periodic timer running at 3 MHz
//!
//! If it's too slow (e.g.  e.g. all/some leds are white or display the wrong color)
//! you may want to try the `slow` feature.

// https://wp.josh.com/2014/05/13/ws2812-neopixels-are-not-so-finicky-once-you-get-to-know-them/
//                                          Min     Typ     Max     Units
// T0H      0 code  high voltage time       200     350     500     ns
// T1H      1 code  high voltage time       550     700     5,500   ns
// TLD      data    low voltage time        450     600     5,000   ns
// TLL      latch   low voltage time        6,000                   ns

// The tricky timing is the T0H between 200 and 350ns timing.
// 3mhz is 333ns per block and should be flexible for all other timings

// But new device reset latch timing is posted as 280us
// https://blog.particle.io/2017/05/11/heads-up-ws2812b-neopixels-are-about-to-change/

#![no_std]

use embedded_hal as hal;

use crate::hal::digital::v2::OutputPin;
use crate::hal::timer::{CountDown, Periodic};
use smart_leds_trait::{SmartLedsWrite, RGB8};

use nb;
use nb::block;

pub struct Ws2812<TIMER, PIN> {
    timer: TIMER,
    pin: PIN,
}

impl<TIMER, PIN> Ws2812<TIMER, PIN>
where
    TIMER: CountDown + Periodic,
    PIN: OutputPin,
{
    /// The timer has to already run at with a frequency of 3 MHz
    pub fn new(timer: TIMER, mut pin: PIN) -> Ws2812<TIMER, PIN> {
        pin.set_low().ok();
        Self { timer, pin }
    }

    /// Write a single color for ws2812 devices
    #[cfg(feature = "slow")]
    fn write_byte(&mut self, mut data: u8) {
        for _ in 0..8 {
            if (data & 0x80) != 0 {
                block!(self.timer.wait()).ok();
                self.pin.set_high().ok();
                block!(self.timer.wait()).ok();
                block!(self.timer.wait()).ok();
                self.pin.set_low().ok();
            } else {
                block!(self.timer.wait()).ok();
                self.pin.set_high().ok();
                self.pin.set_low().ok();
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
                // 1bit, 666ns on time, 666ns off time
                self.pin.set_high().ok();
                block!(self.timer.wait()).ok();
                block!(self.timer.wait()).ok();
                self.pin.set_low().ok();
                block!(self.timer.wait()).ok();
            } else {
                // 0bit, 333ns on time, 666ns off time
                self.pin.set_high().ok();
                block!(self.timer.wait()).ok();
                self.pin.set_low().ok();
                block!(self.timer.wait()).ok();
                block!(self.timer.wait()).ok();
            }
            data <<= 1;
        }
    }
}

impl<TIMER, PIN> SmartLedsWrite for Ws2812<TIMER, PIN>
where
    TIMER: CountDown + Periodic,
    PIN: OutputPin,
{
    type Error = ();
    type Color = RGB8;
    /// Write all the items of an iterator to a ws2812 strip
    fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error>
    where
        T: Iterator<Item = I>,
        I: Into<Self::Color>,
    {
        for item in iterator {
            let item = item.into();
            self.write_byte(item.g);
            self.write_byte(item.r);
            self.write_byte(item.b);
        }

        // Latch for > 280 us
        // 900 * 333 = 299 us
        for _ in 0..900 {
            let _ = block!(self.timer.wait());
        }
        Ok(())
    }
}
