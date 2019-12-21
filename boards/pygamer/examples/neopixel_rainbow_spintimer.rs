#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;
use pygamer as hal;

use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};

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

    let mut pins = hal::Pins::new(peripherals.PORT).split();
    let timer = SpinTimer::new(4);

    let mut neopixel = pins.neopixel.init(timer, &mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    const NUM_LEDS: usize = 5;

    loop {
        for j in 0..255u8 {
            let _ = neopixel.write((0..NUM_LEDS).map(|i| {
                //could have all leds be same color with number = j
                //instead lets offset each of them by 255/5 or 51
                hsv2rgb(Hsv {
                    hue: j.wrapping_add(51 * i as u8),
                    sat: 255,
                    val: 32,
                })
            }));

            delay.delay_ms(5u8);
        }
    }
}
