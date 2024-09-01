#![no_std]
#![no_main]

use bsp::hal;
use samd11_bare as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::ehal::delay::DelayNs;
use hal::ehal_nb::serial::Write;
use hal::fugit::RateExtU32;
use hal::nb;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};

use hal::pac::gclk::{clkctrl::Genselect, genctrl::Srcselect};
use hal::sercom::{
    uart::{self, BaudMode, Oversampling},
    Sercom0,
};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    clocks.configure_gclk_divider_and_source(Genselect::Gclk2, 1, Srcselect::Dfll48m, false);
    let gclk2 = clocks
        .get_gclk(Genselect::Gclk2)
        .expect("Could not get clock 2");

    let pins = bsp::Pins::new(peripherals.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let rx: bsp::UartRx = pins.d1.into();
    let tx: bsp::UartTx = pins.d14.into();

    let uart_clk = clocks
        .sercom0_core(&gclk2)
        .expect("Could not configure sercom0 clock");

    let pads = uart::Pads::<Sercom0>::default().rx(rx).tx(tx);

    let mut uart = uart::Config::new(&peripherals.pm, peripherals.sercom0, pads, uart_clk.freq())
        .baud(9600.Hz(), BaudMode::Fractional(Oversampling::Bits16))
        .enable();

    loop {
        for byte in b"Hello, world!" {
            // NOTE `block!` blocks until `uart.write()` completes and returns
            // `Result<(), Error>`
            nb::block!(uart.write(*byte)).unwrap();
        }
        delay.delay_ms(1000);
    }
}
