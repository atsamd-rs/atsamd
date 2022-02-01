#![no_std]
#![no_main]

use bsp::hal;
use samd11_bare as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};

use hal::pac::gclk::{clkctrl::GEN_A, genctrl::SRC_A};
use hal::sercom::v2::{
    uart::{self, BaudMode, Oversampling},
    Sercom0,
};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL48M, false);
    let gclk2 = clocks
        .get_gclk(GEN_A::GCLK2)
        .expect("Could not get clock 2");

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let rx: bsp::UartRx = pins.d1.into();
    let tx: bsp::UartTx = pins.d14.into();

    let uart_clk = clocks
        .sercom0_core(&gclk2)
        .expect("Could not configure sercom0 clock");

    let pads = uart::Pads::<Sercom0>::default().rx(rx).tx(tx);

    let mut uart = uart::Config::new(&peripherals.PM, peripherals.SERCOM0, pads, uart_clk.freq())
        .baud(9600.hz(), BaudMode::Fractional(Oversampling::Bits16))
        .enable();

    loop {
        for byte in b"Hello, world!" {
            // NOTE `block!` blocks until `uart.write()` completes and returns
            // `Result<(), Error>`
            nb::block!(uart.write(*byte)).unwrap();
        }
        delay.delay_ms(1000u16);
    }
}
