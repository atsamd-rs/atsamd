#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use sodaq_one as bsp;

#[cfg(feature = "use_semihosting")]
use cortex_m_semihosting::hprintln;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
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
    let mut pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led = pins.led_red.into_open_drain_output(&mut pins.port);
    let mut green_led = pins.led_green.into_open_drain_output(&mut pins.port);
    let mut blue_led = pins.led_blue.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let time = 1000u16;
    #[cfg(feature = "use_semihosting")]
    hprintln!("startup");
    loop {
        #[cfg(feature = "use_semihosting")]
        hprintln!("loop");
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
