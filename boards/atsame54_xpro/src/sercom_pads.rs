//! SAM E54 XPlained Pro Sercom Pad Definitions
//!
//! These type definitions are used by the Sam E54 Xplained Pro for its various Sercom functions to Ext 1, 2, and 3,
//! as well as the DGI and EDBG ports.

use super::hal;
use super::pins::*;
use hal::sercom::{i2c, spi, uart};
use uart::{BaudMode, Oversampling};
use hal::sercom::{IoSet1, IoSet2, IoSet3, IoSet4};
use hal::time::Hertz;
use hal::clock::GenericClockController;
use hal::pac;


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
pub type Ext1UartPads = uart::Pads<Ext1UartSercom, IoSet3, Ext1UsartRx, Ext1UsartTx>;

/// Extension 1 UART device
pub type Ext1Uart = uart::Uart<uart::Config<Ext1UartPads>, uart::Duplex>;

/// Set up the extension 1 UART device
pub fn ext1_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    ext1_uart_sercom: Ext1UartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<Ext1UsartRx>,
    uart_tx: impl Into<Ext1UsartTx>,
) -> Ext1Uart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, ext1_uart_sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}


/// UART pads for the extension 1 connection with flow control
pub type Ext1FlowControlUartPads =
    uart::Pads<Ext1UartSercom, IoSet3, Ext1UsartRx, Ext1UsartTx, Ext1UsartRts, Ext1UsartCts>;

/// Extension 1 UART device with flow control
pub type Ext1FlowControlUart = uart::Uart<uart::Config<Ext1FlowControlUartPads>, uart::Duplex>;

/// Set up the extension 1 UART device with flow control
pub fn ext1_flow_control_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    ext1_uart_sercom: Ext1UartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<Ext1UsartRx>,
    uart_tx: impl Into<Ext1UsartTx>,
    uart_rts: impl Into<Ext1UsartRts>,
    uart_cts: impl Into<Ext1UsartCts>,
) -> Ext1FlowControlUart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom0_core(&gclk0).unwrap();
    let baud = baud.into();
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
pub type Ext3UartPads = uart::Pads<Ext3UartSercom, IoSet2, Ext3UsartRx, Ext3UsartTx>;

/// Extension 3 UART device
pub type Ext3Uart = uart::Uart<uart::Config<Ext3UartPads>, uart::Duplex>;

/// Set up the extension 3 UART device
pub fn ext3_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    ext3_uart_sercom: Ext3UartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<Ext3UsartRx>,
    uart_tx: impl Into<Ext3UsartTx>,
) -> Ext3Uart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom1_core(&gclk0).unwrap();
    let baud = baud.into();
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
    baud: impl Into<Hertz>,
    edbg_uart_sercom: EdbgUartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<EdbgUartRx>,
    uart_tx: impl Into<EdbgUartTx>,
) -> EdbgUart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom2_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, edbg_uart_sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}


/// I2C pads for the extension 1 connection
pub type Ext1I2cPads = i2c::Pads<Ext1I2cSercom, IoSet1, Ext1I2cSda, Ext1I2cScl>;


/// SPI pads for the extension 1 connection
pub type Ext1SpiPads = spi::Pads<Ext1SpiSercom, IoSet4, Ext1SpiMiso, Ext1SpiMosi, Ext1SpiSck, Ext1SpiCsA>;


/// UART pads for the extension 2 connection
pub type Ext2UartPads = uart::Pads<Ext2UartSercom, IoSet1, Ext2UsartRx, Ext2UsartTx>;

/// Extension 2 UART device
pub type Ext2Uart = uart::Uart<uart::Config<Ext2UartPads>, uart::Duplex>;

/// Set up the extension UART device
pub fn ext2_uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    ext2_uart_sercom: Ext2UartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<Ext2UsartRx>,
    uart_tx: impl Into<Ext2UsartTx>,
) -> Ext2Uart {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom5_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, ext2_uart_sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// SPI pads for the extension 2 connection
pub type Ext2SpiPads = spi::Pads<Ext2SpiSercom, IoSet2, Ext2SpiMiso, Ext2SpiMosi, Ext2SpiSck, Ext2SpiCsA>;


/// SPI pads for the extension 3 connection
pub type Ext3SpiPads = spi::Pads<Ext3SpiSercom, IoSet2, Ext3SpiMiso, Ext3SpiMosi, Ext3SpiSck, Ext3SpiCsA>;


/// SPI pads for the DGI connection
pub type DgiSpiPads = spi::Pads<DgiSpiSercom, IoSet2, DgiSpiMiso, DgiSpiMosi, DgiSpiSck, DgiSpiCs>;


/// I2C pads for the extension 2 connection
pub type Ext2I2cPads = i2c::Pads<Ext2I2cSercom, IoSet2, Ext2I2cSda, Ext2I2cScl>;


/// I2C pads for the extension 3 connection
pub type Ext3I2cPads = i2c::Pads<Ext3I2cSercom, IoSet2, Ext3I2cSda, Ext3I2cScl>;


/// I2C pads for the DGI connection
pub type DgiI2cPads = i2c::Pads<DgiI2cSercom, IoSet2, Ext3I2cSda, Ext3I2cScl>;
