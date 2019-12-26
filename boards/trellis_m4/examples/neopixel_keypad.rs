#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;
use trellis_m4 as hal;
use ws2812_nop_samd51 as ws2812;

use embedded_hal::digital::v1_compat::OldOutputPin;
use embedded_hal::digital::v2::InputPin;

use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::brightness;
use smart_leds::SmartLedsWrite;
use smart_leds::{colors, Color};

/// Main entrypoint
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
    let neopixel_pin: OldOutputPin<_> = pins.neopixel.into_push_pull_output(&mut pins.port).into();
    let mut neopixel = ws2812::Ws2812::new(neopixel_pin);
    let mut color_values = [Color::default(); hal::NEOPIXEL_COUNT];

    // keypad
    let keypad = hal::Keypad::new(pins.keypad, &mut pins.port);
    let keypad_inputs = keypad.decompose();
    let mut keypad_state = [false; hal::NEOPIXEL_COUNT];
    let mut toggle_values = [false; hal::NEOPIXEL_COUNT];

    loop {
        for j in 0..(256 * 5) {
            for (i, value) in color_values.iter_mut().enumerate() {
                let keypad_column = i % 8;
                let keypad_row = i / 8;
                let keypad_button: &InputPin<Error = ()> =
                    &keypad_inputs[keypad_row][keypad_column];

                if keypad_button.is_high().unwrap() {
                    keypad_state[i] = true;
                } else {
                    // toggle event
                    if keypad_state[i] == true {
                        keypad_state[i] = false;
                        toggle_values[i] = !toggle_values[i];
                    }
                }

                *value = if toggle_values[i] {
                    wheel((((i * 256) as u16 / hal::NEOPIXEL_COUNT as u16 + j) & 255) as u8)
                } else {
                    colors::GHOST_WHITE
                };
            }

            neopixel
                .write(brightness(color_values.iter().cloned(), 32))
                .unwrap();

            delay.delay_ms(5u8);
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
