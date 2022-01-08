#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_spi: [u8; 0x31],
}
impl RegisterBlock {
    #[doc = "0x00..0x31 - USART Mode"]
    #[inline(always)]
    pub fn usart(&self) -> &USART {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART) }
    }
    #[doc = "0x00..0x31 - SPI Mode"]
    #[inline(always)]
    pub fn spi(&self) -> &SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SPI) }
    }
    #[doc = "0x00..0x29 - I2C Slave Mode"]
    #[inline(always)]
    pub fn i2cs(&self) -> &I2CS {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const I2CS) }
    }
    #[doc = "0x00..0x31 - I2C Master Mode"]
    #[inline(always)]
    pub fn i2cm(&self) -> &I2CM {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const I2CM) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct I2CM {
    #[doc = "0x00 - I2CM Control A"]
    pub ctrla: crate::Reg<self::i2cm::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - I2CM Control B"]
    pub ctrlb: crate::Reg<self::i2cm::ctrlb::CTRLB_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - I2CM Baud Rate"]
    pub baud: crate::Reg<self::i2cm::baud::BAUD_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - I2CM Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::i2cm::intenclr::INTENCLR_SPEC>,
    _reserved4: [u8; 0x01],
    #[doc = "0x16 - I2CM Interrupt Enable Set"]
    pub intenset: crate::Reg<self::i2cm::intenset::INTENSET_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x18 - I2CM Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::i2cm::intflag::INTFLAG_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x1a - I2CM Status"]
    pub status: crate::Reg<self::i2cm::status::STATUS_SPEC>,
    #[doc = "0x1c - I2CM Syncbusy"]
    pub syncbusy: crate::Reg<self::i2cm::syncbusy::SYNCBUSY_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - I2CM Address"]
    pub addr: crate::Reg<self::i2cm::addr::ADDR_SPEC>,
    #[doc = "0x28 - I2CM Data"]
    pub data: crate::Reg<self::i2cm::data::DATA_SPEC>,
    _reserved10: [u8; 0x07],
    #[doc = "0x30 - I2CM Debug Control"]
    pub dbgctrl: crate::Reg<self::i2cm::dbgctrl::DBGCTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "I2C Master Mode"]
pub mod i2cm;
#[doc = r"Register block"]
#[repr(C)]
pub struct I2CS {
    #[doc = "0x00 - I2CS Control A"]
    pub ctrla: crate::Reg<self::i2cs::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - I2CS Control B"]
    pub ctrlb: crate::Reg<self::i2cs::ctrlb::CTRLB_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x14 - I2CS Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::i2cs::intenclr::INTENCLR_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x16 - I2CS Interrupt Enable Set"]
    pub intenset: crate::Reg<self::i2cs::intenset::INTENSET_SPEC>,
    _reserved4: [u8; 0x01],
    #[doc = "0x18 - I2CS Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::i2cs::intflag::INTFLAG_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x1a - I2CS Status"]
    pub status: crate::Reg<self::i2cs::status::STATUS_SPEC>,
    #[doc = "0x1c - I2CS Syncbusy"]
    pub syncbusy: crate::Reg<self::i2cs::syncbusy::SYNCBUSY_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x24 - I2CS Address"]
    pub addr: crate::Reg<self::i2cs::addr::ADDR_SPEC>,
    #[doc = "0x28 - I2CS Data"]
    pub data: crate::Reg<self::i2cs::data::DATA_SPEC>,
}
#[doc = r"Register block"]
#[doc = "I2C Slave Mode"]
pub mod i2cs;
#[doc = r"Register block"]
#[repr(C)]
pub struct SPI {
    #[doc = "0x00 - SPI Control A"]
    pub ctrla: crate::Reg<self::spi::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - SPI Control B"]
    pub ctrlb: crate::Reg<self::spi::ctrlb::CTRLB_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - SPI Baud Rate"]
    pub baud: crate::Reg<self::spi::baud::BAUD_SPEC>,
    _reserved3: [u8; 0x07],
    #[doc = "0x14 - SPI Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::spi::intenclr::INTENCLR_SPEC>,
    _reserved4: [u8; 0x01],
    #[doc = "0x16 - SPI Interrupt Enable Set"]
    pub intenset: crate::Reg<self::spi::intenset::INTENSET_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x18 - SPI Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::spi::intflag::INTFLAG_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x1a - SPI Status"]
    pub status: crate::Reg<self::spi::status::STATUS_SPEC>,
    #[doc = "0x1c - SPI Syncbusy"]
    pub syncbusy: crate::Reg<self::spi::syncbusy::SYNCBUSY_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - SPI Address"]
    pub addr: crate::Reg<self::spi::addr::ADDR_SPEC>,
    #[doc = "0x28 - SPI Data"]
    pub data: crate::Reg<self::spi::data::DATA_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - SPI Debug Control"]
    pub dbgctrl: crate::Reg<self::spi::dbgctrl::DBGCTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SPI Mode"]
pub mod spi;
#[doc = r"Register block"]
#[repr(C)]
pub struct USART {
    #[doc = "0x00 - USART Control A"]
    pub ctrla: crate::Reg<self::usart::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - USART Control B"]
    pub ctrlb: crate::Reg<self::usart::ctrlb::CTRLB_SPEC>,
    _reserved2: [u8; 0x04],
    _reserved_2_baud: [u8; 0x02],
    #[doc = "0x0e - USART Receive Pulse Length"]
    pub rxpl: crate::Reg<self::usart::rxpl::RXPL_SPEC>,
    _reserved4: [u8; 0x05],
    #[doc = "0x14 - USART Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::usart::intenclr::INTENCLR_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x16 - USART Interrupt Enable Set"]
    pub intenset: crate::Reg<self::usart::intenset::INTENSET_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x18 - USART Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::usart::intflag::INTFLAG_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x1a - USART Status"]
    pub status: crate::Reg<self::usart::status::STATUS_SPEC>,
    #[doc = "0x1c - USART Syncbusy"]
    pub syncbusy: crate::Reg<self::usart::syncbusy::SYNCBUSY_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x28 - USART Data"]
    pub data: crate::Reg<self::usart::data::DATA_SPEC>,
    _reserved10: [u8; 0x06],
    #[doc = "0x30 - USART Debug Control"]
    pub dbgctrl: crate::Reg<self::usart::dbgctrl::DBGCTRL_SPEC>,
}
impl USART {
    #[doc = "0x0c - USART Baud Rate"]
    #[inline(always)]
    pub fn baud_usartfp_mode(
        &self,
    ) -> &crate::Reg<self::usart::baud_usartfp_mode::BAUD_USARTFP_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart::baud_usartfp_mode::BAUD_USARTFP_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART Baud Rate"]
    #[inline(always)]
    pub fn baud_fracfp_mode(
        &self,
    ) -> &crate::Reg<self::usart::baud_fracfp_mode::BAUD_FRACFP_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart::baud_fracfp_mode::BAUD_FRACFP_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART Baud Rate"]
    #[inline(always)]
    pub fn baud_frac_mode(&self) -> &crate::Reg<self::usart::baud_frac_mode::BAUD_FRAC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart::baud_frac_mode::BAUD_FRAC_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART Baud Rate"]
    #[inline(always)]
    pub fn baud(&self) -> &crate::Reg<self::usart::baud::BAUD_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart::baud::BAUD_SPEC>)
        }
    }
}
#[doc = r"Register block"]
#[doc = "USART Mode"]
pub mod usart;
