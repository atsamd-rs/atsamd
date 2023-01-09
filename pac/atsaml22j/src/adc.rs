#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Reference Control"]
    pub refctrl: REFCTRL,
    #[doc = "0x03 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x07 - Sequence Status"]
    pub seqstatus: SEQSTATUS,
    #[doc = "0x08 - Input Control"]
    pub inputctrl: INPUTCTRL,
    #[doc = "0x0a - Control C"]
    pub ctrlc: CTRLC,
    #[doc = "0x0c - Average Control"]
    pub avgctrl: AVGCTRL,
    #[doc = "0x0d - Sample Time Control"]
    pub sampctrl: SAMPCTRL,
    #[doc = "0x0e - Window Monitor Lower Threshold"]
    pub winlt: WINLT,
    #[doc = "0x10 - Window Monitor Upper Threshold"]
    pub winut: WINUT,
    #[doc = "0x12 - Gain Correction"]
    pub gaincorr: GAINCORR,
    #[doc = "0x14 - Offset Correction"]
    pub offsetcorr: OFFSETCORR,
    _reserved16: [u8; 0x02],
    #[doc = "0x18 - Software Trigger"]
    pub swtrig: SWTRIG,
    _reserved17: [u8; 0x03],
    #[doc = "0x1c - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved18: [u8; 0x03],
    #[doc = "0x20 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    _reserved19: [u8; 0x02],
    #[doc = "0x24 - Result"]
    pub result: RESULT,
    _reserved20: [u8; 0x02],
    #[doc = "0x28 - Sequence Control"]
    pub seqctrl: SEQCTRL,
    #[doc = "0x2c - Calibration"]
    pub calib: CALIB,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "REFCTRL (rw) register accessor: an alias for `Reg<REFCTRL_SPEC>`"]
pub type REFCTRL = crate::Reg<refctrl::REFCTRL_SPEC>;
#[doc = "Reference Control"]
pub mod refctrl;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "SEQSTATUS (r) register accessor: an alias for `Reg<SEQSTATUS_SPEC>`"]
pub type SEQSTATUS = crate::Reg<seqstatus::SEQSTATUS_SPEC>;
#[doc = "Sequence Status"]
pub mod seqstatus;
#[doc = "INPUTCTRL (rw) register accessor: an alias for `Reg<INPUTCTRL_SPEC>`"]
pub type INPUTCTRL = crate::Reg<inputctrl::INPUTCTRL_SPEC>;
#[doc = "Input Control"]
pub mod inputctrl;
#[doc = "CTRLC (rw) register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "AVGCTRL (rw) register accessor: an alias for `Reg<AVGCTRL_SPEC>`"]
pub type AVGCTRL = crate::Reg<avgctrl::AVGCTRL_SPEC>;
#[doc = "Average Control"]
pub mod avgctrl;
#[doc = "SAMPCTRL (rw) register accessor: an alias for `Reg<SAMPCTRL_SPEC>`"]
pub type SAMPCTRL = crate::Reg<sampctrl::SAMPCTRL_SPEC>;
#[doc = "Sample Time Control"]
pub mod sampctrl;
#[doc = "WINLT (rw) register accessor: an alias for `Reg<WINLT_SPEC>`"]
pub type WINLT = crate::Reg<winlt::WINLT_SPEC>;
#[doc = "Window Monitor Lower Threshold"]
pub mod winlt;
#[doc = "WINUT (rw) register accessor: an alias for `Reg<WINUT_SPEC>`"]
pub type WINUT = crate::Reg<winut::WINUT_SPEC>;
#[doc = "Window Monitor Upper Threshold"]
pub mod winut;
#[doc = "GAINCORR (rw) register accessor: an alias for `Reg<GAINCORR_SPEC>`"]
pub type GAINCORR = crate::Reg<gaincorr::GAINCORR_SPEC>;
#[doc = "Gain Correction"]
pub mod gaincorr;
#[doc = "OFFSETCORR (rw) register accessor: an alias for `Reg<OFFSETCORR_SPEC>`"]
pub type OFFSETCORR = crate::Reg<offsetcorr::OFFSETCORR_SPEC>;
#[doc = "Offset Correction"]
pub mod offsetcorr;
#[doc = "SWTRIG (rw) register accessor: an alias for `Reg<SWTRIG_SPEC>`"]
pub type SWTRIG = crate::Reg<swtrig::SWTRIG_SPEC>;
#[doc = "Software Trigger"]
pub mod swtrig;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "RESULT (r) register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result"]
pub mod result;
#[doc = "SEQCTRL (rw) register accessor: an alias for `Reg<SEQCTRL_SPEC>`"]
pub type SEQCTRL = crate::Reg<seqctrl::SEQCTRL_SPEC>;
#[doc = "Sequence Control"]
pub mod seqctrl;
#[doc = "CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "Calibration"]
pub mod calib;
