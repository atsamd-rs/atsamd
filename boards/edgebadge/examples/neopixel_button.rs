//! Joystick y controls the color of a neopixel while Joystick x moves it
//! left and right around the center neopixel
//! Select and Start control a second neopixel left and right while it is
//! automatically rotating through the color wheel
//! When they overlap, joystick takes precedence

#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;

use edgebadge as hal;

use hal::adc::Adc;
use hal::entry;
use hal::pac::gclk::pchctrl::GEN_A::GCLK11;
use hal::pac::{CorePeripherals, Peripherals};
use hal::pins::Keys;
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::{brightness, hsv::RGB8, SmartLedsWrite};

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

        //put y in j for rainbow, turn 4095 into 255, /16
        use core::convert::TryInto;
        let color_joy: u8 = ((y - 1) / 16).try_into().unwrap();

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
        let _ = neopixel.write(brightness(
            (0..NUM_LEDS).map(|i| {
                if i == pos_joy {
                    wheel(color_joy)
                } else if i == pos_button {
                    wheel(color_button)
                } else {
                    RGB8::default()
                }
            }),
            32,
        ));

        //incremement the wheel easing
        color_button = color_button.wrapping_add(1);

        delay.delay_ms(5u8);
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
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
