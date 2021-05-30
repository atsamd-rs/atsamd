//! Uses an RTC in standby mode to blink an LED for maximum power savings.
//!
//! We also use the internal 8MHz RC oscillator & the internal 32k oscillator
//! to save power as it doesn't require the PLL. In standby on my feather_m0,
//! we draw 318 uA when the LED is off and 965 uA when the LED is on at 4.3v.
//! We are slightly limited by the quiescent current draw of the regulator of
//! 90-150 uA.
//!
//! Some more power numbers that I measured:
//! In IDLE2 we draw 740 uA (CPU & AHB & APB domain off)
//! In IDLE1 we draw 870 uA (CPU & AHB domain off)
//! In IDLE0 we draw 870 uA (CPU domain off)
#![no_std]
#![no_main]

use core::sync::atomic;

use cortex_m::peripheral::NVIC;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;

use bsp::entry;
use hal::clock::{enable_internal_32kosc, ClockGenId, ClockSource, GenericClockController};
use hal::prelude::*;
use hal::rtc;
use hal::sleeping_delay::SleepingDelay;
use pac::{interrupt, CorePeripherals, Peripherals, RTC};

/// Shared atomic between RTC interrupt and sleeping_delay module
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
    let rtc_clock = clocks.rtc(&timer_clock).unwrap();
    let timer = rtc::Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);
    let mut sleeping_delay = SleepingDelay::new(timer, &INTERRUPT_FIRED);

    // We can use the RTC in standby for maximum power savings
    core.SCB.set_sleepdeep();

    // enable interrupts
    unsafe {
        core.NVIC.set_priority(interrupt::RTC, 2);
        NVIC::unmask(interrupt::RTC);
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
    let mut red_led: bsp::RedLed = pins.d13.into();
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
