#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control A"]
    pub chctrla: CHCTRLA,
    #[doc = "0x04 - Channel n Control B"]
    pub chctrlb: CHCTRLB,
    #[doc = "0x05 - Channel n Priority Level"]
    pub chprilvl: CHPRILVL,
    #[doc = "0x06 - Channel n Event Control"]
    pub chevctrl: CHEVCTRL,
    _reserved4: [u8; 0x05],
    #[doc = "0x0c - Channel n Interrupt Enable Clear"]
    pub chintenclr: CHINTENCLR,
    #[doc = "0x0d - Channel n Interrupt Enable Set"]
    pub chintenset: CHINTENSET,
    #[doc = "0x0e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: CHINTFLAG,
    #[doc = "0x0f - Channel n Status"]
    pub chstatus: CHSTATUS,
}
#[doc = "CHCTRLA (rw) register accessor: an alias for `Reg<CHCTRLA_SPEC>`"]
pub type CHCTRLA = crate::Reg<chctrla::CHCTRLA_SPEC>;
#[doc = "Channel n Control A"]
pub mod chctrla;
#[doc = "CHCTRLB (rw) register accessor: an alias for `Reg<CHCTRLB_SPEC>`"]
pub type CHCTRLB = crate::Reg<chctrlb::CHCTRLB_SPEC>;
#[doc = "Channel n Control B"]
pub mod chctrlb;
#[doc = "CHPRILVL (rw) register accessor: an alias for `Reg<CHPRILVL_SPEC>`"]
pub type CHPRILVL = crate::Reg<chprilvl::CHPRILVL_SPEC>;
#[doc = "Channel n Priority Level"]
pub mod chprilvl;
#[doc = "CHEVCTRL (rw) register accessor: an alias for `Reg<CHEVCTRL_SPEC>`"]
pub type CHEVCTRL = crate::Reg<chevctrl::CHEVCTRL_SPEC>;
#[doc = "Channel n Event Control"]
pub mod chevctrl;
#[doc = "CHINTENCLR (rw) register accessor: an alias for `Reg<CHINTENCLR_SPEC>`"]
pub type CHINTENCLR = crate::Reg<chintenclr::CHINTENCLR_SPEC>;
#[doc = "Channel n Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "CHINTENSET (rw) register accessor: an alias for `Reg<CHINTENSET_SPEC>`"]
pub type CHINTENSET = crate::Reg<chintenset::CHINTENSET_SPEC>;
#[doc = "Channel n Interrupt Enable Set"]
pub mod chintenset;
#[doc = "CHINTFLAG (rw) register accessor: an alias for `Reg<CHINTFLAG_SPEC>`"]
pub type CHINTFLAG = crate::Reg<chintflag::CHINTFLAG_SPEC>;
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "CHSTATUS (rw) register accessor: an alias for `Reg<CHSTATUS_SPEC>`"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = "Channel n Status"]
pub mod chstatus;
