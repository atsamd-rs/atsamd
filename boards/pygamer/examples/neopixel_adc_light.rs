//! Display light sensor reading on the neopixels.
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer::{entry, hal, pac, Pins};

use hal::adc::Adc;
use hal::prelude::*;
use hal::{clock::GenericClockController, delay::Delay};
use pac::gclk::pchctrl::Genselect::Gclk11;
use pac::{CorePeripherals, Peripherals};
use smart_leds::SmartLedsWrite;
use smart_leds::{
    hsv::{hsv2rgb, Hsv},
    RGB8,
};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = Pins::new(peripherals.port).split();

    let mut adc1 = Adc::adc1(peripherals.adc1, &mut peripherals.mclk, &mut clocks, Gclk11);
    let mut light = pins.light_pin.into_alternate();

    // neopixels
    let mut neopixel = pins.neopixel.init_spi(
        &mut clocks,
        // Unfortunately, the SPI driver requires a clock pin, even though it's not used by the
        // neopixels.
        pins.i2c.scl,
        peripherals.sercom2,
        &mut peripherals.mclk,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);

    const NUM_LEDS: usize = 5;
    let mut j: u8 = 0;

    loop {
        let light_data: u16 = adc1.read(&mut light).unwrap();

        let pos: usize = if light_data < 100 {
            0
        } else if (147..1048).contains(&light_data) {
            1
        } else if (1048..3048).contains(&light_data) {
            2
        } else if (3048..3948).contains(&light_data) {
            3
        } else {
            4
        };

        //finally paint the one led wherever the position is
        let _ = neopixel.write((0..NUM_LEDS).map(|i| {
            if i == pos {
                hsv2rgb(Hsv {
                    hue: j,
                    sat: 255,
                    val: 32,
                })
            } else {
                RGB8::default()
            }
        }));

        //incremement the hue easing
        j = j.wrapping_add(1);

        delay.delay_ms(10u8);
    }
}
