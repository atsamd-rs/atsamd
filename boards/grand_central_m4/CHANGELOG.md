# Unreleased

- Use correct alternate for USB (#661)
- update to `atsamd-hal-0.15` (v2 drivers of peripherals and removal of deprecated things)
- correction to the USB clock in the bsp convenience function

# v0.5.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` and `panic_rtt` (#510)

---

Changelog tracking started at v0.4.0
