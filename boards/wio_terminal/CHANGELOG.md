# Unreleased

- Replace homebrew time library with `fugit` (#672)
- Fix end-of-line glyph loss in usb_serial_display example
- Fix display frequency in examples that did not work
- Fix display offsets in buttons example
- Fix buttons by re-enabling debounce code
- Fix various documentation and clippy warnings
- Fix Wi-Fi by delaying UART init until device reset
- Demote to tier 2, as there have been difficulties maintaining this board, and no current maintainer owns one
- Added an example program which is the snake game (`snake.rs`). User can control the direction of the snake using the 5-way-switch.

# v0.6.1

- Update to `atsamd-hal` version `0.15.1`

# v0.6.0

- Fix removed deprecated modules from HAL
- Update `lib.rs` and examples to reflect removal of `v1` APIs and promotion of `v2` APIs
- Update `i2c_master` convenience function to use the new `sercom::v2::i2c` API
- Add an `i2c` example

# v0.5.0

- Update library and examples to use `atsamd-hal` V2 APIs and upgrade BSP to Tier 1.
- Moved crates used only in examples to `[dev-dependencies]`

---

Changelog tracking started at v0.4.0
