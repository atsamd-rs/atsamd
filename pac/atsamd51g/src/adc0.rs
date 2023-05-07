#[doc = r"Register block"]
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
    _reserved6: [u8; 0x01],
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
    _reserved13: [u8; 0x17],
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
    _reserved22: [u8; 0x02],
    #[doc = "0x44 - Last Sample Result"]
    pub ress: RESS,
    _reserved23: [u8; 0x02],
    #[doc = "0x48 - Calibration"]
    pub calib: CALIB,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "INPUTCTRL (rw) register accessor: an alias for `Reg<INPUTCTRL_SPEC>`"]
pub type INPUTCTRL = crate::Reg<inputctrl::INPUTCTRL_SPEC>;
#[doc = "Input Control"]
pub mod inputctrl;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "REFCTRL (rw) register accessor: an alias for `Reg<REFCTRL_SPEC>`"]
pub type REFCTRL = crate::Reg<refctrl::REFCTRL_SPEC>;
#[doc = "Reference Control"]
pub mod refctrl;
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
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "DSEQDATA (w) register accessor: an alias for `Reg<DSEQDATA_SPEC>`"]
pub type DSEQDATA = crate::Reg<dseqdata::DSEQDATA_SPEC>;
#[doc = "DMA Sequencial Data"]
pub mod dseqdata;
#[doc = "DSEQCTRL (rw) register accessor: an alias for `Reg<DSEQCTRL_SPEC>`"]
pub type DSEQCTRL = crate::Reg<dseqctrl::DSEQCTRL_SPEC>;
#[doc = "DMA Sequential Control"]
pub mod dseqctrl;
#[doc = "DSEQSTAT (r) register accessor: an alias for `Reg<DSEQSTAT_SPEC>`"]
pub type DSEQSTAT = crate::Reg<dseqstat::DSEQSTAT_SPEC>;
#[doc = "DMA Sequencial Status"]
pub mod dseqstat;
#[doc = "RESULT (r) register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result Conversion Value"]
pub mod result;
#[doc = "RESS (r) register accessor: an alias for `Reg<RESS_SPEC>`"]
pub type RESS = crate::Reg<ress::RESS_SPEC>;
#[doc = "Last Sample Result"]
pub mod ress;
#[doc = "CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "Calibration"]
pub mod calib;
