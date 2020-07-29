#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_mode0: [u8; 160usize],
}
impl RegisterBlock {
    #[doc = "0x00 - Clock/Calendar with Alarm"]
    #[inline(always)]
    pub fn mode2(&self) -> &MODE2 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const MODE2) }
    }
    #[doc = "0x00 - Clock/Calendar with Alarm"]
    #[inline(always)]
    pub fn mode2_mut(&self) -> &mut MODE2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut MODE2) }
    }
    #[doc = "0x00 - 16-bit Counter with Two 16-bit Compares"]
    #[inline(always)]
    pub fn mode1(&self) -> &MODE1 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const MODE1) }
    }
    #[doc = "0x00 - 16-bit Counter with Two 16-bit Compares"]
    #[inline(always)]
    pub fn mode1_mut(&self) -> &mut MODE1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut MODE1) }
    }
    #[doc = "0x00 - 32-bit Counter with Single 32-bit Compare"]
    #[inline(always)]
    pub fn mode0(&self) -> &MODE0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const MODE0) }
    }
    #[doc = "0x00 - 32-bit Counter with Single 32-bit Compare"]
    #[inline(always)]
    pub fn mode0_mut(&self) -> &mut MODE0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut MODE0) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE0 {
    #[doc = "0x00 - MODE0 Control A"]
    pub ctrla: self::mode0::CTRLA,
    #[doc = "0x02 - MODE0 Control B"]
    pub ctrlb: self::mode0::CTRLB,
    #[doc = "0x04 - MODE0 Event Control"]
    pub evctrl: self::mode0::EVCTRL,
    #[doc = "0x08 - MODE0 Interrupt Enable Clear"]
    pub intenclr: self::mode0::INTENCLR,
    #[doc = "0x0a - MODE0 Interrupt Enable Set"]
    pub intenset: self::mode0::INTENSET,
    #[doc = "0x0c - MODE0 Interrupt Flag Status and Clear"]
    pub intflag: self::mode0::INTFLAG,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: self::mode0::DBGCTRL,
    _reserved7: [u8; 1usize],
    #[doc = "0x10 - MODE0 Synchronization Busy Status"]
    pub syncbusy: self::mode0::SYNCBUSY,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: self::mode0::FREQCORR,
    _reserved9: [u8; 3usize],
    #[doc = "0x18 - MODE0 Counter Value"]
    pub count: self::mode0::COUNT,
    _reserved10: [u8; 4usize],
    #[doc = "0x20 - MODE0 Compare n Value"]
    pub comp: [self::mode0::COMP; 2],
    _reserved11: [u8; 24usize],
    #[doc = "0x40 - General Purpose"]
    pub gp: [self::mode0::GP; 4],
    _reserved12: [u8; 16usize],
    #[doc = "0x60 - Tamper Control"]
    pub tampctrl: self::mode0::TAMPCTRL,
    #[doc = "0x64 - MODE0 Timestamp"]
    pub timestamp: self::mode0::TIMESTAMP,
    #[doc = "0x68 - Tamper ID"]
    pub tampid: self::mode0::TAMPID,
    _reserved15: [u8; 20usize],
    #[doc = "0x80 - Backup"]
    pub bkup: [self::mode0::BKUP; 8],
}
#[doc = r"Register block"]
#[doc = "32-bit Counter with Single 32-bit Compare"]
pub mod mode0;
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE1 {
    #[doc = "0x00 - MODE1 Control A"]
    pub ctrla: self::mode1::CTRLA,
    #[doc = "0x02 - MODE1 Control B"]
    pub ctrlb: self::mode1::CTRLB,
    #[doc = "0x04 - MODE1 Event Control"]
    pub evctrl: self::mode1::EVCTRL,
    #[doc = "0x08 - MODE1 Interrupt Enable Clear"]
    pub intenclr: self::mode1::INTENCLR,
    #[doc = "0x0a - MODE1 Interrupt Enable Set"]
    pub intenset: self::mode1::INTENSET,
    #[doc = "0x0c - MODE1 Interrupt Flag Status and Clear"]
    pub intflag: self::mode1::INTFLAG,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: self::mode1::DBGCTRL,
    _reserved7: [u8; 1usize],
    #[doc = "0x10 - MODE1 Synchronization Busy Status"]
    pub syncbusy: self::mode1::SYNCBUSY,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: self::mode1::FREQCORR,
    _reserved9: [u8; 3usize],
    #[doc = "0x18 - MODE1 Counter Value"]
    pub count: self::mode1::COUNT,
    _reserved10: [u8; 2usize],
    #[doc = "0x1c - MODE1 Counter Period"]
    pub per: self::mode1::PER,
    _reserved11: [u8; 2usize],
    #[doc = "0x20 - MODE1 Compare n Value"]
    pub comp: [self::mode1::COMP; 4],
    _reserved12: [u8; 24usize],
    #[doc = "0x40 - General Purpose"]
    pub gp: [self::mode1::GP; 4],
    _reserved13: [u8; 16usize],
    #[doc = "0x60 - Tamper Control"]
    pub tampctrl: self::mode1::TAMPCTRL,
    #[doc = "0x64 - MODE1 Timestamp"]
    pub timestamp: self::mode1::TIMESTAMP,
    #[doc = "0x68 - Tamper ID"]
    pub tampid: self::mode1::TAMPID,
    _reserved16: [u8; 20usize],
    #[doc = "0x80 - Backup"]
    pub bkup: [self::mode1::BKUP; 8],
}
#[doc = r"Register block"]
#[doc = "16-bit Counter with Two 16-bit Compares"]
pub mod mode1;
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE2 {
    #[doc = "0x00 - MODE2 Control A"]
    pub ctrla: self::mode2::CTRLA,
    #[doc = "0x02 - MODE2 Control B"]
    pub ctrlb: self::mode2::CTRLB,
    #[doc = "0x04 - MODE2 Event Control"]
    pub evctrl: self::mode2::EVCTRL,
    #[doc = "0x08 - MODE2 Interrupt Enable Clear"]
    pub intenclr: self::mode2::INTENCLR,
    #[doc = "0x0a - MODE2 Interrupt Enable Set"]
    pub intenset: self::mode2::INTENSET,
    #[doc = "0x0c - MODE2 Interrupt Flag Status and Clear"]
    pub intflag: self::mode2::INTFLAG,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: self::mode2::DBGCTRL,
    _reserved7: [u8; 1usize],
    #[doc = "0x10 - MODE2 Synchronization Busy Status"]
    pub syncbusy: self::mode2::SYNCBUSY,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: self::mode2::FREQCORR,
    _reserved9: [u8; 3usize],
    #[doc = "0x18 - MODE2 Clock Value"]
    pub clock: self::mode2::CLOCK,
    _reserved10: [u8; 4usize],
    #[doc = "0x20 - MODE2_ALARM Alarm n Value"]
    pub alarm0: self::mode2::ALARM0,
    #[doc = "0x24 - MODE2_ALARM Alarm n Mask"]
    pub mask0: self::mode2::MASK0,
    _reserved12: [u8; 3usize],
    #[doc = "0x28 - MODE2_ALARM Alarm n Value"]
    pub alarm1: self::mode2::ALARM1,
    #[doc = "0x2c - MODE2_ALARM Alarm n Mask"]
    pub mask1: self::mode2::MASK1,
    _reserved14: [u8; 19usize],
    #[doc = "0x40 - General Purpose"]
    pub gp: [self::mode2::GP; 4],
    _reserved15: [u8; 16usize],
    #[doc = "0x60 - Tamper Control"]
    pub tampctrl: self::mode2::TAMPCTRL,
    #[doc = "0x64 - MODE2 Timestamp"]
    pub timestamp: self::mode2::TIMESTAMP,
    #[doc = "0x68 - Tamper ID"]
    pub tampid: self::mode2::TAMPID,
    _reserved18: [u8; 20usize],
    #[doc = "0x80 - Backup"]
    pub bkup: [self::mode2::BKUP; 8],
}
#[doc = r"Register block"]
#[doc = "Clock/Calendar with Alarm"]
pub mod mode2;
