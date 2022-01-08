#[doc = "PCFG register accessor: an alias for `Reg<PCFG_SPEC>`"]
pub type PCFG = crate::Reg<pcfg::PCFG_SPEC>;
#[doc = "HOST_PIPE End Point Configuration"]
pub mod pcfg;
#[doc = "BINTERVAL register accessor: an alias for `Reg<BINTERVAL_SPEC>`"]
pub type BINTERVAL = crate::Reg<binterval::BINTERVAL_SPEC>;
#[doc = "HOST_PIPE Bus Access Period of Pipe"]
pub mod binterval;
#[doc = "PSTATUSCLR register accessor: an alias for `Reg<PSTATUSCLR_SPEC>`"]
pub type PSTATUSCLR = crate::Reg<pstatusclr::PSTATUSCLR_SPEC>;
#[doc = "HOST_PIPE End Point Pipe Status Clear"]
pub mod pstatusclr;
#[doc = "PSTATUSSET register accessor: an alias for `Reg<PSTATUSSET_SPEC>`"]
pub type PSTATUSSET = crate::Reg<pstatusset::PSTATUSSET_SPEC>;
#[doc = "HOST_PIPE End Point Pipe Status Set"]
pub mod pstatusset;
#[doc = "PSTATUS register accessor: an alias for `Reg<PSTATUS_SPEC>`"]
pub type PSTATUS = crate::Reg<pstatus::PSTATUS_SPEC>;
#[doc = "HOST_PIPE End Point Pipe Status"]
pub mod pstatus;
#[doc = "PINTFLAG register accessor: an alias for `Reg<PINTFLAG_SPEC>`"]
pub type PINTFLAG = crate::Reg<pintflag::PINTFLAG_SPEC>;
#[doc = "HOST_PIPE Pipe Interrupt Flag"]
pub mod pintflag;
#[doc = "PINTENCLR register accessor: an alias for `Reg<PINTENCLR_SPEC>`"]
pub type PINTENCLR = crate::Reg<pintenclr::PINTENCLR_SPEC>;
#[doc = "HOST_PIPE Pipe Interrupt Flag Clear"]
pub mod pintenclr;
#[doc = "PINTENSET register accessor: an alias for `Reg<PINTENSET_SPEC>`"]
pub type PINTENSET = crate::Reg<pintenset::PINTENSET_SPEC>;
#[doc = "HOST_PIPE Pipe Interrupt Flag Set"]
pub mod pintenset;
