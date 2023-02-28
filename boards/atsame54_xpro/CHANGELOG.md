# Unreleased
- Limit RAM memory to avoid HardFaults when `UROW:ECCRAM` is enabled
- Remove re-export of `cortex-m-rt::entry`

# v0.5.0
- update to `atsamd-hal-0.15`
- update to to `panic-semihosting-0.6`
- added functions to create all sercom devices and pads using the XPro extensions 1, 2, and 3
- Changed pin types to use their correct alternate definitions instead of using GPIO functions
- Removed the structs of pin sets which relied on old pin definitions

---

# v0.4.0

- update to `atsamd-hal-0.14` and other latest dependencies (#564)
- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` and `panic_rtt` (#510)

---

Changelog tracking started at v0.3.0
