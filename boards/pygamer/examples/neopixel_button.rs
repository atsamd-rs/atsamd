//! Joystick y controls the color of a neopixel while Joystick x moves it
//! left and right around the center neopixel
//! Select and Start control a second neopixel left and right while it is
//! automatically rotating through the color wheel
//! When they overlap, joystick takes precedence

#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;
use pygamer as hal;

use hal::adc::Adc;
use hal::entry;
use hal::pac::gclk::pchctrl::GEN_A::GCLK11;
use hal::pac::{CorePeripherals, Peripherals};
use hal::pins::Keys;
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::{
    hsv::{hsv2rgb, Hsv, RGB8},
    SmartLedsWrite,
};

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

    let mut buttons = pins.buttons.init(&mut pins.port);

    let mut adc1 = Adc::adc1(peripherals.ADC1, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut joystick = pins.joystick.init(&mut pins.port);

    // neopixels
    let timer = SpinTimer::new(4);

    let mut neopixel = pins.neopixel.init(timer, &mut pins.port);

    const NUM_LEDS: usize = 5;
    let mut pos_button: usize = 2;
    let mut color_button: u8 = 0;
    loop {
        let (x, y) = joystick.read(&mut adc1);

        //put y in j for rainbow, map 4095 into 255
        let color_joy = map_from((0, 4095), (0, 255), y);

        let pos_joy: usize = if x < 147 {
            0
        } else if (x >= 147) && (x < 1048) {
            1
        } else if (x >= 1048) && (x < 3048) {
            2
        } else if (x >= 3048) && (x < 3948) {
            3
        } else {
            4
        };

        for event in buttons.events() {
            match event {
                Keys::SelectDown => {
                    if pos_button > 0 {
                        pos_button -= 1;
                    }
                }
                Keys::StartDown => {
                    if pos_button < 4 {
                        pos_button += 1;
                    }
                }
                _ => {}
            }
        }

        //finally paint the two leds at position, accel priority
        let _ = neopixel.write((0..NUM_LEDS).map(|i| {
            if i == pos_joy {
                hsv2rgb(Hsv {
                    hue: color_joy,
                    sat: 255,
                    val: 32,
                })
            } else if i == pos_button {
                hsv2rgb(Hsv {
                    hue: color_button,
                    sat: 255,
                    val: 32,
                })
            } else {
                RGB8::default()
            }
        }));

        //incremement the hue easing
        color_button = color_button.wrapping_add(1);

        delay.delay_ms(5u8);
    }
}

fn map_from(from_range: (u16, u16), to_range: (u16, u16), input: u16) -> u8 {
    debug_assert!(from_range.0 < from_range.1);
    debug_assert!(to_range.0 < to_range.1);
    debug_assert!(input <= from_range.1);

    let from: f32 = (from_range.1 - from_range.0).into();
    let to: f32 = (to_range.1 - to_range.0).into();
    ((input - from_range.0) as f32 / from * to + to_range.0 as f32) as u8
}
