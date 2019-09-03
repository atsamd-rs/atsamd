#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate metro_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;
extern crate rtfm;

#[cfg(feature = "use_semihosting")]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use cortex_m_semihosting::hio;
            use core::fmt::Write;
            let mut stdout = hio::hstdout().unwrap();
            writeln!(stdout, $($arg)*).ok();
        }
    };
}

#[cfg(not(feature = "use_semihosting"))]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

use hal::clock::GenericClockController;
use hal::prelude::*;
use rtfm::app;

#[app(device=hal::pac)]
const APP: () = {
    static mut RED_LED: hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>> = ();
    static mut TIMER: hal::timer::TimerCounter3 = ();

    /// This function is called each time the tc3 interrupt triggers.
    /// We use it to toggle the LED.  The `wait()` call is important
    /// because it checks and resets the counter ready for the next
    /// period.
    #[interrupt(resources = [TIMER,RED_LED])]
    fn TC3() {
        if resources.TIMER.wait().is_ok() {
            resources.RED_LED.toggle();
        }
    }

    #[init]
    fn init() {
        let mut clocks = GenericClockController::with_internal_32kosc(
            device.GCLK,
            &mut device.PM,
            &mut device.SYSCTRL,
            &mut device.NVMCTRL,
        );
        let gclk0 = clocks.gclk0();
        let mut pins = hal::Pins::new(device.PORT);

        let mut tc3 = hal::timer::TimerCounter::tc3_(
            &clocks.tcc2_tc3(&gclk0).unwrap(),
            device.TC3,
            &mut device.PM,
        );
        dbgprint!("start timer");
        tc3.start(1.hz());
        tc3.enable_interrupt();

        dbgprint!("done init");
        RED_LED = pins.d13.into_open_drain_output(&mut pins.port);
        TIMER = tc3;
    }
};
