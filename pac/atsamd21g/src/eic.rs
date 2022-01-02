#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x01 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x02 - Non-Maskable Interrupt Control"]
    pub nmictrl: crate::Reg<nmictrl::NMICTRL_SPEC>,
    #[doc = "0x03 - Non-Maskable Interrupt Flag Status and Clear"]
    pub nmiflag: crate::Reg<nmiflag::NMIFLAG_SPEC>,
    #[doc = "0x04 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    #[doc = "0x08 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x0c - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x10 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x14 - Wake-Up Enable"]
    pub wakeup: crate::Reg<wakeup::WAKEUP_SPEC>,
    #[doc = "0x18..0x20 - Configuration n"]
    pub config: [crate::Reg<config::CONFIG_SPEC>; 2],
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "NMICTRL register accessor: an alias for `Reg<NMICTRL_SPEC>`"]
pub type NMICTRL = crate::Reg<nmictrl::NMICTRL_SPEC>;
#[doc = "Non-Maskable Interrupt Control"]
pub mod nmictrl;
#[doc = "NMIFLAG register accessor: an alias for `Reg<NMIFLAG_SPEC>`"]
pub type NMIFLAG = crate::Reg<nmiflag::NMIFLAG_SPEC>;
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub mod nmiflag;
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
#[doc = "WAKEUP register accessor: an alias for `Reg<WAKEUP_SPEC>`"]
pub type WAKEUP = crate::Reg<wakeup::WAKEUP_SPEC>;
#[doc = "Wake-Up Enable"]
pub mod wakeup;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration n"]
pub mod config;
