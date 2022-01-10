//! ADXL343 accelerometer example

#![no_std]
#![no_main]

use bsp::hal;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;
use trellis_m4 as bsp;
use ws2812_timer_delay as ws2812;

use hal::ehal::digital::v1_compat::OldOutputPin;

use bsp::entry;
use hal::adxl343::accelerometer::Accelerometer;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::{hsv::RGB8, SmartLedsWrite};

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
    let mut pins = bsp::Pins::new(peripherals.PORT).split();

    // neopixels
    let timer = SpinTimer::new(4);
    let neopixel_pin: OldOutputPin<_> = pins.neopixel.into_push_pull_output(&mut pins.port).into();
    let mut neopixels = ws2812::Ws2812::new(timer, neopixel_pin);

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
            RGB8::from(((ax3.x >> 8 & 0b11000000) as u8, 0, 0)),
            RGB8::from((0, (ax3.y >> 8 & 0b11000000) as u8, 0)),
            RGB8::from((0, 0, (ax3.x >> 8 & 0b11000000) as u8)),
        ];

        neopixels.write(colors.iter().cloned()).unwrap();

        delay.delay_ms(10u8);
    }
}
