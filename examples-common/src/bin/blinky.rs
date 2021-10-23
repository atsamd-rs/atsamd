#![no_std]
#![no_main]

use bsp::hal;
use bsp::pac;
use examples_common::*;

use hal::prelude::*;

#[bsp::entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let (pins, mut clocks) = OurBoard::init(peripherals);

    let mut red_led = pins.d13.into_push_pull_output();
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    loop {
        red_led.set_low().unwrap();
        delay.delay_ms(250_u16);
        red_led.set_high().unwrap();
        delay.delay_ms(500_u16);
    }
}
