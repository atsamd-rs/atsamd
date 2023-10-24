# `wio-terminal` Examples

Various examples demonstrating the use of the Wio Terminal's peripherals.

## Getting Started

All commands below are executed from the project's root directory, ie. that containing `Cargo.toml`.

### Building

Build all examples, in developer mode:

```bash
$ cargo build --examples
```

Build one specific example, in release mode:

```bash
$ cargo build --release --example=blinky
```

### Flashing

Regardless of which method used, be sure to put your device in bootloader mode first by toggling the reset switch twice.

### Linux permissions

On Linux, you may wish to use udev to grant you access to the bootloader USB port,
without being root. You can do this by creating a file `/etc/udev/rules.d/99-wio-terminal.rules`
with the following contents:

```
ATTRS{idVendor}=="2886", ATTRS{idProduct}=="002d", ENV{ID_MM_DEVICE_IGNORE}="1"
SUBSYSTEM=="usb", ATTRS{idVendor}=="2886", ATTRS{idProduct}=="002d", MODE="0666"
SUBSYSTEM=="tty", ATTRS{idVendor}=="2886", ATTRS{idProduct}=="002d", MODE="0666"
```

And reloading your udev rules like this:

```shell
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### arm-none-eabi-objcopy & bossac

Make sure you have built the desired example in release mode first, as described above.

Convert the resulting file to a binary using `arm-none-eabi-objcopy` (make sure this is on your `PATH` first!), then flash to your device using BOSSA. Make sure you change `<PORT>` in the command to the appropriate value.

```bash
$ arm-none-eabi-objcopy \
>   -O binary \
>   /target/thumbv7em-none-eabihf/release/examples/blinky \
>   /target/thumbv7em-none-eabihf/release/examples/blinky.bin
> # alternatively, use cargo-binutils, you can `rust-objcopy` with the same flags, or combine it with the `cargo build` step and use `cargo objcopy`
$ bossac \
>   -i -d -v \
>   --port=<PORT> -U \
>   -w /target/thumbv7em-none-eabihf/release/examples/blinky.bin \
>   -R
```

### cargo-hf2

The [hf2-rs](https://github.com/jacobrosenthal/hf2-rs) project provides a crate, [cargo-hf2](https://github.com/jacobrosenthal/hf2-rs/tree/master/cargo-hf2), which allows us to build and flash all in one go using `cargo`:

```bash
$ cargo install cargo-hf2
$ cargo hf2 --release --example=blinky --vid 0x2886 --pid 0x002d
```

## Examples

### [`blinky`](blinky.rs)

Flash the (blue) User LED, located on the bottom of the device beside the USB port, at a set interval.

### [`buttons`](buttons.rs)

Demonstrate the use of the buttons and 5-way switch. Displays arrows indicating which button was pressed.

### [`orientation`](orientation.rs)

Display Ferris centered on the screen. Maintain the correct orientation of the image as determined by the accelerometer.

### [`usb_serial_display`](usb_serial_display.rs)

Makes the Wio Terminal appear as a USB serial port. The screen can be written to by sending messages down the serial port.

### [`snake`](snake.rs)
Generic snake game where you use the 5-way-switch to control the snake. 