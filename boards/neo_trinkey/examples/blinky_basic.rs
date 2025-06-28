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

use smart_leds::{hsv::RGB8, SmartLedsWrite};
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
    let off = [RGB8::default(); NUM_LEDS];
    let on = [
        RGB8::new(5, 5, 0),
        RGB8::new(0, 5, 5),
        RGB8::new(5, 0, 5),
        RGB8::new(2, 2, 2),
    ];

    loop {
        ws2812.write(off.iter().cloned()).unwrap();
        delay.delay_ms(500);
        ws2812.write(on.iter().cloned()).unwrap();
        delay.delay_ms(500);
    }
}
