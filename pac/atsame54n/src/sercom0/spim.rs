#[doc = r"Register block"]
#[repr(C)]
pub struct SPIM {
    #[doc = "0x00 - SPIM Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - SPIM Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - SPIM Control C"]
    pub ctrlc: CTRLC,
    #[doc = "0x0c - SPIM Baud Rate"]
    pub baud: BAUD,
    _reserved4: [u8; 0x07],
    #[doc = "0x14 - SPIM Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0x01],
    #[doc = "0x16 - SPIM Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved6: [u8; 0x01],
    #[doc = "0x18 - SPIM Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved7: [u8; 0x01],
    #[doc = "0x1a - SPIM Status"]
    pub status: STATUS,
    #[doc = "0x1c - SPIM Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    _reserved9: [u8; 0x02],
    #[doc = "0x22 - SPIM Length"]
    pub length: LENGTH,
    #[doc = "0x24 - SPIM Address"]
    pub addr: ADDR,
    #[doc = "0x28 - SPIM Data"]
    pub data: DATA,
    _reserved12: [u8; 0x04],
    #[doc = "0x30 - SPIM Debug Control"]
    pub dbgctrl: DBGCTRL,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "SPIM Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "SPIM Control B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "SPIM Control C"]
pub mod ctrlc;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "SPIM Baud Rate"]
pub mod baud;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "SPIM Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "SPIM Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "SPIM Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "SPIM Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "SPIM Synchronization Busy"]
pub mod syncbusy;
#[doc = "LENGTH (rw) register accessor: an alias for `Reg<LENGTH_SPEC>`"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "SPIM Length"]
pub mod length;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SPIM Address"]
pub mod addr;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "SPIM Data"]
pub mod data;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "SPIM Debug Control"]
pub mod dbgctrl;
