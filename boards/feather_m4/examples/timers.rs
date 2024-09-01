#![no_std]
#![no_main]

// Simple Peripheral Timer Example

use bsp::hal;
use feather_m4 as bsp;

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
use hal::timer_traits::InterruptDrivenTimer;

use hal::timer::TimerCounter;

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
    // Using the red LED as the feedback for this simple timer example.
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led = pins.d13.into_push_pull_output();

    // gclk0 represents a configured clock using the 120MHz oscillator.
    let gclk0 = clocks.gclk0();
    // Configure a clock for TC2 and TC3 peripherals
    let timer_clock = clocks.tc2_tc3(&gclk0).unwrap();
    //Instantiate a timer object for the TC2 peripheral
    let mut timer = TimerCounter::tc2_(&timer_clock, peripherals.tc2, &mut peripherals.mclk);
    // Start the timer such that it runs at 50Hz
    timer.start(Hertz::Hz(50u32).into_duration());

    // Toggle the red LED at the frequency set by the timer above.
    loop {
        red_led.set_high().unwrap();
        nb::block!(timer.wait()).ok();
        red_led.set_low().unwrap();
        nb::block!(timer.wait()).ok();
    }
}
