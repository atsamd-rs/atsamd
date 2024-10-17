# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.2](https://github.com/atsamd-rs/atsamd/compare/arduino_nano33iot-0.7.1...arduino_nano33iot-0.7.2) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749))

## v0.7.1

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
