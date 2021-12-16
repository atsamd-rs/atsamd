#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_spi: [u8; 26usize],
}
impl RegisterBlock {
    #[doc = "0x00 - USART Mode"]
    #[inline(always)]
    pub fn usart(&self) -> &USART {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART) }
    }
    #[doc = "0x00 - USART Mode"]
    #[inline(always)]
    pub fn usart_mut(&self) -> &mut USART {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut USART) }
    }
    #[doc = "0x00 - SPI Mode"]
    #[inline(always)]
    pub fn spi(&self) -> &SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SPI) }
    }
    #[doc = "0x00 - SPI Mode"]
    #[inline(always)]
    pub fn spi_mut(&self) -> &mut SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut SPI) }
    }
    #[doc = "0x00 - I2C Slave Mode"]
    #[inline(always)]
    pub fn i2cs(&self) -> &I2CS {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const I2CS) }
    }
    #[doc = "0x00 - I2C Slave Mode"]
    #[inline(always)]
    pub fn i2cs_mut(&self) -> &mut I2CS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut I2CS) }
    }
    #[doc = "0x00 - I2C Master Mode"]
    #[inline(always)]
    pub fn i2cm(&self) -> &I2CM {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const I2CM) }
    }
    #[doc = "0x00 - I2C Master Mode"]
    #[inline(always)]
    pub fn i2cm_mut(&self) -> &mut I2CM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut I2CM) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct I2CM {
    #[doc = "0x00 - I2CM Control A"]
    pub ctrla: self::i2cm::CTRLA,
    #[doc = "0x04 - I2CM Control B"]
    pub ctrlb: self::i2cm::CTRLB,
    #[doc = "0x08 - I2CM Debug Control"]
    pub dbgctrl: self::i2cm::DBGCTRL,
    _reserved3: [u8; 1usize],
    #[doc = "0x0a - I2CM Baud Rate"]
    pub baud: self::i2cm::BAUD,
    #[doc = "0x0c - I2CM Interrupt Enable Clear"]
    pub intenclr: self::i2cm::INTENCLR,
    #[doc = "0x0d - I2CM Interrupt Enable Set"]
    pub intenset: self::i2cm::INTENSET,
    #[doc = "0x0e - I2CM Interrupt Flag Status and Clear"]
    pub intflag: self::i2cm::INTFLAG,
    _reserved7: [u8; 1usize],
    #[doc = "0x10 - I2CM Status"]
    pub status: self::i2cm::STATUS,
    _reserved8: [u8; 2usize],
    #[doc = "0x14 - I2CM Address"]
    pub addr: self::i2cm::ADDR,
    _reserved9: [u8; 3usize],
    #[doc = "0x18 - I2CM Data"]
    pub data: self::i2cm::DATA,
}
#[doc = r"Register block"]
#[doc = "I2C Master Mode"]
pub mod i2cm;
#[doc = r"Register block"]
#[repr(C)]
pub struct I2CS {
    #[doc = "0x00 - I2CS Control A"]
    pub ctrla: self::i2cs::CTRLA,
    #[doc = "0x04 - I2CS Control B"]
    pub ctrlb: self::i2cs::CTRLB,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - I2CS Interrupt Enable Clear"]
    pub intenclr: self::i2cs::INTENCLR,
    #[doc = "0x0d - I2CS Interrupt Enable Set"]
    pub intenset: self::i2cs::INTENSET,
    #[doc = "0x0e - I2CS Interrupt Flag Status and Clear"]
    pub intflag: self::i2cs::INTFLAG,
    _reserved5: [u8; 1usize],
    #[doc = "0x10 - I2CS Status"]
    pub status: self::i2cs::STATUS,
    _reserved6: [u8; 2usize],
    #[doc = "0x14 - I2CS Address"]
    pub addr: self::i2cs::ADDR,
    #[doc = "0x18 - I2CS Data"]
    pub data: self::i2cs::DATA,
}
#[doc = r"Register block"]
#[doc = "I2C Slave Mode"]
pub mod i2cs;
#[doc = r"Register block"]
#[repr(C)]
pub struct SPI {
    #[doc = "0x00 - SPI Control A"]
    pub ctrla: self::spi::CTRLA,
    #[doc = "0x04 - SPI Control B"]
    pub ctrlb: self::spi::CTRLB,
    #[doc = "0x08 - SPI Debug Control"]
    pub dbgctrl: self::spi::DBGCTRL,
    _reserved3: [u8; 1usize],
    #[doc = "0x0a - SPI Baud Rate"]
    pub baud: self::spi::BAUD,
    _reserved4: [u8; 1usize],
    #[doc = "0x0c - SPI Interrupt Enable Clear"]
    pub intenclr: self::spi::INTENCLR,
    #[doc = "0x0d - SPI Interrupt Enable Set"]
    pub intenset: self::spi::INTENSET,
    #[doc = "0x0e - SPI Interrupt Flag Status and Clear"]
    pub intflag: self::spi::INTFLAG,
    _reserved7: [u8; 1usize],
    #[doc = "0x10 - SPI Status"]
    pub status: self::spi::STATUS,
    _reserved8: [u8; 2usize],
    #[doc = "0x14 - SPI Address"]
    pub addr: self::spi::ADDR,
    #[doc = "0x18 - SPI Data"]
    pub data: self::spi::DATA,
}
#[doc = r"Register block"]
#[doc = "SPI Mode"]
pub mod spi;
#[doc = r"Register block"]
#[repr(C)]
pub struct USART {
    #[doc = "0x00 - USART Control A"]
    pub ctrla: self::usart::CTRLA,
    #[doc = "0x04 - USART Control B"]
    pub ctrlb: self::usart::CTRLB,
    #[doc = "0x08 - USART Debug Control"]
    pub dbgctrl: self::usart::DBGCTRL,
    _reserved3: [u8; 1usize],
    #[doc = "0x0a - USART Baud"]
    pub baud: self::usart::BAUD,
    #[doc = "0x0c - USART Interrupt Enable Clear"]
    pub intenclr: self::usart::INTENCLR,
    #[doc = "0x0d - USART Interrupt Enable Set"]
    pub intenset: self::usart::INTENSET,
    #[doc = "0x0e - USART Interrupt Flag Status and Clear"]
    pub intflag: self::usart::INTFLAG,
    _reserved7: [u8; 1usize],
    #[doc = "0x10 - USART Status"]
    pub status: self::usart::STATUS,
    _reserved8: [u8; 6usize],
    #[doc = "0x18 - USART Data"]
    pub data: self::usart::DATA,
}
#[doc = r"Register block"]
#[doc = "USART Mode"]
pub mod usart;
