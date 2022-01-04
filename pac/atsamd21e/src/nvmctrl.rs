#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Control B"]
    pub ctrlb: crate::Reg<ctrlb::CTRLB_SPEC>,
    #[doc = "0x08 - NVM Parameter"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved4: [u8; 0x03],
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    _reserved5: [u8; 0x03],
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    _reserved6: [u8; 0x03],
    #[doc = "0x18 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x1c - Address"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x20 - Lock Section"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "PARAM register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "NVM Parameter"]
pub mod param;
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
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address"]
pub mod addr;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Lock Section"]
pub mod lock;
