#![no_std]
#![no_main]

use bsp::hal;
use panic_halt as _;
use trinket_m0 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;

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
    let mut pins = bsp::Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut rgb = pins.dotstar.init(SpinTimer::new(12), &mut pins.port);

    let mut val: u8 = 0;
    loop {
        val = val.wrapping_add(1);
        let color: [RGB8; 1] = [rgb_wheel(val)];
        rgb.write(color.iter().cloned()).unwrap();
        delay.delay_ms(60u8);
    }
}
