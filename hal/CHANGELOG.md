# Unreleased Changes

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
---

Changelog tracking started at v0.13
