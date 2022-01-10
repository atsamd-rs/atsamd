# WIO Lite W600 Board Support Crate

This crate provides a type-safe API for working with the WIO Lite W600(https://wiki.seeedstudio.com/Wio-Lite-W600/).

## Examples
### Requirements
 - Arduino IDE installed
   - You will also need the Wio Lite W600 Arduino tools, [follow the guide here](https://wiki.seeedstudio.com/Wio-Lite-W600/#software).
   - This library contains the version of bossac you will need in `Arduino15/packages/Seeeduino/tools/bossac/1.7.0[-arduino3]/`.
     **It's very important that you use _this_ version only**, as it is possible to lock yourself out of the board with
     the wrong one. You can find Arduino15 in the following locations:
       - **linux**: `Arduino15` is likely something like `~/.arduino15/`
       - **OSX**: `Arduino15` is likely something like `~/Library/Arduino15`
   - You can test that everything is working by installing an example sketch via the IDE
   - Note that the [arduino cli](https://github.com/arduino/arduino-cli) (or just regular bossac) may soon replace this
      section
 - arm-none-eabi tools installed, you need gcc and objcopy.
   -  **Note**: Alternatively, you can use [cargo-binutils](https://github.com/rust-embedded/cargo-binutils), which is
      likely easier to install on OSX and also easier to use, as it will automatically detect the target
 - thumbv6m-none-eabi rust target installed via `rustup target add thumbv6m-none-eabi`

To build your example, you can either use `arm-none-eabi-objcopy` or `cargo-binutils`:

If using `arm-none-eabi-objcopy`

```bash
$ cargo build --release --example blinky_basic
$ arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/examples/blinky_basic target/blinky_basic.bin
```

Or, if using  `cargo-binutils`

```bash
$ cargo objcopy --example blinky_basic --release -- -O binary target/blinky_basic.bin
```

For the purposes of simplicity, the documentation below will use `cargo-binutils`

To copy the example across, first you need to find the port the Wio Lite W600 is attached to. This can be done easily in
the Arduino IDE or on nix based systems, you can find it inside the `/dev` directory. Note, it might not show up, until
you put the device into bootloader mode by double tapping the reset button on the board.

For example, if your device attaches to `/dev/cu.usbmodem14601` you will need `cu.usbmodem14601` below.

You will also need to put the device into bootloader mode before copying the file by double tapping the reset button.
Using the bossac you found above, with the previously found port that the device is attached to.

Example (your install location may vary):

```bash
~/Library/Arduino15/packages/Seeeduino/tools/bossac/1.7.0-arduino3/bossac --port=cu.usbmodem14601 -idewvRU target/blinky_basic.bin
```

### Example - Blinky Basic

#### Wiring

The onboard LED is attached to the W600 module, so blinky_basic uses pin D13 instead. In order to see the example work
you must attach and LED and appropriate resistor to this pin.

c

Build the example
```bash
$ cargo objcopy --example blinky_basic --release -- -O binary target/blinky_basic.bin
```

Connect the device and find what port it has connected using the steps above. Put the device into bootloader mode and
then use Seeeduino's version of boassac to copy the file over.

```bash
~/Library/Arduino15/packages/Seeeduino/tools/bossac/1.7.0-arduino3/bossac --port=cu.usbmodem14601 -idewvRU target/blinky_basic.bin
```

### Example - USB ack

#### Steps

Build the example

```bash
$ cargo objcopy --example usb_ack --release -- -O binary target/usb_ack.bin
```

Connect the device and find what port it has connected using the steps above. Put the device into bootloader mode and
then use Seeeduino's version of boassac to copy the file over.

```bash
~/Library/Arduino15/packages/Seeeduino/tools/bossac/1.7.0-arduino3/bossac --port=cu.usbmodem14601 -idewvRU target/usb_ack.bin
```

Usually this is located in `/dev/cu.usbmodemWIO_LITE_W6001`. though if you have multiple wio_lite_w600s plugged in and
running this example, the last number may change.

You can then send the USB device bytes. Each time the device receives data, it will respond with "Received: X" where X
is the data that it received. To test this in a variety of ways but the easiest is probably with screen.

Connect to the device like this (9600 is the baud rate)

```bash
$ screen /dev/cu.usbmodemWIO_LITE_W6001 9600
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
