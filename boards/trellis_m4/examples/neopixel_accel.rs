//! ADXL343 accelerometer example

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;
extern crate smart_leds;
extern crate trellis_m4 as hal;
extern crate ws2812_nop_samd51 as ws2812;

use hal::adxl343::accelerometer::Accelerometer;
use hal::prelude::*;
use hal::{clock::GenericClockController, delay::Delay};
use hal::{entry, CorePeripherals, Peripherals};

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
    let mut pins = hal::Pins::new(peripherals.PORT).split();

    // neopixels
    let neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixels = ws2812::Ws2812::new(neopixel_pin);

    // accelerometer
    let mut adxl343 = pins
        .accel
        .open(
            &mut clocks,
            peripherals.SERCOM2,
            &mut peripherals.MCLK,
            &mut pins.port,
        )
        .unwrap();

    loop {
        let ax3 = adxl343.acceleration().unwrap();

        // RGB indicators of current accelerometer state
        let colors = [
            Color::from(((ax3.x >> 8 & 0b11000000) as u8, 0, 0)),
            Color::from((0, (ax3.y >> 8 & 0b11000000) as u8, 0)),
            Color::from((0, 0, (ax3.x >> 8 & 0b11000000) as u8)),
        ];

        neopixels.write(colors.iter().cloned()).unwrap();

        delay.delay_ms(10u8);
    }
}
