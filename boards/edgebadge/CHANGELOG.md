# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.1](https://github.com/atsamd-rs/atsamd/compare/edgebadge-0.9.0...edgebadge-0.9.1) - 2024-10-17

### Refactored

- Remove build profiles from Cargo.toml ([#762](https://github.com/atsamd-rs/atsamd/pull/762))
- update path of Cargo config  ([#749](https://github.com/atsamd-rs/atsamd/pull/749))
- Minor examples refactoring ([#655](https://github.com/atsamd-rs/atsamd/pull/655))

### Other

- Re-organize using a proc-macro to support more devices ([#728](https://github.com/atsamd-rs/atsamd/pull/728))
- update embedded-graphics to 0.8 ([#726](https://github.com/atsamd-rs/atsamd/pull/726))

## v0.9.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- remove extraneous `embedded-hal` dependencies from BSPs
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` (#510)

---

Changelog tracking started at v0.8.0
