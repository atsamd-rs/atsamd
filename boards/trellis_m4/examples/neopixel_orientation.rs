//! ADXL343 accelerometer-based orientation tracking example

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
use hal::adxl343::accelerometer::Orientation;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};
use smart_leds::{colors, hsv::RGB8, SmartLedsWrite};

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
    let adxl343 = pins
        .accel
        .open(
            &mut clocks,
            peripherals.SERCOM2,
            &mut peripherals.MCLK,
            &mut pins.port,
        )
        .unwrap();

    let mut accel_tracker = adxl343.try_into_tracker().unwrap();

    loop {
        // update tracker's internal `last_orientation`
        accel_tracker.orientation().unwrap();
        neopixels
            .write(
                colors_for_orientation(accel_tracker.last_orientation())
                    .iter()
                    .cloned(),
            )
            .unwrap();
        delay.delay_ms(10u8);
    }
}

fn colors_for_orientation(orientation: Orientation) -> [RGB8; bsp::NEOPIXEL_COUNT] {
    let mut colors = [colors::DEEP_SKY_BLUE; bsp::NEOPIXEL_COUNT];
    let green = colors::FOREST_GREEN;

    match orientation {
        Orientation::FaceUp | Orientation::Unknown => (),
        Orientation::FaceDown => {
            for cell in &mut colors {
                *cell = green;
            }
        }
        Orientation::PortraitUp => {
            for row in 0..4 {
                for column in 0..4 {
                    colors[row * 8 + column] = green;
                }
            }
        }
        Orientation::PortraitDown => {
            for row in 0..4 {
                for column in 4..8 {
                    colors[row * 8 + column] = green;
                }
            }
        }
        Orientation::LandscapeUp => {
            for cell in &mut colors[(bsp::NEOPIXEL_COUNT / 2)..] {
                *cell = green;
            }
        }
        Orientation::LandscapeDown => {
            for cell in &mut colors[..(bsp::NEOPIXEL_COUNT / 2)] {
                *cell = green;
            }
        }
    }

    colors
}
