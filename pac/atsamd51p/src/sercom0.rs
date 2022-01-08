#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_i2cm: [u8; 0x31],
}
impl RegisterBlock {
    #[doc = "0x00..0x31 - USART INTERNAL CLOCK Mode"]
    #[inline(always)]
    pub fn usart_int(&self) -> &USART_INT {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART_INT) }
    }
    #[doc = "0x00..0x31 - USART EXTERNAL CLOCK Mode"]
    #[inline(always)]
    pub fn usart_ext(&self) -> &USART_EXT {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART_EXT) }
    }
    #[doc = "0x00..0x31 - SPI Master Mode"]
    #[inline(always)]
    pub fn spim(&self) -> &SPIM {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SPIM) }
    }
    #[doc = "0x00..0x31 - SPI Slave Mode"]
    #[inline(always)]
    pub fn spis(&self) -> &SPIS {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SPIS) }
    }
    #[doc = "0x00..0x2c - I2C Slave Mode"]
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
    #[doc = "0x08 - I2CM Control C"]
    pub ctrlc: crate::Reg<self::i2cm::ctrlc::CTRLC_SPEC>,
    #[doc = "0x0c - I2CM Baud Rate"]
    pub baud: crate::Reg<self::i2cm::baud::BAUD_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - I2CM Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::i2cm::intenclr::INTENCLR_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x16 - I2CM Interrupt Enable Set"]
    pub intenset: crate::Reg<self::i2cm::intenset::INTENSET_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x18 - I2CM Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::i2cm::intflag::INTFLAG_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x1a - I2CM Status"]
    pub status: crate::Reg<self::i2cm::status::STATUS_SPEC>,
    #[doc = "0x1c - I2CM Synchronization Busy"]
    pub syncbusy: crate::Reg<self::i2cm::syncbusy::SYNCBUSY_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x24 - I2CM Address"]
    pub addr: crate::Reg<self::i2cm::addr::ADDR_SPEC>,
    #[doc = "0x28 - I2CM Data"]
    pub data: crate::Reg<self::i2cm::data::DATA_SPEC>,
    _reserved11: [u8; 0x07],
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
    #[doc = "0x08 - I2CS Control C"]
    pub ctrlc: crate::Reg<self::i2cs::ctrlc::CTRLC_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x14 - I2CS Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::i2cs::intenclr::INTENCLR_SPEC>,
    _reserved4: [u8; 0x01],
    #[doc = "0x16 - I2CS Interrupt Enable Set"]
    pub intenset: crate::Reg<self::i2cs::intenset::INTENSET_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x18 - I2CS Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::i2cs::intflag::INTFLAG_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x1a - I2CS Status"]
    pub status: crate::Reg<self::i2cs::status::STATUS_SPEC>,
    #[doc = "0x1c - I2CS Synchronization Busy"]
    pub syncbusy: crate::Reg<self::i2cs::syncbusy::SYNCBUSY_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x22 - I2CS Length"]
    pub length: crate::Reg<self::i2cs::length::LENGTH_SPEC>,
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
pub struct SPIS {
    #[doc = "0x00 - SPIS Control A"]
    pub ctrla: crate::Reg<self::spis::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - SPIS Control B"]
    pub ctrlb: crate::Reg<self::spis::ctrlb::CTRLB_SPEC>,
    #[doc = "0x08 - SPIS Control C"]
    pub ctrlc: crate::Reg<self::spis::ctrlc::CTRLC_SPEC>,
    #[doc = "0x0c - SPIS Baud Rate"]
    pub baud: crate::Reg<self::spis::baud::BAUD_SPEC>,
    _reserved4: [u8; 0x07],
    #[doc = "0x14 - SPIS Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::spis::intenclr::INTENCLR_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x16 - SPIS Interrupt Enable Set"]
    pub intenset: crate::Reg<self::spis::intenset::INTENSET_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x18 - SPIS Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::spis::intflag::INTFLAG_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x1a - SPIS Status"]
    pub status: crate::Reg<self::spis::status::STATUS_SPEC>,
    #[doc = "0x1c - SPIS Synchronization Busy"]
    pub syncbusy: crate::Reg<self::spis::syncbusy::SYNCBUSY_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x22 - SPIS Length"]
    pub length: crate::Reg<self::spis::length::LENGTH_SPEC>,
    #[doc = "0x24 - SPIS Address"]
    pub addr: crate::Reg<self::spis::addr::ADDR_SPEC>,
    #[doc = "0x28 - SPIS Data"]
    pub data: crate::Reg<self::spis::data::DATA_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x30 - SPIS Debug Control"]
    pub dbgctrl: crate::Reg<self::spis::dbgctrl::DBGCTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SPI Slave Mode"]
pub mod spis;
#[doc = r"Register block"]
#[repr(C)]
pub struct SPIM {
    #[doc = "0x00 - SPIM Control A"]
    pub ctrla: crate::Reg<self::spim::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - SPIM Control B"]
    pub ctrlb: crate::Reg<self::spim::ctrlb::CTRLB_SPEC>,
    #[doc = "0x08 - SPIM Control C"]
    pub ctrlc: crate::Reg<self::spim::ctrlc::CTRLC_SPEC>,
    #[doc = "0x0c - SPIM Baud Rate"]
    pub baud: crate::Reg<self::spim::baud::BAUD_SPEC>,
    _reserved4: [u8; 0x07],
    #[doc = "0x14 - SPIM Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::spim::intenclr::INTENCLR_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x16 - SPIM Interrupt Enable Set"]
    pub intenset: crate::Reg<self::spim::intenset::INTENSET_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x18 - SPIM Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::spim::intflag::INTFLAG_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x1a - SPIM Status"]
    pub status: crate::Reg<self::spim::status::STATUS_SPEC>,
    #[doc = "0x1c - SPIM Synchronization Busy"]
    pub syncbusy: crate::Reg<self::spim::syncbusy::SYNCBUSY_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x22 - SPIM Length"]
    pub length: crate::Reg<self::spim::length::LENGTH_SPEC>,
    #[doc = "0x24 - SPIM Address"]
    pub addr: crate::Reg<self::spim::addr::ADDR_SPEC>,
    #[doc = "0x28 - SPIM Data"]
    pub data: crate::Reg<self::spim::data::DATA_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x30 - SPIM Debug Control"]
    pub dbgctrl: crate::Reg<self::spim::dbgctrl::DBGCTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SPI Master Mode"]
pub mod spim;
#[doc = r"Register block"]
#[repr(C)]
pub struct USART_EXT {
    #[doc = "0x00 - USART_EXT Control A"]
    pub ctrla: crate::Reg<self::usart_ext::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - USART_EXT Control B"]
    pub ctrlb: crate::Reg<self::usart_ext::ctrlb::CTRLB_SPEC>,
    #[doc = "0x08 - USART_EXT Control C"]
    pub ctrlc: crate::Reg<self::usart_ext::ctrlc::CTRLC_SPEC>,
    _reserved_3_baud: [u8; 0x02],
    #[doc = "0x0e - USART_EXT Receive Pulse Length"]
    pub rxpl: crate::Reg<self::usart_ext::rxpl::RXPL_SPEC>,
    _reserved5: [u8; 0x05],
    #[doc = "0x14 - USART_EXT Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::usart_ext::intenclr::INTENCLR_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x16 - USART_EXT Interrupt Enable Set"]
    pub intenset: crate::Reg<self::usart_ext::intenset::INTENSET_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x18 - USART_EXT Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::usart_ext::intflag::INTFLAG_SPEC>,
    _reserved8: [u8; 0x01],
    #[doc = "0x1a - USART_EXT Status"]
    pub status: crate::Reg<self::usart_ext::status::STATUS_SPEC>,
    #[doc = "0x1c - USART_EXT Synchronization Busy"]
    pub syncbusy: crate::Reg<self::usart_ext::syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x20 - USART_EXT Receive Error Count"]
    pub rxerrcnt: crate::Reg<self::usart_ext::rxerrcnt::RXERRCNT_SPEC>,
    _reserved11: [u8; 0x01],
    #[doc = "0x22 - USART_EXT Length"]
    pub length: crate::Reg<self::usart_ext::length::LENGTH_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x28 - USART_EXT Data"]
    pub data: crate::Reg<self::usart_ext::data::DATA_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x30 - USART_EXT Debug Control"]
    pub dbgctrl: crate::Reg<self::usart_ext::dbgctrl::DBGCTRL_SPEC>,
}
impl USART_EXT {
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud_usartfp_mode(
        &self,
    ) -> &crate::Reg<self::usart_ext::baud_usartfp_mode::BAUD_USARTFP_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart_ext::baud_usartfp_mode::BAUD_USARTFP_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud_fracfp_mode(
        &self,
    ) -> &crate::Reg<self::usart_ext::baud_fracfp_mode::BAUD_FRACFP_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart_ext::baud_fracfp_mode::BAUD_FRACFP_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud_frac_mode(
        &self,
    ) -> &crate::Reg<self::usart_ext::baud_frac_mode::BAUD_FRAC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart_ext::baud_frac_mode::BAUD_FRAC_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud(&self) -> &crate::Reg<self::usart_ext::baud::BAUD_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart_ext::baud::BAUD_SPEC>)
        }
    }
}
#[doc = r"Register block"]
#[doc = "USART EXTERNAL CLOCK Mode"]
pub mod usart_ext;
#[doc = r"Register block"]
#[repr(C)]
pub struct USART_INT {
    #[doc = "0x00 - USART_INT Control A"]
    pub ctrla: crate::Reg<self::usart_int::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - USART_INT Control B"]
    pub ctrlb: crate::Reg<self::usart_int::ctrlb::CTRLB_SPEC>,
    #[doc = "0x08 - USART_INT Control C"]
    pub ctrlc: crate::Reg<self::usart_int::ctrlc::CTRLC_SPEC>,
    _reserved_3_baud: [u8; 0x02],
    #[doc = "0x0e - USART_INT Receive Pulse Length"]
    pub rxpl: crate::Reg<self::usart_int::rxpl::RXPL_SPEC>,
    _reserved5: [u8; 0x05],
    #[doc = "0x14 - USART_INT Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::usart_int::intenclr::INTENCLR_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x16 - USART_INT Interrupt Enable Set"]
    pub intenset: crate::Reg<self::usart_int::intenset::INTENSET_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x18 - USART_INT Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::usart_int::intflag::INTFLAG_SPEC>,
    _reserved8: [u8; 0x01],
    #[doc = "0x1a - USART_INT Status"]
    pub status: crate::Reg<self::usart_int::status::STATUS_SPEC>,
    #[doc = "0x1c - USART_INT Synchronization Busy"]
    pub syncbusy: crate::Reg<self::usart_int::syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x20 - USART_INT Receive Error Count"]
    pub rxerrcnt: crate::Reg<self::usart_int::rxerrcnt::RXERRCNT_SPEC>,
    _reserved11: [u8; 0x01],
    #[doc = "0x22 - USART_INT Length"]
    pub length: crate::Reg<self::usart_int::length::LENGTH_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x28 - USART_INT Data"]
    pub data: crate::Reg<self::usart_int::data::DATA_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x30 - USART_INT Debug Control"]
    pub dbgctrl: crate::Reg<self::usart_int::dbgctrl::DBGCTRL_SPEC>,
}
impl USART_INT {
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud_usartfp_mode(
        &self,
    ) -> &crate::Reg<self::usart_int::baud_usartfp_mode::BAUD_USARTFP_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart_int::baud_usartfp_mode::BAUD_USARTFP_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud_fracfp_mode(
        &self,
    ) -> &crate::Reg<self::usart_int::baud_fracfp_mode::BAUD_FRACFP_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart_int::baud_fracfp_mode::BAUD_FRACFP_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud_frac_mode(
        &self,
    ) -> &crate::Reg<self::usart_int::baud_frac_mode::BAUD_FRAC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart_int::baud_frac_mode::BAUD_FRAC_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud(&self) -> &crate::Reg<self::usart_int::baud::BAUD_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<self::usart_int::baud::BAUD_SPEC>)
        }
    }
}
#[doc = r"Register block"]
#[doc = "USART INTERNAL CLOCK Mode"]
pub mod usart_int;
