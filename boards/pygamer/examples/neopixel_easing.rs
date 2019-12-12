//! Display light sensor reading on the neopixels.

#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;
use pygamer as hal;

use core::f32::consts::FRAC_PI_2;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::trng::Trng;
use micromath::F32Ext;
use smart_leds::hsv::RGB8;
use smart_leds::{brightness, SmartLedsWrite};

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

    let mut pins = hal::Pins::new(peripherals.PORT).split();
    let timer = SpinTimer::new(4);

    let mut neopixel = pins.neopixel.init(timer, &mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let trng = Trng::new(&mut peripherals.MCLK, peripherals.TRNG);

    const NUM_LEDS: usize = 5;

    loop {
        let rand = trng.random_u8();
        let pos: usize = rand.wrapping_rem(5) as usize; //random led
        let rgb = wheel(rand); //random color

        //slowly enable led
        for j in 0..255u8 {
            let _ = neopixel.write(brightness(
                (0..NUM_LEDS).map(|i| if i == pos { rgb } else { RGB8::default() }),
                // current step, start output led off=0, max output of only 32, 255 top
                sine_ease_in(j as f32, 0.0, 32.0, 255.0) as u8,
            ));
            delay.delay_ms(5u8);
        }

        //slowly disable led - note the reverse .rev()
        for j in (0..255u8).rev() {
            let _ = neopixel.write(brightness(
                (0..NUM_LEDS).map(|i| if i == pos { rgb } else { RGB8::default() }),
                // current step, start output led off=0, max output of only 32, 255 top
                sine_ease_in(j as f32, 0.0, 32.0, 255.0) as u8,
            ));
            delay.delay_ms(5u8);
        }
    }
}

#[inline]
// current step, where oputput starts, where output ends, last step
fn sine_ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    -c * (t / d * FRAC_PI_2).cos() + c + b
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
