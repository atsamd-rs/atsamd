//! Uses an external interrupt to blink an LED.
//!
//! You need to connect a button between D46 and ground. Each time the button
//! is pressed, the LED will toggle.
#![no_std]
#![no_main]

use cortex_m::interrupt::{free, Mutex};
use grand_central_m4::pin_alias;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::hal;
use bsp::pac;
use grand_central_m4 as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::eic::{
    self,
    pin::{ExtInt6, ExternalInterrupt, Sense},
};
use hal::gpio::{Pin, PullUpInterrupt};
use hal::prelude::*;
use pac::{interrupt, CorePeripherals, Peripherals};

use core::cell::RefCell;

use cortex_m::peripheral::NVIC;

static RED_LED: Mutex<RefCell<Option<bsp::RedLed>>> = Mutex::new(RefCell::new(None));

fn toggle_led() {
    free(|cs| {
        RED_LED
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .map(|l| l.toggle().unwrap());
    });
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);
    free(|cs| {
        RED_LED
            .borrow(cs)
            .replace(Some(pin_alias!(pins.red_led).into()))
    });

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let gclk0 = clocks.gclk0();
    let eic_clock = clocks.eic(&gclk0).unwrap();
    let mut eic = eic::init_with_ulp32k(&mut peripherals.mclk, eic_clock, peripherals.eic);
    let button: Pin<_, PullUpInterrupt> = pins.d46.into();
    eic.button_debounce_pins(&[button.id()]);
    let mut extint_button = ExtInt6::new(button);
    extint_button.sense(&mut eic, Sense::Both);
    extint_button.enable_interrupt(&mut eic);
    eic.finalize();

    // Enable EIC interrupt in the NVIC
    unsafe {
        core.NVIC.set_priority(interrupt::EIC_EXTINT_6, 1);
        NVIC::unmask(interrupt::EIC_EXTINT_6);
    }

    // Blink the LED once to show that we have started up.
    toggle_led();
    delay.delay_ms(200u8);
    toggle_led();
    delay.delay_ms(200u8);

    loop {
        delay.delay_ms(200u8);
    }
}

#[interrupt]
fn EIC_EXTINT_6() {
    // clear the interrupt and toggle the led
    let eic = unsafe {
        // Accessing registers from interrupts context is safe
        &*pac::Eic::ptr()
    };
    eic.intflag()
        .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 6) });
    toggle_led();
}
