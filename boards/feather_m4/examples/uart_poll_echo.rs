#![no_std]
#![no_main]

// Simple example of reading and writing to the UART
//
// Polls the UART in the main loop. Reads any bytes in
// and echoes them back out. You need to connect a
// TTL level serial port to the TX and RX pins in order
// to see the uart working.

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
use hal::ehal::digital::OutputPin;
use hal::ehal_nb::serial::{Read, Write};
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
    let mut delay = Delay::new(core.SYST, &mut clocks);
    let mut red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

    // Take RX and TX pins
    let uart_rx = pin_alias!(pins.uart_rx);
    let uart_tx = pin_alias!(pins.uart_tx);
    let uart_sercom = periph_alias!(peripherals.uart_sercom);

    let mut uart = bsp::uart(
        &mut clocks,
        19200.Hz(),
        uart_sercom,
        &mut peripherals.mclk,
        uart_rx,
        uart_tx,
    );

    // Write out a message on start up
    for byte in b"Hello, world!" {
        nb::block!(uart.write(*byte)).unwrap();
    }

    loop {
        match uart.read() {
            Ok(byte) => {
                nb::block!(uart.write(byte)).unwrap();

                // Blink the red led to show that a character has arrived
                red_led.set_high().unwrap();
                delay.delay_ms(2);
                red_led.set_low().unwrap();
            }
            Err(_) => delay.delay_ms(5),
        };
    }
}
