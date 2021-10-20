# Adafruit Neokey Trinkey Board Support Crate

This crate provides a type-safe API for working with the [Adafruit Neokey Trinkey
board](https://www.adafruit.com/product/5020).

## Prerequisites
* Install the cross compile toolchain `rustup target add thumbv6m-none-eabi`
* Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example
Check out the repository for examples:

https://github.com/atsamd-rs/atsamd/tree/master/boards/neokey_trinkey/examples

* Be in this directory `cd boards/neokey_trinkey`
* Put your device in bootloader mode usually by hitting the reset button twice.
* Build and upload in one step
```bash
$ cargo hf2 --release --example <example-name> --features <any-required-features> --vid 0x239a --pid 0x00ff
```
> **Note:** If you have a newer version of cargo-hf2 you won't need the pid and vid, it will automatically recognize the
> device

You should see the following output
```text
Finished release [optimized] target(s) in 5.55s
Trying  Ok(Some("Adafruit Industries")) Ok(Some("NeoKey Trinkey M0"))
Flashing "/Users/atsamd/boards/neo_trinkey/target/thumbv6m-none-eabi/release/examples/button"
Finished in 0.042s
```
Note: If hf2 can not find your Neokey Trinkey, you should check the Product ID (pid) and Vendor ID (vid) in your system
settings.

## Examples

### Blinky basic

```bash
$ cargo hf2 --release --example blinky --features leds --vid 0x239a --pid 0x00ff
```

Once the Neokey Trinkey has restarted, you will see the led blink.

**Warning** even though the lights are turned down very low, they are still very bright.

### Rainbow

```bash
$ cargo hf2 --release --example rainbow --features leds --vid 0x239a --pid 0x00ff
```

A slightly more satisfying version of blinky where the light will cycle through the color spectrum.

**Warning** even though the lights are turned down very low, they are still very bright.

### USB echo

```bash
$ cargo hf2 --release --example usb_echo --features usb --vid 0x239a --pid 0x00ff
```

Once the device has reset, all the lights will be off. You will then need to find the USB device on your machine.

Usually this is located in `/dev/cu.usbmodemTRINKEY_ECHO1`. though if you have multiple trinkeys plugged in and running
this example, the number at the end may change.

You can then send the USB device bytes. Each time the device receives data, it will respond with "Received: X" where X
is the data that it received. To test this in a variety of ways but the easiest is probably with screen.

Connect to the device like this (9600 is the baud rate)

```bash
$ screen /dev/cu.usbmodemTRINKEY_ECHO1 9600
```

You can now type, and the characters you type will appear on screen, but the magic here is that what's actually
happening is your key presses are being sent to the device, and the device is responding with the same data which is
what you see appearing on the screen.

To quit screen, use `ctrl-a` followed by `crtl-\` then `y`

### USB ack

```bash
$ cargo hf2 --release --example usb_ack --features usb --vid 0x239a --pid 0x00ff
```

This behaves similarly to the USB echo example above except that each time the device receives data, it will respond
with "Received: X" where X is the data that it received. This makes what's happening a little clearer.

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
