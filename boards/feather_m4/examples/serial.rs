#![no_std]
#![no_main]

use bsp::hal;
use feather_m4 as bsp;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{entry, periph_alias, pin_alias};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::ehal::delay::DelayNs;
use hal::ehal_nb::serial::Write;
use hal::fugit::RateExtU32;
use hal::nb;
use hal::pac::gclk::genctrl::Srcselect;
use hal::pac::gclk::pchctrl::Genselect;
use hal::pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    clocks.configure_gclk_divider_and_source(Genselect::Gclk2, 1, Srcselect::Dfll, false);

    let pins = bsp::Pins::new(peripherals.port);
    let uart_rx = pin_alias!(pins.uart_rx);
    let uart_tx = pin_alias!(pins.uart_tx);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let uart_sercom = periph_alias!(peripherals.uart_sercom);

    let mut uart = bsp::uart(
        &mut clocks,
        19200.Hz(),
        uart_sercom,
        &mut peripherals.mclk,
        uart_rx,
        uart_tx,
    );

    loop {
        for byte in b"Hello, world!" {
            nb::block!(uart.write(*byte)).unwrap();
        }
        delay.delay_ms(1000);
    }
}
