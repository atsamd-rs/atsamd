# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.0](https://github.com/atsamd-rs/atsamd/compare/metro_m4-0.15.0...metro_m4-0.16.0) - 2024-11-28

### Added

- [**breaking**] Add async support for many peripherals ([#635](https://github.com/atsamd-rs/atsamd/pull/635))

### Other

- Refactor common code, use channels

## [0.15.0](https://github.com/atsamd-rs/atsamd/compare/metro_m4-0.14.1...metro_m4-0.15.0) - 2024-11-17

### Examples

- Add DMA examples for SPI, I2C, and UART to Tier 1 BSPs ([#772](https://github.com/atsamd-rs/atsamd/pull/772))

### Dependencies

- Update HAL dependency to `0.19`

## [0.14.1](https://github.com/atsamd-rs/atsamd/compare/metro_m4-0.14.0...metro_m4-0.14.1) - 2024-10-25

### Other

- updated the following local packages: atsamd-hal

## [0.14.0](https://github.com/atsamd-rs/atsamd/compare/metro_m4-0.13.0...metro_m4-0.14.0) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749))
- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))

### Dependencies

- **[breaking]** Upgrade PAC generated code to latest SVD and `svd2rust-0.34.1` [#756](https://github.com/atsamd-rs/atsamd/pull/756):
- Update HAL dependency to `0.18`

## v0.13.0

- Implement `embedded-hal` `1.0` for GPIO, SPI, I2C, UART and fix examples
- Update the PACs to svd2rust 0.30.2.

## v0.12.0

- Use correct alternate for USB (#661)
- Correction to the clock in the usb convenience function
- Add aliases for A6 and A7.

## v0.11.1

- Update to `atsamd-hal` version `0.15.1`
- Make use of `bsp_peripherals` macro

## v0.11.0

- Update `lib.rs` and examples to reflect removal of `v1` APIs and promotion of `v2` APIs
- Update `i2c_master` convenience function to use the new `sercom::v2::i2c` API
- Add an `i2c` example
- Fix incorrect clocking in `uart` helper function
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)

## v0.10.0

- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` (#510)
- Update to use refactored SPI module (#467)

---

Changelog tracking started at v0.9.0
