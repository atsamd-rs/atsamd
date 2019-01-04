#![no_std]
#![no_main]

extern crate atsamd21_hal as atsamd21;
extern crate cortex_m;
extern crate rtfm;
#[macro_use]
extern crate nb;
extern crate embedded_hal;
extern crate panic_halt;
extern crate samd21_mini as hal;

use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::sercom::{PadPin, Sercom0Pad2, Sercom0Pad3, UART0Pinout, UART0};
use hal::target_device::gclk::clkctrl::GENR;
use hal::target_device::gclk::genctrl::SRCR;
use rtfm::app;

macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

#[app(device=hal)]
const APP: () = {
    static mut BLUE_LED: hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>> = ();
    static mut TX_LED: hal::gpio::Pa27<hal::gpio::Output<hal::gpio::OpenDrain>> = ();
    static mut RX_LED: hal::gpio::Pb3<hal::gpio::Output<hal::gpio::OpenDrain>> = ();
    static mut UART: UART0 = ();

    #[init]
    fn init() {
        let mut pins = hal::Pins::new(device.PORT);

        let mut clocks = GenericClockController::with_internal_32kosc(
            device.GCLK,
            &mut device.PM,
            &mut device.SYSCTRL,
            &mut device.NVMCTRL,
        );
        clocks.configure_gclk_divider_and_source(GENR::GCLK2, 1, SRCR::DFLL48M, false);
        let gclk2 = clocks.get_gclk(GENR::GCLK2).expect("Could not get clock 2");

        dbgprint!("Initializing serial port");

        let mut led = pins.led.into_open_drain_output(&mut pins.port);
        led.set_low();

        let rx_pin: Sercom0Pad3 = pins
            .rx
            .into_pull_down_input(&mut pins.port)
            .into_pad(&mut pins.port);
        let tx_pin: Sercom0Pad2 = pins
            .tx
            .into_push_pull_output(&mut pins.port)
            .into_pad(&mut pins.port);
        let uart_clk = clocks
            .sercom0_core(&gclk2)
            .expect("Could not configure sercom0 core clock");

        let uart = UART0::new(
            &uart_clk,
            9600.hz(),
            device.SERCOM0,
            unsafe { &mut hal::CorePeripherals::steal().NVIC },
            &mut device.PM,
            UART0Pinout::Rx3Tx2 {
                rx: rx_pin,
                tx: tx_pin,
            },
        );

        let mut rx_led = pins.rx_led.into_open_drain_output(&mut pins.port);
        let mut tx_led = pins.tx_led.into_open_drain_output(&mut pins.port);

        tx_led.set_high();
        rx_led.set_high();

        dbgprint!("done init");
        BLUE_LED = led;
        TX_LED = tx_led;
        RX_LED = rx_led;
        UART = uart;
    }

    #[interrupt(resources = [UART, RX_LED, TX_LED])]
    fn SERCOM0() {
        resources.RX_LED.set_low();
        let data = match block!(resources.UART.read()) {
            Ok(v) => {
                resources.RX_LED.set_high();
                v
            }
            Err(_) => 0 as u8,
        };

        resources.TX_LED.set_low();
        match block!(resources.UART.write(data)) {
            Ok(_) => {
                resources.TX_LED.set_high();
            }
            Err(_) => unimplemented!(),
        }
    }
};
