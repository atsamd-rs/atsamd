# Adafruit Matrix Portal M4 Board Support Crate

This crate provides a type-safe API for working with the [Adafruit Matrix Portal M4
board](https://www.adafruit.com/product/4745).

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
* Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/matrix_portal_m4/examples

* Be in this directory `cd boards/matrix_portal_m4`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build and upload in one step

```Shell
$ cargo hf2 --release --example blinky_basic --vid 0x239a --pid 0x00c9
    Finished release [optimized] target(s) in 0.74s
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("Matrix Portal M4"))
    Flashing "/Users/User/atsamd/boards/matrix_portal_m4/target/thumbv7em-none-eabihf/release/examples/blinky_basic"
    Finished in 0.051s
```

```Shell
$ cargo hf2 --release --example pwm  --vid 0x239a --pid 0x00c9 --features="unproven"
    Finished release [optimized] target(s) in 0.67s
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("Matrix Portal M4"))
    Flashing "/Users/User/atsamd/boards/matrix_portal_m4/target/thumbv7em-none-eabihf/release/examples/pwm"
    Finished in 0.146s
```

## Matrix Portal M4
[PCB DOCs](https://github.com/adafruit/Adafruit-MatrixPortal-M4-PCB/tree/main)
[Adafruit Pinout](https://cdn-learn.adafruit.com/assets/assets/000/111/881/original/led_matrices_Adafruit_MatrixPortal_M4_Pinout.png?1653078587)