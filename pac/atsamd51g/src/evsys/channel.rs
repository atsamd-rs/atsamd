#[doc = "CHANNEL register accessor: an alias for `Reg<CHANNEL_SPEC>`"]
pub type CHANNEL = crate::Reg<channel::CHANNEL_SPEC>;
#[doc = "Channel n Control"]
pub mod channel;
#[doc = "CHINTENCLR register accessor: an alias for `Reg<CHINTENCLR_SPEC>`"]
pub type CHINTENCLR = crate::Reg<chintenclr::CHINTENCLR_SPEC>;
#[doc = "Channel n Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "CHINTENSET register accessor: an alias for `Reg<CHINTENSET_SPEC>`"]
pub type CHINTENSET = crate::Reg<chintenset::CHINTENSET_SPEC>;
#[doc = "Channel n Interrupt Enable Set"]
pub mod chintenset;
#[doc = "CHINTFLAG register accessor: an alias for `Reg<CHINTFLAG_SPEC>`"]
pub type CHINTFLAG = crate::Reg<chintflag::CHINTFLAG_SPEC>;
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "CHSTATUS register accessor: an alias for `Reg<CHSTATUS_SPEC>`"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = "Channel n Status"]
pub mod chstatus;
