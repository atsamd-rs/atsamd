#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control"]
    pub channelctrl: CHANNELCTRL,
    #[doc = "0x04 - Channel n Interrupt Enable Clear"]
    pub chintenclr: CHINTENCLR,
    #[doc = "0x05 - Channel n Interrupt Enable Set"]
    pub chintenset: CHINTENSET,
    #[doc = "0x06 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: CHINTFLAG,
    #[doc = "0x07 - Channel n Status"]
    pub chstatus: CHSTATUS,
}
#[doc = "CHANNELCTRL (rw) register accessor: an alias for `Reg<CHANNELCTRL_SPEC>`"]
pub type CHANNELCTRL = crate::Reg<channelctrl::CHANNELCTRL_SPEC>;
#[doc = "Channel n Control"]
pub mod channelctrl;
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
#[doc = "CHSTATUS (r) register accessor: an alias for `Reg<CHSTATUS_SPEC>`"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = "Channel n Status"]
pub mod chstatus;
