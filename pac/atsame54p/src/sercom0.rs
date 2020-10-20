#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_i2cm: [u8; 49usize],
}
impl RegisterBlock {
    #[doc = "0x00 - USART INTERNAL CLOCK Mode"]
    #[inline(always)]
    pub fn usart_int(&self) -> &USART_INT {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART_INT) }
    }
    #[doc = "0x00 - USART INTERNAL CLOCK Mode"]
    #[inline(always)]
    pub fn usart_int_mut(&self) -> &mut USART_INT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut USART_INT) }
    }
    #[doc = "0x00 - USART EXTERNAL CLOCK Mode"]
    #[inline(always)]
    pub fn usart_ext(&self) -> &USART_EXT {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART_EXT) }
    }
    #[doc = "0x00 - USART EXTERNAL CLOCK Mode"]
    #[inline(always)]
    pub fn usart_ext_mut(&self) -> &mut USART_EXT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut USART_EXT) }
    }
    #[doc = "0x00 - SPI Master Mode"]
    #[inline(always)]
    pub fn spim(&self) -> &SPIM {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SPIM) }
    }
    #[doc = "0x00 - SPI Master Mode"]
    #[inline(always)]
    pub fn spim_mut(&self) -> &mut SPIM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut SPIM) }
    }
    #[doc = "0x00 - SPI Slave Mode"]
    #[inline(always)]
    pub fn spis(&self) -> &SPIS {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SPIS) }
    }
    #[doc = "0x00 - SPI Slave Mode"]
    #[inline(always)]
    pub fn spis_mut(&self) -> &mut SPIS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut SPIS) }
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
    #[doc = "0x08 - I2CM Control C"]
    pub ctrlc: self::i2cm::CTRLC,
    #[doc = "0x0c - I2CM Baud Rate"]
    pub baud: self::i2cm::BAUD,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - I2CM Interrupt Enable Clear"]
    pub intenclr: self::i2cm::INTENCLR,
    _reserved5: [u8; 1usize],
    #[doc = "0x16 - I2CM Interrupt Enable Set"]
    pub intenset: self::i2cm::INTENSET,
    _reserved6: [u8; 1usize],
    #[doc = "0x18 - I2CM Interrupt Flag Status and Clear"]
    pub intflag: self::i2cm::INTFLAG,
    _reserved7: [u8; 1usize],
    #[doc = "0x1a - I2CM Status"]
    pub status: self::i2cm::STATUS,
    #[doc = "0x1c - I2CM Synchronization Busy"]
    pub syncbusy: self::i2cm::SYNCBUSY,
    _reserved9: [u8; 4usize],
    #[doc = "0x24 - I2CM Address"]
    pub addr: self::i2cm::ADDR,
    #[doc = "0x28 - I2CM Data"]
    pub data: self::i2cm::DATA,
    _reserved11: [u8; 7usize],
    #[doc = "0x30 - I2CM Debug Control"]
    pub dbgctrl: self::i2cm::DBGCTRL,
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
    #[doc = "0x08 - I2CS Control C"]
    pub ctrlc: self::i2cs::CTRLC,
    _reserved3: [u8; 8usize],
    #[doc = "0x14 - I2CS Interrupt Enable Clear"]
    pub intenclr: self::i2cs::INTENCLR,
    _reserved4: [u8; 1usize],
    #[doc = "0x16 - I2CS Interrupt Enable Set"]
    pub intenset: self::i2cs::INTENSET,
    _reserved5: [u8; 1usize],
    #[doc = "0x18 - I2CS Interrupt Flag Status and Clear"]
    pub intflag: self::i2cs::INTFLAG,
    _reserved6: [u8; 1usize],
    #[doc = "0x1a - I2CS Status"]
    pub status: self::i2cs::STATUS,
    #[doc = "0x1c - I2CS Synchronization Busy"]
    pub syncbusy: self::i2cs::SYNCBUSY,
    _reserved8: [u8; 2usize],
    #[doc = "0x22 - I2CS Length"]
    pub length: self::i2cs::LENGTH,
    #[doc = "0x24 - I2CS Address"]
    pub addr: self::i2cs::ADDR,
    #[doc = "0x28 - I2CS Data"]
    pub data: self::i2cs::DATA,
}
#[doc = r"Register block"]
#[doc = "I2C Slave Mode"]
pub mod i2cs;
#[doc = r"Register block"]
#[repr(C)]
pub struct SPIS {
    #[doc = "0x00 - SPIS Control A"]
    pub ctrla: self::spis::CTRLA,
    #[doc = "0x04 - SPIS Control B"]
    pub ctrlb: self::spis::CTRLB,
    #[doc = "0x08 - SPIS Control C"]
    pub ctrlc: self::spis::CTRLC,
    #[doc = "0x0c - SPIS Baud Rate"]
    pub baud: self::spis::BAUD,
    _reserved4: [u8; 7usize],
    #[doc = "0x14 - SPIS Interrupt Enable Clear"]
    pub intenclr: self::spis::INTENCLR,
    _reserved5: [u8; 1usize],
    #[doc = "0x16 - SPIS Interrupt Enable Set"]
    pub intenset: self::spis::INTENSET,
    _reserved6: [u8; 1usize],
    #[doc = "0x18 - SPIS Interrupt Flag Status and Clear"]
    pub intflag: self::spis::INTFLAG,
    _reserved7: [u8; 1usize],
    #[doc = "0x1a - SPIS Status"]
    pub status: self::spis::STATUS,
    #[doc = "0x1c - SPIS Synchronization Busy"]
    pub syncbusy: self::spis::SYNCBUSY,
    _reserved9: [u8; 2usize],
    #[doc = "0x22 - SPIS Length"]
    pub length: self::spis::LENGTH,
    #[doc = "0x24 - SPIS Address"]
    pub addr: self::spis::ADDR,
    #[doc = "0x28 - SPIS Data"]
    pub data: self::spis::DATA,
    _reserved12: [u8; 4usize],
    #[doc = "0x30 - SPIS Debug Control"]
    pub dbgctrl: self::spis::DBGCTRL,
}
#[doc = r"Register block"]
#[doc = "SPI Slave Mode"]
pub mod spis;
#[doc = r"Register block"]
#[repr(C)]
pub struct SPIM {
    #[doc = "0x00 - SPIM Control A"]
    pub ctrla: self::spim::CTRLA,
    #[doc = "0x04 - SPIM Control B"]
    pub ctrlb: self::spim::CTRLB,
    #[doc = "0x08 - SPIM Control C"]
    pub ctrlc: self::spim::CTRLC,
    #[doc = "0x0c - SPIM Baud Rate"]
    pub baud: self::spim::BAUD,
    _reserved4: [u8; 7usize],
    #[doc = "0x14 - SPIM Interrupt Enable Clear"]
    pub intenclr: self::spim::INTENCLR,
    _reserved5: [u8; 1usize],
    #[doc = "0x16 - SPIM Interrupt Enable Set"]
    pub intenset: self::spim::INTENSET,
    _reserved6: [u8; 1usize],
    #[doc = "0x18 - SPIM Interrupt Flag Status and Clear"]
    pub intflag: self::spim::INTFLAG,
    _reserved7: [u8; 1usize],
    #[doc = "0x1a - SPIM Status"]
    pub status: self::spim::STATUS,
    #[doc = "0x1c - SPIM Synchronization Busy"]
    pub syncbusy: self::spim::SYNCBUSY,
    _reserved9: [u8; 2usize],
    #[doc = "0x22 - SPIM Length"]
    pub length: self::spim::LENGTH,
    #[doc = "0x24 - SPIM Address"]
    pub addr: self::spim::ADDR,
    #[doc = "0x28 - SPIM Data"]
    pub data: self::spim::DATA,
    _reserved12: [u8; 4usize],
    #[doc = "0x30 - SPIM Debug Control"]
    pub dbgctrl: self::spim::DBGCTRL,
}
#[doc = r"Register block"]
#[doc = "SPI Master Mode"]
pub mod spim;
#[doc = r"Register block"]
#[repr(C)]
pub struct USART_EXT {
    #[doc = "0x00 - USART_EXT Control A"]
    pub ctrla: self::usart_ext::CTRLA,
    #[doc = "0x04 - USART_EXT Control B"]
    pub ctrlb: self::usart_ext::CTRLB,
    #[doc = "0x08 - USART_EXT Control C"]
    pub ctrlc: self::usart_ext::CTRLC,
    _reserved_3_baud: [u8; 2usize],
    #[doc = "0x0e - USART_EXT Receive Pulse Length"]
    pub rxpl: self::usart_ext::RXPL,
    _reserved5: [u8; 5usize],
    #[doc = "0x14 - USART_EXT Interrupt Enable Clear"]
    pub intenclr: self::usart_ext::INTENCLR,
    _reserved6: [u8; 1usize],
    #[doc = "0x16 - USART_EXT Interrupt Enable Set"]
    pub intenset: self::usart_ext::INTENSET,
    _reserved7: [u8; 1usize],
    #[doc = "0x18 - USART_EXT Interrupt Flag Status and Clear"]
    pub intflag: self::usart_ext::INTFLAG,
    _reserved8: [u8; 1usize],
    #[doc = "0x1a - USART_EXT Status"]
    pub status: self::usart_ext::STATUS,
    #[doc = "0x1c - USART_EXT Synchronization Busy"]
    pub syncbusy: self::usart_ext::SYNCBUSY,
    #[doc = "0x20 - USART_EXT Receive Error Count"]
    pub rxerrcnt: self::usart_ext::RXERRCNT,
    _reserved11: [u8; 1usize],
    #[doc = "0x22 - USART_EXT Length"]
    pub length: self::usart_ext::LENGTH,
    _reserved12: [u8; 4usize],
    #[doc = "0x28 - USART_EXT Data"]
    pub data: self::usart_ext::DATA,
    _reserved13: [u8; 4usize],
    #[doc = "0x30 - USART_EXT Debug Control"]
    pub dbgctrl: self::usart_ext::DBGCTRL,
}
impl USART_EXT {
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud_usartfp_mode(&self) -> &self::usart_ext::BAUD_USARTFP_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const self::usart_ext::BAUD_USARTFP_MODE)
        }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud_usartfp_mode_mut(&self) -> &mut self::usart_ext::BAUD_USARTFP_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(12usize)
                as *mut self::usart_ext::BAUD_USARTFP_MODE)
        }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud_fracfp_mode(&self) -> &self::usart_ext::BAUD_FRACFP_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const self::usart_ext::BAUD_FRACFP_MODE)
        }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud_fracfp_mode_mut(&self) -> &mut self::usart_ext::BAUD_FRACFP_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(12usize)
                as *mut self::usart_ext::BAUD_FRACFP_MODE)
        }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud_frac_mode(&self) -> &self::usart_ext::BAUD_FRAC_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const self::usart_ext::BAUD_FRAC_MODE)
        }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud_frac_mode_mut(&self) -> &mut self::usart_ext::BAUD_FRAC_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(12usize)
                as *mut self::usart_ext::BAUD_FRAC_MODE)
        }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud(&self) -> &self::usart_ext::BAUD {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize) as *const self::usart_ext::BAUD)
        }
    }
    #[doc = "0x0c - USART_EXT Baud Rate"]
    #[inline(always)]
    pub fn baud_mut(&self) -> &mut self::usart_ext::BAUD {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut self::usart_ext::BAUD)
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
    pub ctrla: self::usart_int::CTRLA,
    #[doc = "0x04 - USART_INT Control B"]
    pub ctrlb: self::usart_int::CTRLB,
    #[doc = "0x08 - USART_INT Control C"]
    pub ctrlc: self::usart_int::CTRLC,
    _reserved_3_baud: [u8; 2usize],
    #[doc = "0x0e - USART_INT Receive Pulse Length"]
    pub rxpl: self::usart_int::RXPL,
    _reserved5: [u8; 5usize],
    #[doc = "0x14 - USART_INT Interrupt Enable Clear"]
    pub intenclr: self::usart_int::INTENCLR,
    _reserved6: [u8; 1usize],
    #[doc = "0x16 - USART_INT Interrupt Enable Set"]
    pub intenset: self::usart_int::INTENSET,
    _reserved7: [u8; 1usize],
    #[doc = "0x18 - USART_INT Interrupt Flag Status and Clear"]
    pub intflag: self::usart_int::INTFLAG,
    _reserved8: [u8; 1usize],
    #[doc = "0x1a - USART_INT Status"]
    pub status: self::usart_int::STATUS,
    #[doc = "0x1c - USART_INT Synchronization Busy"]
    pub syncbusy: self::usart_int::SYNCBUSY,
    #[doc = "0x20 - USART_INT Receive Error Count"]
    pub rxerrcnt: self::usart_int::RXERRCNT,
    _reserved11: [u8; 1usize],
    #[doc = "0x22 - USART_INT Length"]
    pub length: self::usart_int::LENGTH,
    _reserved12: [u8; 4usize],
    #[doc = "0x28 - USART_INT Data"]
    pub data: self::usart_int::DATA,
    _reserved13: [u8; 4usize],
    #[doc = "0x30 - USART_INT Debug Control"]
    pub dbgctrl: self::usart_int::DBGCTRL,
}
impl USART_INT {
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud_usartfp_mode(&self) -> &self::usart_int::BAUD_USARTFP_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const self::usart_int::BAUD_USARTFP_MODE)
        }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud_usartfp_mode_mut(&self) -> &mut self::usart_int::BAUD_USARTFP_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(12usize)
                as *mut self::usart_int::BAUD_USARTFP_MODE)
        }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud_fracfp_mode(&self) -> &self::usart_int::BAUD_FRACFP_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const self::usart_int::BAUD_FRACFP_MODE)
        }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud_fracfp_mode_mut(&self) -> &mut self::usart_int::BAUD_FRACFP_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(12usize)
                as *mut self::usart_int::BAUD_FRACFP_MODE)
        }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud_frac_mode(&self) -> &self::usart_int::BAUD_FRAC_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const self::usart_int::BAUD_FRAC_MODE)
        }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud_frac_mode_mut(&self) -> &mut self::usart_int::BAUD_FRAC_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(12usize)
                as *mut self::usart_int::BAUD_FRAC_MODE)
        }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud(&self) -> &self::usart_int::BAUD {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize) as *const self::usart_int::BAUD)
        }
    }
    #[doc = "0x0c - USART_INT Baud Rate"]
    #[inline(always)]
    pub fn baud_mut(&self) -> &mut self::usart_int::BAUD {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut self::usart_int::BAUD)
        }
    }
}
#[doc = r"Register block"]
#[doc = "USART INTERNAL CLOCK Mode"]
pub mod usart_int;
