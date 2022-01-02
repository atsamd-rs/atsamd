#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Reference Control"]
    pub refctrl: crate::Reg<refctrl::REFCTRL_SPEC>,
    #[doc = "0x02 - Average Control"]
    pub avgctrl: crate::Reg<avgctrl::AVGCTRL_SPEC>,
    #[doc = "0x03 - Sampling Time Control"]
    pub sampctrl: crate::Reg<sampctrl::SAMPCTRL_SPEC>,
    #[doc = "0x04 - Control B"]
    pub ctrlb: crate::Reg<ctrlb::CTRLB_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x08 - Window Monitor Control"]
    pub winctrl: crate::Reg<winctrl::WINCTRL_SPEC>,
    _reserved6: [u8; 0x03],
    #[doc = "0x0c - Software Trigger"]
    pub swtrig: crate::Reg<swtrig::SWTRIG_SPEC>,
    _reserved7: [u8; 0x03],
    #[doc = "0x10 - Input Control"]
    pub inputctrl: crate::Reg<inputctrl::INPUTCTRL_SPEC>,
    #[doc = "0x14 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    _reserved9: [u8; 0x01],
    #[doc = "0x16 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x17 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x18 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x19 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x1a - Result"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    #[doc = "0x1c - Window Monitor Lower Threshold"]
    pub winlt: crate::Reg<winlt::WINLT_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x20 - Window Monitor Upper Threshold"]
    pub winut: crate::Reg<winut::WINUT_SPEC>,
    _reserved16: [u8; 0x02],
    #[doc = "0x24 - Gain Correction"]
    pub gaincorr: crate::Reg<gaincorr::GAINCORR_SPEC>,
    #[doc = "0x26 - Offset Correction"]
    pub offsetcorr: crate::Reg<offsetcorr::OFFSETCORR_SPEC>,
    #[doc = "0x28 - Calibration"]
    pub calib: crate::Reg<calib::CALIB_SPEC>,
    #[doc = "0x2a - Debug Control"]
    pub dbgctrl: crate::Reg<dbgctrl::DBGCTRL_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "REFCTRL register accessor: an alias for `Reg<REFCTRL_SPEC>`"]
pub type REFCTRL = crate::Reg<refctrl::REFCTRL_SPEC>;
#[doc = "Reference Control"]
pub mod refctrl;
#[doc = "AVGCTRL register accessor: an alias for `Reg<AVGCTRL_SPEC>`"]
pub type AVGCTRL = crate::Reg<avgctrl::AVGCTRL_SPEC>;
#[doc = "Average Control"]
pub mod avgctrl;
#[doc = "SAMPCTRL register accessor: an alias for `Reg<SAMPCTRL_SPEC>`"]
pub type SAMPCTRL = crate::Reg<sampctrl::SAMPCTRL_SPEC>;
#[doc = "Sampling Time Control"]
pub mod sampctrl;
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "WINCTRL register accessor: an alias for `Reg<WINCTRL_SPEC>`"]
pub type WINCTRL = crate::Reg<winctrl::WINCTRL_SPEC>;
#[doc = "Window Monitor Control"]
pub mod winctrl;
#[doc = "SWTRIG register accessor: an alias for `Reg<SWTRIG_SPEC>`"]
pub type SWTRIG = crate::Reg<swtrig::SWTRIG_SPEC>;
#[doc = "Software Trigger"]
pub mod swtrig;
#[doc = "INPUTCTRL register accessor: an alias for `Reg<INPUTCTRL_SPEC>`"]
pub type INPUTCTRL = crate::Reg<inputctrl::INPUTCTRL_SPEC>;
#[doc = "Input Control"]
pub mod inputctrl;
#[doc = "EVCTRL register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result"]
pub mod result;
#[doc = "WINLT register accessor: an alias for `Reg<WINLT_SPEC>`"]
pub type WINLT = crate::Reg<winlt::WINLT_SPEC>;
#[doc = "Window Monitor Lower Threshold"]
pub mod winlt;
#[doc = "WINUT register accessor: an alias for `Reg<WINUT_SPEC>`"]
pub type WINUT = crate::Reg<winut::WINUT_SPEC>;
#[doc = "Window Monitor Upper Threshold"]
pub mod winut;
#[doc = "GAINCORR register accessor: an alias for `Reg<GAINCORR_SPEC>`"]
pub type GAINCORR = crate::Reg<gaincorr::GAINCORR_SPEC>;
#[doc = "Gain Correction"]
pub mod gaincorr;
#[doc = "OFFSETCORR register accessor: an alias for `Reg<OFFSETCORR_SPEC>`"]
pub type OFFSETCORR = crate::Reg<offsetcorr::OFFSETCORR_SPEC>;
#[doc = "Offset Correction"]
pub mod offsetcorr;
#[doc = "CALIB register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "Calibration"]
pub mod calib;
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
