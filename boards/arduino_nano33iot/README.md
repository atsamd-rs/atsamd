# Arduino Nano 33 IOT Board Support Crate

This crate provides a type-safe API for working with the [Arduino nano 33 IOT board](https://store.arduino.cc/usa/nano-33-iot).

## Requirements

1. Arduino IDE or [arduino CLI](https://github.com/arduino/arduino-cli) installed.
2. `samd` package installed. You can do this by going to Tools->Board->BoardManager and then searching for `samd` or run `arduino-cli core install arduino:samd`.
3. Now the arduino distribution contains bossac.exe in `ArduinoData/packages/arduino/tools/bossac/1.7.0[-arduino3]/` add it to your path
  - **linux**: `ArduinoData` is likely something like `~/.arduino15/`
  - **OSX**: `ArduinoData` is likely something like `~/Library/Arduino15`
4. Probably best to install an example sketch via the IDE just to make sure everything is working.
5. `arm-none-eabi` tools installed, you need `gcc` and `objcopy`.
  -  **Note**: Alternatively, you can use [cargo-binutils](https://github.com/rust-embedded/cargo-binutils), which is likely easier to install on OSX and also easier to use, as it will automatically detect the target
6. `thumbv6m-none-eabi` rust target installed via `rustup target add thumbv6m-none-eabi`. Some features may also require nightly rust.

## Steps
```bash
cargo build --release --example blinky_basic
# If using cargo-binutils, you can `rust-objcopy` with the same flags, or combine these 2 steps with `cargo objcopy`
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/examples/blinky_basic target/blinky_basic.bin

# If using bossac
bossac -i -d -U true -i -e -w -v target/blinky_basic.bin -R

# If using arduino-cli
arduino-cli upload -i target/blinky_basic.bin -b arduino:samd:nano_33_iot -p /dev/ttyACM0
```
(You may need to use `--port` with something like `/dev/ttyACM0`/`/dev/ttyACM1`, or `/dev/tty.usbmodemNNNNN` on OSX)

## Notes
 - It may help to double-press the center button to reset when re-flashing the device. This sets the device in a bootloader mode.
 - For the usb example, `picocom` is a good simple terminal serial emulator, installable with your os's package manager or `brew`
   - On OSX, after flashing the tty for serial communication may be different, for example `/dev/tty.usbmodemTEST1`
