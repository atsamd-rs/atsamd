# Unreleased Changes

- Fix failing `bsp_pins!` invocation with no aliases (#605 fixes #599)
- Add Advanced Encryption Standard (AES) peripheral support including RustCrypto compatible backend

# v0.15.1

- Fix `sercom::uart` pad definitions to reject pads conflicting with XCK.
- Add support for L-Variant of the SAMD21D

# v0.15.0

- Rework USB API
- Remove deprecated modules ([#480](https://github.com/atsamd-rs/atsamd/pull/480)) :
  - Remove `gpio::v1`, `sercom::v1` module, promote `gpio::v2` to `gpio` and `sercom::v2` to `sercom`.
  - Remove deprecated `hal` in favour of `ehal`
  - Remove deprecated `target_device` in favour of `pac`
  - Remove deprecated `spi_common` module
  - Remove deprecated `common` module
  - Remove deprecated `samd51`, `same51`, `same53`, `same54` modules
  - Remove deprecated `SpinTimer`
  - Provide the necessary fixes to support those changes.
- Cleanup most `clippy` lints
- Update `seq_macro` and remove `replace_with` dependencies (#568)
- Add a `bsp_peripherals!` macro and fix a bug in `bsp_pins!` (#515)
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- updated HAL to be compatible with PAC updates due to svd2rust upgrade
- Add an `sercom::v2::i2c` API
- Modified traits to support the Grand Central M4 Express.

# v0.14.0

- Add implementation of InputPin for Interrupt pins
- Add additional undocumented but valid IOSet for ATSAMD5x/ATSAME5x
- Update PACs to v0.11.0 (#518)
- Improve the `bsp_pins!` macro (#475 & #483)
- Add undocumented but valid IOSet for ATSAMD5x/ATSAME5x (#506)
- Fixed the RTC implementation of embedded-hal timer traits to be periodic again (#490)
- Add Integrity Check Monitor (ICM) abstraction (#480)
- Add Public Key Cryptography Controller (PUKCC) support (#486)
- Refactor the SPI module (#467)
- Bump Rust edition to 2021 and MSRV to 1.56 (#535)
- Implement `rtic::Monotonic` for `RTC` using `fugit` (#540)
  - RTIC spawn task API will now require `fugit::Duration<_, _, _>` (aliased
  at `atsamd_hal::rtc::Duration`) instead of `embedded_time::Duration`
- Add Device Service Unit (DSU), Non-volatile Controller (NVM), SmartEEPROM support (#526)
- Expand Public Key Cryptography Controller (PUKCC) to support modular
  exponentiation (RSA) (#544)

---

Changelog tracking started at v0.13
