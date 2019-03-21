//! ADXL343 accelerometer example

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;
extern crate smart_leds;
extern crate trellis_m4 as hal;
extern crate ws2812_nop_samd51 as ws2812;

use hal::prelude::*;
use hal::{entry, Peripherals, CorePeripherals};
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::{Color, SmartLedsWrite};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core_peripherals = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );


    let mut delay = Delay::new(core_peripherals.SYST, &mut clocks);
    let hal::pins::Sets { neopixel, accel, mut port, .. } = hal::Pins::new(peripherals.PORT).split();

    // neopixels
    let neopixel_pin = neopixel.into_push_pull_output(&mut port);
    let mut neopixels = ws2812::Ws2812::new(neopixel_pin);

    // accelerometer
    let mut adxl343 = accel.open(
        &mut clocks,
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut port
    ).unwrap();

    loop {
        let ax3 = adxl343.xyz().unwrap();

        // RGB indicators of current accelerometer state
        let colors = [
            Color::from(((ax3.x >> 8 & 0b11000000) as u8, 0, 0)),
            Color::from((0, (ax3.y >> 8 & 0b11000000) as u8, 0)),
            Color::from((0, 0, (ax3.x >> 8 & 0b11000000) as u8)),
        ];

        neopixels
            .write(colors.iter().cloned())
            .unwrap();

        delay.delay_ms(10u8);
    }
}
