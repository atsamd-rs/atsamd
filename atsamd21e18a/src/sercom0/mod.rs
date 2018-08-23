#[doc = r" Register block"]
#[repr(C)]
pub union RegisterBlock {
    #[doc = "0x00 - USART Mode"]
    pub usart: USART,
    #[doc = "0x00 - SPI Mode"]
    pub spi: SPI,
    #[doc = "0x00 - I2C Slave Mode"]
    pub i2cs: I2CS,
    #[doc = "0x00 - I2C Master Mode"]
    pub i2cm: I2CM,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct I2CM {
    #[doc = "0x00 - I2CM Control A"]
    pub ctrla: self::i2cm::CTRLA,
    #[doc = "0x04 - I2CM Control B"]
    pub ctrlb: self::i2cm::CTRLB,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - I2CM Baud Rate"]
    pub baud: self::i2cm::BAUD,
    _reserved3: [u8; 4usize],
    #[doc = "0x14 - I2CM Interrupt Enable Clear"]
    pub intenclr: self::i2cm::INTENCLR,
    _reserved4: [u8; 1usize],
    #[doc = "0x16 - I2CM Interrupt Enable Set"]
    pub intenset: self::i2cm::INTENSET,
    _reserved5: [u8; 1usize],
    #[doc = "0x18 - I2CM Interrupt Flag Status and Clear"]
    pub intflag: self::i2cm::INTFLAG,
    _reserved6: [u8; 1usize],
    #[doc = "0x1a - I2CM Status"]
    pub status: self::i2cm::STATUS,
    #[doc = "0x1c - I2CM Syncbusy"]
    pub syncbusy: self::i2cm::SYNCBUSY,
    _reserved8: [u8; 4usize],
    #[doc = "0x24 - I2CM Address"]
    pub addr: self::i2cm::ADDR,
    #[doc = "0x28 - I2CM Data"]
    pub data: self::i2cm::DATA,
    _reserved10: [u8; 7usize],
    #[doc = "0x30 - I2CM Debug Control"]
    pub dbgctrl: self::i2cm::DBGCTRL,
}
#[doc = r" Register block"]
#[doc = "I2C Master Mode"]
pub mod i2cm;
#[doc = r" Register block"]
#[repr(C)]
pub struct I2CS {
    #[doc = "0x00 - I2CS Control A"]
    pub ctrla: self::i2cs::CTRLA,
    #[doc = "0x04 - I2CS Control B"]
    pub ctrlb: self::i2cs::CTRLB,
    _reserved2: [u8; 12usize],
    #[doc = "0x14 - I2CS Interrupt Enable Clear"]
    pub intenclr: self::i2cs::INTENCLR,
    _reserved3: [u8; 1usize],
    #[doc = "0x16 - I2CS Interrupt Enable Set"]
    pub intenset: self::i2cs::INTENSET,
    _reserved4: [u8; 1usize],
    #[doc = "0x18 - I2CS Interrupt Flag Status and Clear"]
    pub intflag: self::i2cs::INTFLAG,
    _reserved5: [u8; 1usize],
    #[doc = "0x1a - I2CS Status"]
    pub status: self::i2cs::STATUS,
    #[doc = "0x1c - I2CS Syncbusy"]
    pub syncbusy: self::i2cs::SYNCBUSY,
    _reserved7: [u8; 4usize],
    #[doc = "0x24 - I2CS Address"]
    pub addr: self::i2cs::ADDR,
    #[doc = "0x28 - I2CS Data"]
    pub data: self::i2cs::DATA,
}
#[doc = r" Register block"]
#[doc = "I2C Slave Mode"]
pub mod i2cs;
#[doc = r" Register block"]
#[repr(C)]
pub struct SPI {
    #[doc = "0x00 - SPI Control A"]
    pub ctrla: self::spi::CTRLA,
    #[doc = "0x04 - SPI Control B"]
    pub ctrlb: self::spi::CTRLB,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - SPI Baud Rate"]
    pub baud: self::spi::BAUD,
    _reserved3: [u8; 7usize],
    #[doc = "0x14 - SPI Interrupt Enable Clear"]
    pub intenclr: self::spi::INTENCLR,
    _reserved4: [u8; 1usize],
    #[doc = "0x16 - SPI Interrupt Enable Set"]
    pub intenset: self::spi::INTENSET,
    _reserved5: [u8; 1usize],
    #[doc = "0x18 - SPI Interrupt Flag Status and Clear"]
    pub intflag: self::spi::INTFLAG,
    _reserved6: [u8; 1usize],
    #[doc = "0x1a - SPI Status"]
    pub status: self::spi::STATUS,
    #[doc = "0x1c - SPI Syncbusy"]
    pub syncbusy: self::spi::SYNCBUSY,
    _reserved8: [u8; 4usize],
    #[doc = "0x24 - SPI Address"]
    pub addr: self::spi::ADDR,
    #[doc = "0x28 - SPI Data"]
    pub data: self::spi::DATA,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - SPI Debug Control"]
    pub dbgctrl: self::spi::DBGCTRL,
}
#[doc = r" Register block"]
#[doc = "SPI Mode"]
pub mod spi;
#[doc = r" Register block"]
#[repr(C)]
pub struct USART {
    #[doc = "0x00 - USART Control A"]
    pub ctrla: self::usart::CTRLA,
    #[doc = "0x04 - USART Control B"]
    pub ctrlb: self::usart::CTRLB,
    _reserved2: [u8; 4usize],
    #[doc = "USART Baud Rate"]
    pub baud: BAUD_UNION,
    #[doc = "0x0e - USART Receive Pulse Length"]
    pub rxpl: self::usart::RXPL,
    _reserved4: [u8; 5usize],
    #[doc = "0x14 - USART Interrupt Enable Clear"]
    pub intenclr: self::usart::INTENCLR,
    _reserved5: [u8; 1usize],
    #[doc = "0x16 - USART Interrupt Enable Set"]
    pub intenset: self::usart::INTENSET,
    _reserved6: [u8; 1usize],
    #[doc = "0x18 - USART Interrupt Flag Status and Clear"]
    pub intflag: self::usart::INTFLAG,
    _reserved7: [u8; 1usize],
    #[doc = "0x1a - USART Status"]
    pub status: self::usart::STATUS,
    #[doc = "0x1c - USART Syncbusy"]
    pub syncbusy: self::usart::SYNCBUSY,
    _reserved9: [u8; 8usize],
    #[doc = "0x28 - USART Data"]
    pub data: self::usart::DATA,
    _reserved10: [u8; 6usize],
    #[doc = "0x30 - USART Debug Control"]
    pub dbgctrl: self::usart::DBGCTRL,
}
#[doc = "USART Baud Rate"]
#[repr(C)]
pub union BAUD_UNION {
    #[doc = "0x0c - USART Baud Rate"]
    pub baud_usartfp_mode: self::usart::BAUD_USARTFP_MODE,
    #[doc = "0x0c - USART Baud Rate"]
    pub baud_fracfp_mode: self::usart::BAUD_FRACFP_MODE,
    #[doc = "0x0c - USART Baud Rate"]
    pub baud_frac_mode: self::usart::BAUD_FRAC_MODE,
    #[doc = "0x0c - USART Baud Rate"]
    pub baud: self::usart::BAUD,
}
#[doc = r" Register block"]
#[doc = "USART Mode"]
pub mod usart;
