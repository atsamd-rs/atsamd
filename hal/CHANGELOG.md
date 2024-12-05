# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.20.1](https://github.com/atsamd-rs/atsamd/compare/atsamd-hal-0.20.0...atsamd-hal-0.20.1) - 2024-12-05

### Added

- *(can)* Allow the use of any GCLK with the CAN peripheral ([#797](https://github.com/atsamd-rs/atsamd/pull/797))

### Fixed

- *(pwm)* Fix missing sync of timer ENABLE bit ([#795](https://github.com/atsamd-rs/atsamd/pull/795))

## [0.20.0](https://github.com/atsamd-rs/atsamd/compare/atsamd-hal-0.19.0...atsamd-hal-0.20.0) - 2024-11-28

### Added

- [**breaking**] Add async support for many peripherals ([#635](https://github.com/atsamd-rs/atsamd/pull/635)):
    * Supported peripherals: SPI, I2C, UART, DMAC, Timer/counter, external interrupts (EIC)
    * [Additional reading](https://docs.rs/atsamd-hal/latest/atsamd_hal/async_hal/index.html)

### Changed

- *(eic)* [**breaking**] Overhaul the `eic` API  ([#635](https://github.com/atsamd-rs/atsamd/pull/635), [792](https://github.com/atsamd-rs/atsamd/pull/792)):
    * API now uses a typestate pattern for `ExtInt` struct instead of individual `ExtInt1`, `ExtInt2`, ...
    * `ExtInt` methods no longer require a reference to the underlying `Eic`
    * `ExtInt`s take ownership of an EXTINT channel, preventing erroneous reuse
    * [Additional reading](https://docs.rs/atsamd-hal/latest/atsamd_hal/eic/index.html)


### Fixed

- *(i2c)* Send repeated starts in byte-by-byte I2C transactions

## [0.19.0](https://github.com/atsamd-rs/atsamd/compare/atsamd-hal-0.18.2...atsamd-hal-0.19.0) - 2024-11-17

### Added

- *(spi)* Unlock DMA transfers for SpiBus::transfer_in_place ([#780](https://github.com/atsamd-rs/atsamd/pull/780))
- Make `OwnedPeripheral`, `RxPin` and `TxPin` traits from the `can` module public ([#778](https://github.com/atsamd-rs/atsamd/pull/778))
- *(spi)* Add PanicOnRead and PanicOnWrite for simplex SPI transactions ([#772](https://github.com/atsamd-rs/atsamd/pull/772))
- *(uart)* embedded_io implementations for UART DMA transfers ([#772](https://github.com/atsamd-rs/atsamd/pull/772))
- *(i2c)* embedded_hal I2c implementation for I2C DMA transfers ([#772](https://github.com/atsamd-rs/atsamd/pull/772))
- *(spi)* embedded_hal SpiBus implementation for SPI DMA transfers ([#772](https://github.com/atsamd-rs/atsamd/pull/772))

### Fixed

- *(spi)* Only enable SPI receiver when it has RX capability ([#772](https://github.com/atsamd-rs/atsamd/pull/772))

### Other

- [**breaking**] Bump MSRV to Rust 1.77.2

## [0.18.2](https://github.com/atsamd-rs/atsamd/compare/atsamd-hal-0.18.1...atsamd-hal-0.18.2) - 2024-10-30

### Other

- updated the following local packages: atsamd11c, atsamd11d, atsamd21e, atsamd21g, atsamd21j, atsamd51g, atsamd51j, atsamd51n, atsamd51p, atsame51g, atsame51j, atsame51n, atsame53j, atsame53n, atsame54n, atsame54p

## [0.18.1](https://github.com/atsamd-rs/atsamd/compare/atsamd-hal-0.18.0...atsamd-hal-0.18.1) - 2024-10-25

### Fixed

- Changes to the `dmac` public API ([#764](https://github.com/atsamd-rs/atsamd/pull/764)):
  * Make `BlockTransferControl` and `DmacDescriptor` structs private instead of `#[doc(hidden)]`
  * Add getters/setters for `level0`, `level1`, `level2`, and `level3` for `PriorityLevelMask` and `RoundRobinMask` structs
- Cleanup newly introduced nightly Clippy lints ([#763](https://github.com/atsamd-rs/atsamd/pull/763))

## [0.18.0](https://github.com/atsamd-rs/atsamd/compare/atsamd-hal-0.17.0...atsamd-hal-0.18.0) - 2024-10-17

### Dependencies

- **[breaking]** Upgrade PAC generated code to latest SVD and `svd2rust-0.34.1` [#756](https://github.com/atsamd-rs/atsamd/pull/756):
  - All peripheral types are now `PascalCase`
  - All register field accessors are now methods instead of struct members
  - Members of the `Peripherals` struct are now `snake_case`

### Removed

- SAMx5x: removed: `EnabledOscUlp32kBase::set_calibration` [#756](https://github.com/atsamd-rs/atsamd/pull/756)
- **[breaking]**: SAMD51G: removed I2S support ([#735](https://github.com/atsamd-rs/atsamd/pull/735))

### Fixed

- Fix samd51j not having i2s support ([#735](https://github.com/atsamd-rs/atsamd/pull/735))
- Fix EIC issue leading to lost interrupts on SAMD11 platforms ([#739](https://github.com/atsamd-rs/atsamd/pull/739))
- Fix I2C transaction to be as continuous as possible according to `embedded-hal` specification ([#741](https://github.com/atsamd-rs/atsamd/pull/741))
- Fix `embedded-hal` 0.2 CountDown timer implementation ([#749](https://github.com/atsamd-rs/atsamd/pull/749))
- Fix blocking behaviour for `embedded-hal` 0.2 SPI writes ([#743](https://github.com/atsamd-rs/atsamd/pull/743))
- Allow configuring USB clock with `GenericClockController` on atsamd11 ([#734](https://github.com/atsamd-rs/atsamd/pull/734))
- Fix Clippy nightly build errors ([#755](https://github.com/atsamd-rs/atsamd/pull/755), [#737](https://github.com/atsamd-rs/atsamd/pull/737))
- Fix build failures for SAMD11 USB ([#746](https://github.com/atsamd-rs/atsamd/pull/746))

### Refactored

- Upgrade bitflags and get rid of TryFrom<()> implementations ([#746](https://github.com/atsamd-rs/atsamd/pull/746))

## v0.17.0

- Remove `unproven` Cargo feature
- Implement `embedded-hal` `1.0` for GPIO, SPI, I2C, UART, delay and PWM 
- CI/CD pipeline now uses `cargo clippy` instead of `cargo build` and denies clippy warnings by default
- Fix HAL clippy lints
- Add compile error for combined `library` and `dma` features
- Add `dma` feature to docs metadata
- Update the PACs to svd2rust 0.30.2.
- Fix warnings for thumbv7 targets
- Update README.md - moves some content to wiki
- Remove pin `pa28` from the `d21el` target (#717)
- Fix a pwm configuration for the `tc4` on `D5x` targets (#720)
- Update to usb-device 0.3.1 (#718)
- Internal re-organization to support more devices (#728)

## v0.16.0

- Implement `Debug, Clone, Copy, Eq, PartialEq` for all HAL error types (#691).
- Replace homebrew time library with `fugit` (#672)
- Add `defmt` feature and derive `defmt::Format` for error types (#684, obsoletes #522).
- Add `mcan` integration (#654)
- Fix incorrect PAC provided for `same51g` target
- Fix NVM User Row Mapping for `BOD12` Calibration Parameters
- Fix `ExternalInterrupt` implementations for `eic`
- Fix for incorrect feature gates for pins of `samd21gl` chip
- Fix bug in `dmac` where software trigger would not work
- Correct thumbv6 DFLL multiplier to fix USB clock correction
- Fix failing `bsp_pins!` invocation with no aliases (#605 fixes #599)
- Add Advanced Encryption Standard (AES) peripheral support including RustCrypto compatible backend
- Add embedded-hal `InputPin` trait to EIC pins
- Change NVM API
  - Add the ability to modify the user row
  - Add security bit and chip-erase lock management
  - Add escape hatch to access the underlying NVMCTRL PAC
  - Add `Nvm::region_lock` method
  - Change flash read/write/erase method signatures
      - `Nvm::userpage` -> `Nvm::read_userpage`
      - `Nvm::write` -> `Nvm::write_flash`
      - `Nvm::write_from_slice` -> `Nvm::write_flash_from_slice`
      - `Nvm::erase` -> `Nvm::erase_flash`
  - Refactor `Nvm::command_sync` to be less error-prone
- Add the missing ADC traits for the SAMD11D

## v0.15.1

- Fix `sercom::uart` pad definitions to reject pads conflicting with XCK.
- Add support for L-Variant of the SAMD21D

## v0.15.0

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

## v0.14.0

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
