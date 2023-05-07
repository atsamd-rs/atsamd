#[doc = r"Register block"]
#[repr(C)]
pub struct SPIS {
    #[doc = "0x00 - SPIS Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - SPIS Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - SPIS Control C"]
    pub ctrlc: CTRLC,
    #[doc = "0x0c - SPIS Baud Rate"]
    pub baud: BAUD,
    _reserved4: [u8; 0x07],
    #[doc = "0x14 - SPIS Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0x01],
    #[doc = "0x16 - SPIS Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved6: [u8; 0x01],
    #[doc = "0x18 - SPIS Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved7: [u8; 0x01],
    #[doc = "0x1a - SPIS Status"]
    pub status: STATUS,
    #[doc = "0x1c - SPIS Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    _reserved9: [u8; 0x02],
    #[doc = "0x22 - SPIS Length"]
    pub length: LENGTH,
    #[doc = "0x24 - SPIS Address"]
    pub addr: ADDR,
    #[doc = "0x28 - SPIS Data"]
    pub data: DATA,
    _reserved12: [u8; 0x04],
    #[doc = "0x30 - SPIS Debug Control"]
    pub dbgctrl: DBGCTRL,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "SPIS Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "SPIS Control B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "SPIS Control C"]
pub mod ctrlc;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "SPIS Baud Rate"]
pub mod baud;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "SPIS Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "SPIS Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "SPIS Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "SPIS Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "SPIS Synchronization Busy"]
pub mod syncbusy;
#[doc = "LENGTH (rw) register accessor: an alias for `Reg<LENGTH_SPEC>`"]
pub type LENGTH = crate::Reg<length::LENGTH_SPEC>;
#[doc = "SPIS Length"]
pub mod length;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SPIS Address"]
pub mod addr;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "SPIS Data"]
pub mod data;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "SPIS Debug Control"]
pub mod dbgctrl;
