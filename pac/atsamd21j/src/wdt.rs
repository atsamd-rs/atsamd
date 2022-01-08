#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x01 - Configuration"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x02 - Early Warning Interrupt Control"]
    pub ewctrl: crate::Reg<ewctrl::EWCTRL_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x07 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Clear"]
    pub clear: crate::Reg<clear::CLEAR_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration"]
pub mod config;
#[doc = "EWCTRL register accessor: an alias for `Reg<EWCTRL_SPEC>`"]
pub type EWCTRL = crate::Reg<ewctrl::EWCTRL_SPEC>;
#[doc = "Early Warning Interrupt Control"]
pub mod ewctrl;
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
#[doc = "CLEAR register accessor: an alias for `Reg<CLEAR_SPEC>`"]
pub type CLEAR = crate::Reg<clear::CLEAR_SPEC>;
#[doc = "Clear"]
pub mod clear;
