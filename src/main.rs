#![feature(used)]
#![feature(proc_macro)]
#![no_std]

extern crate atsamd21_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_abort;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;
extern crate sx1509;

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
use hal::sercom::{I2CMaster3, PadPin};
use rtfm::{app, Threshold};

app! {
    device: hal::atsamd21g18a,

    resources: {
        static RED_LED: hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>;
        static I2C: I2CMaster3;
        static SX1509: sx1509::Sx1509<I2CMaster3>;
        static TIMER: hal::timer::TimerCounter3;
    },

    // Each of the late resources need to be listed in at least
    // one of these resources:[] blocks otherwise compilation
    // will fail with an inscrutable error.  We're throwing them
    // all into the idle block for now.
    idle: {
        resources:[I2C, SX1509 /*, RED_LED, TIMER*/],

    },

    tasks: {
        TC3: {
            path: timer,
            resources: [TIMER, RED_LED],
        },
    }
}

fn timer(_t: &mut Threshold, mut r: TC3::Resources) {
    if r.TIMER.wait().is_ok() {
        r.RED_LED.toggle();
    }
}

fn idle(_t: &mut Threshold, _r: idle::Resources) -> ! {
    loop {
        rtfm::wfi();
    }
}

fn init(mut p: init::Peripherals /* , r: init::Resources */) -> init::LateResources {
    let interval = 1.hz();

    let mut clocks = GenericClockController::new(
        p.device.GCLK,
        &mut p.device.PM,
        &mut p.device.SYSCTRL,
        &mut p.device.NVMCTRL,
    );
    let gclk0 = clocks.gclk0();
    let mut pins = p.device.PORT.split();

    let mut i2c = I2CMaster3::new(
        &clocks.sercom3_core(&gclk0).unwrap(),
        400.khz(),
        p.device.SERCOM3,
        &mut p.device.PM,
        // Metro M0 express has I2C on pins PA22, PA23
        pins.pa22.into_pad(&mut pins.port),
        pins.pa23.into_pad(&mut pins.port),
    );

    let mut expander = sx1509::Sx1509::new(&mut i2c, sx1509::DEFAULT_ADDRESS);

    dbgprint!("do first write");
    // Let's try to init an sx1509 attached to the i2c bus
    let res1 = expander.borrow(&mut i2c).software_reset();
    dbgprint!("send reset {:?}", res1.is_ok());

    let res3 = expander
        .borrow(&mut i2c)
        .read_16(sx1509::Register::RegInterruptMaskA);
    match res3 {
        Err(e) => dbgprint!("read intmaska fail {:?}", e),
        Ok(val) => dbgprint!("read intmaska {:x}", val),
    };

    let mut tc3 = hal::timer::TimerCounter::tc3_(
        &clocks.tcc2_tc3(&gclk0).unwrap(),
        p.device.TC3,
        &mut p.device.PM,
    );
    dbgprint!("start timer");
    tc3.start(interval);
    tc3.enable_interrupt();

    dbgprint!("done init");
    init::LateResources {
        RED_LED: pins.pa17.into_open_drain_output(&mut pins.port),
        I2C: i2c,
        SX1509: expander,
        TIMER: tc3,
    }
}
