//! Randomly choose and led and color to breath in and out
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer::{entry, hal, pac, Pins};

use core::f32::consts::FRAC_PI_2;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::trng::Trng;

use pac::{CorePeripherals, Peripherals};

use hal::ehal::delay::DelayNs;

use micromath::F32Ext;
use smart_leds::SmartLedsWrite;
use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    RGB8,
};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = Pins::new(peripherals.port).split();

    // neopixels
    let mut neopixel = pins.neopixel.init_spi(
        &mut clocks,
        // Unfortunately, the SPI driver requires a clock pin, even though it's not used by the
        // neopixels.
        pins.i2c.scl,
        peripherals.sercom2,
        &mut peripherals.mclk,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let trng = Trng::new(&mut peripherals.mclk, peripherals.trng);

    const NUM_LEDS: usize = 5;

    loop {
        let rand = trng.random_u8();
        let pos: usize = rand.wrapping_rem(5) as usize; //random led

        //slowly enable led
        for j in 0..255u8 {
            let _ = neopixel.write((0..NUM_LEDS).map(|i| {
                if i == pos {
                    hsv2rgb(Hsv {
                        hue: rand,
                        sat: 255,
                        val: sine_ease_in(j as f32, 0.0, 32.0, 255.0) as u8,
                    })
                } else {
                    RGB8::default()
                }
            }));
            delay.delay_ms(5);
        }

        //slowly disable led - note the reverse .rev()
        for j in (0..255u8).rev() {
            let _ = neopixel.write((0..NUM_LEDS).map(|i| {
                if i == pos {
                    hsv2rgb(Hsv {
                        hue: rand,
                        sat: 255,
                        val: sine_ease_in(j as f32, 0.0, 32.0, 255.0) as u8,
                    })
                } else {
                    RGB8::default()
                }
            }));
            delay.delay_ms(5);
        }
    }
}

#[inline]
// current step, where oputput starts, where output ends, last step
fn sine_ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
    -c * (t / d * FRAC_PI_2).cos() + c + b
}
