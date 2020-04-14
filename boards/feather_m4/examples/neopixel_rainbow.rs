#![no_std]
#![no_main]

// Neopixel Rainbow
// This only functions when the --release version is compiled. Using the debug version
// leads to slow pulse durations which results in a straight white LED output. 
// 
// // Needs to be compiled with --release for the timing to be correct

extern crate panic_halt;
extern crate cortex_m;
extern crate feather_m4 as hal;

use hal::entry;
use hal::timer::*;
use hal::prelude::*;
use hal::delay::Delay;
use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals};

use ws2812_timer_delay as ws2812;
use smart_leds::{SmartLedsWrite, hsv::{hsv2rgb, Hsv}};


#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    
    // (Re-)configure PB3 as output
    let ws_data_pin = pins.neopixel.into_push_pull_output(&mut pins.port);
    // Create a spin timer whoes period will be 9 x 120MHz clock cycles (75ns)
    let timer = SpinTimer::new(9);
    let mut neopixel = ws2812::Ws2812::new(timer, ws_data_pin);
    
    // Loop through all of the available hue values (colors) to make a
    // rainbow effect from the onboard neopixel
    loop {
        for j in 0..255u8 {
            let colors = [hsv2rgb(Hsv {
                hue: j,
                sat: 255,
                val: 2,
            })];
            neopixel.write(colors.iter().cloned()).unwrap();
            delay.delay_ms(5u8);
        }
    }
}


