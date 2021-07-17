#![no_std]
#![no_main]

use neo_trinkey as bsp;
use panic_halt as _;

use bsp::entry;
use bsp::hal;
use bsp::NeoPixel;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::SpinTimer;

use embedded_hal::digital::v1_compat::OldOutputPin;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::SmartLedsWrite;

use atsamd_hal::timer::TimerCounter;
use neo_trinkey::hal::clock::Tcc2Tc3Clock;
use ws2812_timer_delay::Ws2812;

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

    // let mut delay = Delay::new(core.SYST, &mut clocks);

    let pins = bsp::Pins::new(peripherals.PORT);
    // let mut neo_pixel: NeoPixel = pins.neo_pixel.into();
    // let mut neo_pixel: OldOutputPin<_> = pins.neo_pixel.into_push_pull_output().into();
    // let neo_pixel: OldOutputPin<_> = pins.neo_pixel.into_push_pull_output().into();
    let neo_pixel = pins.neo_pixel.into_push_pull_output();

    // let gclk0 = clocks.gclk0();
    // let timer_clock = clocks.tc4_tc5(&gclk0).unwrap();
    // let mut timer = TimerCounter::tc4_(&timer_clock, peripherals.TC4, &mut peripherals.PM);
    // timer.start(3.mhz());
    let timer = SpinTimer::new(16);

    let mut ws2812 = Ws2812::new(timer, neo_pixel);

    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        for j in 0..255u8 {
            let colors = [hsv2rgb(Hsv {
                hue: j,
                sat: 255,
                val: 32,
            })];
            ws2812.write(colors.iter().cloned()).unwrap();
            delay.delay_ms(100u8);
        }
    }
}
