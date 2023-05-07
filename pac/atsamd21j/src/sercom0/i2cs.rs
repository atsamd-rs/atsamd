#[doc = r"Register block"]
#[repr(C)]
pub struct I2CS {
    #[doc = "0x00 - I2CS Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - I2CS Control B"]
    pub ctrlb: CTRLB,
    _reserved2: [u8; 0x0c],
    #[doc = "0x14 - I2CS Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 0x01],
    #[doc = "0x16 - I2CS Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved4: [u8; 0x01],
    #[doc = "0x18 - I2CS Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved5: [u8; 0x01],
    #[doc = "0x1a - I2CS Status"]
    pub status: STATUS,
    #[doc = "0x1c - I2CS Syncbusy"]
    pub syncbusy: SYNCBUSY,
    _reserved7: [u8; 0x04],
    #[doc = "0x24 - I2CS Address"]
    pub addr: ADDR,
    #[doc = "0x28 - I2CS Data"]
    pub data: DATA,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "I2CS Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "I2CS Control B"]
pub mod ctrlb;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "I2CS Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "I2CS Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "I2CS Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "I2CS Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "I2CS Syncbusy"]
pub mod syncbusy;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "I2CS Address"]
pub mod addr;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "I2CS Data"]
pub mod data;
