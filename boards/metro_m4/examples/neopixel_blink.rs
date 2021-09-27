#![no_std]
#![no_main]

use bsp::hal;
use metro_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;
use smart_leds;
use ws2812_timer_delay as ws2812;

use bsp::entry;
use embedded_hal::digital::v1_compat::OldOutputPin;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::{brightness, colors, hsv::RGB8, SmartLedsWrite};

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
    let mut pins = bsp::Pins::new(peripherals.PORT);

    let timer = SpinTimer::new(4);
    let neopixel_pin: OldOutputPin<_> = pins.neopixel.into_push_pull_output(&mut pins.port).into();
    let mut neopixel = ws2812::Ws2812::new(timer, neopixel_pin);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        let data = [colors::RED; 1];
        neopixel
            .write(brightness(data.iter().cloned(), 32))
            .unwrap();
        delay.delay_ms(250u8);
        let data2 = [RGB8::default(); 1];
        neopixel
            .write(brightness(data2.iter().cloned(), 32))
            .unwrap();
        delay.delay_ms(250u8);
    }
}
