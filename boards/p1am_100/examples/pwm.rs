#![no_std]
#![no_main]

use bsp::hal;
use p1am_100 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use core::fmt::Write;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use cortex_m_rt::entry;

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

    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let pins = bsp::Pins::new(peripherals.PORT);

    let mut uart = bsp::uart(
        &mut clocks,
        9600.hz(),
        peripherals.SERCOM5,
        &mut peripherals.PM,
        pins.d13.into(),
        pins.d14.into(),
    );

    // Manually configure TC4 for normal PWM operation.
    // The pwm::Pwm4 type can only be used for "match PWM" operation, which
    // is not available on the LED pin.
    let gclk0 = clocks.gclk0();
    let clock = &clocks.tc4_tc5(&gclk0).unwrap();
    let tc = peripherals.TC4;
    let pm = &mut peripherals.PM;
    let count = tc.count16();
    // Enable TC4 clock
    pm.apbcmask.modify(|_, w| w.tc4_().set_bit());
    // Reset TC4
    count.ctrla.write(|w| w.swrst().set_bit());
    while count.ctrla.read().bits() & 1 != 0 {}
    // Disable TC4
    count.ctrla.modify(|_, w| w.enable().clear_bit());
    // Divide GCLK by 4 to get TC4 clock
    count.ctrla.modify(|_, w| {
        w.prescaler().div4();
        w.prescsync().presc()
    });
    // Normal PWM mode
    count.ctrla.write(|w| w.wavegen().npwm());
    // Start at 0 duty cycle
    count.cc[0].write(|w| unsafe { w.cc().bits(0) });
    // Enable PWM
    count.ctrla.modify(|_, w| w.enable().set_bit());

    uprintln!(uart, "tc4 configured");
    // Enable PWM output on the LED pin
    let _led = pins.led.into_mode::<hal::gpio::v2::AlternateE>();

    let max_duty = u16::MAX;

    uprintln!(uart, "clock freq: {:?}", clock.freq());
    uprintln!(
        uart,
        "prescaler: {:?}",
        count.ctrla.read().prescaler().variant()
    );
    uprintln!(uart, "max_duty: {}", max_duty);

    loop {
        uprintln!(uart, "starting at 0");
        for i in 0..256 {
            count.cc[0].write(|w| unsafe { w.cc().bits(i * i) });
            delay.delay_ms(5u16);
        }
    }
}
