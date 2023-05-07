#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x01 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x02 - Interrupt Flag"]
    pub intflag: INTFLAG,
    #[doc = "0x03 - Status"]
    pub status: STATUS,
    #[doc = "0x04 - Error Address"]
    pub erraddr: ERRADDR,
    _reserved5: [u8; 0x07],
    #[doc = "0x0f - Debug Control"]
    pub dbgctrl: DBGCTRL,
}
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
#[doc = "Interrupt Flag"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "ERRADDR (r) register accessor: an alias for `Reg<ERRADDR_SPEC>`"]
pub type ERRADDR = crate::Reg<erraddr::ERRADDR_SPEC>;
#[doc = "Error Address"]
pub mod erraddr;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
