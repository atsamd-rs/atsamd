//! Left and right on 'joystick' controls the first neopixel while it is
//! automatically rotating through the color wheel
//! Select and Start control a second neopixel
//! When they overlap, joystick takes precedence
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

use edgebadge::{self as hal, entry, pac, pins::Keys, Pins};
use panic_halt as _;

use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};
use pac::{CorePeripherals, Peripherals};
use smart_leds::hsv::SmartLedsWrite;
use smart_leds::hsv::{hsv2rgb, Hsv, RGB8};

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
    let mut pins = Pins::new(peripherals.PORT).split();

    let mut buttons = pins.buttons.init(&mut pins.port);

    // neopixels
    let timer = SpinTimer::new(4);

    let mut neopixel = pins.neopixel.init(timer, &mut pins.port);

    const NUM_LEDS: usize = 5;
    let mut pos_joy: usize = 1;
    let mut pos_button: usize = 3;
    let mut color_button: u8 = 0;
    loop {
        for event in buttons.events() {
            match event {
                Keys::LeftDown => {
                    pos_joy = pos_joy.saturating_sub(1);
                }
                Keys::RightDown => {
                    if pos_joy < 4 {
                        pos_joy += 1;
                    }
                }
                Keys::BDown => {
                    pos_button = pos_button.saturating_sub(1);
                }
                Keys::ADown => {
                    if pos_button < 4 {
                        pos_button += 1;
                    }
                }
                _ => {}
            }
        }

        //finally paint the two leds at position, accel priority
        let _ = neopixel.write((0..NUM_LEDS).map(|i| {
            if i == pos_joy || i == pos_button {
                hsv2rgb(Hsv {
                    hue: color_button,
                    sat: 255,
                    val: 32,
                })
            } else {
                RGB8::default()
            }
        }));
        //incremement the wheel easing
        color_button = color_button.wrapping_add(1);

        delay.delay_ms(5u8);
    }
}
