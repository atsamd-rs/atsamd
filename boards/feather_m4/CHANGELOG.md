# Unreleased

# v0.10.1

- Update to `atsamd-hal` version `0.15.1`
- Make use of `bsp_peripherals` macro

# v0.10.0

- Update `lib.rs` and examples to reflect removal of `v1` APIs and promotion of `v2` APIs
- Add an `i2c` example
- Update `i2c_master` convenience function to use the new `sercom::v2::i2c` API
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)

# v0.9.0

- replace deprecated `SpinTimer` with `TimerCounter` in the `neopixel_rainbow` example
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` (#510)
- add Public Key Cryptography Controller (PUKCC) example (#486)
- Update to use refactored SPI module (#467)

---

Changelog tracking started at v0.8.0
