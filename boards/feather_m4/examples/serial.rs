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
use hal::pac::gclk::genctrl::SRCSELECT_A;
use hal::pac::gclk::pchctrl::GENSELECT_A;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    clocks.configure_gclk_divider_and_source(GENSELECT_A::GCLK2, 1, SRCSELECT_A::DFLL, false);

    let pins = bsp::Pins::new(peripherals.PORT);
    let uart_rx = pin_alias!(pins.uart_rx);
    let uart_tx = pin_alias!(pins.uart_tx);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let uart_sercom = periph_alias!(peripherals.uart_sercom);

    let mut uart = bsp::uart(
        &mut clocks,
        19200.Hz(),
        uart_sercom,
        &mut peripherals.MCLK,
        uart_rx,
        uart_tx,
    );

    loop {
        for byte in b"Hello, world!" {
            nb::block!(uart.write(*byte)).unwrap();
        }
        delay.delay_ms(1000u16);
    }
}
