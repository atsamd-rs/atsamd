# 0.8.0

- update hal dependency to v0.15
  - Removed use of i2c v1 API
  - Make use of the `bsp::peripherals!` macro to alias SERCOM 1,2 and 3
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- add `sercom_interrupt` example to show how to
  - manually configure sercom for uart operation
  - use sercom interrupts

# 0.7.0

- update bsp to use the v2 API
- bump cortex-m dependency to 0.7
- add SPI example
- change dotstar and usb_serial examples to replace deprecated `SpinTimer` with `TimerCounter`
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` and `panic_rtt` (#510)

---

Changelog tracking started at v0.6.0
