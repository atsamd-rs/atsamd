//! Uses an external interrupt to blink an LED.
//!
//! You need to connect a button between D12 and ground. Each time the button
//! is pressed, the LED will count the total number of button presses so far.
#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use feather_m0 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::eic::{Eic, Sense};
use hal::gpio::{Pin, PullUpInterrupt};
use hal::prelude::*;
use pac::{interrupt, CorePeripherals, Peripherals};

use core::sync::atomic::{AtomicUsize, Ordering};

use cortex_m::peripheral::NVIC;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );
    let gclk0 = clocks.gclk0();
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pins.d13.into();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let eic_clock = clocks.eic(&gclk0).unwrap();
    let eic_channels = Eic::new(&mut peripherals.pm, eic_clock, peripherals.eic).split();

    let button: Pin<_, PullUpInterrupt> = pins.d10.into();
    let mut extint = eic_channels.2.with_pin(button);
    extint.sense(Sense::Fall);
    extint.enable_interrupt();

    // Enable EIC interrupt in the NVIC
    unsafe {
        core.NVIC.set_priority(interrupt::EIC, 1);
        NVIC::unmask(interrupt::EIC);
    }

    // Blink the LED once to show that we have started up.
    red_led.set_high().unwrap();
    delay.delay_ms(200u8);
    red_led.set_low().unwrap();
    delay.delay_ms(200u8);

    let mut last_counter_value = COUNTER.load(Ordering::SeqCst);
    loop {
        let new_counter_value = COUNTER.load(Ordering::SeqCst);
        if last_counter_value != new_counter_value {
            last_counter_value = new_counter_value;
            for _ in 0..new_counter_value {
                red_led.set_high().unwrap();
                delay.delay_ms(200u8);
                red_led.set_low().unwrap();
                delay.delay_ms(200u8);
            }
        }
    }
}

#[interrupt]
fn EIC() {
    // Increase the counter and clear the interrupt.
    unsafe {
        // Accessing registers from interrupts context is safe
        let eic = &*pac::Eic::ptr();
        eic.intflag().modify(|_, w| w.extint2().set_bit());
    }
    COUNTER.store(COUNTER.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
}
