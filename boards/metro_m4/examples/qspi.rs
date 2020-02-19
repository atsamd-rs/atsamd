#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate metro_m4 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::qspi::Qspi;
use hal::qspi::Command;

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

    let mut pins = hal::Pins::new(peripherals.PORT);

    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let qspi = Qspi::new(
        &mut peripherals.MCLK, 
        &mut pins.port,
        peripherals.QSPI, 
        pins.flash_sck,
        pins.flash_cs,
        pins.flash_io0,
        pins.flash_io1,
        pins.flash_io2,
        pins.flash_io3,
    );
    
    qspi.erase_command(Command::EraseChip, 0x0).unwrap();
    let write_buf = [ 0x0d, 0xd0, 0x01, 0xc0 ]; // cool dude
    qspi.write_memory(0x1337, &write_buf);
    let mut read_buf = [0u8; 4];
    qspi.read_memory(0x1337, &mut read_buf);
    
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    if read_buf == write_buf {
        red_led.set_high().unwrap();
    } else {
        loop {
            red_led.set_high().unwrap();
            delay.delay_ms(250u8);
            red_led.set_low().unwrap();
            delay.delay_ms(250u8);
        }
    }
    loop { continue; }
}
