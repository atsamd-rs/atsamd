#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A Register"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B Register"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Config A register"]
    pub cfga: CFGA,
    _reserved3: [u8; 0x04],
    #[doc = "0x08 - Interrupt Enable Clear Register"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt Enable Set Register"]
    pub intenset: INTENSET,
    #[doc = "0x0a - Interrupt Flag Register"]
    pub intflag: INTFLAG,
    #[doc = "0x0b - Status Register"]
    pub status: STATUS,
    #[doc = "0x0c - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x10 - Count Value Register"]
    pub value: VALUE,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A Register"]
pub mod ctrla;
#[doc = "CTRLB (w) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B Register"]
pub mod ctrlb;
#[doc = "CFGA (rw) register accessor: an alias for `Reg<CFGA_SPEC>`"]
pub type CFGA = crate::Reg<cfga::CFGA_SPEC>;
#[doc = "Config A register"]
pub mod cfga;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear Register"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set Register"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "VALUE (r) register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Count Value Register"]
pub mod value;
