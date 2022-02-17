#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use itsybitsy_m0 as bsp;

use bsp::entry;

use hal::{
    clock::GenericClockController,
    delay::Delay,
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    time::MegaHertz,
    timer::TimerCounter,
};

use smart_leds::{hsv::RGB8, SmartLedsWrite};

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

    // gclk0 represents a configured clock using the system 48MHz oscillator
    let gclk0 = clocks.gclk0();
    // configure a clock for the TC4 and TC5 peripherals
    let tc45 = &clocks.tc4_tc5(&gclk0).unwrap();
    // instantiate a timer objec for the TC4 peripheral
    let mut timer = TimerCounter::tc4_(tc45, peripherals.TC4, &mut peripherals.PM);
    // start a 4 MHz timer
    timer.start(MegaHertz(4));
    let mut rgb = bsp::dotstar_bitbang(
        pins.dotstar_miso.into(),
        pins.dotstar_mosi.into(),
        pins.dotstar_sck.into(),
        timer,
    );
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
