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
    let (mut clocks, led, _usb) = OurBoard::init(peripherals);

    let mut red_led = led;
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);
    loop {
        red_led.set_low().unwrap();
        delay.delay_ms(200_u8);
        red_led.set_high().unwrap();
        delay.delay_ms(200_u8);
    }
}
