#![no_std]
#![no_main]

// Neopixel Rainbow
// This only functions when the --release version is compiled. Using the debug
// version leads to slow pulse durations which results in a straight white LED
// output.
//
// // Needs to be compiled with --release for the timing to be correct

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use circuit_playground_express as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::prelude::*;
use hal::timer::TimerCounter;
use pac::{CorePeripherals, Peripherals};

use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
};
use ws2812_timer_delay as ws2812;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
    timer.start(3.mhz());

    let neopixel_pin: bsp::NeoPixel = pins.d8.into();
    let mut neopixel = ws2812::Ws2812::new(timer, neopixel_pin);

    // Loop through all of the available hue values (colors) to make a
    // rainbow effect from the onboard neopixel
    loop {
        for j in 0..255u8 {
            let colors = [hsv2rgb(Hsv {
                hue: j,
                sat: 255,
                val: 2,
            }); 10];
            neopixel.write(colors.iter().cloned()).unwrap();
            delay.delay_ms(5u8);
        }
    }
}
