#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_mode0: [u8; 0x1d],
}
impl RegisterBlock {
    #[doc = "0x00..0x1d - Clock/Calendar with Alarm"]
    #[inline(always)]
    pub fn mode2(&self) -> &MODE2 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const MODE2) }
    }
    #[doc = "0x00..0x1c - 16-bit Counter with Two 16-bit Compares"]
    #[inline(always)]
    pub fn mode1(&self) -> &MODE1 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const MODE1) }
    }
    #[doc = "0x00..0x1c - 32-bit Counter with Single 32-bit Compare"]
    #[inline(always)]
    pub fn mode0(&self) -> &MODE0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const MODE0) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE0 {
    #[doc = "0x00 - MODE0 Control"]
    pub ctrl: crate::Reg<self::mode0::ctrl::CTRL_SPEC>,
    #[doc = "0x02 - Read Request"]
    pub readreq: crate::Reg<self::mode0::readreq::READREQ_SPEC>,
    #[doc = "0x04 - MODE0 Event Control"]
    pub evctrl: crate::Reg<self::mode0::evctrl::EVCTRL_SPEC>,
    #[doc = "0x06 - MODE0 Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::mode0::intenclr::INTENCLR_SPEC>,
    #[doc = "0x07 - MODE0 Interrupt Enable Set"]
    pub intenset: crate::Reg<self::mode0::intenset::INTENSET_SPEC>,
    #[doc = "0x08 - MODE0 Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::mode0::intflag::INTFLAG_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0a - Status"]
    pub status: crate::Reg<self::mode0::status::STATUS_SPEC>,
    #[doc = "0x0b - Debug Control"]
    pub dbgctrl: crate::Reg<self::mode0::dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x0c - Frequency Correction"]
    pub freqcorr: crate::Reg<self::mode0::freqcorr::FREQCORR_SPEC>,
    _reserved9: [u8; 0x03],
    #[doc = "0x10 - MODE0 Counter Value"]
    pub count: crate::Reg<self::mode0::count::COUNT_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x18 - MODE0 Compare n Value"]
    pub comp: [crate::Reg<self::mode0::comp::COMP_SPEC>; 1],
}
#[doc = r"Register block"]
#[doc = "32-bit Counter with Single 32-bit Compare"]
pub mod mode0;
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE1 {
    #[doc = "0x00 - MODE1 Control"]
    pub ctrl: crate::Reg<self::mode1::ctrl::CTRL_SPEC>,
    #[doc = "0x02 - Read Request"]
    pub readreq: crate::Reg<self::mode1::readreq::READREQ_SPEC>,
    #[doc = "0x04 - MODE1 Event Control"]
    pub evctrl: crate::Reg<self::mode1::evctrl::EVCTRL_SPEC>,
    #[doc = "0x06 - MODE1 Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::mode1::intenclr::INTENCLR_SPEC>,
    #[doc = "0x07 - MODE1 Interrupt Enable Set"]
    pub intenset: crate::Reg<self::mode1::intenset::INTENSET_SPEC>,
    #[doc = "0x08 - MODE1 Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::mode1::intflag::INTFLAG_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0a - Status"]
    pub status: crate::Reg<self::mode1::status::STATUS_SPEC>,
    #[doc = "0x0b - Debug Control"]
    pub dbgctrl: crate::Reg<self::mode1::dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x0c - Frequency Correction"]
    pub freqcorr: crate::Reg<self::mode1::freqcorr::FREQCORR_SPEC>,
    _reserved9: [u8; 0x03],
    #[doc = "0x10 - MODE1 Counter Value"]
    pub count: crate::Reg<self::mode1::count::COUNT_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x14 - MODE1 Counter Period"]
    pub per: crate::Reg<self::mode1::per::PER_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x18 - MODE1 Compare n Value"]
    pub comp: [crate::Reg<self::mode1::comp::COMP_SPEC>; 2],
}
#[doc = r"Register block"]
#[doc = "16-bit Counter with Two 16-bit Compares"]
pub mod mode1;
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE2 {
    #[doc = "0x00 - MODE2 Control"]
    pub ctrl: crate::Reg<self::mode2::ctrl::CTRL_SPEC>,
    #[doc = "0x02 - Read Request"]
    pub readreq: crate::Reg<self::mode2::readreq::READREQ_SPEC>,
    #[doc = "0x04 - MODE2 Event Control"]
    pub evctrl: crate::Reg<self::mode2::evctrl::EVCTRL_SPEC>,
    #[doc = "0x06 - MODE2 Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::mode2::intenclr::INTENCLR_SPEC>,
    #[doc = "0x07 - MODE2 Interrupt Enable Set"]
    pub intenset: crate::Reg<self::mode2::intenset::INTENSET_SPEC>,
    #[doc = "0x08 - MODE2 Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::mode2::intflag::INTFLAG_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x0a - Status"]
    pub status: crate::Reg<self::mode2::status::STATUS_SPEC>,
    #[doc = "0x0b - Debug Control"]
    pub dbgctrl: crate::Reg<self::mode2::dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x0c - Frequency Correction"]
    pub freqcorr: crate::Reg<self::mode2::freqcorr::FREQCORR_SPEC>,
    _reserved9: [u8; 0x03],
    #[doc = "0x10 - MODE2 Clock Value"]
    pub clock: crate::Reg<self::mode2::clock::CLOCK_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x18 - MODE2 Alarm n Value"]
    pub alarm0: crate::Reg<self::mode2::alarm::ALARM_SPEC>,
    #[doc = "0x1c - MODE2 Alarm n Mask"]
    pub mask0: crate::Reg<self::mode2::mask::MASK_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Clock/Calendar with Alarm"]
pub mod mode2;
