#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x09 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    _reserved5: [u8; 0x15],
    #[doc = "0x20 - Output Data"]
    pub data: crate::Reg<data::DATA_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
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
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Output Data"]
pub mod data;
