#![no_std]
#![no_main]

//! The idea of this example is to show two different things
//! - How to manually configure SERCOM for a desired mode (UART in this case)
//! - How to enable an use SERCOM interrupts
//! The second one is particularly handy because some drivers will give
//! you a `poll()` or `update()` function, and you are expected to call it
//! on the receive interrupt of the peripheral i.e. SERCOM UART RXC
//! (receive complete flag).
//!
//! This example is basically just a blinky script. The MCU sends itself
//! a byte via UART that will trigger the RXC interrupt and then toggles
//! one of the builtin leds
//!
//! Wiring
//! - RX(A7) - A4
//! - TX(A6) - A5

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;
use xiao_m0 as bsp;

use bsp::{entry, A4Sercom0Pad0, A5Sercom0Pad1};
use core::cell::RefCell;
use cortex_m::{interrupt::Mutex, peripheral::NVIC};

use bsp::hal::{
    clock::GenericClockController,
    delay::Delay,
    ehal::blocking::delay::DelayMs,
    gpio::v2::{Pin, PushPullOutput, PA17},
    pac::{self, interrupt, CorePeripherals, Peripherals},
    prelude::*,
    sercom::v2::{
        uart::{self, BaudMode, Oversampling},
        Sercom0,
    },
    time::Hertz,
};

type UartPads0 = uart::Pads<Sercom0, A5Sercom0Pad1, A4Sercom0Pad0>;
type Uart0 = uart::Uart<uart::Config<UartPads0>, uart::Duplex>;

/// Utility function for setting up SERCOM0 pins as an additional
/// UART peripheral.
pub fn uart0(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom0: pac::SERCOM0,
    pm: &mut pac::PM,
    uart_rx: impl Into<A5Sercom0Pad1>,
    uart_tx: impl Into<A4Sercom0Pad0>,
) -> Uart0 {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, sercom0, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// If you want to know more on ways to share data safely,
/// give this a read: <https://docs.rust-embedded.org/book/concurrency/>
static LED: Mutex<RefCell<Option<Pin<PA17, PushPullOutput>>>> = Mutex::new(RefCell::new(None));
static SERIAL: Mutex<RefCell<Option<bsp::Uart>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    //basic setup
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // custom sercom uart configuration
    let mut serial_sercom0 = uart0(
        &mut clocks,
        Hertz(115200),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.a5,
        pins.a4,
    );

    // labeled "default" uart
    let mut serial_sercom4 = bsp::uart(
        &mut clocks,
        Hertz(115200),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.a7,
        pins.a6,
    );
    // We must enable the interrupt flag by flag, in this case,
    // we just want RXC
    serial_sercom4.enable_interrupts(uart::Flags::RXC);
    let led = pins.led0.into_push_pull_output();
    //move shared peripherals into the mutexes
    cortex_m::interrupt::free(|cs| {
        SERIAL.borrow(cs).replace(Some(serial_sercom4));
        LED.borrow(cs).replace(Some(led));
    });
    unsafe {
        // interrupt must be enabled and unmasked in the NVIC
        // (Nested Vector Interrupt Controller) peripheral
        core.NVIC.set_priority(interrupt::SERCOM4, 1);
        NVIC::unmask(interrupt::SERCOM4);
    }
    loop {
        delay.delay_ms(1000u16);
        let _ = nb::block!(serial_sercom0.write(b'A'));
    }
}

#[interrupt]
fn SERCOM4() {
    // must read sercom data register or the interrupt
    // won't trigger again
    cortex_m::interrupt::free(|cs| {
        if let Some(serial) = SERIAL.borrow(cs).borrow_mut().as_mut() {
            if let Ok(c) = serial.read() {
                let _ = nb::block!(serial.write(c));
            }
        }
    });
    toggle()
}

fn toggle() {
    cortex_m::interrupt::free(|cs| {
        if let Some(led) = LED.borrow(cs).borrow_mut().as_mut() {
            led.toggle().ok();
        }
    });
}
