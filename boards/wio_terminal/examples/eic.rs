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
use wio_terminal as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::eic::{Eic, Sense};
use hal::gpio::{Pin, Pins, PullUpInterrupt};
use hal::prelude::*;
use pac::{interrupt, CorePeripherals, Peripherals};
use wio_terminal::aliases::UserLed;

use core::sync::atomic::{AtomicUsize, Ordering};

use cortex_m::peripheral::NVIC;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let gclk0 = clocks.gclk0();
    let pins = Pins::new(peripherals.port);
    //  let mut red_led: bsp::RedLed = pins.d13.into();
    let mut user_led: UserLed = pins.pa15.into();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let eic_clock = clocks.eic(&gclk0).unwrap();
    let eic_channels = Eic::new(&mut peripherals.mclk, &eic_clock, peripherals.eic).split();

    let button: Pin<_, PullUpInterrupt> = pins.pd10.into();
    let mut extint = eic_channels.5.with_pin(button);
    extint.sense(Sense::Fall);
    extint.enable_interrupt();

    // Enable EIC interrupt in the NVIC
    unsafe {
        core.NVIC.set_priority(interrupt::EIC_EXTINT_5, 1);
        NVIC::unmask(interrupt::EIC_EXTINT_5);
    }

    // Blink the LED once to show that we have started up.
    user_led.set_high().unwrap();
    delay.delay_ms(200u8);
    user_led.set_low().unwrap();
    delay.delay_ms(200u8);

    let mut last_counter_value = COUNTER.load(Ordering::SeqCst);
    loop {
        let new_counter_value = COUNTER.load(Ordering::SeqCst);
        if last_counter_value != new_counter_value {
            last_counter_value = new_counter_value;
            for _ in 0..new_counter_value {
                user_led.set_high().unwrap();
                delay.delay_ms(200u8);
                user_led.set_low().unwrap();
                delay.delay_ms(200u8);
            }
        }
    }
}

#[interrupt]
fn EIC_EXTINT_5() {
    // Increase the counter and clear the interrupt.
    unsafe {
        // Accessing registers from interrupts context is safe
        let eic = &*pac::Eic::ptr();
        eic.intflag().modify(|_, w| w.extint().bits(0x01u16 << 5));
    }
    COUNTER.store(COUNTER.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
}
