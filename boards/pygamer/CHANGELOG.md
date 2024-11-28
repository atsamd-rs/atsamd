# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
