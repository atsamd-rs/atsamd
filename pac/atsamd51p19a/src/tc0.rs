#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_count8: [u8; 56usize],
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
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: self::count8::CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: self::count8::CTRLBSET,
    #[doc = "0x06 - Event Control"]
    pub evctrl: self::count8::EVCTRL,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: self::count8::INTENCLR,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: self::count8::INTENSET,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: self::count8::INTFLAG,
    #[doc = "0x0b - Status"]
    pub status: self::count8::STATUS,
    #[doc = "0x0c - Waveform Generation Control"]
    pub wave: self::count8::WAVE,
    #[doc = "0x0d - Control C"]
    pub drvctrl: self::count8::DRVCTRL,
    _reserved10: [u8; 1usize],
    #[doc = "0x0f - Debug Control"]
    pub dbgctrl: self::count8::DBGCTRL,
    #[doc = "0x10 - Synchronization Status"]
    pub syncbusy: self::count8::SYNCBUSY,
    #[doc = "0x14 - COUNT8 Count"]
    pub count: self::count8::COUNT,
    _reserved13: [u8; 6usize],
    #[doc = "0x1b - COUNT8 Period"]
    pub per: self::count8::PER,
    #[doc = "0x1c - COUNT8 Compare and Capture"]
    pub cc: [self::count8::CC; 2],
    _reserved15: [u8; 17usize],
    #[doc = "0x2f - COUNT8 Period Buffer"]
    pub perbuf: self::count8::PERBUF,
    #[doc = "0x30 - COUNT8 Compare and Capture Buffer"]
    pub ccbuf: [self::count8::CCBUF; 2],
}
#[doc = r"Register block"]
#[doc = "8-bit Counter Mode"]
pub mod count8;
#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT16 {
    #[doc = "0x00 - Control A"]
    pub ctrla: self::count16::CTRLA,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: self::count16::CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: self::count16::CTRLBSET,
    #[doc = "0x06 - Event Control"]
    pub evctrl: self::count16::EVCTRL,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: self::count16::INTENCLR,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: self::count16::INTENSET,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: self::count16::INTFLAG,
    #[doc = "0x0b - Status"]
    pub status: self::count16::STATUS,
    #[doc = "0x0c - Waveform Generation Control"]
    pub wave: self::count16::WAVE,
    #[doc = "0x0d - Control C"]
    pub drvctrl: self::count16::DRVCTRL,
    _reserved10: [u8; 1usize],
    #[doc = "0x0f - Debug Control"]
    pub dbgctrl: self::count16::DBGCTRL,
    #[doc = "0x10 - Synchronization Status"]
    pub syncbusy: self::count16::SYNCBUSY,
    #[doc = "0x14 - COUNT16 Count"]
    pub count: self::count16::COUNT,
    _reserved13: [u8; 6usize],
    #[doc = "0x1c - COUNT16 Compare and Capture"]
    pub cc: [self::count16::CC; 2],
    _reserved14: [u8; 16usize],
    #[doc = "0x30 - COUNT16 Compare and Capture Buffer"]
    pub ccbuf: [self::count16::CCBUF; 2],
}
#[doc = r"Register block"]
#[doc = "16-bit Counter Mode"]
pub mod count16;
#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT32 {
    #[doc = "0x00 - Control A"]
    pub ctrla: self::count32::CTRLA,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: self::count32::CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: self::count32::CTRLBSET,
    #[doc = "0x06 - Event Control"]
    pub evctrl: self::count32::EVCTRL,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: self::count32::INTENCLR,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: self::count32::INTENSET,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: self::count32::INTFLAG,
    #[doc = "0x0b - Status"]
    pub status: self::count32::STATUS,
    #[doc = "0x0c - Waveform Generation Control"]
    pub wave: self::count32::WAVE,
    #[doc = "0x0d - Control C"]
    pub drvctrl: self::count32::DRVCTRL,
    _reserved10: [u8; 1usize],
    #[doc = "0x0f - Debug Control"]
    pub dbgctrl: self::count32::DBGCTRL,
    #[doc = "0x10 - Synchronization Status"]
    pub syncbusy: self::count32::SYNCBUSY,
    #[doc = "0x14 - COUNT32 Count"]
    pub count: self::count32::COUNT,
    _reserved13: [u8; 4usize],
    #[doc = "0x1c - COUNT32 Compare and Capture"]
    pub cc: [self::count32::CC; 2],
    _reserved14: [u8; 12usize],
    #[doc = "0x30 - COUNT32 Compare and Capture Buffer"]
    pub ccbuf: [self::count32::CCBUF; 2],
}
#[doc = r"Register block"]
#[doc = "32-bit Counter Mode"]
pub mod count32;
