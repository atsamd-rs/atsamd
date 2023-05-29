//! SAM E54 XPlained Pro sercom device definitions
//!
//! These type definitions are used by the Sam E54 Xplained Pro for its
//! various Sercom functions in extensions 1, 2, and 3,
//! as well as the DGI and EDBG ports and USB connections.

use super::hal;
use super::pins::*;
use hal::clock::GenericClockController;
use hal::pac;
use hal::sercom::{i2c, spi, uart};
use hal::sercom::{IoSet1, IoSet2, IoSet3, IoSet4};
use hal::time::Hertz;
use uart::{BaudMode, Oversampling};

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

hal::bsp_peripherals!(
    SERCOM0 { Ext1UartSercom }
    SERCOM1 { Ext3UartSercom }
    SERCOM2 { EdbgUartSercom }
    SERCOM3 { Ext1I2cSercom }
    SERCOM4 { Ext1SpiSercom }
    SERCOM5 { Ext2UartSercom }
    SERCOM6 { Ext2SpiSercom }
    SERCOM6 { Ext3SpiSercom }
    SERCOM6 { DgiSpiSercom }
    SERCOM7 { Ext2I2cSercom }
    SERCOM7 { Ext3I2cSercom }
    SERCOM7 { DgiI2cSercom }
);

/// UART pads for the extension 1 connection
pub type Ext1UartPads = uart::Pads<Ext1UartSercom, IoSet3, Ext1UartRx, Ext1UartTx>;

/// Extension 1 UART device
pub type Ext1Uart = uart::Uart<uart::Config<Ext1UartPads>, uart::Duplex>;

/// Set up the extension 1 UART device
pub fn ext1_uart(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext1_uart_sercom: Ext1UartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<Ext1UartRx>,
    uart_tx: impl Into<Ext1UartTx>,
) -> Ext1Uart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom0_core(&gclk0).unwrap();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, ext1_uart_sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// UART pads for the extension 1 connection with flow control
pub type Ext1FlowControlUartPads =
    uart::Pads<Ext1UartSercom, IoSet3, Ext1UartRx, Ext1UartTx, Ext1UartRts, Ext1UartCts>;

/// Extension 1 UART device with flow control
pub type Ext1FlowControlUart = uart::Uart<uart::Config<Ext1FlowControlUartPads>, uart::Duplex>;

/// Set up the extension 1 UART device with flow control
#[allow(clippy::too_many_arguments)]
pub fn ext1_flow_control_uart(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext1_uart_sercom: Ext1UartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<Ext1UartRx>,
    uart_tx: impl Into<Ext1UartTx>,
    uart_rts: impl Into<Ext1UartRts>,
    uart_cts: impl Into<Ext1UartCts>,
) -> Ext1FlowControlUart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom0_core(&gclk0).unwrap();
    let pads = uart::Pads::default()
        .rx(uart_rx.into())
        .tx(uart_tx.into())
        .rts(uart_rts.into())
        .cts(uart_cts.into());
    uart::Config::new(mclk, ext1_uart_sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// UART pads for the extension 3 connection
pub type Ext3UartPads = uart::Pads<Ext3UartSercom, IoSet2, Ext3UartRx, Ext3UartTx>;

/// Extension 3 UART device
pub type Ext3Uart = uart::Uart<uart::Config<Ext3UartPads>, uart::Duplex>;

/// Set up the extension 3 UART device
pub fn ext3_uart(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext3_uart_sercom: Ext3UartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<Ext3UartRx>,
    uart_tx: impl Into<Ext3UartTx>,
) -> Ext3Uart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom1_core(&gclk0).unwrap();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, ext3_uart_sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// UART pads for the EDBG connection
pub type EdbgUartPads = uart::Pads<EdbgUartSercom, IoSet4, EdbgUartRx, EdbgUartTx>;

/// EDBG connection UART device
pub type EdbgUart = uart::Uart<uart::Config<EdbgUartPads>, uart::Duplex>;

/// Set up the EDBG UART device
pub fn edbg_uart(
    clocks: &mut GenericClockController,
    baud: Hertz,
    edbg_uart_sercom: EdbgUartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<EdbgUartRx>,
    uart_tx: impl Into<EdbgUartTx>,
) -> EdbgUart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom2_core(&gclk0).unwrap();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, edbg_uart_sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// I2C pads for the extension 1 connection
pub type Ext1I2cPads = i2c::Pads<Ext1I2cSercom, IoSet1, Ext1I2cSda, Ext1I2cScl>;

/// Extension 1 I2C device
pub type Ext1I2c = i2c::I2c<i2c::Config<Ext1I2cPads>>;

/// Set up the extension 1 I2C device
pub fn ext1_i2c(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext1_i2c_sercom: Ext1I2cSercom,
    mclk: &mut pac::MCLK,
    sda: impl Into<Ext1I2cSda>,
    scl: impl Into<Ext1I2cScl>,
) -> Ext1I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom3_core(&gclk0).unwrap();
    let freq = clock.freq();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(mclk, ext1_i2c_sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// SPI pads for the extension 1 connection
pub type Ext1SpiPads = spi::Pads<Ext1SpiSercom, IoSet4, Ext1SpiMiso, Ext1SpiMosi, Ext1SpiSck>;

/// Extension 1 SPI device
pub type Ext1Spi = spi::Spi<spi::Config<Ext1SpiPads>, spi::Duplex>;

/// Set up the extension 1 SPI device
pub fn ext1_spi(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext1_spi_sercom: Ext1SpiSercom,
    mclk: &mut pac::MCLK,
    miso: impl Into<Ext1SpiMiso>,
    mosi: impl Into<Ext1SpiMosi>,
    sck: impl Into<Ext1SpiSck>,
) -> Ext1Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom4_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sck) = (miso.into(), mosi.into(), sck.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sck);
    spi::Config::new(mclk, ext1_spi_sercom, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// UART pads for the extension 2 connection
pub type Ext2UartPads = uart::Pads<Ext2UartSercom, IoSet1, Ext2UartRx, Ext2UartTx>;

/// Extension 2 UART device
pub type Ext2Uart = uart::Uart<uart::Config<Ext2UartPads>, uart::Duplex>;

/// Set up the extension 2 UART device
pub fn ext2_uart(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext2_uart_sercom: Ext2UartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<Ext2UartRx>,
    uart_tx: impl Into<Ext2UartTx>,
) -> Ext2Uart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom5_core(&gclk0).unwrap();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, ext2_uart_sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// SPI pads for the extension 2 connection
pub type Ext2SpiPads = spi::Pads<Ext2SpiSercom, IoSet2, Ext2SpiMiso, Ext2SpiMosi, Ext2SpiSck>;

/// Extension 1 SPI device
pub type Ext2Spi = spi::Spi<spi::Config<Ext2SpiPads>, spi::Duplex>;

/// Set up the extension 2 SPI device
pub fn ext2_spi(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext2_spi_sercom: Ext2SpiSercom,
    mclk: &mut pac::MCLK,
    miso: impl Into<Ext2SpiMiso>,
    mosi: impl Into<Ext2SpiMosi>,
    sck: impl Into<Ext2SpiSck>,
) -> Ext2Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom6_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sck) = (miso.into(), mosi.into(), sck.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sck);
    spi::Config::new(mclk, ext2_spi_sercom, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// SPI pads for the extension 3 connection
pub type Ext3SpiPads = spi::Pads<Ext3SpiSercom, IoSet2, Ext3SpiMiso, Ext3SpiMosi, Ext3SpiSck>;

/// Extension 3 SPI device
pub type Ext3Spi = spi::Spi<spi::Config<Ext3SpiPads>, spi::Duplex>;

/// Set up the extension 3 SPI device
pub fn ext3_spi(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext3_spi_sercom: Ext3SpiSercom,
    mclk: &mut pac::MCLK,
    miso: impl Into<Ext3SpiMiso>,
    mosi: impl Into<Ext3SpiMosi>,
    sck: impl Into<Ext3SpiSck>,
) -> Ext3Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom6_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sck) = (miso.into(), mosi.into(), sck.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sck);
    spi::Config::new(mclk, ext3_spi_sercom, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// SPI pads for the DGI connection
pub type DgiSpiPads = spi::Pads<DgiSpiSercom, IoSet2, DgiSpiMiso, DgiSpiMosi, DgiSpiSck>;

/// DGI SPI device
pub type DgiSpi = spi::Spi<spi::Config<DgiSpiPads>, spi::Duplex>;

/// Set up the DGI SPI device
pub fn dgi_spi(
    clocks: &mut GenericClockController,
    baud: Hertz,
    dgi_spi_sercom: DgiSpiSercom,
    mclk: &mut pac::MCLK,
    miso: impl Into<DgiSpiMiso>,
    mosi: impl Into<DgiSpiMosi>,
    sck: impl Into<DgiSpiSck>,
) -> DgiSpi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom6_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sck) = (miso.into(), mosi.into(), sck.into());
    let pads = spi::Pads::default().data_in(miso).data_out(mosi).sclk(sck);
    spi::Config::new(mclk, dgi_spi_sercom, pads, freq)
        .baud(baud)
        .spi_mode(spi::MODE_0)
        .enable()
}

/// I2C pads for the extension 2 connection
pub type Ext2I2cPads = i2c::Pads<Ext2I2cSercom, IoSet2, Ext2I2cSda, Ext2I2cScl>;

/// Extension 2 I2C device
pub type Ext2I2c = i2c::I2c<i2c::Config<Ext2I2cPads>>;

/// Set up the extension 1 I2C device
pub fn ext2_i2c(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext2_i2c_sercom: Ext2I2cSercom,
    mclk: &mut pac::MCLK,
    sda: impl Into<Ext2I2cSda>,
    scl: impl Into<Ext2I2cScl>,
) -> Ext2I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom7_core(&gclk0).unwrap();
    let freq = clock.freq();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(mclk, ext2_i2c_sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// I2C pads for the extension 3 connection
pub type Ext3I2cPads = i2c::Pads<Ext3I2cSercom, IoSet2, Ext3I2cSda, Ext3I2cScl>;

/// Extension 3 I2C device
pub type Ext3I2c = i2c::I2c<i2c::Config<Ext3I2cPads>>;

/// Set up the extension 1 I2C device
pub fn ext3_i2c(
    clocks: &mut GenericClockController,
    baud: Hertz,
    ext3_i2c_sercom: Ext3I2cSercom,
    mclk: &mut pac::MCLK,
    sda: impl Into<Ext3I2cSda>,
    scl: impl Into<Ext3I2cScl>,
) -> Ext3I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom7_core(&gclk0).unwrap();
    let freq = clock.freq();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(mclk, ext3_i2c_sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// I2C pads for the DGI connection
pub type DgiI2cPads = i2c::Pads<DgiI2cSercom, IoSet2, Ext3I2cSda, Ext3I2cScl>;

/// DGI I2C device
pub type DgiI2c = i2c::I2c<i2c::Config<DgiI2cPads>>;

/// Set up the DGI I2C device
pub fn dgi_i2c(
    clocks: &mut GenericClockController,
    baud: Hertz,
    dgi_i2c_sercom: DgiI2cSercom,
    mclk: &mut pac::MCLK,
    sda: impl Into<DgiI2cSda>,
    scl: impl Into<DgiI2cScl>,
) -> DgiI2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom7_core(&gclk0).unwrap();
    let freq = clock.freq();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(mclk, dgi_i2c_sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// Convenience for setting up the USB
#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    mclk: &mut pac::MCLK,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    use pac::gclk::{genctrl::SRC_A, pchctrl::GEN_A};

    clocks.configure_gclk_divider_and_source(GEN_A::GCLK2, 1, SRC_A::DFLL, false);
    let usb_gclk = clocks.get_gclk(GEN_A::GCLK2).unwrap();
    let usb_clock = &clocks.usb(&usb_gclk).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(usb_clock, mclk, dm, dp, usb))
}
