#![no_std]
#![no_main]
#![feature(used)]
#![feature(proc_macro_gen)]

extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting;
extern crate metro_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_abort;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;
#[macro_use(entry)]
extern crate cortex_m_rt;

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
use rtfm::{app, Threshold};

app! {
    device: hal,

    resources: {
        static RED_LED: hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>;
        static TIMER: hal::timer::TimerCounter3;
    },

    tasks: {
        TC3: {
            path: timer,
            resources: [TIMER, RED_LED],
        },
    }
}

/// This function is called each time the tc3 interrupt triggers.
/// We use it to toggle the LED.  The `wait()` call is important
/// because it checks and resets the counter ready for the next
/// period.
fn timer(_t: &mut Threshold, mut r: TC3::Resources) {
    if r.TIMER.wait().is_ok() {
        r.RED_LED.toggle();
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn init(mut p: init::Peripherals) -> init::LateResources {
    let mut clocks = GenericClockController::with_internal_32kosc(
        p.device.GCLK,
        &mut p.device.PM,
        &mut p.device.SYSCTRL,
        &mut p.device.NVMCTRL,
    );
    let gclk0 = clocks.gclk0();
    let mut pins = hal::Pins::new(p.device.PORT);

    let mut tc3 = hal::timer::TimerCounter::tc3_(
        &clocks.tcc2_tc3(&gclk0).unwrap(),
        p.device.TC3,
        &mut p.device.PM,
    );
    dbgprint!("start timer");
    tc3.start(1.hz());
    tc3.enable_interrupt();

    dbgprint!("done init");
    init::LateResources {
        RED_LED: pins.d13.into_open_drain_output(&mut pins.port),
        TIMER: tc3,
    }
}

fn run_app() -> ! {
    main();
    loop {}
}

entry!(run_app);
