# Ws2812 driver with timer based delays

For usage with the [smart-leds](https://github.com/smart-leds-rs/smart-leds)
crate.

If your timer/micro is to slow (e.g. all/some leds are white or display the
wrong color), you may wish to enable the `slow` feature. It will remove any
delay for the high part of the zero bits. This may be too short for some led 
strips, which may display wrong data. In that case, you might want to clock 
higher or use another driver.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
