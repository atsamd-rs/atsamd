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
