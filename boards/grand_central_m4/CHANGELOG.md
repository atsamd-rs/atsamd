# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.2](https://github.com/atsamd-rs/atsamd/compare/grand_central_m4-0.8.1...grand_central_m4-0.8.2) - 2024-11-28

### Other

- *(grand_central_m4)* Remove path dependency to `atsamd-hal` ([#790](https://github.com/atsamd-rs/atsamd/pull/790))

## [0.8.1](https://github.com/atsamd-rs/atsamd/compare/grand_central_m4-0.8.0...grand_central_m4-0.8.1) - 2024-11-21

### Other

- *(grand_central_m4)* Add an I2C example ([#787](https://github.com/atsamd-rs/atsamd/pull/787))

## [0.8.0](https://github.com/atsamd-rs/atsamd/compare/grand_central_m4-0.7.0...grand_central_m4-0.8.0) - 2024-11-17

### Added

- *(grand_central_m4)* Add an EIC example ([#785](https://github.com/atsamd-rs/atsamd/pull/785))

### Other

- *(grand_central_m4)* [**breaking**] Update HAL dependency to v0.19 ([#786](https://github.com/atsamd-rs/atsamd/pull/786), [#779](https://github.com/atsamd-rs/atsamd/pull/779))

## [0.7.0](https://github.com/atsamd-rs/atsamd/compare/grand_central_m4-0.6.0...grand_central_m4-0.7.0) - 2024-10-17

### Refactored

- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))
- update path of Cargo config  ([#749](https://github.com/atsamd-rs/atsamd/pull/749))

### Dependencies

- **[breaking]** Bump usb-device dependency to `0.3`
- **[breaking]** Bump HAL dependency to `0.17` ([#753](https://github.com/atsamd-rs/atsamd/pull/753))

### Other

- Re-organize using a proc-macro to support more devices ([#728](https://github.com/atsamd-rs/atsamd/pull/728))

## v0.6.0

- Use correct alternate for USB (#661)
- update to `atsamd-hal-0.15` (v2 drivers of peripherals and removal of deprecated things)
- correction to the USB clock in the bsp convenience function

## v0.5.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` and `panic_rtt` (#510)

---

Changelog tracking started at v0.4.0
