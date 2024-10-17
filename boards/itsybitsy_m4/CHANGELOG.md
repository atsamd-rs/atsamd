# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0](https://github.com/atsamd-rs/atsamd/compare/itsybitsy_m4-0.8.0...itsybitsy_m4-0.9.0) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749)) 
- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))

### Dependencies

- **[breaking]** Update HAL dependency to `0.17` ([#754](https://github.com/atsamd-rs/atsamd/pull/754))

## 0.8.0

- update hal dependency to v0.15
  - Removed use of i2c v1 API
  - Make use of the `bsp::peripherals!` macro to alias SERCOM 1,2 and 3
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- add `sercom_interrupt` example to show how to
  - manually configure sercom for uart operation
  - use sercom interrupts

## 0.7.0

- update bsp to use the v2 API
- bump cortex-m dependency to 0.7
- add SPI example
- change dotstar and usb_serial examples to replace deprecated `SpinTimer` with `TimerCounter`
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` and `panic_rtt` (#510)

---

Changelog tracking started at v0.6.0
