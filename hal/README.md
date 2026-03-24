# HAL for working with Atmels ATSAMD/E devices, and Microchip's ARM PIC32 devices

This crate provides a type-safe API for working with `samd11`, `samd21`, `samd51`, `same51`, `same53`, `same54` and `pic32cxsg` based devices.

[![Crates.io](https://img.shields.io/crates/v/atsamd-hal.svg)](https://crates.io/crates/atsamd-hal)
[![Docs](https://docs.rs/atsamd-hal/badge.svg)](https://docs.rs/atsamd-hal/)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)

## [Documentation](https://docs.rs/atsamd-hal)

## Supported Devices

### Formerly Atmel devices

* `atsamd11c` (via the `samd11c` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd11c)
* `atsamd21e` (via the `samd21e` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd21e)
* `atsamd21g` (via the `samd21g` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd21g)
* `atsamd21j` (via the `samd21j` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd21j)
* `atsamd51g` (via the `samd51g` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51g)
* `atsamd51j` (via the `samd51j` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51j)
* `atsamd51j` (via the `samd51j` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51j)
* `atsamd51n` (via the `samd51n` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51n)
* `atsamd51p` (via the `samd51p` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51p)
* `atsamd51p` (via the `samd51p` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51p)
* `atsame51g` (via the `same51g` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsame51g)
* `atsame51j` (via the `same51j` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsame51j)
* `atsame51n` (via the `same51n` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsame51n)
* `atsame53j` (via the `same53j` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsame53j)
* `atsame53n` (via the `same53n` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsame53n)
* `atsame54n` (via the `same54n` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsame54n)
* `atsame54p` (via the `same54p` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsame54p)

### Microchip devices

* `PIC32CXSG41064` (via the `pic32cxsg41064` feature) [[pac]]()
* `PIC32CXSG41080` (via the `pic32cxsg41080` feature) [[pac]]()
* `PIC32CXSG41100` (via the `pic32cxsg41100` feature) [[pac]]()
* `PIC32CXSG41128` (via the `pic32cxsg41128` feature) [[pac]]()
* `PIC32CXSG60100` (via the `pic32cxsg60100` feature) [[pac]]()
* `PIC32CXSG60128` (via the `pic32cxsg60128` feature) [[pac]]()
* `PIC32CXSG61100` (via the `pic32cxsg61100` feature) [[pac]]()
* `PIC32CXSG61128` (via the `pic32cxsg61128` feature) [[pac]]()

This crate can support other variants in a similar fashion; pull requests for this are welcomed!

## Examples?

Check out the `feather_m0` board support crate examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/feather_m0/examples

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/atsamd-rs/atsamd/blob/master/LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/atsamd-rs/atsamd/blob/master/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
