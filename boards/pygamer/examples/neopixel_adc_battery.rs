//! Display battery percentage on the neopixels.
//!
//! Note leds may appear white during debug. Either build for release or add
//! opt-level = 2 to profile.dev in Cargo.toml

#![no_std]
#![no_main]

#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer::{entry, hal, pac, Pins};

use hal::adc::Adc;
use hal::{clock::GenericClockController, delay::Delay};

use pac::gclk::pchctrl::Genselect::Gclk11;

use hal::ehal::delay::DelayNs;

use pac::{CorePeripherals, Peripherals};
use smart_leds::{brightness, SmartLedsWrite, RGB8};

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

    let mut adc0 = Adc::adc0(peripherals.adc0, &mut peripherals.mclk, &mut clocks, Gclk11);
    let mut battery = pins.battery.init();

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

    //todo put this on a .. 10minute, 30min, update timer
    loop {
        let battery_data = battery.read(&mut adc0);

        let mut colors = [
            RGB8::default(),
            RGB8::default(),
            RGB8::default(),
            RGB8::default(),
            RGB8::default(),
        ];

        if battery_data < 3.6 {
            enable_leds(1, &mut colors);
        } else if (3.6..3.8).contains(&battery_data) {
            enable_leds(2, &mut colors);
        } else if (3.8..3.9).contains(&battery_data) {
            enable_leds(3, &mut colors);
        } else if (3.9..4.0).contains(&battery_data) {
            enable_leds(4, &mut colors);
        } else {
            enable_leds(5, &mut colors);
        };

        neopixel
            .write(brightness(colors.iter().cloned(), 1))
            .unwrap();

        // Reset the LEDs
        delay.delay_ms(10);
    }
}

/// Turn on the specified number of LEDs and set the color to red.
fn enable_leds(num_leds: usize, colors: &mut [RGB8]) {
    for color in colors.iter_mut().take(num_leds) {
        *color = RGB8::from((255, 0, 0));
    }
}
