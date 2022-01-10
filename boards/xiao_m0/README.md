# Seeeduino XIAO Board Support Crate

This crate provides a type-safe API for working with the [Seeed Studio
Seeeduino XIAO](http://wiki.seeedstudio.com/Seeeduino-XIAO/).

## Prerequisites

* Install the cross compile toolchain `rustup target add thumbv6m-none-eabi`
* Install the [cargo-hf2 tool](https://crates.io/crates/cargo-hf2) however your
  platform requires

## Uploading an example

Check out [the
repository](https://github.com/atsamd-rs/atsamd/tree/master/boards/xiao_m0/examples)
for examples.

* Be in this directory `cd boards/xiao_m0`
* Put your device in bootloader mode by bridging the `RST` pads _twice_ in
  quick succession. The orange LED will pulse when the device is in bootloader
  mode.
* Build and upload in one step: `cargo hf2 --release --example blink --features="unproven"`
  * Note that if you're using an older `cargo-hf2` that you'll need to specify
    the VID/PID when flashing: `cargo hf2 --vid 0x2886 --pid 0x002f --release
    --example blink --features="unproven"`

If you are on Linux and hf2 fails to flash your board even if it is connected and in bootloader mode, you
might need to add some `udev` rules if you have not done that yet.

The example for adafruit boards is shown [here](https://crates.io/crates/hf2).

You might want to have all the hf2 related rules in a single file, i.e. `/etc/udev/rules.d/99-hf2-boards.rules`, or have
a different rules file for each vendor. The rules for Seeeduino boards look like this:

```Shell
#seeeduino rules
ATTRS{idVendor}=="2886", ENV{ID_MM_DEVICE_IGNORE}="1"
SUBSYSTEM=="usb", ATTRS{idVendor}=="2886", MODE="0666"
SUBSYSTEM=="tty", ATTRS{idVendor}=="2886", MODE="0666"
```

After adding the rules remember to reboot or run:

```Shell
sudo udevadm control --reload-rules
sudo udevadm trigger
```

For more information on hf2 and other methods of uploading your code, take a look at
the base project [README](https://github.com/atsamd-rs/atsamd).
