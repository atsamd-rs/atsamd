# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.1](https://github.com/atsamd-rs/atsamd/compare/feather_m4-0.16.0...feather_m4-0.16.1) - 2025-01-31

### Examples

- Updates all Tier 1 BSP examples to use RTIC v2 and the RTC monotonic where applicable (#817)

## [0.16.0](https://github.com/atsamd-rs/atsamd/compare/feather_m4-0.15.1...feather_m4-0.16.0) - 2025-01-13

### Other

- updated the following local packages: atsamd-hal

## [0.15.1](https://github.com/atsamd-rs/atsamd/compare/feather_m4-0.15.0...feather_m4-0.15.1) - 2024-12-11

### Other

- updated the following local packages: atsamd-hal

## [0.15.0](https://github.com/atsamd-rs/atsamd/compare/feather_m4-0.14.0...feather_m4-0.15.0) - 2024-11-28

### Added

- [**breaking**] Add async support for many peripherals ([#635](https://github.com/atsamd-rs/atsamd/pull/635))

## [0.14.0](https://github.com/atsamd-rs/atsamd/compare/feather_m4-0.13.1...feather_m4-0.14.0) - 2024-11-17

### Examples

- Add DMA examples for SPI, I2C, and UART to Tier 1 BSPs ([#772](https://github.com/atsamd-rs/atsamd/pull/772))

### Dependencies

- Update HAL dependency to `0.19`

## [0.13.1](https://github.com/atsamd-rs/atsamd/compare/feather_m4-0.13.0...feather_m4-0.13.1) - 2024-10-25

### Other

- updated the following local packages: atsamd-hal

## [0.13.0](https://github.com/atsamd-rs/atsamd/compare/feather_m4-0.12.0...feather_m4-0.13.0) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749))
- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))

### Dependencies

- **[breaking]** Upgrade PAC generated code to latest SVD and `svd2rust-0.34.1` [#756](https://github.com/atsamd-rs/atsamd/pull/756):
- Update HAL dependency to `0.18`
## v0.12.0

- Implement `embedded-hal` `1.0` for GPIO, SPI, I2C, UART and fix examples
- Update the PACs to svd2rust 0.30.2.

## v0.11.0

- Replace homebrew time library with `fugit` (#672)
- Use correct alternate for USB (#661)

## v0.10.1

- Update to `atsamd-hal` version `0.15.1`
- Make use of `bsp_peripherals` macro

## v0.10.0

- Update `lib.rs` and examples to reflect removal of `v1` APIs and promotion of `v2` APIs
- Add an `i2c` example
- Update `i2c_master` convenience function to use the new `sercom::v2::i2c` API
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)

## v0.9.0

- replace deprecated `SpinTimer` with `TimerCounter` in the `neopixel_rainbow` example
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` (#510)
- add Public Key Cryptography Controller (PUKCC) example (#486)
- Update to use refactored SPI module (#467)

---

Changelog tracking started at v0.8.0
