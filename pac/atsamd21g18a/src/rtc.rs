#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_mode0: [u8; 29usize],
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
    #[doc = "0x00 - MODE0 Control"]
    pub ctrl: self::mode0::CTRL,
    #[doc = "0x02 - Read Request"]
    pub readreq: self::mode0::READREQ,
    #[doc = "0x04 - MODE0 Event Control"]
    pub evctrl: self::mode0::EVCTRL,
    #[doc = "0x06 - MODE0 Interrupt Enable Clear"]
    pub intenclr: self::mode0::INTENCLR,
    #[doc = "0x07 - MODE0 Interrupt Enable Set"]
    pub intenset: self::mode0::INTENSET,
    #[doc = "0x08 - MODE0 Interrupt Flag Status and Clear"]
    pub intflag: self::mode0::INTFLAG,
    _reserved6: [u8; 1usize],
    #[doc = "0x0a - Status"]
    pub status: self::mode0::STATUS,
    #[doc = "0x0b - Debug Control"]
    pub dbgctrl: self::mode0::DBGCTRL,
    #[doc = "0x0c - Frequency Correction"]
    pub freqcorr: self::mode0::FREQCORR,
    _reserved9: [u8; 3usize],
    #[doc = "0x10 - MODE0 Counter Value"]
    pub count: self::mode0::COUNT,
    _reserved10: [u8; 4usize],
    #[doc = "0x18 - MODE0 Compare n Value"]
    pub comp: [self::mode0::COMP; 1],
}
#[doc = r"Register block"]
#[doc = "32-bit Counter with Single 32-bit Compare"]
pub mod mode0;
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE1 {
    #[doc = "0x00 - MODE1 Control"]
    pub ctrl: self::mode1::CTRL,
    #[doc = "0x02 - Read Request"]
    pub readreq: self::mode1::READREQ,
    #[doc = "0x04 - MODE1 Event Control"]
    pub evctrl: self::mode1::EVCTRL,
    #[doc = "0x06 - MODE1 Interrupt Enable Clear"]
    pub intenclr: self::mode1::INTENCLR,
    #[doc = "0x07 - MODE1 Interrupt Enable Set"]
    pub intenset: self::mode1::INTENSET,
    #[doc = "0x08 - MODE1 Interrupt Flag Status and Clear"]
    pub intflag: self::mode1::INTFLAG,
    _reserved6: [u8; 1usize],
    #[doc = "0x0a - Status"]
    pub status: self::mode1::STATUS,
    #[doc = "0x0b - Debug Control"]
    pub dbgctrl: self::mode1::DBGCTRL,
    #[doc = "0x0c - Frequency Correction"]
    pub freqcorr: self::mode1::FREQCORR,
    _reserved9: [u8; 3usize],
    #[doc = "0x10 - MODE1 Counter Value"]
    pub count: self::mode1::COUNT,
    _reserved10: [u8; 2usize],
    #[doc = "0x14 - MODE1 Counter Period"]
    pub per: self::mode1::PER,
    _reserved11: [u8; 2usize],
    #[doc = "0x18 - MODE1 Compare n Value"]
    pub comp: [self::mode1::COMP; 2],
}
#[doc = r"Register block"]
#[doc = "16-bit Counter with Two 16-bit Compares"]
pub mod mode1;
#[doc = r"Register block"]
#[repr(C)]
pub struct MODE2 {
    #[doc = "0x00 - MODE2 Control"]
    pub ctrl: self::mode2::CTRL,
    #[doc = "0x02 - Read Request"]
    pub readreq: self::mode2::READREQ,
    #[doc = "0x04 - MODE2 Event Control"]
    pub evctrl: self::mode2::EVCTRL,
    #[doc = "0x06 - MODE2 Interrupt Enable Clear"]
    pub intenclr: self::mode2::INTENCLR,
    #[doc = "0x07 - MODE2 Interrupt Enable Set"]
    pub intenset: self::mode2::INTENSET,
    #[doc = "0x08 - MODE2 Interrupt Flag Status and Clear"]
    pub intflag: self::mode2::INTFLAG,
    _reserved6: [u8; 1usize],
    #[doc = "0x0a - Status"]
    pub status: self::mode2::STATUS,
    #[doc = "0x0b - Debug Control"]
    pub dbgctrl: self::mode2::DBGCTRL,
    #[doc = "0x0c - Frequency Correction"]
    pub freqcorr: self::mode2::FREQCORR,
    _reserved9: [u8; 3usize],
    #[doc = "0x10 - MODE2 Clock Value"]
    pub clock: self::mode2::CLOCK,
    _reserved10: [u8; 4usize],
    #[doc = "0x18 - MODE2 Alarm n Value"]
    pub alarm0: self::mode2::ALARM,
    #[doc = "0x1c - MODE2 Alarm n Mask"]
    pub mask0: self::mode2::MASK,
}
#[doc = r"Register block"]
#[doc = "Clock/Calendar with Alarm"]
pub mod mode2;
