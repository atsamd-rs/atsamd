#![no_std]
#![no_main]

extern crate cortex_m;
extern crate feather_m0 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{interrupt, CorePeripherals, Peripherals, TC4};
use hal::prelude::*;
use hal::sleeping_delay::SleepingDelay;
use hal::timer;

use core::sync::atomic;
use cortex_m::peripheral::NVIC;

/// Shared atomic between TC4 interrupt and sleeping_delay module
static mut INTERRUPT_FIRED: Option<atomic::AtomicBool> = None;

#[entry]
fn main() -> ! {
    // Configure all of our peripherals/clocks
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    // Set up the int fired global for use in the interrupt handler / SleepingDelay
    let interrupt_fired = unsafe {
        INTERRUPT_FIRED = Some(atomic::AtomicBool::default());
        INTERRUPT_FIRED.as_ref().unwrap()
    };

    // Get a clock & make a sleeping delay object
    let gclk0 = clocks.gclk0();
    let tc45 = &clocks.tc4_tc5(&gclk0).unwrap();
    let timer = timer::TimerCounter::tc4_(tc45, peripherals.TC4, &mut peripherals.PM);
    let mut sleeping_delay = SleepingDelay::new(timer, interrupt_fired);

    // enable interrupts
    unsafe {
        core.NVIC.set_priority(interrupt::TC4, 2);
        NVIC::unmask(interrupt::TC4);
    }

    // Configure our red LED and blink forever, sleeping between!
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    red_led.set_high().unwrap();
    loop {
        red_led.set_high().unwrap();
        sleeping_delay.delay_ms(500u32);
        red_led.set_low().unwrap();
        sleeping_delay.delay_ms(1000u32);
    }
}

#[interrupt]
fn TC4() {
    unsafe {
        // Let the sleepingtimer know that the interrupt fired, and clear it
        INTERRUPT_FIRED
            .as_ref()
            .unwrap()
            .store(true, atomic::Ordering::Relaxed);
        TC4::ptr()
            .as_ref()
            .unwrap()
            .count16()
            .intflag
            .modify(|_, w| w.ovf().set_bit());
    }
}
