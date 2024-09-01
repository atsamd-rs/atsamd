#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::ehal::digital::OutputPin;
use hal::nb;
use hal::pac::Peripherals;
use hal::time::Hertz;
use hal::timer::TimerCounter;
use hal::timer_traits::InterruptDrivenTimer;

use nb::block;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);

    let gclk0 = clocks.gclk0();
    let tc2_3 = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&tc2_3, peripherals.tc3, &mut peripherals.mclk);
    // start a 5Hz timer
    timer.start(Hertz::Hz(5).into_duration());
    let mut red_led = pins.d13.into_push_pull_output();

    // toggle the led at the frequency set by the timer
    loop {
        red_led.set_high().unwrap();
        block!(timer.wait()).ok();
        red_led.set_low().unwrap();
        block!(timer.wait()).ok();
    }
}
