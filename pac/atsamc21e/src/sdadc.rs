#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Reference Control"]
    pub refctrl: crate::Reg<refctrl::REFCTRL_SPEC>,
    #[doc = "0x02 - Control B"]
    pub ctrlb: crate::Reg<ctrlb::CTRLB_SPEC>,
    #[doc = "0x04 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    #[doc = "0x05 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x06 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x07 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x08 - Sequence Status"]
    pub seqstatus: crate::Reg<seqstatus::SEQSTATUS_SPEC>,
    #[doc = "0x09 - Input Control"]
    pub inputctrl: crate::Reg<inputctrl::INPUTCTRL_SPEC>,
    #[doc = "0x0a - Control C"]
    pub ctrlc: crate::Reg<ctrlc::CTRLC_SPEC>,
    #[doc = "0x0b - Window Monitor Control"]
    pub winctrl: crate::Reg<winctrl::WINCTRL_SPEC>,
    #[doc = "0x0c - Window Monitor Lower Threshold"]
    pub winlt: crate::Reg<winlt::WINLT_SPEC>,
    #[doc = "0x10 - Window Monitor Upper Threshold"]
    pub winut: crate::Reg<winut::WINUT_SPEC>,
    #[doc = "0x14 - Offset Correction"]
    pub offsetcorr: crate::Reg<offsetcorr::OFFSETCORR_SPEC>,
    #[doc = "0x18 - Gain Correction"]
    pub gaincorr: crate::Reg<gaincorr::GAINCORR_SPEC>,
    #[doc = "0x1a - Shift Correction"]
    pub shiftcorr: crate::Reg<shiftcorr::SHIFTCORR_SPEC>,
    _reserved16: [u8; 0x01],
    #[doc = "0x1c - Software Trigger"]
    pub swtrig: crate::Reg<swtrig::SWTRIG_SPEC>,
    _reserved17: [u8; 0x03],
    #[doc = "0x20 - Synchronization Busy"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x24 - Result"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    #[doc = "0x28 - Sequence Control"]
    pub seqctrl: crate::Reg<seqctrl::SEQCTRL_SPEC>,
    _reserved20: [u8; 0x03],
    #[doc = "0x2c - Analog Control"]
    pub anactrl: crate::Reg<anactrl::ANACTRL_SPEC>,
    _reserved21: [u8; 0x01],
    #[doc = "0x2e - Debug Control"]
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
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
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
#[doc = "SEQSTATUS register accessor: an alias for `Reg<SEQSTATUS_SPEC>`"]
pub type SEQSTATUS = crate::Reg<seqstatus::SEQSTATUS_SPEC>;
#[doc = "Sequence Status"]
pub mod seqstatus;
#[doc = "INPUTCTRL register accessor: an alias for `Reg<INPUTCTRL_SPEC>`"]
pub type INPUTCTRL = crate::Reg<inputctrl::INPUTCTRL_SPEC>;
#[doc = "Input Control"]
pub mod inputctrl;
#[doc = "CTRLC register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "WINCTRL register accessor: an alias for `Reg<WINCTRL_SPEC>`"]
pub type WINCTRL = crate::Reg<winctrl::WINCTRL_SPEC>;
#[doc = "Window Monitor Control"]
pub mod winctrl;
#[doc = "WINLT register accessor: an alias for `Reg<WINLT_SPEC>`"]
pub type WINLT = crate::Reg<winlt::WINLT_SPEC>;
#[doc = "Window Monitor Lower Threshold"]
pub mod winlt;
#[doc = "WINUT register accessor: an alias for `Reg<WINUT_SPEC>`"]
pub type WINUT = crate::Reg<winut::WINUT_SPEC>;
#[doc = "Window Monitor Upper Threshold"]
pub mod winut;
#[doc = "OFFSETCORR register accessor: an alias for `Reg<OFFSETCORR_SPEC>`"]
pub type OFFSETCORR = crate::Reg<offsetcorr::OFFSETCORR_SPEC>;
#[doc = "Offset Correction"]
pub mod offsetcorr;
#[doc = "GAINCORR register accessor: an alias for `Reg<GAINCORR_SPEC>`"]
pub type GAINCORR = crate::Reg<gaincorr::GAINCORR_SPEC>;
#[doc = "Gain Correction"]
pub mod gaincorr;
#[doc = "SHIFTCORR register accessor: an alias for `Reg<SHIFTCORR_SPEC>`"]
pub type SHIFTCORR = crate::Reg<shiftcorr::SHIFTCORR_SPEC>;
#[doc = "Shift Correction"]
pub mod shiftcorr;
#[doc = "SWTRIG register accessor: an alias for `Reg<SWTRIG_SPEC>`"]
pub type SWTRIG = crate::Reg<swtrig::SWTRIG_SPEC>;
#[doc = "Software Trigger"]
pub mod swtrig;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result"]
pub mod result;
#[doc = "SEQCTRL register accessor: an alias for `Reg<SEQCTRL_SPEC>`"]
pub type SEQCTRL = crate::Reg<seqctrl::SEQCTRL_SPEC>;
#[doc = "Sequence Control"]
pub mod seqctrl;
#[doc = "ANACTRL register accessor: an alias for `Reg<ANACTRL_SPEC>`"]
pub type ANACTRL = crate::Reg<anactrl::ANACTRL_SPEC>;
#[doc = "Analog Control"]
pub mod anactrl;
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
