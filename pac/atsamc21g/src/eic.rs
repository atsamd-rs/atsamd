#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - NMI Control"]
    pub nmictrl: crate::Reg<nmictrl::NMICTRL_SPEC>,
    #[doc = "0x02 - NMI Interrupt Flag"]
    pub nmiflag: crate::Reg<nmiflag::NMIFLAG_SPEC>,
    #[doc = "0x04 - Syncbusy register"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    #[doc = "0x08 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x18 - EIC Asynchronous edge Detection Enable"]
    pub asynch: crate::Reg<asynch::ASYNCH_SPEC>,
    #[doc = "0x1c..0x24 - Configuration n"]
    pub config: [crate::Reg<config::CONFIG_SPEC>; 2],
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "NMICTRL register accessor: an alias for `Reg<NMICTRL_SPEC>`"]
pub type NMICTRL = crate::Reg<nmictrl::NMICTRL_SPEC>;
#[doc = "NMI Control"]
pub mod nmictrl;
#[doc = "NMIFLAG register accessor: an alias for `Reg<NMIFLAG_SPEC>`"]
pub type NMIFLAG = crate::Reg<nmiflag::NMIFLAG_SPEC>;
#[doc = "NMI Interrupt Flag"]
pub mod nmiflag;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Syncbusy register"]
pub mod syncbusy;
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
#[doc = "ASYNCH register accessor: an alias for `Reg<ASYNCH_SPEC>`"]
pub type ASYNCH = crate::Reg<asynch::ASYNCH_SPEC>;
#[doc = "EIC Asynchronous edge Detection Enable"]
pub mod asynch;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration n"]
pub mod config;
