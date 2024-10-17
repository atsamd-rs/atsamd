# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.0](https://github.com/atsamd-rs/atsamd/compare/itsybitsy_m0-0.13.1...itsybitsy_m0-0.14.0) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749)) 
- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))

### Dependencies

- **[breaking]** Update HAL dependency to `0.17` ([#754](https://github.com/atsamd-rs/atsamd/pull/754))

## v0.13.1

- Fix issue with twitching_usb_mouse example not working on MacOS

## 0.13.0

- Big rework of board code by copying the code from feather_m0 and implementing the HW differences (both boards are now more similar to each other) and the corresponding examples ([#559](https://github.com/atsamd-rs/atsamd/pull/559))

---

Changelog tracking started at v0.12.0
