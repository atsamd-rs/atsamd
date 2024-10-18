//! Use a hardware timer to wiggle a pin by blocking until the timer is up
//! and then toggling a pin.

#![no_std]
#![no_main]

use bsp::{entry, hal, pac, Pins, RedLed};
#[cfg(not(feature = "panic_led"))]
use panic_halt as _;
use pygamer as bsp;

use hal::clock::GenericClockController;
use hal::ehal::digital::OutputPin;
use hal::nb;
use hal::time::Hertz;
use hal::timer::TimerCounter;
use hal::timer_traits::InterruptDrivenTimer;
use pac::Peripherals;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = Pins::new(peripherals.port);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.tc3, &mut peripherals.mclk);
    timer.start(Hertz::kHz(250).into_duration());
    let mut d13: RedLed = pins.d13.into();

    // Cycle red LED through 50%, 100%, 0%
    loop {
        for _ in 0..125000 {
            //50% duty cycle, so 125khz period
            let _ = d13.set_high();
            let _ = nb::block!(timer.wait());
            let _ = d13.set_low();
            let _ = nb::block!(timer.wait());
        }
        let _ = d13.set_high();
        for _ in 0..125000 {
            let _ = nb::block!(timer.wait());
            let _ = nb::block!(timer.wait());
        }
        let _ = d13.set_low();
        for _ in 0..125000 {
            let _ = nb::block!(timer.wait());
            let _ = nb::block!(timer.wait());
        }
    }
}
