# HAL for working with atsamd & atsame devices

This crate provides a type-safe API for working with `samd11`, `samd21`, `samd51`, and `same54` based devices.

[![Build Status](https://travis-ci.org/atsamd-rs/atsamd.svg?branch=master)](https://travis-ci.org/atsamd-rs/atsamd)
[![Crates.io](https://img.shields.io/crates/v/atsamd-hal.svg)](https://crates.io/crates/atsamd-hal)

## [Documentation](https://docs.rs/atsamd-hal)

## Supported Devices

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
* `atsame54p` (via the `same54p` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsame54p)

This crate can support other variants in a similar fashion; pull requests for this are welcomed!

## Examples?

Check out the metro_m0 board support crate examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/metro_m0/examples

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
