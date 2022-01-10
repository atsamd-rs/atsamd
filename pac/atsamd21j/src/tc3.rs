#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_count8: [u8; 0x20],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - 32-bit Counter Mode"]
    #[inline(always)]
    pub fn count32(&self) -> &COUNT32 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const COUNT32) }
    }
    #[doc = "0x00..0x1c - 16-bit Counter Mode"]
    #[inline(always)]
    pub fn count16(&self) -> &COUNT16 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const COUNT16) }
    }
    #[doc = "0x00..0x1a - 8-bit Counter Mode"]
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
    #[doc = "0x02 - Read Request"]
    pub readreq: crate::Reg<self::count8::readreq::READREQ_SPEC>,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: crate::Reg<self::count8::ctrlbclr::CTRLBCLR_SPEC>,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: crate::Reg<self::count8::ctrlbset::CTRLBSET_SPEC>,
    #[doc = "0x06 - Control C"]
    pub ctrlc: crate::Reg<self::count8::ctrlc::CTRLC_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x08 - Debug Control"]
    pub dbgctrl: crate::Reg<self::count8::dbgctrl::DBGCTRL_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0a - Event Control"]
    pub evctrl: crate::Reg<self::count8::evctrl::EVCTRL_SPEC>,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::count8::intenclr::INTENCLR_SPEC>,
    #[doc = "0x0d - Interrupt Enable Set"]
    pub intenset: crate::Reg<self::count8::intenset::INTENSET_SPEC>,
    #[doc = "0x0e - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::count8::intflag::INTFLAG_SPEC>,
    #[doc = "0x0f - Status"]
    pub status: crate::Reg<self::count8::status::STATUS_SPEC>,
    #[doc = "0x10 - COUNT8 Counter Value"]
    pub count: crate::Reg<self::count8::count::COUNT_SPEC>,
    _reserved12: [u8; 0x03],
    #[doc = "0x14 - COUNT8 Period Value"]
    pub per: crate::Reg<self::count8::per::PER_SPEC>,
    _reserved13: [u8; 0x03],
    #[doc = "0x18 - COUNT8 Compare/Capture"]
    pub cc: [crate::Reg<self::count8::cc::CC_SPEC>; 2],
}
#[doc = r"Register block"]
#[doc = "8-bit Counter Mode"]
pub mod count8;
#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT16 {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<self::count16::ctrla::CTRLA_SPEC>,
    #[doc = "0x02 - Read Request"]
    pub readreq: crate::Reg<self::count16::readreq::READREQ_SPEC>,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: crate::Reg<self::count16::ctrlbclr::CTRLBCLR_SPEC>,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: crate::Reg<self::count16::ctrlbset::CTRLBSET_SPEC>,
    #[doc = "0x06 - Control C"]
    pub ctrlc: crate::Reg<self::count16::ctrlc::CTRLC_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x08 - Debug Control"]
    pub dbgctrl: crate::Reg<self::count16::dbgctrl::DBGCTRL_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0a - Event Control"]
    pub evctrl: crate::Reg<self::count16::evctrl::EVCTRL_SPEC>,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::count16::intenclr::INTENCLR_SPEC>,
    #[doc = "0x0d - Interrupt Enable Set"]
    pub intenset: crate::Reg<self::count16::intenset::INTENSET_SPEC>,
    #[doc = "0x0e - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::count16::intflag::INTFLAG_SPEC>,
    #[doc = "0x0f - Status"]
    pub status: crate::Reg<self::count16::status::STATUS_SPEC>,
    #[doc = "0x10 - COUNT16 Counter Value"]
    pub count: crate::Reg<self::count16::count::COUNT_SPEC>,
    _reserved12: [u8; 0x06],
    #[doc = "0x18 - COUNT16 Compare/Capture"]
    pub cc: [crate::Reg<self::count16::cc::CC_SPEC>; 2],
}
#[doc = r"Register block"]
#[doc = "16-bit Counter Mode"]
pub mod count16;
#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT32 {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<self::count32::ctrla::CTRLA_SPEC>,
    #[doc = "0x02 - Read Request"]
    pub readreq: crate::Reg<self::count32::readreq::READREQ_SPEC>,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: crate::Reg<self::count32::ctrlbclr::CTRLBCLR_SPEC>,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: crate::Reg<self::count32::ctrlbset::CTRLBSET_SPEC>,
    #[doc = "0x06 - Control C"]
    pub ctrlc: crate::Reg<self::count32::ctrlc::CTRLC_SPEC>,
    _reserved5: [u8; 0x01],
    #[doc = "0x08 - Debug Control"]
    pub dbgctrl: crate::Reg<self::count32::dbgctrl::DBGCTRL_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0a - Event Control"]
    pub evctrl: crate::Reg<self::count32::evctrl::EVCTRL_SPEC>,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::count32::intenclr::INTENCLR_SPEC>,
    #[doc = "0x0d - Interrupt Enable Set"]
    pub intenset: crate::Reg<self::count32::intenset::INTENSET_SPEC>,
    #[doc = "0x0e - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::count32::intflag::INTFLAG_SPEC>,
    #[doc = "0x0f - Status"]
    pub status: crate::Reg<self::count32::status::STATUS_SPEC>,
    #[doc = "0x10 - COUNT32 Counter Value"]
    pub count: crate::Reg<self::count32::count::COUNT_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x18..0x20 - COUNT32 Compare/Capture"]
    pub cc: [crate::Reg<self::count32::cc::CC_SPEC>; 2],
}
#[doc = r"Register block"]
#[doc = "32-bit Counter Mode"]
pub mod count32;
