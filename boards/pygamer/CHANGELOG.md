# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.15.2](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.15.1...pygamer-0.15.2) - 2025-06-26

### Other

- updated the following local packages: atsamd-hal

## [0.15.1](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.15.0...pygamer-0.15.1) - 2025-06-22

### Other

- updated the following local packages: atsamd-hal

## [0.15.0](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.14.4...pygamer-0.15.0) - 2025-05-08

### Added

- Refactor IoSet trait using sorted HList

### Removed

- Removed neopixel examples ([#814](https://github.com/atsamd-rs/atsamd/pull/814))

### Other

- updated the following local packages: atsamd-hal

## [0.14.4](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.14.3...pygamer-0.14.4) - 2025-05-07

### Other

- updated the following local packages: atsamd-hal

## [0.14.3](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.14.2...pygamer-0.14.3) - 2025-04-22

### Other

- updated the following local packages: atsamd-hal

## [0.14.2](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.14.1...pygamer-0.14.2) - 2025-04-12

### Other

- updated the following local packages: atsamd-hal

## [0.14.1](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.14.0...pygamer-0.14.1) - 2025-01-31

### Dependencies

- *(pygamer)* Upgrade some `pygamer` BSP dependencies (#815)

### Examples

- Updates all Tier 1 BSP examples to use RTIC v2 and the RTC monotonic where applicable (#817)

## [0.14.0](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.13.1...pygamer-0.14.0) - 2025-01-13

### Other

- updated the following local packages: atsamd-hal

## [0.13.1](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.13.0...pygamer-0.13.1) - 2024-12-11

### Examples

- *(pygamer)* Restore neopixel examples using SPI driver

## [0.13.0](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.12.0...pygamer-0.13.0) - 2024-11-28

### Added

- [**breaking**] Add async support for many peripherals ([#635](https://github.com/atsamd-rs/atsamd/pull/635))

## [0.12.0](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.11.0...pygamer-0.12.0) - 2024-11-17

### Other

- *(pygamer)* [**breaking**] Upgrade `pygamer` BSP display driver and graphics dependencies ([#777](https://github.com/atsamd-rs/atsamd/pull/777))

## [0.11.0](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.10.0...pygamer-0.11.0) - 2024-10-30

### Other

- *(pygamer)* [**breaking**] Update pygamer HAL dependency to 0.18 and promote to Tier 1 status ([#766](https://github.com/atsamd-rs/atsamd/pull/766))

## [0.10.0](https://github.com/atsamd-rs/atsamd/compare/pygamer-0.9.0...pygamer-0.10.0) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749)) 
- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))

### Removed

- **[breaking]**Temporary removal of neopixel support and examples (which are currently unreliable) ([#750](https://github.com/atsamd-rs/atsamd/pull/750))

### Dependencies

- **[breaking]** Update HAL dependency to `0.17` ([#750](https://github.com/atsamd-rs/atsamd/pull/750))

## v0.9.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- remove extraneous `embedded-hal` dependencies from BSPs
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` (#510)

---

Changelog tracking started at v0.8.0
