# Changelog

## Unreleased

- add Nina pin aliases

## v0.7.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
* move `usbd-x` crates used only in examples to `[dev-dependencies]`
* removed unnecessary dependency on `nb` (#510)

## 0.6.0

* `Pins` is now generated through `bsp_pins` macro and uses `gpio.v2` pins.
* `usb_allocator`, `i2c_master`, `uart` and `spi_master` functions no longer require `pins.port` parameter and have now been updated to use `gpio.v2` and `sercom.v2` libraries.

---

Changelog began tracking from 0.5.0
