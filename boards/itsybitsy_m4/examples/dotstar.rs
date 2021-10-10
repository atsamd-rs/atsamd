#![no_std]
#![no_main]

use bsp::hal;
use itsybitsy_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;

use smart_leds::{hsv::RGB8, SmartLedsWrite};

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
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut pins = bsp::Pins::new(peripherals.PORT).split();

    let mut rgb = bsp::dotstar_bitbang(pins.dotstar, &mut pins.port, SpinTimer::new(12));
    let off: [RGB8; 1] = [RGB8 { r: 0, g: 0, b: 0 }];
    let red: [RGB8; 1] = [RGB8 { r: 100, g: 0, b: 0 }];
    let green: [RGB8; 1] = [RGB8 { r: 0, g: 100, b: 0 }];

    rgb.write(off.iter().cloned()).unwrap();
    delay.delay_ms(1200u16);

    loop {
        rgb.write(red.iter().cloned()).unwrap();
        delay.delay_ms(60u8);
        rgb.write(green.iter().cloned()).unwrap();
        delay.delay_ms(60u8);
    }
}
