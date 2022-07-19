//! SAM E54 XPlained Pro Sercom Pad Definitions
//! 
//! These type definitions are used by the Sam E54 Xplained Pro for its various Sercom functions to Ext 1, 2, and 3, 
//! as well as the DGI and EDBG ports.


use super::hal;
use super::pins::*;
use hal::sercom::{uart, i2c, spi};
use hal::sercom::{Sercom0, Sercom1, Sercom2, Sercom3, Sercom4, Sercom5, Sercom6, Sercom7};
use hal::sercom::{IoSet1, IoSet2, IoSet3, IoSet4};

pub type Ext1UartPads = uart::Pads<Sercom0, IoSet3, Ext1UsartRx, Ext1UsartTx>;

pub type Ext1UartFlowControlPads = uart::Pads<Sercom0, IoSet3, Ext1UsartRx, Ext1UsartTx, Ext1UsartRts, Ext1UsartCts>;

pub type Ext3UartPads = uart::Pads<Sercom1, IoSet2, Ext3UsartRx, Ext3UsartTx>;

pub type EdbgUartPads = uart::Pads<Sercom2, IoSet4, EdbgUartRx, EdbgUartTx>;

pub type Ext1I2cPads = i2c::Pads<Sercom3, IoSet1, Ext1I2cSda, Ext1I2cScl>;

pub type Ext1SpiPads = spi::Pads<Sercom4, IoSet4, Ext1SpiMiso, Ext1SpiMosi, Ext1SpiSck, Ext1SpiCsA>;

pub type Ext2UartPads = uart::Pads<Sercom5, IoSet1, Ext2UsartRx, Ext2UsartTx>;

pub type Ext2SpiPads = spi::Pads<Sercom6, IoSet2, Ext2SpiMiso, Ext2SpiMosi, Ext2SpiSck, Ext2SpiCsA>;

pub type Ext3SpiPads = spi::Pads<Sercom6, IoSet2, Ext3SpiMiso, Ext3SpiMosi, Ext3SpiSck, Ext3SpiCsA>;

pub type DgiSpiPads = spi::Pads<Sercom6, IoSet2, DgiSpiMiso, DgiSpiMosi, DgiSpiSck, DgiSpiCs>;

pub type Ext2I2cPads = i2c::Pads<Sercom7, IoSet2, Ext2I2cSda, Ext2I2cScl>;

pub type Ext3I2cPads = i2c::Pads<Sercom7, IoSet2, Ext3I2cSda, Ext3I2cScl>;

pub type DgiI2cPads = i2c::Pads<Sercom7, IoSet2, Ext3I2cSda, Ext3I2cScl>;
