#![no_std]
#![no_main]

extern crate atsamd21_hal as atsamd21;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
#[macro_use]
extern crate nb;
extern crate embedded_hal;
extern crate panic_abort;
extern crate samd21_mini as hal;

use hal::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::sercom::{PadPin, Sercom0Pad2, Sercom0Pad3, UART0Pinout, UART0};
use hal::target_device::gclk::clkctrl::GENR;
use hal::target_device::gclk::genctrl::SRCR;
use rtfm::{app, Threshold};

macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

app! {
    device: hal,

    resources: {
        static BLUE_LED: hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>;
        static TX_LED: hal::gpio::Pa27<hal::gpio::Output<hal::gpio::OpenDrain>>;
        static RX_LED: hal::gpio::Pb3<hal::gpio::Output<hal::gpio::OpenDrain>>;

        static UART: UART0;
    },

    tasks: {
        SERCOM0: {
            path: int_uart,
            resources: [UART, RX_LED, TX_LED, BLUE_LED]
        },
    }
}

fn int_uart(_t: &mut Threshold, mut r: SERCOM0::Resources) {
    r.RX_LED.set_low();
    let data = match block!(r.UART.read()) {
        Ok(v) => {
            r.RX_LED.set_high();
            v
        }
        Err(_) => 0 as u8,
    };

    r.TX_LED.set_low();
    match block!(r.UART.write(data)) {
        Ok(_) => {
            r.TX_LED.set_high();
        }
        Err(_) => unimplemented!(),
    }
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

// init::Peripherals contains two fields
// core: CorePeripherals
// device: Peripherals
fn init(mut p: init::Peripherals) -> init::LateResources {
    let mut pins = hal::Pins::new(p.device.PORT);

    let mut clocks = GenericClockController::with_internal_32kosc(
        p.device.GCLK,
        &mut p.device.PM,
        &mut p.device.SYSCTRL,
        &mut p.device.NVMCTRL,
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
        p.device.SERCOM0,
        &mut p.core.NVIC,
        &mut p.device.PM,
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
    init::LateResources {
        BLUE_LED: led,
        TX_LED: tx_led,
        RX_LED: rx_led,
        UART: uart,
    }
}

#[entry]
fn run_app() -> ! {
    main();
    loop {}
}
