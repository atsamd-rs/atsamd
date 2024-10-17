# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.12.2](https://github.com/atsamd-rs/atsamd/compare/arduino_mkrzero-0.12.1...arduino_mkrzero-0.12.2) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749))

## v0.12.1

- update to `atsamd-hal-0.16`, along with redefining pins using the bsp_pins! macro
- split up library into pin definitions and peripheral & USB setup function definitions

## v0.12.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` (#510)

---

Changelog tracking started at v0.11.0
