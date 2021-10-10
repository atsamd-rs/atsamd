# Adafruit Neo Trinkey Board Support Crate

This crate provides a type-safe API for working with the [Adafruit Neo Trinkey
board](https://www.adafruit.com/product/4870).

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv6m-none-eabi`
* Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/neo_trinkey/examples

* Be in this directory `cd boards/neo_trinkey`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build and upload in one step
```bash
$ cargo hf2 --release --example <example-name> --features <any-required-features>
```

You should see the following output
```text
Finished release [optimized] target(s) in 5.55s
Trying  Ok(Some("Adafruit Industries")) Ok(Some("NeoPixel Trinkey M0"))
Flashing "/Users/danielmason/projects/rust/atsamd/boards/neo_trinkey/target/thumbv6m-none-eabi/release/examples/blinky_basic"
Finished in 0.051s
```
Note: If hf2 can not find your Neo Trinkey, check that you have the latest version of cargo-hf2.

If it still doesn't work you can add the Product ID (pid) and Vendor ID (vid) which are usually `0x00ef` and `0x239a`
respectively.

```bash
$ cargo hf2 --release --example <example-name> --features <any-required-features> --pid 0x00ef --vid 0x239a
```

If this _still_ doesn't work, check the USB device in your system settings in case your pid and vid are different.

## Examples

### Blinky basic

```bash
$ cargo hf2 --release --example blinky_basic --features leds
```

Once the Neo Trinkey has restarted, you will see the 4 leds flash in unison. Each led will be a different color (pink,
cyan, yellow and white).

**Warning** even though the lights are turned down very low, they are still very bright.

### Blinky rainbow

```bash
$ cargo hf2 --release --example blinky_rainbow --features leds
```

A slightly more satisfying version of blinky where the lights will cycle through the color spectrum.

**Warning** even though the lights are turned down very low, they are still very bright.

### USB ack

```bash
$ cargo hf2 --release --example usb_ack --features usb
```

Once the device has reset, all the lights will be off. You will then need to find the USB device on your machine.

Usually this is located in `/dev/cu.usbmodemTRINKEY_ACK1`. though if you have multiple trinkeys plugged in and running
this example, the number at the end may change.

You can then send the USB device bytes. Each time the device receives data, it will respond with "Received: X" where X
is the data that it received. To test this in a variety of ways but the easiest is probably with screen.

Connect to the device like this (9600 is the baud rate)

```bash
$ screen /dev/cu.usbmodemTRINKEY_ECHO1 9600
```

You can then press keys and you should get a response Eg:

```text
Received: h
Received: e
Received: l
Received: l
Received: o
Received:
Received: w
Received: o
Received: r
Received: l
Received: d
```

To quit screen, use `ctrl-a` followed by `crtl-\` then `y`

