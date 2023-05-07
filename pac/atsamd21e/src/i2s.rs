#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x03],
    #[doc = "0x04..0x0c - Clock Unit n Control"]
    pub clkctrl: [CLKCTRL; 2],
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 0x02],
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved5: [u8; 0x02],
    #[doc = "0x18 - Synchronization Status"]
    pub syncbusy: SYNCBUSY,
    _reserved6: [u8; 0x06],
    #[doc = "0x20..0x28 - Serializer n Control"]
    pub serctrl: [SERCTRL; 2],
    _reserved7: [u8; 0x08],
    #[doc = "0x30..0x38 - Data n"]
    pub data: [DATA; 2],
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CLKCTRL (rw) register accessor: an alias for `Reg<CLKCTRL_SPEC>`"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Clock Unit n Control"]
pub mod clkctrl;
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
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Status"]
pub mod syncbusy;
#[doc = "SERCTRL (rw) register accessor: an alias for `Reg<SERCTRL_SPEC>`"]
pub type SERCTRL = crate::Reg<serctrl::SERCTRL_SPEC>;
#[doc = "Serializer n Control"]
pub mod serctrl;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data n"]
pub mod data;
