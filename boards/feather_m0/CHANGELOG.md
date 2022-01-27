# Unreleased

- Add an `i2c` example
- Make use of `bsp_peripherals`, `periph_alias` and `pin_alias` macros
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)

# v0.11.0

- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
* move `usbd-x` crates used only in examples to `[dev-dependencies]`
* removed unnecessary dependency on `nb` and `panic_rtt` (#510)
* Bump `cortex-m`/`cortex-m-rt` dependencies to fix a build issue
- Update to use refactored SPI module (#467)

# v0.10.1

* Bump dependencies `rtic-monotonic` to `0.1.0-rc.1` and `cortex-m-rtic` to `0.6.0-rc.2`.

* Change Cargo feature resolver to `resolver = "2"`

---

Changelog tracking started at v0.10.0
