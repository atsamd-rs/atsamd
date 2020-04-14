# Arduino MKR Vidor 4000 Board Support Crate

This crate provides a type-safe API for working with the [Arduino MKR Vidor board](https://store.arduino.cc/usa/mkr-vidor-4000).

## Examples
### Blinky Basic
#### Requirements
 - Arduino IDE installed
    - samd package installed
    - Now the arduino distribution contains bossac.exe in `ArduinoData/packages/arduino/tools/bossac/1.7.0/` add it to your path
    - Probably best to install an example sketch via the IDE just to make sure everything is working
    - Note that the [arduino cli](https://github.com/arduino/arduino-cli) (or just regular bossac) may soon replace this section
 - arm-none-eabi tools installed, you need gcc and objcopy.
 - thumbv6m-none-eabi rust target installed via `rustup target add thumbv6m-none-eabi`

#### Steps

```bash
cargo build --release --example blinky_basic
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/examples/blinky_basic target/blinky_basic.bin
```

Then, press the reset button twice quickly on the board. The red LED should be fading in and out. Now you can flash the board.

```bash
bossac -i -d -U true -i -e -w -v target/blinky_basic.bin -R
```
