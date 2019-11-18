#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;
extern crate pygamer as hal;
extern crate smart_leds;
extern crate ws2812_spi as ws2812;

use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::sercom::PadPin;
use hal::time::MegaHertz;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::hsv::RGB8;
use smart_leds::{brightness, SmartLedsWrite};

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
    let gclk = clocks.gclk0();

    let spi: hal::sercom::SPIMaster2<
        hal::sercom::Sercom2Pad0<hal::gpio::Pa12<hal::gpio::PfC>>,
        hal::sercom::Sercom2Pad3<hal::gpio::Pa15<hal::gpio::PfC>>,
        hal::sercom::Sercom2Pad1<hal::gpio::Pa13<hal::gpio::PfC>>,
    > = hal::sercom::SPIMaster2::new(
        &clocks.sercom2_core(&gclk).unwrap(),
        MegaHertz(3),
        embedded_hal::spi::Mode {
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
            polarity: embedded_hal::spi::Polarity::IdleLow,
        },
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        (
            pins.sda.into_pad(&mut pins.port),
            pins.neopixel.into_pad(&mut pins.port),
            pins.scl.into_pad(&mut pins.port),
        ),
    );

    let mut neopixel = ws2812::Ws2812::new(spi);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    const NUM_LEDS: usize = 5;
    let mut data = [RGB8::default(); NUM_LEDS];

    loop {
        for j in 0..(256 * 5) {
            //why
            for _ in 0..1 {
                for i in 0..NUM_LEDS {
                    data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
                }
            }
            let _ = neopixel.write(brightness(data.iter().cloned(), 32));
            delay.delay_ms(5u8);
        }
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
