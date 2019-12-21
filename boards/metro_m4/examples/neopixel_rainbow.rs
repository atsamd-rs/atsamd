#![no_std]
#![no_main]

extern crate cortex_m;
extern crate embedded_hal;
extern crate metro_m4 as hal;
extern crate panic_halt;
extern crate smart_leds;
extern crate ws2812_timer_delay as ws2812;

use embedded_hal::digital::v1_compat::OldOutputPin;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::{clock::GenericClockController, delay::Delay, timer::TimerCounter};

use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
};

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

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);
    timer.start(3_000_000u32.hz());

    let mut neopixel_pin: OldOutputPin<_> =
        pins.neopixel.into_push_pull_output(&mut pins.port).into();
    let mut neopixel = ws2812::Ws2812::new(timer, &mut neopixel_pin);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        for j in 0..255u8 {
            let colors = [hsv2rgb(Hsv {
                hue: j,
                sat: 255,
                val: 32,
            })];
            neopixel.write(colors.iter().cloned()).unwrap();
            delay.delay_ms(5u8);
        }
    }
}
