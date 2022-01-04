#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x04 - Control B"]
    pub ctrlb: crate::Reg<ctrlb::CTRLB_SPEC>,
    #[doc = "0x08 - Baud Rate"]
    pub baud: crate::Reg<baud::BAUD_SPEC>,
    #[doc = "0x0c - Receive Data"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x10 - Transmit Data"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x14 - Interrupt Enable Clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x18 - Interrupt Enable Set"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x1c - Interrupt Flag Status and Clear"]
    pub intflag: crate::Reg<intflag::INTFLAG_SPEC>,
    #[doc = "0x20 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - Instruction Address"]
    pub instraddr: crate::Reg<instraddr::INSTRADDR_SPEC>,
    #[doc = "0x34 - Instruction Code"]
    pub instrctrl: crate::Reg<instrctrl::INSTRCTRL_SPEC>,
    #[doc = "0x38 - Instruction Frame"]
    pub instrframe: crate::Reg<instrframe::INSTRFRAME_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - Scrambling Mode"]
    pub scrambctrl: crate::Reg<scrambctrl::SCRAMBCTRL_SPEC>,
    #[doc = "0x44 - Scrambling Key"]
    pub scrambkey: crate::Reg<scrambkey::SCRAMBKEY_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "BAUD register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud Rate"]
pub mod baud;
#[doc = "RXDATA register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "Receive Data"]
pub mod rxdata;
#[doc = "TXDATA register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "Transmit Data"]
pub mod txdata;
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
#[doc = "Status Register"]
pub mod status;
#[doc = "INSTRADDR register accessor: an alias for `Reg<INSTRADDR_SPEC>`"]
pub type INSTRADDR = crate::Reg<instraddr::INSTRADDR_SPEC>;
#[doc = "Instruction Address"]
pub mod instraddr;
#[doc = "INSTRCTRL register accessor: an alias for `Reg<INSTRCTRL_SPEC>`"]
pub type INSTRCTRL = crate::Reg<instrctrl::INSTRCTRL_SPEC>;
#[doc = "Instruction Code"]
pub mod instrctrl;
#[doc = "INSTRFRAME register accessor: an alias for `Reg<INSTRFRAME_SPEC>`"]
pub type INSTRFRAME = crate::Reg<instrframe::INSTRFRAME_SPEC>;
#[doc = "Instruction Frame"]
pub mod instrframe;
#[doc = "SCRAMBCTRL register accessor: an alias for `Reg<SCRAMBCTRL_SPEC>`"]
pub type SCRAMBCTRL = crate::Reg<scrambctrl::SCRAMBCTRL_SPEC>;
#[doc = "Scrambling Mode"]
pub mod scrambctrl;
#[doc = "SCRAMBKEY register accessor: an alias for `Reg<SCRAMBKEY_SPEC>`"]
pub type SCRAMBKEY = crate::Reg<scrambkey::SCRAMBKEY_SPEC>;
#[doc = "Scrambling Key"]
pub mod scrambkey;
