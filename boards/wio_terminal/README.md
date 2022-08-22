# `wio-terminal`

[![Crates.io](https://img.shields.io/crates/v/wio-terminal.svg)](https://crates.io/crates/wio-terminal)
[![Docs](https://docs.rs/wio-terminal/badge.svg)](https://docs.rs/wio-terminal/)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)

> A Board Support Package (BSP) which provides a type-safe API for the Seeed Studio [Wio Terminal](https://www.seeedstudio.com/Wio-Terminal-p-4509.html).

This project is made possible thanks to the following crates:

* [atsamd-hal](https://github.com/atsamd-rs/atsamd)
* [ili9341-rs](https://github.com/yuri91/ili9341-rs)
* [lis3dh-rs](https://github.com/BenBergman/lis3dh-rs)
* [embedded-sdmmc](https://github.com/rust-embedded-community/embedded-sdmmc-rs)
* [seeed-erpc-rs](https://github.com/twitchyliquid64/seeed-erpc-rs)

## [Documentation]

[Documentation]: https://docs.rs/wio-terminal/

## Resources

* [Wio Terminal product page](https://www.seeedstudio.com/Wio-Terminal-p-4509.html)
* [Wio Terminal wiki](https://wiki.seeedstudio.com/Wio-Terminal-Getting-Started/)
* [Wio Terminal user manual](https://files.seeedstudio.com/wiki/Wio-Terminal/res/Wio-Terminal-User-Manual.pdf)

## Display

The ILI9341 datasheet suggests that the maximum speed would be 10 MHz, but
this is conservative and based on low voltage sources.  58 MHz is used in the BSP examples.

## Wifi

If you want to use the Wifi features of this crate, you must have updated the wifi core firmware to at least version `2.0.1`. You
can read instructions for this process [here](https://wiki.seeedstudio.com/Wio-Terminal-Network-Overview/).

## Examples

For information on building and flashing the examples to your device, as well as a list of all examples with brief explanations, please see the [examples README](examples/README.md).

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
