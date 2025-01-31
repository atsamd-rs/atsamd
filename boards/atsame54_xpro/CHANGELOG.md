# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.1](https://github.com/atsamd-rs/atsamd/compare/atsame54_xpro-0.11.0...atsame54_xpro-0.11.1) - 2025-01-31

### Examples

- Updates all Tier 1 BSP examples to use RTIC v2 and the RTC monotonic where applicable (#817)

## [0.11.0](https://github.com/atsamd-rs/atsamd/compare/atsame54_xpro-0.10.1...atsame54_xpro-0.11.0) - 2025-01-13

### Other

- updated the following local packages: atsamd-hal

## [0.10.1](https://github.com/atsamd-rs/atsamd/compare/atsame54_xpro-0.10.0...atsame54_xpro-0.10.1) - 2024-12-11

### Other

- updated the following local packages: atsamd-hal

## [0.10.0](https://github.com/atsamd-rs/atsamd/compare/atsame54_xpro-0.9.0...atsame54_xpro-0.10.0) - 2024-11-28

### Added

- [**breaking**] Add async support for many peripherals ([#635](https://github.com/atsamd-rs/atsamd/pull/635))

### Other

- Fix atsame54_xpro mcan example

## [0.9.0](https://github.com/atsamd-rs/atsamd/compare/atsame54_xpro-0.8.1...atsame54_xpro-0.9.0) - 2024-11-17

### Examples

- Add DMA examples for SPI, I2C, and UART to Tier 1 BSPs ([#772](https://github.com/atsamd-rs/atsamd/pull/772))

### Dependencies

- Update HAL dependency to `0.19`

## [0.8.1](https://github.com/atsamd-rs/atsamd/compare/atsame54_xpro-0.8.0...atsame54_xpro-0.8.1) - 2024-10-25

### Other

- updated the following local packages: atsamd-hal

## [0.8.0](https://github.com/atsamd-rs/atsamd/compare/atsame54_xpro-0.7.0...atsame54_xpro-0.8.0) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749))
- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))

### Dependencies

- **[breaking]** Upgrade PAC generated code to latest SVD and `svd2rust-0.34.1` [#756](https://github.com/atsamd-rs/atsamd/pull/756):
- Update HAL dependency to `0.18`

## v0.7.0

- Implement `embedded-hal` `1.0` for GPIO, SPI, I2C, UART and fix examples
- Update the PACs to svd2rust 0.30.2.

## v0.6.0
- Limit RAM memory to avoid HardFaults when `UROW:ECCRAM` is enabled
- Remove re-export of `cortex-m-rt::entry`

## v0.5.0
- update to `atsamd-hal-0.15`
- update to to `panic-semihosting-0.6`
- added functions to create all sercom devices and pads using the XPro extensions 1, 2, and 3
- Changed pin types to use their correct alternate definitions instead of using GPIO functions
- Removed the structs of pin sets which relied on old pin definitions

## v0.4.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` and `panic_rtt` (#510)

---

Changelog tracking started at v0.3.0
