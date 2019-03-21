# Adafruit NeoTrellis M4 Express Board Support Crate

This crate provides a type-safe Rust API for working with the
[Adafruit NeoTrellis M4 board].

![Adafruit NeoTrellis M4](https://cdn-shop.adafruit.com/970x728/3938-05.jpg)

## Board Features

- Microchip [ATSAMD51G19A] Cortex-M4 microcontroller @ 120 MHz (32-bit, 3.3V logic and power)
  - 512kB Flash
  - 192kB SRAM
- 8 MB SPI Flash chip
- USB device controller (for e.g. MIDI)
- 4-JST hacking port with 3.3V power, ground, and two GPIO (can be I2C/ADC/UART)
- Analog Devices [ADXL343] triple-axis accelerometer

## Optional `trellis_m4` Cargo Features

The following optional hardware drivers can be enabled as cargo features:

- `adxl343`: [ADXL343] accelerometer support
- `keypad-unproven`: (alpha) support for button input via the [keypad] crate

To enable them, use the `features` option when adding a crate dependency to
your Cargo.toml:

```toml
[dependencies]
trellis_m4 = { version = "~0.1", features = ["adxl343", "keypad-unproven"] }
```

## Examples?

Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/trellis_m4/examples

[Adafruit NeoTrellis M4 board]: https://www.adafruit.com/product/3938
[ATSAMD51G19A]: https://www.microchip.com/wwwproducts/en/ATSAMD51G19A
[ADXL343]: https://www.analog.com/en/products/adxl343.html
[keypad]: https://crates.io/crates/keypad
