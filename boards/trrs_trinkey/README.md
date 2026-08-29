# Adafruit TRRS Trinkey Board Support Crate

This crate provides a type-safe API for working with the [Adafruit TRRS Trinkey
board](https://www.adafruit.com/product/5954) ([Adafruit tutorial page](https://learn.adafruit.com/adafruit-trrs-trinkey/)).

<figure>
  <img src="https://raw.githubusercontent.com/adafruit/Adafruit-TRRS-Trinkey-PCB/refs/heads/main/Adafruit%20TRRS%20Trinkey%20PrettyPins.svg" alt="Adafruit TRRS Trinkey Pinout">
  <figcaption>Adafruit TRRS Trinkey Pinout</figcaption>
</figure>

## Prerequisites

- Install the cross compile toolchain `rustup target add thumbv6m-none-eabi`
- Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example

NOTE: `cargo-hf2` does not seem to work. It will require the following PR: https://github.com/jacobrosenthal/hf2-rs/pull/49

Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/trrs_trinkey/examples

- Be in this directory `cd boards/trrs_trinkey`
- Put your device in bootloader mode usually by hitting the reset button twice.
- Build and upload in one step

```
$ cargo hf2 --features leds --release --example blinky_basic
    Finished `release` profile [optimized + debuginfo] target(s) in 0.06s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("TRRS Trinkey M0"))
    Flashing "/home/quantum_p/LocalDocs/atsamd/target/thumbv6m-none-eabi/release/examples/blinky_basic"
    Finished in 0.123s
$
```

The `usb_hid` example is the most useful given what the intended use of the device is. If you need details about other peripherials, the examples from other boards like the `feather_m0` should be sufficient, but have not yet been explicitly tested on this board.
