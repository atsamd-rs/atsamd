# Unreleased

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
