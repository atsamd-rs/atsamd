#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x02 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x03 - Debug Control"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x04 - Input Control"]
    pub inputctrl: INPUTCTRL,
    #[doc = "0x06 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - Reference Control"]
    pub refctrl: REFCTRL,
    _reserved6: [u8; 1usize],
    #[doc = "0x0a - Average Control"]
    pub avgctrl: AVGCTRL,
    #[doc = "0x0b - Sample Time Control"]
    pub sampctrl: SAMPCTRL,
    #[doc = "0x0c - Window Monitor Lower Threshold"]
    pub winlt: WINLT,
    #[doc = "0x0e - Window Monitor Upper Threshold"]
    pub winut: WINUT,
    #[doc = "0x10 - Gain Correction"]
    pub gaincorr: GAINCORR,
    #[doc = "0x12 - Offset Correction"]
    pub offsetcorr: OFFSETCORR,
    #[doc = "0x14 - Software Trigger"]
    pub swtrig: SWTRIG,
    _reserved13: [u8; 23usize],
    #[doc = "0x2c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x2d - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x2e - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x2f - Status"]
    pub status: STATUS,
    #[doc = "0x30 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x34 - DMA Sequencial Data"]
    pub dseqdata: DSEQDATA,
    #[doc = "0x38 - DMA Sequential Control"]
    pub dseqctrl: DSEQCTRL,
    #[doc = "0x3c - DMA Sequencial Status"]
    pub dseqstat: DSEQSTAT,
    #[doc = "0x40 - Result Conversion Value"]
    pub result: RESULT,
    _reserved22: [u8; 2usize],
    #[doc = "0x44 - Last Sample Result"]
    pub ress: RESS,
    _reserved23: [u8; 2usize],
    #[doc = "0x48 - Calibration"]
    pub calib: CALIB,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Input Control"]
pub struct INPUTCTRL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Input Control"]
pub mod inputctrl;
#[doc = "Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "Reference Control"]
pub struct REFCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Reference Control"]
pub mod refctrl;
#[doc = "Average Control"]
pub struct AVGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Average Control"]
pub mod avgctrl;
#[doc = "Sample Time Control"]
pub struct SAMPCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Sample Time Control"]
pub mod sampctrl;
#[doc = "Window Monitor Lower Threshold"]
pub struct WINLT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Window Monitor Lower Threshold"]
pub mod winlt;
#[doc = "Window Monitor Upper Threshold"]
pub struct WINUT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Window Monitor Upper Threshold"]
pub mod winut;
#[doc = "Gain Correction"]
pub struct GAINCORR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Gain Correction"]
pub mod gaincorr;
#[doc = "Offset Correction"]
pub struct OFFSETCORR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Offset Correction"]
pub mod offsetcorr;
#[doc = "Software Trigger"]
pub struct SWTRIG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Software Trigger"]
pub mod swtrig;
#[doc = "Interrupt Enable Clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear"]
pub struct INTFLAG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Synchronization Busy"]
pub struct SYNCBUSY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "DMA Sequencial Data"]
pub struct DSEQDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Sequencial Data"]
pub mod dseqdata;
#[doc = "DMA Sequential Control"]
pub struct DSEQCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Sequential Control"]
pub mod dseqctrl;
#[doc = "DMA Sequencial Status"]
pub struct DSEQSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Sequencial Status"]
pub mod dseqstat;
#[doc = "Result Conversion Value"]
pub struct RESULT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Result Conversion Value"]
pub mod result;
#[doc = "Last Sample Result"]
pub struct RESS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Last Sample Result"]
pub mod ress;
#[doc = "Calibration"]
pub struct CALIB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Calibration"]
pub mod calib;
