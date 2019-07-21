#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: CTRLBSET,
    _reserved3: [u8; 2usize],
    #[doc = "0x08 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - Recoverable Fault A Configuration"]
    pub fctrla: FCTRLA,
    #[doc = "0x10 - Recoverable Fault B Configuration"]
    pub fctrlb: FCTRLB,
    #[doc = "0x14 - Waveform Extension Configuration"]
    pub wexctrl: WEXCTRL,
    #[doc = "0x18 - Driver Control"]
    pub drvctrl: DRVCTRL,
    _reserved8: [u8; 2usize],
    #[doc = "0x1e - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved9: [u8; 1usize],
    #[doc = "0x20 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x24 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x28 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x2c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x30 - Status"]
    pub status: STATUS,
    _reserved_14_count: [u8; 4usize],
    #[doc = "0x38 - Pattern"]
    pub patt: PATT,
    _reserved16: [u8; 2usize],
    #[doc = "0x3c - Waveform Control"]
    pub wave: WAVE,
    _reserved_17_per: [u8; 4usize],
    _reserved_18_cc: [u8; 16usize],
    _reserved19: [u8; 16usize],
    #[doc = "0x64 - Pattern Buffer"]
    pub pattb: PATTB,
    _reserved20: [u8; 2usize],
    #[doc = "0x68 - Waveform Control Buffer"]
    pub waveb: WAVEB,
    _reserved_21_perb: [u8; 4usize],
    _reserved_22_ccb: [u8; 16usize],
}
impl RegisterBlock {
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith6(&self) -> &COUNT_DITH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH6) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith6_mut(&self) -> &mut COUNT_DITH6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT_DITH6) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith5(&self) -> &COUNT_DITH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH5) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith5_mut(&self) -> &mut COUNT_DITH5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT_DITH5) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith4(&self) -> &COUNT_DITH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH4) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith4_mut(&self) -> &mut COUNT_DITH4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT_DITH4) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count(&self) -> &COUNT {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_mut(&self) -> &mut COUNT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith6(&self) -> &PER_DITH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH6) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith6_mut(&self) -> &mut PER_DITH6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER_DITH6) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith5(&self) -> &PER_DITH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH5) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith5_mut(&self) -> &mut PER_DITH5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER_DITH5) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith4(&self) -> &PER_DITH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH4) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith4_mut(&self) -> &mut PER_DITH4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER_DITH4) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per(&self) -> &PER {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_mut(&self) -> &mut PER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith6(&self) -> &[CC_DITH6; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH6; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith6_mut(&self) -> &mut [CC_DITH6; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC_DITH6; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith5(&self) -> &[CC_DITH5; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH5; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith5_mut(&self) -> &mut [CC_DITH5; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC_DITH5; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith4(&self) -> &[CC_DITH4; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH4; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith4_mut(&self) -> &mut [CC_DITH4; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC_DITH4; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc(&self) -> &[CC; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_mut(&self) -> &mut [CC; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC; 4]) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith6(&self) -> &PERB_DITH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB_DITH6) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith6_mut(&self) -> &mut PERB_DITH6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERB_DITH6) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith5(&self) -> &PERB_DITH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB_DITH5) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith5_mut(&self) -> &mut PERB_DITH5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERB_DITH5) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith4(&self) -> &PERB_DITH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB_DITH4) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith4_mut(&self) -> &mut PERB_DITH4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERB_DITH4) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb(&self) -> &PERB {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_mut(&self) -> &mut PERB {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERB) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith6(&self) -> &[CCB_DITH6; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB_DITH6; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith6_mut(&self) -> &mut [CCB_DITH6; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCB_DITH6; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith5(&self) -> &[CCB_DITH5; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB_DITH5; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith5_mut(&self) -> &mut [CCB_DITH5; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCB_DITH5; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith4(&self) -> &[CCB_DITH4; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB_DITH4; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith4_mut(&self) -> &mut [CCB_DITH4; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCB_DITH4; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb(&self) -> &[CCB; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_mut(&self) -> &mut [CCB; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCB; 4]) }
    }
}
#[doc = "Control A"]
pub struct CTRLA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B Clear"]
pub struct CTRLBCLR {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "Control B Set"]
pub struct CTRLBSET {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "Recoverable Fault A Configuration"]
pub struct FCTRLA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Recoverable Fault A Configuration"]
pub mod fctrla;
#[doc = "Recoverable Fault B Configuration"]
pub struct FCTRLB {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Recoverable Fault B Configuration"]
pub mod fctrlb;
#[doc = "Waveform Extension Configuration"]
pub struct WEXCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Waveform Extension Configuration"]
pub mod wexctrl;
#[doc = "Driver Control"]
pub struct DRVCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Driver Control"]
pub mod drvctrl;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Status"]
pub struct STATUS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Count"]
pub struct COUNT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Count"]
pub mod count;
#[doc = "Count"]
pub struct COUNT_DITH4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Count"]
pub mod count_dith4;
#[doc = "Count"]
pub struct COUNT_DITH5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Count"]
pub mod count_dith5;
#[doc = "Count"]
pub struct COUNT_DITH6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Count"]
pub mod count_dith6;
#[doc = "Pattern"]
pub struct PATT {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Pattern"]
pub mod patt;
#[doc = "Waveform Control"]
pub struct WAVE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Waveform Control"]
pub mod wave;
#[doc = "Period"]
pub struct PER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Period"]
pub mod per;
#[doc = "Period"]
pub struct PER_DITH4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Period"]
pub mod per_dith4;
#[doc = "Period"]
pub struct PER_DITH5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Period"]
pub mod per_dith5;
#[doc = "Period"]
pub struct PER_DITH6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Period"]
pub mod per_dith6;
#[doc = "Compare and Capture"]
pub struct CC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture"]
pub mod cc;
#[doc = "Compare and Capture"]
pub struct CC_DITH4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture"]
pub mod cc_dith4;
#[doc = "Compare and Capture"]
pub struct CC_DITH5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture"]
pub mod cc_dith5;
#[doc = "Compare and Capture"]
pub struct CC_DITH6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture"]
pub mod cc_dith6;
#[doc = "Pattern Buffer"]
pub struct PATTB {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Pattern Buffer"]
pub mod pattb;
#[doc = "Waveform Control Buffer"]
pub struct WAVEB {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Waveform Control Buffer"]
pub mod waveb;
#[doc = "Period Buffer"]
pub struct PERB {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Period Buffer"]
pub mod perb;
#[doc = "Period Buffer"]
pub struct PERB_DITH4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Period Buffer"]
pub mod perb_dith4;
#[doc = "Period Buffer"]
pub struct PERB_DITH5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Period Buffer"]
pub mod perb_dith5;
#[doc = "Period Buffer"]
pub struct PERB_DITH6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Period Buffer"]
pub mod perb_dith6;
#[doc = "Compare and Capture Buffer"]
pub struct CCB {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture Buffer"]
pub mod ccb;
#[doc = "Compare and Capture Buffer"]
pub struct CCB_DITH4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith4;
#[doc = "Compare and Capture Buffer"]
pub struct CCB_DITH5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith5;
#[doc = "Compare and Capture Buffer"]
pub struct CCB_DITH6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith6;
