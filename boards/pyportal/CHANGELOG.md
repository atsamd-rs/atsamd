# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.2](https://github.com/atsamd-rs/atsamd/compare/pyportal-0.10.1...pyportal-0.10.2) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749)) 
- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))

### Other

- Re-organize using a proc-macro to support more devices ([#728](https://github.com/atsamd-rs/atsamd/pull/728))

## v0.10.1

- Use correct alternate for USB (#661)

## v0.10.0

- update to `atsamd-hal` to 0.15.1 and `cortex-m-rt` to 0.7.1
- Added usb feature to bsp, and example
- Added display feature to bsp and example

## v0.9.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- removed unnecessary dependency on `nb` and `panic_rtt` (#510)

---

Changelog tracking started at v0.8.0
