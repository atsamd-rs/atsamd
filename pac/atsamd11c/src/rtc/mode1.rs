#[doc = r"Register block"]
#[repr(C)]
pub struct MODE1 {
    #[doc = "0x00 - MODE1 Control"]
    pub ctrl: CTRL,
    #[doc = "0x02 - Read Request"]
    pub readreq: READREQ,
    #[doc = "0x04 - MODE1 Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x06 - MODE1 Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x07 - MODE1 Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - MODE1 Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 0x01],
    #[doc = "0x0a - Status"]
    pub status: STATUS,
    #[doc = "0x0b - Debug Control"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x0c - Frequency Correction"]
    pub freqcorr: FREQCORR,
    _reserved9: [u8; 0x03],
    #[doc = "0x10 - MODE1 Counter Value"]
    pub count: COUNT,
    _reserved10: [u8; 0x02],
    #[doc = "0x14 - MODE1 Counter Period"]
    pub per: PER,
    _reserved11: [u8; 0x02],
    #[doc = "0x18 - MODE1 Compare n Value"]
    pub comp: [COMP; 2],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "MODE1 Control"]
pub mod ctrl;
#[doc = "READREQ (rw) register accessor: an alias for `Reg<READREQ_SPEC>`"]
pub type READREQ = crate::Reg<readreq::READREQ_SPEC>;
#[doc = "Read Request"]
pub mod readreq;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "MODE1 Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "MODE1 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "MODE1 Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "MODE1 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "FREQCORR (rw) register accessor: an alias for `Reg<FREQCORR_SPEC>`"]
pub type FREQCORR = crate::Reg<freqcorr::FREQCORR_SPEC>;
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "COUNT (rw) register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "MODE1 Counter Value"]
pub mod count;
#[doc = "PER (rw) register accessor: an alias for `Reg<PER_SPEC>`"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "MODE1 Counter Period"]
pub mod per;
#[doc = "COMP (rw) register accessor: an alias for `Reg<COMP_SPEC>`"]
pub type COMP = crate::Reg<comp::COMP_SPEC>;
#[doc = "MODE1 Compare n Value"]
pub mod comp;
