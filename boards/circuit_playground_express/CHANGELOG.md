# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.1](https://github.com/atsamd-rs/atsamd/compare/circuit_playground_express-0.11.0...circuit_playground_express-0.11.1) - 2024-10-17

### Added

- Add aliases for accel I2C pins ([#567](https://github.com/atsamd-rs/atsamd/pull/567))

### Refactored

- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))
- update path of Cargo config  ([#749](https://github.com/atsamd-rs/atsamd/pull/749))

### Other

- Remove HAL path dependency -> board is Tier 2 ([#557](https://github.com/atsamd-rs/atsamd/pull/557))

## v0.11.0

- added the `neopixel_rainbow` example
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- removed unnecessary dependency on `nb` (#510)
- changed pin naming to follow the labels on the board more closely
- added support for UART
- added support for USB
- added support for the SPI bus on A1, A2, A3

---

Changelog tracking started at v0.10.0
