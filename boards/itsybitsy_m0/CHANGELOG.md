# Unreleased

- Board code completely redone as copy from feather_m0 (#559), which contains:
  - Added more examples, rework existing examples to work with bumped dependencies
  - update bsp to use the v2 API
  - Removed unnecessary dependency on `panic_rtt`
  - Bump `cortex-m`/`cortex-m-rt` dependencies
  - Update to use refactored SPI module (#467)
  - remove extraneous `embedded-hal` dependencies from BSPs
  - cleanup `cortex_m` dependency
  - move `usbd-x` crates used only in examples to `[dev-dependencies]`
  - removed unnecessary dependency on `nb` (#510)

---

Changelog tracking started at v0.11.0
