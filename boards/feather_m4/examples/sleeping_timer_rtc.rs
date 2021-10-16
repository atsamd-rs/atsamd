//! Uses an RTC in standby mode to blink an LED for maximum power savings.
#![no_std]
#![no_main]

use bsp::hal;
use feather_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::{interrupt, CorePeripherals, Peripherals, RTC};
use hal::prelude::*;
use hal::rtc;
use hal::sleeping_delay::SleepingDelay;

use core::sync::atomic;
use cortex_m::peripheral::NVIC;

/// Shared atomic between RTC interrupt and sleeping_delay module
static INTERRUPT_FIRED: atomic::AtomicBool = atomic::AtomicBool::new(false);

#[entry]
fn main() -> ! {
    // Configure all of our peripherals/clocks
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let _clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    // Configure the RTC. a 1024 Hz clock is configured for us when enabling our
    // main clock
    let timer = rtc::Rtc::count32_mode(peripherals.RTC, 1024.hz(), &mut peripherals.MCLK);
    let mut sleeping_delay = SleepingDelay::new(timer, &INTERRUPT_FIRED);

    // We can use the RTC in standby for maximum power savings
    core.SCB.set_sleepdeep();

    // enable interrupts
    unsafe {
        core.NVIC.set_priority(interrupt::RTC, 2);
        NVIC::unmask(interrupt::RTC);
    }

    // Turn off unnecessary peripherals
    peripherals.MCLK.ahbmask.modify(|_, w| {
        w.usb_().clear_bit();
        w.dmac_().clear_bit()
    });
    peripherals.MCLK.apbamask.modify(|_, w| {
        w.eic_().clear_bit();
        w.wdt_().clear_bit()
    });
    peripherals.MCLK.apbbmask.modify(|_, w| {
        w.usb_().clear_bit();
        w.nvmctrl_().clear_bit();
        w.dsu_().clear_bit()
    });

    // Configure our red LED and blink forever, sleeping between!
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_push_pull_output();
    loop {
        red_led.set_low().unwrap();
        sleeping_delay.delay_ms(1_000u32);
        red_led.set_high().unwrap();
        sleeping_delay.delay_ms(100u32);
    }
}

#[interrupt]
fn RTC() {
    // Let the sleepingtimer know that the interrupt fired, and clear it
    INTERRUPT_FIRED.store(true, atomic::Ordering::Relaxed);
    unsafe {
        RTC::ptr()
            .as_ref()
            .unwrap()
            .mode0()
            .intflag
            .modify(|_, w| w.cmp0().set_bit());
    }
}
