#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 0x01],
    #[doc = "0x08 - Status A"]
    pub statusa: STATUSA,
    #[doc = "0x09 - Status B"]
    pub statusb: STATUSB,
    #[doc = "0x0a - Status C"]
    pub statusc: STATUSC,
    _reserved9: [u8; 0x01],
    #[doc = "0x0c - Window Control"]
    pub winctrl: WINCTRL,
    _reserved10: [u8; 0x03],
    #[doc = "0x10..0x18 - Comparator Control n"]
    pub compctrl: [COMPCTRL; 2],
    _reserved11: [u8; 0x08],
    #[doc = "0x20 - Scaler n"]
    pub scaler: [SCALER; 2],
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (w) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
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
#[doc = "STATUSA (r) register accessor: an alias for `Reg<STATUSA_SPEC>`"]
pub type STATUSA = crate::Reg<statusa::STATUSA_SPEC>;
#[doc = "Status A"]
pub mod statusa;
#[doc = "STATUSB (r) register accessor: an alias for `Reg<STATUSB_SPEC>`"]
pub type STATUSB = crate::Reg<statusb::STATUSB_SPEC>;
#[doc = "Status B"]
pub mod statusb;
#[doc = "STATUSC (r) register accessor: an alias for `Reg<STATUSC_SPEC>`"]
pub type STATUSC = crate::Reg<statusc::STATUSC_SPEC>;
#[doc = "Status C"]
pub mod statusc;
#[doc = "WINCTRL (rw) register accessor: an alias for `Reg<WINCTRL_SPEC>`"]
pub type WINCTRL = crate::Reg<winctrl::WINCTRL_SPEC>;
#[doc = "Window Control"]
pub mod winctrl;
#[doc = "COMPCTRL (rw) register accessor: an alias for `Reg<COMPCTRL_SPEC>`"]
pub type COMPCTRL = crate::Reg<compctrl::COMPCTRL_SPEC>;
#[doc = "Comparator Control n"]
pub mod compctrl;
#[doc = "SCALER (rw) register accessor: an alias for `Reg<SCALER_SPEC>`"]
pub type SCALER = crate::Reg<scaler::SCALER_SPEC>;
#[doc = "Scaler n"]
pub mod scaler;
