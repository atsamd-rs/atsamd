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
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::{
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

    let mut pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core_peripherals.SYST, &mut clocks);
    let timer = SpinTimer::new(4);
    let neopixel_pin: OldOutputPin<_> = pins.neopixel.into_push_pull_output(&mut pins.port).into();
    let mut neopixel = ws2812::Ws2812::new(timer, neopixel_pin);
    let mut values = [RGB8::default(); bsp::NEOPIXEL_COUNT];

    loop {
        for j in 0..(256 * 5) {
            for (i, value) in values.iter_mut().enumerate() {
                *value = hsv2rgb(Hsv {
                    hue: (((i * 256) as u16 / bsp::NEOPIXEL_COUNT as u16 + j) & 255) as u8,
                    sat: 255,
                    val: 32,
                });
            }

            neopixel.write(values.iter().cloned()).unwrap();

            delay.delay_ms(5u8);
        }
    }
}
