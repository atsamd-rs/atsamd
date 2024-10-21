# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.1](https://github.com/atsamd-rs/atsamd/compare/qt_py_m0-0.10.0...qt_py_m0-0.10.1) - 2024-10-17

### Refactored

- update path of Cargo config ([#749](https://github.com/atsamd-rs/atsamd/pull/749)) 
- Add `usb_echo` example to `Cargo.toml` ([#712](https://github.com/atsamd-rs/atsamd/pull/712))

## v0.10.0

- fix `cortex-m` dev-dependency (#563)
- update to latest HAL and v2 APIs (#563)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` (#510)

---

Changelog tracking started at v0.10.0
