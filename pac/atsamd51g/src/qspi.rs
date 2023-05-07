#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - Baud Rate"]
    pub baud: BAUD,
    #[doc = "0x0c - Receive Data"]
    pub rxdata: RXDATA,
    #[doc = "0x10 - Transmit Data"]
    pub txdata: TXDATA,
    #[doc = "0x14 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x18 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x1c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x20 - Status Register"]
    pub status: STATUS,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - Instruction Address"]
    pub instraddr: INSTRADDR,
    #[doc = "0x34 - Instruction Code"]
    pub instrctrl: INSTRCTRL,
    #[doc = "0x38 - Instruction Frame"]
    pub instrframe: INSTRFRAME,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - Scrambling Mode"]
    pub scrambctrl: SCRAMBCTRL,
    #[doc = "0x44 - Scrambling Key"]
    pub scrambkey: SCRAMBKEY,
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "BAUD (rw) register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud Rate"]
pub mod baud;
#[doc = "RXDATA (r) register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Data"]
pub mod rxdata;
#[doc = "TXDATA (w) register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit Data"]
pub mod txdata;
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
#[doc = "Status Register"]
pub mod status;
#[doc = "INSTRADDR (rw) register accessor: an alias for `Reg<INSTRADDR_SPEC>`"]
pub type INSTRADDR = crate::Reg<instraddr::INSTRADDR_SPEC>;
#[doc = "Instruction Address"]
pub mod instraddr;
#[doc = "INSTRCTRL (rw) register accessor: an alias for `Reg<INSTRCTRL_SPEC>`"]
pub type INSTRCTRL = crate::Reg<instrctrl::INSTRCTRL_SPEC>;
#[doc = "Instruction Code"]
pub mod instrctrl;
#[doc = "INSTRFRAME (rw) register accessor: an alias for `Reg<INSTRFRAME_SPEC>`"]
pub type INSTRFRAME = crate::Reg<instrframe::INSTRFRAME_SPEC>;
#[doc = "Instruction Frame"]
pub mod instrframe;
#[doc = "SCRAMBCTRL (rw) register accessor: an alias for `Reg<SCRAMBCTRL_SPEC>`"]
pub type SCRAMBCTRL = crate::Reg<scrambctrl::SCRAMBCTRL_SPEC>;
#[doc = "Scrambling Mode"]
pub mod scrambctrl;
#[doc = "SCRAMBKEY (w) register accessor: an alias for `Reg<SCRAMBKEY_SPEC>`"]
pub type SCRAMBKEY = crate::Reg<scrambkey::SCRAMBKEY_SPEC>;
#[doc = "Scrambling Key"]
pub mod scrambkey;
