#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;
extern crate sodaq_one as hal;

#[cfg(feature = "use_semihosting")]
use cortex_m_semihosting::hprintln;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};

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
    let mut red_led = pins.led_red.into_open_drain_output(&mut pins.port);
    let mut green_led = pins.led_green.into_open_drain_output(&mut pins.port);
    let mut blue_led = pins.led_blue.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let time = 1000u16;
    #[cfg(feature = "use_semihosting")]
    hprintln!("startup").unwrap();
    loop {
        #[cfg(feature = "use_semihosting")]
        hprintln!("loop").unwrap();
        red_led.set_low().unwrap();
        green_led.set_high().unwrap();
        blue_led.set_high().unwrap();
        delay.delay_ms(time);
        red_led.set_high().unwrap();
        green_led.set_low().unwrap();
        blue_led.set_high().unwrap();
        delay.delay_ms(time);
        red_led.set_high().unwrap();
        green_led.set_high().unwrap();
        blue_led.set_low().unwrap();
        delay.delay_ms(time);
    }
}
