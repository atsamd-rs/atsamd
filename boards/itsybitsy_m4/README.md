# Adafruit ItsyBitsy M4 Express Board Support Crate

This crate provides a type-safe Rust API for working with the
[Adafruit ItsyBitsy M4 Express board](https://www.adafruit.com/product/3800).

## Board Features

- Microchip [ATSAMD51G] Cortex-M4 microcontroller @ 120 MHz (32-bit, 3.3V logic and power)
  - 512kB Flash
  - 192kB SRAM
- 2 MB SPI Flash chip

## Prerequisites

- Install the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
- Install [cargo-hf2 the hf2 bootloader flasher tool](https://crates.io/crates/cargo-hf2) however your platform requires

## Uploading an example

Check out the repository for examples:

<https://github.com/atsamd-rs/atsamd/tree/master/boards/itsybitsy_m4/examples>

- Be in this directory `cd boards/itsybitsy_m4`
- Put your device in bootloader mode usually by hitting the reset button twice.
- Build and upload in one step
  
```Shell
$ cargo hf2 --release --example blinky_basic
    Finished release [optimized + debuginfo] target(s) in 0.19s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("PyBadge"))
    Flashing "/Users/User/atsamd/boards/itsybitsy_m4/target/thumbv7em-none-eabihf/release/examples/blinky_basic"
    Finished in 0.079s
$
```

Note some examples will tell you they need more features enabled

```Shell
$ cargo hf2 --release --example usb_serial
error: target `usb_serial` in package `itsybitsy_m4` requires the features: `usb`
Consider enabling them by passing, e.g., `--features="usb"`
```

Just follow the instructions to add --features like

```Shell
cargo hf2 --release --example usb_serial --features="usb"
    Finished release [optimized + debuginfo] target(s) in 0.09s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("PyBadge"))
    Flashing "/Users/User/atsamd/boards/itsybitsy_m4/target/thumbv7em-none-eabihf/release/examples/usb_serial"
    Finished in 0.167s
$
```

If you are on Linux and hf2 fails to flash your board even if it is connected and in bootloader mode, you
might need to add some `udev` rules if you have not done that yet.

The example for adafruit boards is shown [here](https://crates.io/crates/hf2).

You might want to have all the hf2 related rules in a single file, i.e. `/etc/udev/rules.d/99-hf2-boards.rules`, or have
a different rules file for each vendor. The rules for Adafruit boards look like this:

```Shell
#adafruit rules
ATTRS{idVendor}=="239a", ENV{ID_MM_DEVICE_IGNORE}="1"
SUBSYSTEM=="usb", ATTRS{idVendor}=="239a", MODE="0666"
SUBSYSTEM=="tty", ATTRS{idVendor}=="239a", MODE="0666"
```

After adding the rules remember to reboot or run:

```Shell
sudo udevadm control --reload-rules
sudo udevadm trigger
```

For more information on hf2 and other methods of uploading your code, take a look at
the base project [README](https://github.com/atsamd-rs/atsamd).
