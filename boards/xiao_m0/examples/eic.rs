//! It uses an external interrupt to toggle an LED.
//!
//! You need to connect a button between Pin a9 and ground. Each time the button
//! is pressed, the LED will toggle his state.
//!
//! To select the correct ExtInt

#![no_std]
#![no_main]

use bsp::{hal, Led0, Led1};
use core::cell::RefCell;
use cortex_m::{
    interrupt::{free, Mutex},
    peripheral::NVIC,
};
use hal::{
    clock::{ClockGenId, GenericClockController},
    eic::{
        pin::{ExtInt5, Sense},
        EIC,
    },
    gpio::v2::{Pin, PullUpInterrupt},
    pac::{self, interrupt, CorePeripherals, Peripherals},
    prelude::*,
};
use panic_halt as _;
use xiao_m0 as bsp;

static LED_1: Mutex<RefCell<Option<Led1>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_8mhz(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    // setup Pins, red status LED, LED to toggle, and button to toggle the led
    let pins = xiao_m0::Pins::new(peripherals.PORT);
    let mut status_led: Led0 = pins.led0.into();
    free(|cs| LED_1.borrow(cs).replace(Some(pins.led1.into())));
    let button_a9: Pin<_, PullUpInterrupt> = pins.a9.into();

    // define clock generator 2 and connect it to the 8Mhz OSC
    let gclk2 = clocks
        .configure_gclk_divider_and_source(
            ClockGenId::GCLK2,
            1,
            pac::gclk::genctrl::SRC_A::OSC8M,
            false,
        )
        .unwrap();

    // "connect" eic clock to gclk2
    let eic_clock = clocks.eic(&gclk2).unwrap();

    // init External interrupt controller
    let mut eic = EIC::init(&mut peripherals.PM, eic_clock, peripherals.EIC);

    // configure external interrupt 5 connected to the Button at PA09
    //
    // check the definition to see connected Pins to the ExternalInterrupt
    // e.g.:
    // ei!(ExtInt[5] { Pa5, Pa21 });
    // Pa5 => PA05 => Pin A5/D5/SCL
    // Pa21 => inaccessible
    let mut extint5 = ExtInt5::new(button_a9);

    // configure ExtInt
    extint5.sense(&mut eic, Sense::RISE);
    extint5.filter(&mut eic, true);
    extint5.enable_interrupt(&mut eic);

    // Enable EIC interrupt in the NVIC
    unsafe {
        core.NVIC.set_priority(interrupt::EIC, 2);
        NVIC::unmask(interrupt::EIC);
    }

    // toggle red led to symbolize hard work
    loop {
        let _ = status_led.toggle();

        for _ in 0..0xffff {
            cortex_m::asm::nop();
        }
    }
}
#[interrupt]
fn EIC() {
    // Accessing registers from interrupts context is safe
    let eic = unsafe { &*pac::EIC::ptr() };

    // Enter critical section
    free(|cs| {
        // check INTFLAG to validate interrupt source if more than one is in use
        if eic.intflag.read().extint5().bit_is_set() {
            // toggle LED
            LED_1.borrow(cs).borrow_mut().as_mut().map(|l| l.toggle());

            // The interrupt request remains active until the interrupt flag is cleared,
            // the interrupt is disabled or the peripheral is reset. An interrupt flag is
            // cleared by writing a one to the corresponding bit in the INTFLAG register.
            // read more: SAM-D21DA1-Family-Data-Sheet-DS40001882G.pdf # 16.6.5 Interrupts
            eic.intflag.modify(|_, w| w.extint5().set_bit());
        }
    });
}
