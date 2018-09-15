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
    #[doc = "Count"]
    pub count: COUNT_UNION,
    #[doc = "0x38 - Pattern"]
    pub patt: PATT,
    _reserved16: [u8; 2usize],
    #[doc = "0x3c - Waveform Control"]
    pub wave: WAVE,
    #[doc = "Period"]
    pub per: PER_UNION,
    #[doc = "Compare and Capture"]
    pub cc: CC_UNION,
    _reserved19: [u8; 16usize],
    #[doc = "0x64 - Pattern Buffer"]
    pub pattb: PATTB,
    _reserved20: [u8; 2usize],
    #[doc = "0x68 - Waveform Control Buffer"]
    pub waveb: WAVEB,
    #[doc = "Period Buffer"]
    pub perb: PERB_UNION,
    #[doc = "Compare and Capture Buffer"]
    pub ccb: CCB_UNION,
}
#[doc = "Count"]
#[repr(C)]
pub union COUNT_UNION {
    #[doc = "0x34 - Count"]
    pub count_dith6: COUNT_DITH6,
    #[doc = "0x34 - Count"]
    pub count_dith5: COUNT_DITH5,
    #[doc = "0x34 - Count"]
    pub count_dith4: COUNT_DITH4,
    #[doc = "0x34 - Count"]
    pub count: COUNT,
}
#[doc = "Period"]
#[repr(C)]
pub union PER_UNION {
    #[doc = "0x40 - Period"]
    pub per_dith6: PER_DITH6,
    #[doc = "0x40 - Period"]
    pub per_dith5: PER_DITH5,
    #[doc = "0x40 - Period"]
    pub per_dith4: PER_DITH4,
    #[doc = "0x40 - Period"]
    pub per: PER,
}
#[doc = "Compare and Capture"]
#[repr(C)]
pub union CC_UNION {
    #[doc = "0x44 - Compare and Capture"]
    pub cc_dith6: [CC_DITH6; 4],
    #[doc = "0x44 - Compare and Capture"]
    pub cc_dith5: [CC_DITH5; 4],
    #[doc = "0x44 - Compare and Capture"]
    pub cc_dith4: [CC_DITH4; 4],
    #[doc = "0x44 - Compare and Capture"]
    pub cc: [CC; 4],
}
#[doc = "Period Buffer"]
#[repr(C)]
pub union PERB_UNION {
    #[doc = "0x6c - Period Buffer"]
    pub perb_dith6: PERB_DITH6,
    #[doc = "0x6c - Period Buffer"]
    pub perb_dith5: PERB_DITH5,
    #[doc = "0x6c - Period Buffer"]
    pub perb_dith4: PERB_DITH4,
    #[doc = "0x6c - Period Buffer"]
    pub perb: PERB,
}
#[doc = "Compare and Capture Buffer"]
#[repr(C)]
pub union CCB_UNION {
    #[doc = "0x70 - Compare and Capture Buffer"]
    pub ccb_dith6: [CCB_DITH6; 4],
    #[doc = "0x70 - Compare and Capture Buffer"]
    pub ccb_dith5: [CCB_DITH5; 4],
    #[doc = "0x70 - Compare and Capture Buffer"]
    pub ccb_dith4: [CCB_DITH4; 4],
    #[doc = "0x70 - Compare and Capture Buffer"]
    pub ccb: [CCB; 4],
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B Clear"]
pub struct CTRLBCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "Control B Set"]
pub struct CTRLBSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "Recoverable Fault A Configuration"]
pub struct FCTRLA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Recoverable Fault A Configuration"]
pub mod fctrla;
#[doc = "Recoverable Fault B Configuration"]
pub struct FCTRLB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Recoverable Fault B Configuration"]
pub mod fctrlb;
#[doc = "Waveform Extension Configuration"]
pub struct WEXCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Waveform Extension Configuration"]
pub mod wexctrl;
#[doc = "Driver Control"]
pub struct DRVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Driver Control"]
pub mod drvctrl;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Count"]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count"]
pub mod count;
#[doc = "Count"]
pub struct COUNT_DITH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count"]
pub mod count_dith4;
#[doc = "Count"]
pub struct COUNT_DITH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count"]
pub mod count_dith5;
#[doc = "Count"]
pub struct COUNT_DITH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count"]
pub mod count_dith6;
#[doc = "Pattern"]
pub struct PATT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Pattern"]
pub mod patt;
#[doc = "Waveform Control"]
pub struct WAVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Waveform Control"]
pub mod wave;
#[doc = "Period"]
pub struct PER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Period"]
pub mod per;
#[doc = "Period"]
pub struct PER_DITH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Period"]
pub mod per_dith4;
#[doc = "Period"]
pub struct PER_DITH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Period"]
pub mod per_dith5;
#[doc = "Period"]
pub struct PER_DITH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Period"]
pub mod per_dith6;
#[doc = "Compare and Capture"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture"]
pub mod cc;
#[doc = "Compare and Capture"]
pub struct CC_DITH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture"]
pub mod cc_dith4;
#[doc = "Compare and Capture"]
pub struct CC_DITH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture"]
pub mod cc_dith5;
#[doc = "Compare and Capture"]
pub struct CC_DITH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture"]
pub mod cc_dith6;
#[doc = "Pattern Buffer"]
pub struct PATTB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Pattern Buffer"]
pub mod pattb;
#[doc = "Waveform Control Buffer"]
pub struct WAVEB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Waveform Control Buffer"]
pub mod waveb;
#[doc = "Period Buffer"]
pub struct PERB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Period Buffer"]
pub mod perb;
#[doc = "Period Buffer"]
pub struct PERB_DITH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Period Buffer"]
pub mod perb_dith4;
#[doc = "Period Buffer"]
pub struct PERB_DITH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Period Buffer"]
pub mod perb_dith5;
#[doc = "Period Buffer"]
pub struct PERB_DITH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Period Buffer"]
pub mod perb_dith6;
#[doc = "Compare and Capture Buffer"]
pub struct CCB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture Buffer"]
pub mod ccb;
#[doc = "Compare and Capture Buffer"]
pub struct CCB_DITH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith4;
#[doc = "Compare and Capture Buffer"]
pub struct CCB_DITH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith5;
#[doc = "Compare and Capture Buffer"]
pub struct CCB_DITH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith6;
