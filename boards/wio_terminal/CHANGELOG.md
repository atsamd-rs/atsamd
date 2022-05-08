# Unreleased

# v0.6.1

- Update to `atsamd-hal` version `0.15.1`

# v0.6.0

- Fix removed deprecated modules from HAL
- Update `lib.rs` and examples to reflect removal of `v1` APIs and promotion of `v2` APIs
- Update `i2c_master` convenience function to use the new `sercom::v2::i2c` API
- Add an `i2c` example

# v0.5.0

- Update library and examples to use `atsamd-hal` V2 APIs and upgrade BSP to Tier 1.
- Moved crates used only in examples to `[dev-dependencies]`

---

Changelog tracking started at v0.4.0
