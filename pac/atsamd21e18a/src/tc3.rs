#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_count8: [u8; 32usize],
}
impl RegisterBlock {
    #[doc = "0x00 - 32-bit Counter Mode"]
    #[inline(always)]
    pub fn count32(&self) -> &COUNT32 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const COUNT32) }
    }
    #[doc = "0x00 - 32-bit Counter Mode"]
    #[inline(always)]
    pub fn count32_mut(&self) -> &mut COUNT32 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut COUNT32) }
    }
    #[doc = "0x00 - 16-bit Counter Mode"]
    #[inline(always)]
    pub fn count16(&self) -> &COUNT16 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const COUNT16) }
    }
    #[doc = "0x00 - 16-bit Counter Mode"]
    #[inline(always)]
    pub fn count16_mut(&self) -> &mut COUNT16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut COUNT16) }
    }
    #[doc = "0x00 - 8-bit Counter Mode"]
    #[inline(always)]
    pub fn count8(&self) -> &COUNT8 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const COUNT8) }
    }
    #[doc = "0x00 - 8-bit Counter Mode"]
    #[inline(always)]
    pub fn count8_mut(&self) -> &mut COUNT8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut COUNT8) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT8 {
    #[doc = "0x00 - Control A"]
    pub ctrla: self::count8::CTRLA,
    #[doc = "0x02 - Read Request"]
    pub readreq: self::count8::READREQ,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: self::count8::CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: self::count8::CTRLBSET,
    #[doc = "0x06 - Control C"]
    pub ctrlc: self::count8::CTRLC,
    _reserved5: [u8; 1usize],
    #[doc = "0x08 - Debug Control"]
    pub dbgctrl: self::count8::DBGCTRL,
    _reserved6: [u8; 1usize],
    #[doc = "0x0a - Event Control"]
    pub evctrl: self::count8::EVCTRL,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: self::count8::INTENCLR,
    #[doc = "0x0d - Interrupt Enable Set"]
    pub intenset: self::count8::INTENSET,
    #[doc = "0x0e - Interrupt Flag Status and Clear"]
    pub intflag: self::count8::INTFLAG,
    #[doc = "0x0f - Status"]
    pub status: self::count8::STATUS,
    #[doc = "0x10 - COUNT8 Counter Value"]
    pub count: self::count8::COUNT,
    _reserved12: [u8; 3usize],
    #[doc = "0x14 - COUNT8 Period Value"]
    pub per: self::count8::PER,
    _reserved13: [u8; 3usize],
    #[doc = "0x18 - COUNT8 Compare/Capture"]
    pub cc: [self::count8::CC; 2],
}
#[doc = r"Register block"]
#[doc = "8-bit Counter Mode"]
pub mod count8;
#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT16 {
    #[doc = "0x00 - Control A"]
    pub ctrla: self::count16::CTRLA,
    #[doc = "0x02 - Read Request"]
    pub readreq: self::count16::READREQ,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: self::count16::CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: self::count16::CTRLBSET,
    #[doc = "0x06 - Control C"]
    pub ctrlc: self::count16::CTRLC,
    _reserved5: [u8; 1usize],
    #[doc = "0x08 - Debug Control"]
    pub dbgctrl: self::count16::DBGCTRL,
    _reserved6: [u8; 1usize],
    #[doc = "0x0a - Event Control"]
    pub evctrl: self::count16::EVCTRL,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: self::count16::INTENCLR,
    #[doc = "0x0d - Interrupt Enable Set"]
    pub intenset: self::count16::INTENSET,
    #[doc = "0x0e - Interrupt Flag Status and Clear"]
    pub intflag: self::count16::INTFLAG,
    #[doc = "0x0f - Status"]
    pub status: self::count16::STATUS,
    #[doc = "0x10 - COUNT16 Counter Value"]
    pub count: self::count16::COUNT,
    _reserved12: [u8; 6usize],
    #[doc = "0x18 - COUNT16 Compare/Capture"]
    pub cc: [self::count16::CC; 2],
}
#[doc = r"Register block"]
#[doc = "16-bit Counter Mode"]
pub mod count16;
#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT32 {
    #[doc = "0x00 - Control A"]
    pub ctrla: self::count32::CTRLA,
    #[doc = "0x02 - Read Request"]
    pub readreq: self::count32::READREQ,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: self::count32::CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: self::count32::CTRLBSET,
    #[doc = "0x06 - Control C"]
    pub ctrlc: self::count32::CTRLC,
    _reserved5: [u8; 1usize],
    #[doc = "0x08 - Debug Control"]
    pub dbgctrl: self::count32::DBGCTRL,
    _reserved6: [u8; 1usize],
    #[doc = "0x0a - Event Control"]
    pub evctrl: self::count32::EVCTRL,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: self::count32::INTENCLR,
    #[doc = "0x0d - Interrupt Enable Set"]
    pub intenset: self::count32::INTENSET,
    #[doc = "0x0e - Interrupt Flag Status and Clear"]
    pub intflag: self::count32::INTFLAG,
    #[doc = "0x0f - Status"]
    pub status: self::count32::STATUS,
    #[doc = "0x10 - COUNT32 Counter Value"]
    pub count: self::count32::COUNT,
    _reserved12: [u8; 4usize],
    #[doc = "0x18 - COUNT32 Compare/Capture"]
    pub cc: [self::count32::CC; 2],
}
#[doc = r"Register block"]
#[doc = "32-bit Counter Mode"]
pub mod count32;
