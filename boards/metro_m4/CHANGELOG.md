# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.13.1](https://github.com/atsamd-rs/atsamd/compare/metro_m4-0.13.0...metro_m4-0.13.1) - 2024-10-17

### Other

- *(ci)* `release-plz`: Complete overhaul of the release process ([#762](https://github.com/atsamd-rs/atsamd/pull/762))
- Update PACS to svd2rust 0.34 ([#756](https://github.com/atsamd-rs/atsamd/pull/756))
- Various small fixes ([#749](https://github.com/atsamd-rs/atsamd/pull/749))
# Unreleased

- Upgrade PACs to latest SVD and `svd2rust`:
  - All peripheral types are now `PascalCase`
  - All register field accessors are now methods instead of struct members
  - Members of the `Peripherals` struct are now `snake_case`
  - Removed: `EnabledOscUlp32kBase::set_calibration`
- fix missing `use_semihosting` Cargo feature
- update path of Cargo config

# v0.13.0

- Implement `embedded-hal` `1.0` for GPIO, SPI, I2C, UART and fix examples
- Update the PACs to svd2rust 0.30.2.

# v0.12.0

- Use correct alternate for USB (#661)
- Correction to the clock in the usb convenience function
- Add aliases for A6 and A7.

# v0.11.1

- Update to `atsamd-hal` version `0.15.1`
- Make use of `bsp_peripherals` macro

# v0.11.0

- Update `lib.rs` and examples to reflect removal of `v1` APIs and promotion of `v2` APIs
- Update `i2c_master` convenience function to use the new `sercom::v2::i2c` API
- Add an `i2c` example
- Fix incorrect clocking in `uart` helper function
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)

# v0.10.0

- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` (#510)
- Update to use refactored SPI module (#467)

---

Changelog tracking started at v0.9.0
