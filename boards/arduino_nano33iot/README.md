# Arduino Nano 33 IOT Board Support Crate

This crate provides a type-safe API for working with the [Arduino nano 33 IOT board](https://store.arduino.cc/usa/nano-33-iot).

## Examples
### Blinky Basic
#### Requirements
 - Arduino IDE installed
    - samd package installed
    - Now the arduino distribution contains bossac.exe in `ArduinoData/packages/arduino/tools/bossac/1.7.0/` add it to your path
       - `ArduinoData` is likely something like `~/.arduino15/`
    - Probably best to install an example sketch via the IDE just to make sure everything is working
    - Note that the [arduino cli](https://github.com/arduino/arduino-cli) (or just regular bossac) may soon replace this section
 - arm-none-eabi tools installed, you need gcc and objcopy.
 - thumbv6m-none-eabi rust target installed via `rustup target add thumbv6m-none-eabi`

#### Steps
```bash
cargo build --release --example blinky_basic
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/examples/blinky_basic target/blinky_basic.bin
bossac -i -d -U true -i -e -w -v target/blinky_basic.bin -R
```
(You may need to use `--port` with something like `/dev/ttyACM0` or `/dev/ttyACM1`

#### Notes
 - It may help to double-press the center button to reset when re-flashing the device. This sets the device in a bootloader mode.
 - For the usb example, `picocom` is a good simple terminal serial emulator

