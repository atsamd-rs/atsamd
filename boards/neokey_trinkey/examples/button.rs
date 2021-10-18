#![no_std]
#![no_main]

use neokey_trinkey as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;

#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use bsp::hal;
use hal::clock::GenericClockController;
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::timer::TimerCounter;

use smart_leds::{hsv::RGB8, SmartLedsWrite};
use ws2812_timer_delay::Ws2812;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = bsp::Pins::new(peripherals.PORT);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
    timer.start(3.mhz());
    let neo_pixel = pins.neo_pixel.into_push_pull_output();
    let button = pins.button.into_pull_down_input();
    let mut ws2812 = Ws2812::new(timer, neo_pixel);

    let off = [RGB8::default()];
    let on = [RGB8::new(0, 5, 5)];

    loop {
        if button.is_high().unwrap() {
            ws2812.write(on.iter().cloned()).unwrap();
        } else {
            ws2812.write(off.iter().cloned()).unwrap();
        }
    }
}
