#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Control B"]
    pub ctrlb: crate::Reg<ctrlb::CTRLB_SPEC>,
    #[doc = "0x02 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x08 - Status A"]
    pub statusa: crate::Reg<statusa::STATUSA_SPEC>,
    #[doc = "0x09 - Status B"]
    pub statusb: crate::Reg<statusb::STATUSB_SPEC>,
    #[doc = "0x0a - Status C"]
    pub statusc: crate::Reg<statusc::STATUSC_SPEC>,
    _reserved9: [u8; 0x01],
    #[doc = "0x0c - Window Control"]
    pub winctrl: crate::Reg<winctrl::WINCTRL_SPEC>,
    _reserved10: [u8; 0x03],
    #[doc = "0x10..0x18 - Comparator Control n"]
    pub compctrl: [crate::Reg<compctrl::COMPCTRL_SPEC>; 2],
    _reserved11: [u8; 0x08],
    #[doc = "0x20 - Scaler n"]
    pub scaler: [crate::Reg<scaler::SCALER_SPEC>; 2],
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
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
#[doc = "STATUSA register accessor: an alias for `Reg<STATUSA_SPEC>`"]
pub type STATUSA = crate::Reg<statusa::STATUSA_SPEC>;
#[doc = "Status A"]
pub mod statusa;
#[doc = "STATUSB register accessor: an alias for `Reg<STATUSB_SPEC>`"]
pub type STATUSB = crate::Reg<statusb::STATUSB_SPEC>;
#[doc = "Status B"]
pub mod statusb;
#[doc = "STATUSC register accessor: an alias for `Reg<STATUSC_SPEC>`"]
pub type STATUSC = crate::Reg<statusc::STATUSC_SPEC>;
#[doc = "Status C"]
pub mod statusc;
#[doc = "WINCTRL register accessor: an alias for `Reg<WINCTRL_SPEC>`"]
pub type WINCTRL = crate::Reg<winctrl::WINCTRL_SPEC>;
#[doc = "Window Control"]
pub mod winctrl;
#[doc = "COMPCTRL register accessor: an alias for `Reg<COMPCTRL_SPEC>`"]
pub type COMPCTRL = crate::Reg<compctrl::COMPCTRL_SPEC>;
#[doc = "Comparator Control n"]
pub mod compctrl;
#[doc = "SCALER register accessor: an alias for `Reg<SCALER_SPEC>`"]
pub type SCALER = crate::Reg<scaler::SCALER_SPEC>;
#[doc = "Scaler n"]
pub mod scaler;
