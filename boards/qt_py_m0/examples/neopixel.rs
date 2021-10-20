#![no_std]
#![no_main]

//! Neopixel example for the Adafruit QT Py board. Demonstrates powering up the
//! neopixel using the attached GPIO line.
//!
//! *NOTE*: This example currently only works in release mode.

use hal::ehal::digital::v1_compat::OldOutputPin;
use panic_halt as _;
use smart_leds::hsv::hsv2rgb;
use smart_leds::hsv::Hsv;
use smart_leds::SmartLedsWrite;
use ws2812_timer_delay::Ws2812;

use bsp::entry;
use bsp::hal;
use bsp::Pins;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::CorePeripherals;
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::timer::TimerCounter;
use qt_py_m0 as bsp;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_8mhz(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = Pins::new(peripherals.PORT).split();

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
    timer.start(3.mhz());

    // The neopixel sources power from a GPIO pin. It must be driven high to power
    // up the neopixel before it can be used.
    pins.neopixel
        .power
        .into_push_pull_output()
        .set_high()
        .unwrap();

    let neopixel_data: OldOutputPin<_> = pins.neopixel.data.into_push_pull_output().into();
    let mut neopixel = Ws2812::new(timer, neopixel_data);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        for j in 0..255u8 {
            neopixel
                .write(
                    [hsv2rgb(Hsv {
                        hue: j,
                        sat: 255,
                        val: 16,
                    })]
                    .iter()
                    .cloned(),
                )
                .unwrap();
            delay.delay_ms(5u8);
        }
    }
}
