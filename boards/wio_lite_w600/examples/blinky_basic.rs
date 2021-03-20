#![no_std]
#![no_main]

extern crate wio_lite_w600 as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut d13 = pins.d13.into_open_drain_output(&mut pins.port);
    let mut d12 = pins.d12.into_open_drain_output(&mut pins.port);
    let mut d11 = pins.d11.into_open_drain_output(&mut pins.port);
    let mut d10 = pins.d10.into_open_drain_output(&mut pins.port);
    let mut d9 = pins.d9.into_open_drain_output(&mut pins.port);
    let mut d6 = pins.d6.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        d13.set_high().unwrap();
        delay.delay_ms(100u16);
        d13.set_low().unwrap();

        d12.set_high().unwrap();
        delay.delay_ms(100u16);
        d12.set_low().unwrap();

        d11.set_high().unwrap();
        delay.delay_ms(100u16);
        d11.set_low().unwrap();

        d10.set_high().unwrap();
        delay.delay_ms(100u16);
        d10.set_low().unwrap();

        d9.set_high().unwrap();
        delay.delay_ms(100u16);
        d9.set_low().unwrap();

        d6.set_high().unwrap();
        delay.delay_ms(100u16);
        d6.set_low().unwrap();
    }
}
