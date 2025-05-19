use crate::hal;
use hal::pac;

use embedded_hal;

use hal::clock::GenericClockController;
use hal::sercom::{i2c, spi, uart};
use hal::time::Hertz;
use uart::{BaudMode, Oversampling};

use crate::pins::*;

hal::bsp_peripherals!(
    SERCOM1 { SpiSercom }
    SERCOM2 { I2cSercom }
    SERCOM5 { UartSercom }
);

/// SPI pads for the labelled SPI peripheral
pub type SpiPads = spi::Pads<SpiSercom, SpiCipo, SpiCopi, SpiSck>;

/// SPI master for the labelled SPI peripheral
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Set up the SPI peripheral
#[allow(clippy::too_many_arguments)]
pub fn setup_spi(
    clocks: &mut GenericClockController,
    baud: Hertz,
    spi_sercom: SpiSercom,
    pm: &pac::PM,
    cipo: impl Into<SpiCipo>,
    copi: impl Into<SpiCopi>,
    sck: impl Into<SpiSck>,
    mode: embedded_hal::spi::Mode,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom1_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (cipo, copi, sck) = (cipo.into(), copi.into(), sck.into());
    let pads = spi::Pads::default().data_in(cipo).data_out(copi).sclk(sck);
    spi::Config::new(pm, spi_sercom, pads, freq)
        .baud(baud)
        .spi_mode(mode)
        .enable()
}

/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined [`i2c::Config`]
/// configurations.
pub type I2cPads = i2c::Pads<I2cSercom, I2cSda, I2cScl>;

/// I2C master for the labelled I2C peripheral
///
/// This type implements [`Read`](ehal::blocking::i2c::Read),
/// [`Write`](ehal::blocking::i2c::Write) and
/// [`WriteRead`](ehal::blocking::i2c::WriteRead).
pub type I2c = i2c::I2c<i2c::Config<I2cPads>>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn setup_i2c(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: I2cSercom,
    pm: &mut pac::PM,
    sda: impl Into<I2cSda>,
    sck: impl Into<I2cScl>,
) -> I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom0_core(&gclk0).unwrap();
    let freq = clock.freq();
    let baud = baud.into();
    let pads = i2c::Pads::new(sda.into(), sck.into());
    i2c::Config::new(pm, sercom, pads, freq).baud(baud).enable()
}

/// UART pads for the labelled RX & TX pins
pub type UartPads = uart::Pads<UartSercom, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: UartSercom,
    pm: &mut pac::PM,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom5_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}
