#[doc = r"Register block"]
#[repr(C)]
pub struct SPI {
    #[doc = "0x00 - SPI Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - SPI Control B"]
    pub ctrlb: CTRLB,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - SPI Baud Rate"]
    pub baud: BAUD,
    _reserved3: [u8; 0x07],
    #[doc = "0x14 - SPI Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved4: [u8; 0x01],
    #[doc = "0x16 - SPI Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved5: [u8; 0x01],
    #[doc = "0x18 - SPI Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 0x01],
    #[doc = "0x1a - SPI Status"]
    pub status: STATUS,
    #[doc = "0x1c - SPI Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - SPI Address"]
    pub addr: ADDR,
    #[doc = "0x28 - SPI Data"]
    pub data: DATA,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - SPI Debug Control"]
    pub dbgctrl: DBGCTRL,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "SPI Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "SPI Control B"]
pub mod ctrlb;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "SPI Baud Rate"]
pub mod baud;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "SPI Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "SPI Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "SPI Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "SPI Status"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "SPI Synchronization Busy"]
pub mod syncbusy;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SPI Address"]
pub mod addr;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "SPI Data"]
pub mod data;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "SPI Debug Control"]
pub mod dbgctrl;
