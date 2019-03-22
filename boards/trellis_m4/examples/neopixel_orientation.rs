//! ADXL343 accelerometer-based orientation tracking example

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
use hal::orientation::{self, Orientation};

use smart_leds::{colors, SmartLedsWrite};

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

    // accelerometer-based orientation tracker
    let mut tracker = orientation::Tracker::new(accel.open(
        &mut clocks,
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut port
    ).unwrap());

    let mut last_orientation = Orientation::Up;

    loop {
        let mut colors = [colors::DEEP_SKY_BLUE; hal::NEOPIXEL_COUNT];
        let orientation = tracker.orientation().unwrap();

        let pointing_up = match orientation {
            Orientation::Up => true,
            Orientation::Down => false,
            Orientation::Unknown => {
                match last_orientation {
                    Orientation::Up => true,
                    Orientation::Down => false,
                    Orientation::Unknown => true
                }
            }
        };

        if orientation != Orientation::Unknown {
            last_orientation = orientation;
        }

        if pointing_up {
            for cell in &mut colors[(hal::NEOPIXEL_COUNT / 2)..] {
                *cell = colors::FOREST_GREEN;
            }
        } else {
            for cell in &mut colors[..(hal::NEOPIXEL_COUNT / 2)] {
                *cell = colors::FOREST_GREEN;
            }
        }

        let n = tracker.direction_avg();

        if n < 0.0 {
            colors[0] = (((-n) * 256.0) as u8, 0, 0).into();
        } else {
            colors[0] = (0, (n * 256.0) as u8, 0).into();
        }

        let (x_samples, y_samples, z_samples) = tracker.samples();
        let x_scaled = (256.0 * x_samples.corrected_mean()) as u8;
        let y_scaled = (256.0 * y_samples.corrected_mean()) as u8;
        let z_scaled = (256.0 * z_samples.corrected_mean()) as u8;

        colors[1] = (0, 0, x_scaled).into();
        colors[2] = (0, y_scaled, 0).into();
        colors[3] = (z_scaled, 0, 0).into();

        neopixels
            .write(colors.iter().cloned())
            .unwrap();

        delay.delay_ms(10u8);
    }
}
