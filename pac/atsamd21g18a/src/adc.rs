#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Reference Control"]
    pub refctrl: REFCTRL,
    #[doc = "0x02 - Average Control"]
    pub avgctrl: AVGCTRL,
    #[doc = "0x03 - Sampling Time Control"]
    pub sampctrl: SAMPCTRL,
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    _reserved5: [u8; 2usize],
    #[doc = "0x08 - Window Monitor Control"]
    pub winctrl: WINCTRL,
    _reserved6: [u8; 3usize],
    #[doc = "0x0c - Software Trigger"]
    pub swtrig: SWTRIG,
    _reserved7: [u8; 3usize],
    #[doc = "0x10 - Input Control"]
    pub inputctrl: INPUTCTRL,
    #[doc = "0x14 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved9: [u8; 1usize],
    #[doc = "0x16 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x17 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x18 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x19 - Status"]
    pub status: STATUS,
    #[doc = "0x1a - Result"]
    pub result: RESULT,
    #[doc = "0x1c - Window Monitor Lower Threshold"]
    pub winlt: WINLT,
    _reserved15: [u8; 2usize],
    #[doc = "0x20 - Window Monitor Upper Threshold"]
    pub winut: WINUT,
    _reserved16: [u8; 2usize],
    #[doc = "0x24 - Gain Correction"]
    pub gaincorr: GAINCORR,
    #[doc = "0x26 - Offset Correction"]
    pub offsetcorr: OFFSETCORR,
    #[doc = "0x28 - Calibration"]
    pub calib: CALIB,
    #[doc = "0x2a - Debug Control"]
    pub dbgctrl: DBGCTRL,
}
#[doc = "Control A"]
pub struct CTRLA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control A"]
pub mod ctrla;
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
#[doc = "Sampling Time Control"]
pub struct SAMPCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Sampling Time Control"]
pub mod sampctrl;
#[doc = "Control B"]
pub struct CTRLB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "Window Monitor Control"]
pub struct WINCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Window Monitor Control"]
pub mod winctrl;
#[doc = "Software Trigger"]
pub struct SWTRIG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Software Trigger"]
pub mod swtrig;
#[doc = "Input Control"]
pub struct INPUTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Control"]
pub mod inputctrl;
#[doc = "Event Control"]
pub struct EVCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "Result"]
pub struct RESULT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Result"]
pub mod result;
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
#[doc = "Calibration"]
pub struct CALIB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Calibration"]
pub mod calib;
#[doc = "Debug Control"]
pub struct DBGCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Debug Control"]
pub mod dbgctrl;
