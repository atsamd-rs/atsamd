#![no_std]
#![no_main]

use bsp::hal;
use panic_halt as _;
use trinket_m0 as bsp;

use core::fmt::Write;

use bsp::{entry, reset_cause};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::watchdog::{Watchdog, WatchdogTimeout};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\r\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\r\n"), $($arg)*)
    };
}

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
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut pins = bsp::Pins::new(peripherals.PORT);

    let mut uart = bsp::uart(
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.d3,
        pins.d4,
        &mut pins.port,
    );

    let cause = reset_cause(&peripherals.PM);
    uprintln!(uart, "Reset cause: {:?}", cause);

    let mut wdt = Watchdog::new(peripherals.WDT);
    wdt.start(WatchdogTimeout::Cycles16K as u8);

    loop {
        // If we don't feed the watchdog, it will reset the device. This
        // confirms that the watchdog is working when it prints out the reset
        // cause on next startup. Uncommenting the below line will cause the
        // device to spin forever without resetting.
        // wdt.feed();

        delay.delay_ms(100u16);
    }
}
