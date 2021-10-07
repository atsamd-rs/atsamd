//! Uses a timer in standby mode to blink an LED for maximum power savings.
//!
//! We also use the internal 8MHz RC oscillator & the internal 32k oscillator
//! to save power as it doesn't require the PLL.
#![no_std]
#![no_main]

use bsp::hal;
use p1am_100 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController};
use hal::pac::{interrupt, CorePeripherals, Peripherals, TC4};
use hal::prelude::*;
use hal::sleeping_delay::SleepingDelay;
use hal::timer;

use core::sync::atomic;
use cortex_m::peripheral::NVIC;

/// Shared atomic between TC4 interrupt and sleeping_delay module
static INTERRUPT_FIRED: atomic::AtomicBool = atomic::AtomicBool::new(false);

#[entry]
fn main() -> ! {
    // Configure all of our peripherals/clocks
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_8mhz(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    // Get a clock & make a sleeping delay object. use internal 32k clock that runs
    // in standby
    enable_internal_32kosc(&mut peripherals.SYSCTRL);
    let timer_clock = clocks
        .configure_gclk_divider_and_source(ClockGenId::GCLK1, 1, ClockSource::OSC32K, false)
        .unwrap();
    clocks.configure_standby(ClockGenId::GCLK1, true);
    let tc45 = &clocks.tc4_tc5(&timer_clock).unwrap();
    let timer = timer::TimerCounter::tc4_(tc45, peripherals.TC4, &mut peripherals.PM);
    let mut sleeping_delay = SleepingDelay::new(timer, &INTERRUPT_FIRED);

    // Timer overflow interrupts are asynchronous, we can use IDLE2 sleep for max
    //   power savings. The CPU, AHB and APB clock domains are stopped
    // peripherals.PM.sleep.modify(|_, w| w.idle().apb());

    // We can also use it in standby mode, if all of the clocks are configured to
    //   opperate in standby, for even more power savings
    core.SCB.set_sleepdeep();

    // enable interrupts
    unsafe {
        core.NVIC.set_priority(interrupt::TC4, 2);
        NVIC::unmask(interrupt::TC4);
    }

    // Turn off unnecessary peripherals
    peripherals.PM.ahbmask.modify(|_, w| {
        w.usb_().clear_bit();
        w.dmac_().clear_bit()
    });
    peripherals.PM.apbamask.modify(|_, w| {
        w.eic_().clear_bit();
        w.wdt_().clear_bit();
        w.sysctrl_().clear_bit();
        w.pac0_().clear_bit()
    });
    peripherals.PM.apbbmask.modify(|_, w| {
        w.usb_().clear_bit();
        w.dmac_().clear_bit();
        w.nvmctrl_().clear_bit();
        w.dsu_().clear_bit();
        w.pac1_().clear_bit()
    });
    // Thankfully the only one default on here is ADC
    peripherals.PM.apbcmask.modify(|_, w| w.adc_().clear_bit());

    // Configure our red LED and blink forever, sleeping between!
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led: bsp::Led = pins.led.into();
    loop {
        led.set_low().unwrap();
        sleeping_delay.delay_ms(1_000u32);
        led.set_high().unwrap();
        sleeping_delay.delay_ms(100u32);
    }
}

#[interrupt]
fn TC4() {
    // Let the sleepingtimer know that the interrupt fired, and clear it
    INTERRUPT_FIRED.store(true, atomic::Ordering::Relaxed);
    unsafe {
        TC4::ptr()
            .as_ref()
            .unwrap()
            .count16()
            .intflag
            .modify(|_, w| w.ovf().set_bit());
    }
}
