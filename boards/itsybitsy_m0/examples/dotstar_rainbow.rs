#![no_std]
#![no_main]

use bsp::hal;
use itsybitsy_m0 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::time::MegaHertz;
use hal::timer::TimerCounter;

use smart_leds::{hsv::RGB8, SmartLedsWrite};

fn rgb_wheel(position: u8) -> RGB8 {
    match position {
        0..=85 => RGB8 {
            r: (255 - position * 3),
            g: (position * 3),
            b: 0,
        },
        86..=170 => {
            let position = position - 85;
            RGB8 {
                r: 0,
                g: (255 - position * 3),
                b: (position * 3),
            }
        }
        _ => {
            let position = position - 170;
            RGB8 {
                r: (position * 3),
                g: 0,
                b: (255 - position * 3),
            }
        }
    }
}

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
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let pins = bsp::Pins::new(peripherals.PORT);

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

    let mut val: u8 = 0;
    loop {
        val = val.wrapping_add(1);
        let color: [RGB8; 1] = [rgb_wheel(val)];
        rgb.write(color.iter().cloned()).unwrap();
        delay.delay_ms(60u8);
    }
}
