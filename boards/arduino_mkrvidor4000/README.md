# Arduino MKR Vidor 4000 Board Support Crate

This crate provides a type-safe API for working with the [Arduino MKR Vidor board](https://store.arduino.cc/usa/mkr-vidor-4000).

## Examples
### Blinky Basic
#### Requirements
 - Arduino IDE installed
    - samd package installed (You can do this by going to Tools->Board->BoardManager and then searching for `samd`
    - Now the arduino distribution contains bossac.exe in `ArduinoData/packages/arduino/tools/bossac/1.7.0[-arduino3]/` add it to your path
       - **linux**: `ArduinoData` is likely something like `~/.arduino15/`
       - **OSX**: `ArduinoData` is likely something like `~/Library/Arduino15`
    - Now the arduino distribution contains bossac.exe in `ArduinoData/packages/arduino/tools/bossac/1.7.0/` add it to your path
    - Probably best to install an example sketch via the IDE just to make sure everything is working
    - Note that the [arduino cli](https://github.com/arduino/arduino-cli) (or just regular bossac) may soon replace this section
 - arm-none-eabi tools installed, you need gcc and objcopy.
   -  **Note**: Alternatively, you can use [cargo-binutils](https://github.com/rust-embedded/cargo-binutils), which is likely easier to install on OSX and also easier to use, as it will automatically detect the target
 - thumbv6m-none-eabi rust target installed via `rustup target add thumbv6m-none-eabi`

#### Steps

```bash
cargo build --release --example blinky_basic
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/examples/blinky_basic target/blinky_basic.bin
# if using cargo-binutils, you can `rust-objcopy` with the same flags, or combine the previous 2 steps with `cargo objcopy`
```

Then, press the reset button twice quickly on the board. The red LED should be fading in and out. Now you can flash the board.

```bash
bossac -i -d -U true -i -e -w -v target/blinky_basic.bin -R
```
