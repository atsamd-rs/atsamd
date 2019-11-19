#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;
use pygamer as hal;

use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::sercom::PadPin;
use hal::time::MegaHertz;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::hsv::RGB8;
use smart_leds::{brightness, SmartLedsWrite};
use ws2812_spi as ws2812;

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

    loop {
        for j in 0..255u8 {
            let colors = [
                // split the color changes across all 5 leds evenly, 255/5=51
                // and have them safely wrap over when they go above 255
                wheel(j),
                wheel(j.wrapping_add(51)),
                wheel(j.wrapping_add(102)),
                wheel(j.wrapping_add(153)),
                wheel(j.wrapping_add(204)),
            ];
            neopixel
                .write(brightness(colors.iter().cloned(), 32))
                .unwrap();

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
