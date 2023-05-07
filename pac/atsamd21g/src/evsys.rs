#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Channel"]
    pub channel: CHANNEL,
    #[doc = "0x08 - User Multiplexer"]
    pub user: USER,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - Channel Status"]
    pub chstatus: CHSTATUS,
    #[doc = "0x10 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x14 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x18 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
}
#[doc = "CTRL (w) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "CHANNEL (rw) register accessor: an alias for `Reg<CHANNEL_SPEC>`"]
pub type CHANNEL = crate::Reg<channel::CHANNEL_SPEC>;
#[doc = "Channel"]
pub mod channel;
#[doc = "USER (rw) register accessor: an alias for `Reg<USER_SPEC>`"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "User Multiplexer"]
pub mod user;
#[doc = "CHSTATUS (r) register accessor: an alias for `Reg<CHSTATUS_SPEC>`"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = "Channel Status"]
pub mod chstatus;
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
