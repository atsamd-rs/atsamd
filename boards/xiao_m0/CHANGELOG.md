# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.12.2](https://github.com/atsamd-rs/atsamd/compare/xiao_m0-0.12.1...xiao_m0-0.12.2) - 2024-10-17

### Other

- *(ci)* `release-plz`: Complete overhaul of the release process ([#762](https://github.com/atsamd-rs/atsamd/pull/762))
- Updated XIAO M0 dependencies and examples ([#742](https://github.com/atsamd-rs/atsamd/pull/742))
- Fix xiao_m0 spi clock used in spi_master ([#711](https://github.com/atsamd-rs/atsamd/pull/711))
- bump hal to 0.16 for xiao_m0 ([#708](https://github.com/atsamd-rs/atsamd/pull/708))
# Unreleased

- Add cortex-m critical section feature
- Update usbd-serial to 0.2
- Update ssd1306 to 0.8.4
- Update embedded-graphics to 0.8.1
- Update shared-bus to 0.3.1
- Remove unproven feature
- Update examples with new package updates
- Update atsamd-hal to 0.17

# v0.12.1

- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)

# v0.12.0

- bump hal dependency to 0.14.0
- bump cortex-m dependency to 0.7
- add `sercom_interrupt` example to show how to
  - manually configure sercom for uart operation
  - use sercom interrupts
- create `eic` example to show the relation between `Pin`, `ExtIntX` and `INTFLAG`

# v0.11.0

- update gpio and sercom dependencies to v2
- create shared_i2c example with I2C bus used for SSD1306 OLED screen and accelerometer
- create ssd1306_i2c example with basic usage of OLED screen for animation with I2C
- update blink example to be more readable for newcomers
- clean up usb_echo example (and extend of blinking on data transfer)
- remove extraneous `embedded-hal` dependencies from BSPs
- cleanup `cortex_m` dependency
- move `usbd-x` crates used only in examples to `[dev-dependencies]`
- removed unnecessary dependency on `nb` (#510)

---

Changelog tracking started at v0.10.0
