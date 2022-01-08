# Unreleased

- Updated to 2021 edition, updated dependencies, removed unused dependencies (#562)
# 0.12.0

- bump hal dependency to 0.14.0
- bump cortex-m dependency to 0.7
- add `sercom_interrupt` example to show how to
  - manually configure sercom for uart operation
  - use sercom interrupts
- create `eic` example to show the relation between `Pin`, `ExtIntX` and `INTFLAG`

# 0.11.0

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
