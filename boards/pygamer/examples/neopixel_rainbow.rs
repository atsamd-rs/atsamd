#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;
extern crate pygamer as hal;
extern crate smart_leds;
extern crate ws2812_nop_samd51 as ws2812;

use hal::prelude::*;
use hal::{clock::GenericClockController, delay::Delay};
use hal::{entry, CorePeripherals, Peripherals};

use smart_leds::brightness;
use smart_leds::Color;
use smart_leds::SmartLedsWrite;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    let neopixel_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    let mut neopixel = ws2812::Ws2812::new(neopixel_pin);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    const NUM_LEDS: usize = 1;
    let mut data = [Color::default(); NUM_LEDS];

    loop {
        for j in 0..(256 * 5) {
            for _ in 0..1 {
                for i in 0..NUM_LEDS {
                    data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
                }
            }
            neopixel
                .write(brightness(data.iter().cloned(), 32))
                .unwrap();
            delay.delay_ms(1u8);
        }
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> Color {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
