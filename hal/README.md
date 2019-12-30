# HAL for working with atsamd & atsame devices

This crate provides a type-safe API for working with atsamd11, atsamd21, atsamd51, and atsame54 based devices.

## Supported Devices

* `atsamd11c14a` (via the `samd11c14a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd11c14a)
* `atsamd21e18a` (via the `samd21e18a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd21e18a)
* `atsamd21g18a` (via the `samd21g18a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd21g18a)
* `atsamd21j18a` (via the `samd21j18a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd21j18a)
* `atsamd51g19a` (via the `samd51g19a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51g19a)
* `atsamd51j19a` (via the `samd51j19a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51j19a)
* `atsamd51j20a` (via the `samd51j20a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsamd51j20a)
* `atsame54p20a` (via the `same54p20a` feature) [[pac]](https://github.com/atsamd-rs/atsamd/tree/master/pac/atsame54p20a)

This crate can support other variants in a similar fashion; pull requests for this are welcomed!

## Examples?

Check out the metro_m0 board support crate examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/metro_m0/examples
