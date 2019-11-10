# HAL for working with atsamd devices

This crate provides a type-safe API for working with atsamd11, atsamd21 and atsamd51 based devices.
Currently this crate supports `atsamd11c14a`, `atsamd21g18a`, `atsamd21e18a` or `atsamd51j19a` 
(selectable via the `samd11c14a`, `samd21g18a`, `samd21e18a`, or `samd51j19a` features),
and should be able to support other variants in a similar fashion; 
pull requests for this are welcomed!

## Examples?

Check out the metro_m0 board support crate examples:

https://github.com/atsamd-rs/atsamd/tree/master/metro_m0/examples
