# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.1](https://github.com/atsamd-rs/atsamd/compare/atsame53j-0.14.0...atsame53j-0.14.1) - 2024-10-30

### Other

- *(pacs)* Update to svd2rust 0.33.5 and re-generate PACs ([#774](https://github.com/atsamd-rs/atsamd/pull/774))

## [0.14.0](https://github.com/atsamd-rs/atsamd/compare/atsame53j-0.13.0...atsame53j-0.14.0) - 2024-10-17

### Changed

- **[breaking]** Upgrade PAC generated code to latest SVD and `svd2rust-0.34.1` [#756](https://github.com/atsamd-rs/atsamd/pull/756):
  - All peripheral types are now `PascalCase`
  - All register field accessors are now methods instead of struct members
  - Members of the `Peripherals` struct are now `snake_case`

## 0.13.0

- Update PAC generated code to use `svd2rust-0.30.2` and `form-0.10.0`

## 0.12.0

- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- update PAC generated code to use `svd2rust-0.20.0` and `form-0.8.0` and make them 2021 edition

## v0.11.0

* Bump `cortex-m`/`cortex-m-rt` dependencies to fix a build issue

---

Changelog tracking started at v0.10.0
