#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_mode0: [u8; 0xa0],
}
impl RegisterBlock {
    #[doc = "0x00..0xa0 - Clock/Calendar with Alarm"]
    #[inline(always)]
    pub fn mode2(&self) -> &MODE2 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const MODE2) }
    }
    #[doc = "0x00..0xa0 - 16-bit Counter with Two 16-bit Compares"]
    #[inline(always)]
    pub fn mode1(&self) -> &MODE1 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const MODE1) }
    }
    #[doc = "0x00..0xa0 - 32-bit Counter with Single 32-bit Compare"]
    #[inline(always)]
    pub fn mode0(&self) -> &MODE0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const MODE0) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE0 {
    #[doc = "0x00 - MODE0 Control A"]
    pub ctrla: crate::Reg<self::mode0::ctrla::CTRLA_SPEC>,
    #[doc = "0x02 - MODE0 Control B"]
    pub ctrlb: crate::Reg<self::mode0::ctrlb::CTRLB_SPEC>,
    #[doc = "0x04 - MODE0 Event Control"]
    pub evctrl: crate::Reg<self::mode0::evctrl::EVCTRL_SPEC>,
    #[doc = "0x08 - MODE0 Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::mode0::intenclr::INTENCLR_SPEC>,
    #[doc = "0x0a - MODE0 Interrupt Enable Set"]
    pub intenset: crate::Reg<self::mode0::intenset::INTENSET_SPEC>,
    #[doc = "0x0c - MODE0 Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::mode0::intflag::INTFLAG_SPEC>,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: crate::Reg<self::mode0::dbgctrl::DBGCTRL_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - MODE0 Synchronization Busy Status"]
    pub syncbusy: crate::Reg<self::mode0::syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: crate::Reg<self::mode0::freqcorr::FREQCORR_SPEC>,
    _reserved9: [u8; 0x03],
    #[doc = "0x18 - MODE0 Counter Value"]
    pub count: crate::Reg<self::mode0::count::COUNT_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x20..0x28 - MODE0 Compare n Value"]
    pub comp: [crate::Reg<self::mode0::comp::COMP_SPEC>; 2],
    _reserved11: [u8; 0x18],
    #[doc = "0x40..0x50 - General Purpose"]
    pub gp: [crate::Reg<self::mode0::gp::GP_SPEC>; 4],
    _reserved12: [u8; 0x10],
    #[doc = "0x60 - Tamper Control"]
    pub tampctrl: crate::Reg<self::mode0::tampctrl::TAMPCTRL_SPEC>,
    #[doc = "0x64 - MODE0 Timestamp"]
    pub timestamp: crate::Reg<self::mode0::timestamp::TIMESTAMP_SPEC>,
    #[doc = "0x68 - Tamper ID"]
    pub tampid: crate::Reg<self::mode0::tampid::TAMPID_SPEC>,
    _reserved15: [u8; 0x14],
    #[doc = "0x80..0xa0 - Backup"]
    pub bkup: [crate::Reg<self::mode0::bkup::BKUP_SPEC>; 8],
}
#[doc = r"Register block"]
#[doc = "32-bit Counter with Single 32-bit Compare"]
pub mod mode0;
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE1 {
    #[doc = "0x00 - MODE1 Control A"]
    pub ctrla: crate::Reg<self::mode1::ctrla::CTRLA_SPEC>,
    #[doc = "0x02 - MODE1 Control B"]
    pub ctrlb: crate::Reg<self::mode1::ctrlb::CTRLB_SPEC>,
    #[doc = "0x04 - MODE1 Event Control"]
    pub evctrl: crate::Reg<self::mode1::evctrl::EVCTRL_SPEC>,
    #[doc = "0x08 - MODE1 Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::mode1::intenclr::INTENCLR_SPEC>,
    #[doc = "0x0a - MODE1 Interrupt Enable Set"]
    pub intenset: crate::Reg<self::mode1::intenset::INTENSET_SPEC>,
    #[doc = "0x0c - MODE1 Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::mode1::intflag::INTFLAG_SPEC>,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: crate::Reg<self::mode1::dbgctrl::DBGCTRL_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - MODE1 Synchronization Busy Status"]
    pub syncbusy: crate::Reg<self::mode1::syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: crate::Reg<self::mode1::freqcorr::FREQCORR_SPEC>,
    _reserved9: [u8; 0x03],
    #[doc = "0x18 - MODE1 Counter Value"]
    pub count: crate::Reg<self::mode1::count::COUNT_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x1c - MODE1 Counter Period"]
    pub per: crate::Reg<self::mode1::per::PER_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x20..0x28 - MODE1 Compare n Value"]
    pub comp: [crate::Reg<self::mode1::comp::COMP_SPEC>; 4],
    _reserved12: [u8; 0x18],
    #[doc = "0x40..0x50 - General Purpose"]
    pub gp: [crate::Reg<self::mode1::gp::GP_SPEC>; 4],
    _reserved13: [u8; 0x10],
    #[doc = "0x60 - Tamper Control"]
    pub tampctrl: crate::Reg<self::mode1::tampctrl::TAMPCTRL_SPEC>,
    #[doc = "0x64 - MODE1 Timestamp"]
    pub timestamp: crate::Reg<self::mode1::timestamp::TIMESTAMP_SPEC>,
    #[doc = "0x68 - Tamper ID"]
    pub tampid: crate::Reg<self::mode1::tampid::TAMPID_SPEC>,
    _reserved16: [u8; 0x14],
    #[doc = "0x80..0xa0 - Backup"]
    pub bkup: [crate::Reg<self::mode1::bkup::BKUP_SPEC>; 8],
}
#[doc = r"Register block"]
#[doc = "16-bit Counter with Two 16-bit Compares"]
pub mod mode1;
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE2 {
    #[doc = "0x00 - MODE2 Control A"]
    pub ctrla: crate::Reg<self::mode2::ctrla::CTRLA_SPEC>,
    #[doc = "0x02 - MODE2 Control B"]
    pub ctrlb: crate::Reg<self::mode2::ctrlb::CTRLB_SPEC>,
    #[doc = "0x04 - MODE2 Event Control"]
    pub evctrl: crate::Reg<self::mode2::evctrl::EVCTRL_SPEC>,
    #[doc = "0x08 - MODE2 Interrupt Enable Clear"]
    pub intenclr: crate::Reg<self::mode2::intenclr::INTENCLR_SPEC>,
    #[doc = "0x0a - MODE2 Interrupt Enable Set"]
    pub intenset: crate::Reg<self::mode2::intenset::INTENSET_SPEC>,
    #[doc = "0x0c - MODE2 Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<self::mode2::intflag::INTFLAG_SPEC>,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: crate::Reg<self::mode2::dbgctrl::DBGCTRL_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x10 - MODE2 Synchronization Busy Status"]
    pub syncbusy: crate::Reg<self::mode2::syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: crate::Reg<self::mode2::freqcorr::FREQCORR_SPEC>,
    _reserved9: [u8; 0x03],
    #[doc = "0x18 - MODE2 Clock Value"]
    pub clock: crate::Reg<self::mode2::clock::CLOCK_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x20 - MODE2_ALARM Alarm n Value"]
    pub alarm0: crate::Reg<self::mode2::alarm0::ALARM0_SPEC>,
    #[doc = "0x24 - MODE2_ALARM Alarm n Mask"]
    pub mask0: crate::Reg<self::mode2::mask0::MASK0_SPEC>,
    _reserved12: [u8; 0x03],
    #[doc = "0x28 - MODE2_ALARM Alarm n Value"]
    pub alarm1: crate::Reg<self::mode2::alarm1::ALARM1_SPEC>,
    #[doc = "0x2c - MODE2_ALARM Alarm n Mask"]
    pub mask1: crate::Reg<self::mode2::mask1::MASK1_SPEC>,
    _reserved14: [u8; 0x13],
    #[doc = "0x40..0x50 - General Purpose"]
    pub gp: [crate::Reg<self::mode2::gp::GP_SPEC>; 4],
    _reserved15: [u8; 0x10],
    #[doc = "0x60 - Tamper Control"]
    pub tampctrl: crate::Reg<self::mode2::tampctrl::TAMPCTRL_SPEC>,
    #[doc = "0x64 - MODE2 Timestamp"]
    pub timestamp: crate::Reg<self::mode2::timestamp::TIMESTAMP_SPEC>,
    #[doc = "0x68 - Tamper ID"]
    pub tampid: crate::Reg<self::mode2::tampid::TAMPID_SPEC>,
    _reserved18: [u8; 0x14],
    #[doc = "0x80..0xa0 - Backup"]
    pub bkup: [crate::Reg<self::mode2::bkup::BKUP_SPEC>; 8],
}
#[doc = r"Register block"]
#[doc = "Clock/Calendar with Alarm"]
pub mod mode2;
