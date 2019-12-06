//! Display light sensor reading on the neopixels.

#![no_std]
#![no_main]

#[allow(unused_imports)]
use panic_halt;
use pygamer as hal;

use hal::adc::Adc;
use hal::entry;
use hal::pac::gclk::pchctrl::GEN_A::GCLK11;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;
use hal::{clock::GenericClockController, delay::Delay};

use smart_leds::hsv::RGB8;
use smart_leds::{brightness, SmartLedsWrite};

use embedded_hal::digital::v1_compat::OldOutputPin;
use ws2812_timer_delay as ws2812;

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

    let mut adc1 = Adc::adc1(peripherals.ADC1, &mut peripherals.MCLK, &mut clocks, GCLK11);
    let mut light = pins.light.into_function_b(&mut pins.port);

    let timer = SpinTimer::new(4);
    let neopixel_pin: OldOutputPin<_> = pins.neopixel.into_push_pull_output(&mut pins.port).into();
    let mut neopixel = ws2812::Ws2812::new(timer, neopixel_pin);

    let mut delay = Delay::new(core.SYST, &mut clocks);

    const NUM_LEDS: usize = 5;
    let mut j: u8 = 0;

    loop {
        let light_data: u16 = adc1.read(&mut light).unwrap();

        let pos: usize = if light_data < 100 {
            0
        } else if (light_data >= 147) && (light_data < 1048) {
            1
        } else if (light_data >= 1048) && (light_data < 3048) {
            2
        } else if (light_data >= 3048) && (light_data < 3948) {
            3
        } else {
            4
        };

        //finally paint the one led wherever the position is
        let _ = neopixel.write(brightness(
            (0..NUM_LEDS).map(|i| if i == pos { wheel(j) } else { RGB8::default() }),
            32,
        ));

        //incremement the wheel easing
        j = j.wrapping_add(1);

        delay.delay_ms(10u8);
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
