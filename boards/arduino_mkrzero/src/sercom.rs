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
    SERCOM4 { SdSercom }
    SERCOM5 { UartSercom }
);

/// SPI pads
pub type SpiPads = spi::Pads<SpiSercom, SpiCipo, SpiCopi, SpiSck>;

/// SPI peripheral type
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

/// I2C pads
pub type I2cPads = i2c::Pads<I2cSercom, I2cSda, I2cScl>;

/// Extension 2 I2C device
pub type I2c = i2c::I2c<i2c::Config<I2cPads>>;

/// Set up the extension 1 I2C device
pub fn setup_i2c(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext2_i2c_sercom: I2cSercom,
    pm: &pac::PM,
    sda: impl Into<I2cSda>,
    scl: impl Into<I2cScl>,
) -> I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom2_core(&gclk0).unwrap();
    let freq = clock.freq();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(pm, ext2_i2c_sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// SD Card pads
pub type SdPads = spi::Pads<SdSercom, SdCipo, SdCopi, SdSck>;

/// SD Card peripheral type
pub type Sd = spi::Spi<spi::Config<SdPads>, spi::Duplex>;

/// Set up the SD Card SPI peripheral
pub fn setup_sd(
    clocks: &mut GenericClockController,
    baud: Hertz,
    spi_sercom: SdSercom,
    pm: &pac::PM,
    cipo: impl Into<SdCipo>,
    copi: impl Into<SdCopi>,
    sck: impl Into<SdSck>,
) -> Sd {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom4_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (cipo, copi, sck) = (cipo.into(), copi.into(), sck.into());
    let pads = spi::Pads::default().data_in(cipo).data_out(copi).sclk(sck);
    spi::Config::new(pm, spi_sercom, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// UART pads
pub type UartPads = uart::Pads<UartSercom, UartRx, UartTx>;

/// UART device
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Set up the UART peripheral
pub fn setup_uart(
    clocks: &mut GenericClockController,
    baud: Hertz,
    uart_sercom: UartSercom,
    pm: &pac::PM,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom5_core(&gclk0).unwrap();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(pm, uart_sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}
