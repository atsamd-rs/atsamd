#![no_std]
#![no_main]

/// Demonstrates reading and writing to the onboard 4MB flash.
/// The entire chip is erased, some data written, and then read back.
/// The Blue LED blink incessantly if the data written + read back
/// was not the same.
use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::qspi::Command;
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let mut user_led = sets.user_led.into_open_drain_output(&mut sets.port);
    user_led.set_high().unwrap();

    let mut flash = sets
        .flash
        .init(&mut peripherals.MCLK, &mut sets.port, peripherals.QSPI);

    // We don't know the current state of the chip, so lets chill out and
    // reset it.
    delay.delay_ms(20u8);
    wait_ready(&mut flash);
    flash.run_command(Command::EnableReset).unwrap();
    flash.run_command(Command::Reset).unwrap();
    delay.delay_ms(50u8);

    // WARP SPEEEED (133Mhz = divide by 1 (plus one)).
    flash.set_clk_divider(2);

    // Enable Quad SPI mode.
    wait_ready(&mut flash);
    flash.run_command(Command::WriteEnable).unwrap();
    flash
        .write_command(Command::WriteStatus, &[0x00, 0x02])
        .unwrap();

    // Erase this bad boi.
    wait_ready(&mut flash);
    flash.run_command(Command::WriteEnable).unwrap();
    flash.erase_command(Command::EraseChip, 0x0).unwrap();

    let write_buf = [0x0d, 0xd0, 0x01, 0xc0];
    wait_ready(&mut flash);
    flash.write_memory(0, &write_buf);

    let mut read_buf = [0u8; 4];
    wait_ready(&mut flash);
    flash.read_memory(0, &mut read_buf);

    if read_buf != write_buf {
        // If we did not read back the same data flash the status
        // LED.
        loop {
            user_led.toggle();
            delay.delay_ms(200u8);
        }
    }

    user_led.set_low().unwrap();
    loop {}
}

/// Wait for the write-in-progress and suspended write/erase.
fn wait_ready(flash: &mut wio::hal::qspi::Qspi) {
    while flash_status(flash, Command::ReadStatus) & 0x01 != 0 {}
    while flash_status(flash, Command::ReadStatus2) & 0x80 != 0 {}
}

/// Returns the contents of the status register indicated by cmd.
fn flash_status(flash: &mut wio::hal::qspi::Qspi, cmd: Command) -> u8 {
    let mut out = [0u8; 1];
    flash.read_command(cmd, &mut out).ok().unwrap();
    out[0]
}
