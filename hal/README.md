# HAL for working with atsamd & atsame devices

This crate provides a type-safe API for working with `samd11`, `samd21`, `samd51`, and `same54` based devices.

[![Build Status](https://travis-ci.org/atsamd-rs/atsamd.svg?branch=master)](https://travis-ci.org/atsamd-rs/atsamd)
[![Crates.io](https://img.shields.io/crates/v/atsamd-hal.svg)](https://crates.io/crates/atsamd-hal)

## [Documentation](https://docs.rs/atsamd-hal)

## Supported Devices

* `atsamd11c14a` (via the `samd11c14a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd11c14a)
* `atsamd21e18a` (via the `samd21e18a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd21e18a)
* `atsamd21g18a` (via the `samd21g18a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd21g18a)
* `atsamd21j18a` (via the `samd21j18a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd21j18a)
* `atsamd51g19a` (via the `samd51g19a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51g19a)
* `atsamd51j19a` (via the `samd51j19a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51j19a)
* `atsamd51j20a` (via the `samd51j20a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51j20a)
* `atsamd51n20a` (via the `samd51n20a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51n20a)
* `atsamd51p19a` (via the `samd51p19a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51p19a)
* `atsame54p20a` (via the `same54p20a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsame54p20a)

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
