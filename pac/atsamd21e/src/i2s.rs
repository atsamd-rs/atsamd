#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04..0x0c - Clock Unit n Control"]
    pub clkctrl: [crate::Reg<clkctrl::CLKCTRL_SPEC>; 2],
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x18 - Synchronization Status"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    _reserved6: [u8; 0x06],
    #[doc = "0x20..0x28 - Serializer n Control"]
    pub serctrl: [crate::Reg<serctrl::SERCTRL_SPEC>; 2],
    _reserved7: [u8; 0x08],
    #[doc = "0x30..0x38 - Data n"]
    pub data: [crate::Reg<data::DATA_SPEC>; 2],
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CLKCTRL register accessor: an alias for `Reg<CLKCTRL_SPEC>`"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Clock Unit n Control"]
pub mod clkctrl;
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
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Status"]
pub mod syncbusy;
#[doc = "SERCTRL register accessor: an alias for `Reg<SERCTRL_SPEC>`"]
pub type SERCTRL = crate::Reg<serctrl::SERCTRL_SPEC>;
#[doc = "Serializer n Control"]
pub mod serctrl;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data n"]
pub mod data;
