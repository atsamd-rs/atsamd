# Unreleased Changes

- Upgrade PAC generated code to latest SVD and `svd2rust-0.34.1`:
  - All peripheral types are now `PascalCase`
  - All register field accessors are now methods instead of struct members
  - Members of the `Peripherals` struct are now `snake_case`

# 0.13.0

- Update PAC generated code to use `svd2rust-0.30.2` and `form-0.10.0`

# 0.12.0

- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- update PAC generated code to use `svd2rust-0.20.0` and `form-0.8.0` and make them 2021 edition

# v0.11.0

* Bump `cortex-m`/`cortex-m-rt` dependencies to fix a build issue

---

Changelog tracking started at v0.10.0
