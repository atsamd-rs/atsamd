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
use hal::ehal::digital::v2::InputPin;

use bsp::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::{
    brightness, colors,
    hsv::{hsv2rgb, Hsv, RGB8},
    SmartLedsWrite,
};

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

    let mut pins = bsp::Pins::new(peripherals.PORT).split();

    // neopixels
    let timer = SpinTimer::new(4);
    let neopixel_pin: OldOutputPin<_> = pins.neopixel.into_push_pull_output(&mut pins.port).into();
    let mut neopixel = ws2812::Ws2812::new(timer, neopixel_pin);
    let mut color_values = [RGB8::default(); bsp::NEOPIXEL_COUNT];

    // keypad
    let keypad = bsp::Keypad::new(pins.keypad, &mut pins.port);
    let keypad_inputs = keypad.decompose();
    let mut keypad_state = [false; bsp::NEOPIXEL_COUNT];
    let mut toggle_values = [false; bsp::NEOPIXEL_COUNT];

    loop {
        for j in 0..(256 * 5) {
            for (i, value) in color_values.iter_mut().enumerate() {
                let keypad_column = i % 8;
                let keypad_row = i / 8;
                let keypad_button: &dyn InputPin<Error = ()> =
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
                    hsv2rgb(Hsv {
                        hue: (((i * 256) as u16 / bsp::NEOPIXEL_COUNT as u16 + j) & 255) as u8,
                        sat: 255,
                        val: 255, //brightness is lowered globally later
                    })
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
