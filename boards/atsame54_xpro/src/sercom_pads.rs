//! SAM E54 XPlained Pro Sercom Pad Definitions
//!
//! These type definitions are used by the Sam E54 Xplained Pro for its various Sercom functions to Ext 1, 2, and 3,
//! as well as the DGI and EDBG ports.

use super::hal;
use super::pins::*;
use hal::sercom::{i2c, spi, uart};
use hal::sercom::{IoSet1, IoSet2, IoSet3, IoSet4};

hal::bsp_peripherals!(
    SERCOM0 { Ext1Uart }
    SERCOM1 { Ext3Uart }
    SERCOM2 { EdbgUart }
    SERCOM3 { Ext1I2c }
    SERCOM4 { Ext1Spi }
    SERCOM5 { Ext2Uart }
    SERCOM6 { Ext2Spi }
    SERCOM6 { Ext3Spi }
    SERCOM6 { DgiSpi }
    SERCOM7 { Ext2I2c }
    SERCOM7 { Ext3I2c }
    SERCOM7 { DgiI2c }
);

pub type Ext1UartPads = uart::Pads<Ext1Uart, IoSet3, Ext1UsartRx, Ext1UsartTx>;

pub type Ext1FlowControlUartPads =
    uart::Pads<Ext1Uart, IoSet3, Ext1UsartRx, Ext1UsartTx, Ext1UsartRts, Ext1UsartCts>;

pub type Ext3UartPads = uart::Pads<Ext3Uart, IoSet2, Ext3UsartRx, Ext3UsartTx>;

pub type EdbgUartPads = uart::Pads<EdbgUart, IoSet4, EdbgUartRx, EdbgUartTx>;

pub type Ext1I2cPads = i2c::Pads<Ext1I2c, IoSet1, Ext1I2cSda, Ext1I2cScl>;

pub type Ext1SpiPads = spi::Pads<Ext1Spi, IoSet4, Ext1SpiMiso, Ext1SpiMosi, Ext1SpiSck, Ext1SpiCsA>;

pub type Ext2UartPads = uart::Pads<Ext2Uart, IoSet1, Ext2UsartRx, Ext2UsartTx>;

pub type Ext2SpiPads = spi::Pads<Ext2Spi, IoSet2, Ext2SpiMiso, Ext2SpiMosi, Ext2SpiSck, Ext2SpiCsA>;

pub type Ext3SpiPads = spi::Pads<Ext3Spi, IoSet2, Ext3SpiMiso, Ext3SpiMosi, Ext3SpiSck, Ext3SpiCsA>;

pub type DgiSpiPads = spi::Pads<DgiSpi, IoSet2, DgiSpiMiso, DgiSpiMosi, DgiSpiSck, DgiSpiCs>;

pub type Ext2I2cPads = i2c::Pads<Ext2I2c, IoSet2, Ext2I2cSda, Ext2I2cScl>;

pub type Ext3I2cPads = i2c::Pads<Ext3I2c, IoSet2, Ext3I2cSda, Ext3I2cScl>;

pub type DgiI2cPads = i2c::Pads<DgiI2c, IoSet2, Ext3I2cSda, Ext3I2cScl>;
