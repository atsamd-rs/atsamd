//! Turn on and off with an LED
#![no_std]
#![no_main]

use core::{cell::RefCell, mem};

use atsamd_hal::ehal::digital::{OutputPin, StatefulOutputPin};
use bsp::{
    pac::{interrupt, CorePeripherals, Interrupt, Peripherals},
    pin_alias, RedLed,
};
use cortex_m::{asm, peripheral::NVIC};
use cortex_m_rt::entry;
use critical_section::Mutex;
use pyportal as bsp;
use rtt_target::rprintln;

use panic_semihosting as _;

macro_rules! sync_wait {
    ($mode0:expr, $register:ident) => {
        while $mode0.syncbusy().read().$register().bit_is_set() {}
    };
}

static BACKLIGHT_PIN: Mutex<RefCell<RedLed>> = Mutex::new(RefCell::new(unsafe { mem::zeroed() }));

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let peripherals = Peripherals::take().unwrap();
    let _core = CorePeripherals::take().unwrap();

    let pins = bsp::Pins::new(peripherals.port);
    let red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    critical_section::with(|cs| {
        let _ = BACKLIGHT_PIN.replace(cs, red_led);
        BACKLIGHT_PIN.borrow_ref_mut(cs).set_low().unwrap();
    });

    rprintln!("starting up");

    // enable global interrupts
    unsafe {
        NVIC::mask(Interrupt::RTC);
        // core.NVIC.set_priority(Interrupt::RTC, 8);
        cortex_m::interrupt::enable();
    };

    // use the 32k clock
    peripherals
        .osc32kctrl
        .rtcctrl()
        .write(|w| w.rtcsel().ulp32k());

    let mode0 = peripherals.rtc.mode0();

    // Run RTC when main chip is paused
    mode0.dbgctrl().write(|w| w.dbgrun().set_bit());

    // disable the clock
    mode0.ctrla().write(|w| w.enable().clear_bit());
    // write sync for RTC enable
    sync_wait!(mode0, enable);

    // trigger a reset
    mode0.ctrla().modify(|_, w| w.swrst().set_bit());
    // write sync reset
    sync_wait!(mode0, swrst);

    // set the mode
    mode0.ctrla().modify(|_, w| w.mode().count32());

    mode0.ctrla().modify(|_, w| {
        // Use 32 bit counter
        w.prescaler().div1();
        // The COUNT register requires synchronization when reading.
        // Disabling the synchronization will prevent reading valid values from the
        // COUNT register.
        w.countsync().set_bit();
        w.matchclr().clear_bit();
        w
    });
    // write sync for countsync
    let init = mode0.count().read().count().bits();
    sync_wait!(mode0, countsync);
    // When CTRLA.COUNTSYNC is enabled, the first COUNT value is not correctly
    // synchronized and thus it is a wrong value.

    // clear flag
    mode0.intflag().write(|w| w.cmp0().set_bit());

    // wait for count to be ready
    sync_wait!(mode0, count);
    // read the current count
    let count: u32 = mode0.count().read().count().bits();
    // add 5 seconds
    let next = count + (5 * 32_768);
    rprintln!("count is {}, waking up at {}", count, next);

    mode0.comp(0).write(|w| unsafe { w.comp().bits(next) });
    // wait write

    sync_wait!(mode0, comp0);

    mode0.intflag().write(|w| w.cmp0().set_bit());
    // enable interupt
    mode0.intenset().write(|w| w.cmp0().set_bit());

    // critical_section::with(|cs| {
    //     BACKLIGHT_PIN.borrow_ref_mut(cs).set_low();
    // });

    unsafe {
        NVIC::unmask(Interrupt::RTC);
    }

    // Enable the RTC
    mode0.ctrla().modify(|_, w| w.enable().set_bit());

    // Block to wait for countsync to be correct
    while mode0.count().read().count().bits() == init {}
    // write sync for RTC enable
    sync_wait!(mode0, enable);

    loop {
        asm::wfi();
    }
}

#[interrupt]
fn RTC() {
    // unsafe { NVIC::mask(Interrupt::RTC) };

    critical_section::with(|_cs| {
        let peripherals = unsafe { Peripherals::steal() };

        let mode0 = peripherals.rtc.mode0();

        // is this actually an RTC compare0 interrupt
        if mode0.intflag().read().cmp0().bit_is_set() {
            // clear the interrupt bit

            critical_section::with(|cs| {
                BACKLIGHT_PIN.borrow_ref_mut(cs).toggle().unwrap();
            });

            sync_wait!(mode0, count);
            let count: u32 = mode0.count().read().count().bits();
            // add 5 seconds
            let next = count + (2 * 32_768);
            rprintln!("count is {}, waking up at {}", count, next);

            mode0.comp(0).write(|w| unsafe { w.comp().bits(next) });
            // wait write
            sync_wait!(mode0, comp0);
            mode0.intflag().write(|w| w.cmp0().set_bit());
        }
        // unsafe { NVIC::unmask(Interrupt::RTC) };
    })
}
