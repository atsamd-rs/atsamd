#[doc = r"Register block"]
#[repr(C)]
pub struct I2CM {
    #[doc = "0x00 - I2CM Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - I2CM Control B"]
    pub ctrlb: CTRLB,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - I2CM Baud Rate"]
    pub baud: BAUD,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - I2CM Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved4: [u8; 0x01],
    #[doc = "0x16 - I2CM Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved5: [u8; 0x01],
    #[doc = "0x18 - I2CM Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 0x01],
    #[doc = "0x1a - I2CM Status"]
    pub status: STATUS,
    #[doc = "0x1c - I2CM Syncbusy"]
    pub syncbusy: SYNCBUSY,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - I2CM Address"]
    pub addr: ADDR,
    #[doc = "0x28 - I2CM Data"]
    pub data: DATA,
    _reserved10: [u8; 0x07],
    #[doc = "0x30 - I2CM Debug Control"]
    pub dbgctrl: DBGCTRL,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "I2CM Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "I2CM Control B"]
pub mod ctrlb;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "I2CM Baud Rate"]
pub mod baud;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "I2CM Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "I2CM Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "I2CM Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "I2CM Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "I2CM Syncbusy"]
pub mod syncbusy;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "I2CM Address"]
pub mod addr;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "I2CM Data"]
pub mod data;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "I2CM Debug Control"]
pub mod dbgctrl;
