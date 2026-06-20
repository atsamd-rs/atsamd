#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_count8: [u8; 0x38],
}
impl RegisterBlock {
    #[doc = "0x00..0x38 - 32-bit Counter Mode"]
    #[inline(always)]
    pub fn count32(&self) -> &COUNT32 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const COUNT32) }
    }
    #[doc = "0x00..0x34 - 16-bit Counter Mode"]
    #[inline(always)]
    pub fn count16(&self) -> &COUNT16 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const COUNT16) }
    }
    #[doc = "0x00..0x32 - 8-bit Counter Mode"]
    #[inline(always)]
    pub fn count8(&self) -> &COUNT8 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const COUNT8) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT8 {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<self::count8::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: crate::Reg<self::count8::ctrlbclr::CTRLBCLR_SPEC>,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: crate::Reg<self::count8::ctrlbset::CTRLBSET_SPEC>,
    #[doc = "0x06 - Event Control"]
    pub evctrl: crate::Reg<self::count8::evctrl::EVCTRL_SPEC>,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::count8::intenclr::INTENCLR_SPEC>,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: crate::Reg<self::count8::intenset::INTENSET_SPEC>,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::count8::intflag::INTFLAG_SPEC>,
    #[doc = "0x0b - Status"]
    pub status: crate::Reg<self::count8::status::STATUS_SPEC>,
    #[doc = "0x0c - Waveform Generation Control"]
    pub wave: crate::Reg<self::count8::wave::WAVE_SPEC>,
    #[doc = "0x0d - Control C"]
    pub drvctrl: crate::Reg<self::count8::drvctrl::DRVCTRL_SPEC>,
    _reserved10: [u8; 0x01],
    #[doc = "0x0f - Debug Control"]
    pub dbgctrl: crate::Reg<self::count8::dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x10 - Synchronization Status"]
    pub syncbusy: crate::Reg<self::count8::syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x14 - COUNT8 Count"]
    pub count: crate::Reg<self::count8::count::COUNT_SPEC>,
    _reserved13: [u8; 0x06],
    #[doc = "0x1b - COUNT8 Period"]
    pub per: crate::Reg<self::count8::per::PER_SPEC>,
    #[doc = "0x1c - COUNT8 Compare and Capture"]
    pub cc: [crate::Reg<self::count8::cc::CC_SPEC>; 2],
    _reserved15: [u8; 0x11],
    #[doc = "0x2f - COUNT8 Period Buffer"]
    pub perbuf: crate::Reg<self::count8::perbuf::PERBUF_SPEC>,
    #[doc = "0x30 - COUNT8 Compare and Capture Buffer"]
    pub ccbuf: [crate::Reg<self::count8::ccbuf::CCBUF_SPEC>; 2],
}
#[doc = r"Register block"]
#[doc = "8-bit Counter Mode"]
pub mod count8;
#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT16 {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<self::count16::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: crate::Reg<self::count16::ctrlbclr::CTRLBCLR_SPEC>,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: crate::Reg<self::count16::ctrlbset::CTRLBSET_SPEC>,
    #[doc = "0x06 - Event Control"]
    pub evctrl: crate::Reg<self::count16::evctrl::EVCTRL_SPEC>,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::count16::intenclr::INTENCLR_SPEC>,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: crate::Reg<self::count16::intenset::INTENSET_SPEC>,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::count16::intflag::INTFLAG_SPEC>,
    #[doc = "0x0b - Status"]
    pub status: crate::Reg<self::count16::status::STATUS_SPEC>,
    #[doc = "0x0c - Waveform Generation Control"]
    pub wave: crate::Reg<self::count16::wave::WAVE_SPEC>,
    #[doc = "0x0d - Control C"]
    pub drvctrl: crate::Reg<self::count16::drvctrl::DRVCTRL_SPEC>,
    _reserved10: [u8; 0x01],
    #[doc = "0x0f - Debug Control"]
    pub dbgctrl: crate::Reg<self::count16::dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x10 - Synchronization Status"]
    pub syncbusy: crate::Reg<self::count16::syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x14 - COUNT16 Count"]
    pub count: crate::Reg<self::count16::count::COUNT_SPEC>,
    _reserved13: [u8; 0x06],
    #[doc = "0x1c - COUNT16 Compare and Capture"]
    pub cc: [crate::Reg<self::count16::cc::CC_SPEC>; 2],
    _reserved14: [u8; 0x10],
    #[doc = "0x30 - COUNT16 Compare and Capture Buffer"]
    pub ccbuf: [crate::Reg<self::count16::ccbuf::CCBUF_SPEC>; 2],
}
#[doc = r"Register block"]
#[doc = "16-bit Counter Mode"]
pub mod count16;
#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT32 {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<self::count32::ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: crate::Reg<self::count32::ctrlbclr::CTRLBCLR_SPEC>,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: crate::Reg<self::count32::ctrlbset::CTRLBSET_SPEC>,
    #[doc = "0x06 - Event Control"]
    pub evctrl: crate::Reg<self::count32::evctrl::EVCTRL_SPEC>,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::count32::intenclr::INTENCLR_SPEC>,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: crate::Reg<self::count32::intenset::INTENSET_SPEC>,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::count32::intflag::INTFLAG_SPEC>,
    #[doc = "0x0b - Status"]
    pub status: crate::Reg<self::count32::status::STATUS_SPEC>,
    #[doc = "0x0c - Waveform Generation Control"]
    pub wave: crate::Reg<self::count32::wave::WAVE_SPEC>,
    #[doc = "0x0d - Control C"]
    pub drvctrl: crate::Reg<self::count32::drvctrl::DRVCTRL_SPEC>,
    _reserved10: [u8; 0x01],
    #[doc = "0x0f - Debug Control"]
    pub dbgctrl: crate::Reg<self::count32::dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x10 - Synchronization Status"]
    pub syncbusy: crate::Reg<self::count32::syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x14 - COUNT32 Count"]
    pub count: crate::Reg<self::count32::count::COUNT_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x1c..0x24 - COUNT32 Compare and Capture"]
    pub cc: [crate::Reg<self::count32::cc::CC_SPEC>; 2],
    _reserved14: [u8; 0x0c],
    #[doc = "0x30..0x38 - COUNT32 Compare and Capture Buffer"]
    pub ccbuf: [crate::Reg<self::count32::ccbuf::CCBUF_SPEC>; 2],
}
#[doc = r"Register block"]
#[doc = "32-bit Counter Mode"]
pub mod count32;
