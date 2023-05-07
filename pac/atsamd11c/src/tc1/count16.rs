#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT16 {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x02 - Read Request"]
    pub readreq: READREQ,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: CTRLBSET,
    #[doc = "0x06 - Control C"]
    pub ctrlc: CTRLC,
    _reserved5: [u8; 0x01],
    #[doc = "0x08 - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved6: [u8; 0x01],
    #[doc = "0x0a - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x0d - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0e - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0f - Status"]
    pub status: STATUS,
    #[doc = "0x10 - COUNT16 Counter Value"]
    pub count: COUNT,
    _reserved12: [u8; 0x06],
    #[doc = "0x18 - COUNT16 Compare/Capture"]
    pub cc: [CC; 2],
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "READREQ (rw) register accessor: an alias for `Reg<READREQ_SPEC>`"]
pub type READREQ = crate::Reg<readreq::READREQ_SPEC>;
#[doc = "Read Request"]
pub mod readreq;
#[doc = "CTRLBCLR (rw) register accessor: an alias for `Reg<CTRLBCLR_SPEC>`"]
pub type CTRLBCLR = crate::Reg<ctrlbclr::CTRLBCLR_SPEC>;
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "CTRLBSET (rw) register accessor: an alias for `Reg<CTRLBSET_SPEC>`"]
pub type CTRLBSET = crate::Reg<ctrlbset::CTRLBSET_SPEC>;
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "CTRLC (rw) register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "COUNT (rw) register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "COUNT16 Counter Value"]
pub mod count;
#[doc = "CC (rw) register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "COUNT16 Compare/Capture"]
pub mod cc;
