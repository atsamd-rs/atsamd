#![no_std]
#![no_main]

use panic_halt as _;

use neo_trinkey as bsp;

use bsp::entry;
use bsp::hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::ehal::delay::DelayNs;
use hal::pac::{CorePeripherals, Peripherals};
use hal::time::Hertz;
use hal::timer::TimerCounter;
use hal::timer_traits::InterruptDrivenTimer;

use smart_leds::{brightness, hsv::RGB8, SmartLedsWrite};
use ws2812_timer_delay::Ws2812;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = bsp::Pins::new(peripherals.port);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.tc3, &mut peripherals.pm);
    timer.start(Hertz::MHz(3).into_duration());
    let neo_pixel = pins.neo_pixel.into_push_pull_output();
    let mut ws2812 = Ws2812::new(timer, neo_pixel);

    let mut delay = Delay::new(core.SYST, &mut clocks);

    const NUM_LEDS: usize = 4;
    let mut data = [RGB8::default(); NUM_LEDS];

    loop {
        for j in 0..(256 * 5) {
            for (i, item) in data.iter_mut().enumerate().take(NUM_LEDS) {
                *item = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
            }
            ws2812.write(brightness(data.iter().cloned(), 32)).unwrap();
            delay.delay_ms(5);
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
