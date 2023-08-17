# Unreleased

# v0.12.1

- update to `atsamd-hal-0.16`, along with redefining pins using the bsp_pins! macro
- split up library into pin definitions and peripheral & USB setup function definitions

# v0.12.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` (#510)

---

Changelog tracking started at v0.11.0
