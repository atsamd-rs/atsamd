# Unreleased

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
