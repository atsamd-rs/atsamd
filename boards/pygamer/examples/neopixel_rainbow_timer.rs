#![no_std]
#![no_main]

extern crate cortex_m;
extern crate embedded_hal;
extern crate panic_halt;
extern crate pygamer as hal;
extern crate smart_leds;
extern crate ws2812_timer_delay as ws2812;

use embedded_hal::digital::v1_compat::OldOutputPin;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::{clock::GenericClockController, delay::Delay, timer::TimerCounter};

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

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.MCLK);
    // supposed to be 3mhz?
    // slow mode always 1 led turns white.. so disable
    // havent seen lto false work
    // lto true 3.1-3.8 works,
    timer.start(3_100_000u32.hz());

    let mut neopixel_pin: OldOutputPin<_> =
        pins.neopixel.into_push_pull_output(&mut pins.port).into();
    let mut neopixel = ws2812::Ws2812::new(timer, &mut neopixel_pin);
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
