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
//! the builtin red led
//!
//! Wiring
//! - RX - A4
//! - TX - A5
//!
//! > In the ATSAMX5x MCUs SERCOM UART configuration requires at least
//! two pins from the same IOSET. You can find more info about this in
//! the hal documentation or in the MCU datasheet at:
//! <https://www.microchip.com/en-us/product/ATSAMD51G19A>

use itsybitsy_m4 as bsp;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{entry, IoSet3Sercom0Pad0, IoSet3Sercom0Pad2};
use core::cell::RefCell;
use cortex_m::{interrupt::Mutex, peripheral::NVIC};

use bsp::hal::{
    clock::GenericClockController,
    delay::Delay,
    ehal::blocking::delay::DelayMs,
    gpio::v2::{Pin, PushPullOutput, PA22},
    pac::{self, interrupt, CorePeripherals, Peripherals},
    prelude::*,
    sercom::v2::{
        uart::{self, BaudMode, Flags, Oversampling},
        IoSet3, Sercom0,
    },
    time::Hertz,
};

type UartPads0 = uart::Pads<Sercom0, IoSet3, IoSet3Sercom0Pad2, IoSet3Sercom0Pad0>;
type Uart0 = uart::Uart<uart::Config<UartPads0>, uart::Duplex>;

/// Utility function for setting up SERCOM0 pins as an additional
/// UART peripheral.
pub fn uart0(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom0: pac::SERCOM0,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<IoSet3Sercom0Pad2>,
    uart_tx: impl Into<IoSet3Sercom0Pad0>,
) -> Uart0 {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, sercom0, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// If you want to know more on ways to share data safely,
/// give this a read: <https://docs.rust-embedded.org/book/concurrency/>
static LED: Mutex<RefCell<Option<Pin<PA22, PushPullOutput>>>> = Mutex::new(RefCell::new(None));
static SERIAL: Mutex<RefCell<Option<bsp::Uart>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    //basic setup
    let mut dp = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        dp.GCLK,
        &mut dp.MCLK,
        &mut dp.OSC32KCTRL,
        &mut dp.OSCCTRL,
        &mut dp.NVMCTRL,
    );

    let pins = bsp::Pins::new(dp.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // custom sercom uart configuration
    let mut serial_sercom0 = uart0(
        &mut clocks,
        Hertz(115200),
        dp.SERCOM0,
        &mut dp.MCLK,
        pins.a5,
        pins.a4,
    );

    // labeled "default" uart
    let mut serial_sercom3 = bsp::uart(
        &mut clocks,
        Hertz(115200),
        dp.SERCOM3,
        &mut dp.MCLK,
        pins.d0_rx,
        pins.d1_tx,
    );

    // We must enable the interrupt flag by flag, in this case,
    // we just want RXC
    serial_sercom3.enable_interrupts(Flags::RXC);
    let led = pins.d13.into_push_pull_output();
    //move shared peripherals into the mutexes
    cortex_m::interrupt::free(|cs| {
        SERIAL.borrow(cs).replace(Some(serial_sercom3));
        LED.borrow(cs).replace(Some(led));
    });

    unsafe {
        // interrupt must be enabled and unmasked in the NVIC
        // (Nested Vector Interrupt Controller) peripheral
        core.NVIC.set_priority(interrupt::SERCOM3_2, 1);
        NVIC::unmask(interrupt::SERCOM3_2);
    }

    loop {
        // wait a second then send a byte
        delay.delay_ms(1000u16);
        let _ = nb::block!(serial_sercom0.write(b'A'));
    }
}

#[interrupt]
fn SERCOM3_2() {
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
