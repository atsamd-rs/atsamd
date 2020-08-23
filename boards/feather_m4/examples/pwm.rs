#![no_std]
#![no_main]

// Pulse Width Modulation
//
// cargo build --features="unproven"

extern crate panic_halt;
extern crate cortex_m_rt;
extern crate feather_m4 as hal;

use hal::clock::GenericClockController;
use hal::pac::{Peripherals, CorePeripherals};
use hal::prelude::*;
use hal::delay::Delay;
use hal::pwm::Pwm4;
use cortex_m_rt::entry;

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
    let mut pins = hal::Pins::new(peripherals.PORT);
    let red_led = pins.d13.into_function_e(&mut pins.port);
    
    let gclk0 = clocks.gclk0();
    let mut pwm4 = Pwm4::new(
        &clocks.tc4_tc5(&gclk0).unwrap(),
        1.khz(),
        peripherals.TC4,
        hal::pwm::TC4Pinout::Pa23(red_led),
        &mut peripherals.MCLK,
    );
    let max_duty = pwm4.get_max_duty();
    
    // The red led will light up and toggle between 2 brightness
    // levels every second.
    loop {
        pwm4.set_duty(max_duty / 2);
        delay.delay_ms(1000u16);
        pwm4.set_duty(max_duty / 8);
        delay.delay_ms(1000u16);
        
    }
}