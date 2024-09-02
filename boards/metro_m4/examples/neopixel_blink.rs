#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::hal;
use bsp::pac;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::ehal::delay::DelayNs;
use hal::time::Hertz;
use hal::timer::TimerCounter;
use hal::timer_traits::InterruptDrivenTimer;
use pac::{CorePeripherals, Peripherals};

use ws2812_timer_delay as ws2812;

use smart_leds::{brightness, colors, hsv::RGB8, SmartLedsWrite};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let gclk0 = clocks.gclk0();
    let tc2_3 = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&tc2_3, peripherals.tc3, &mut peripherals.mclk);
    timer.start(Hertz::MHz(3).into_duration());

    let pins = bsp::Pins::new(peripherals.port);
    let neopixel_pin = pins.neopixel.into_push_pull_output();
    let mut neopixel = ws2812::Ws2812::new(timer, neopixel_pin);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        let data = [colors::RED; 1];
        neopixel
            .write(brightness(data.iter().cloned(), 32))
            .unwrap();
        delay.delay_ms(250);
        let data2 = [RGB8::default(); 1];
        neopixel
            .write(brightness(data2.iter().cloned(), 32))
            .unwrap();
        delay.delay_ms(250);
    }
}
