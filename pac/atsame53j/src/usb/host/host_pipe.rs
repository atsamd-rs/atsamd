#[doc = r"Register block"]
#[repr(C)]
pub struct HOST_PIPE {
    #[doc = "0x00 - HOST_PIPE End Point Configuration"]
    pub pcfg: PCFG,
    _reserved1: [u8; 0x02],
    #[doc = "0x03 - HOST_PIPE Bus Access Period of Pipe"]
    pub binterval: BINTERVAL,
    #[doc = "0x04 - HOST_PIPE End Point Pipe Status Clear"]
    pub pstatusclr: PSTATUSCLR,
    #[doc = "0x05 - HOST_PIPE End Point Pipe Status Set"]
    pub pstatusset: PSTATUSSET,
    #[doc = "0x06 - HOST_PIPE End Point Pipe Status"]
    pub pstatus: PSTATUS,
    #[doc = "0x07 - HOST_PIPE Pipe Interrupt Flag"]
    pub pintflag: PINTFLAG,
    #[doc = "0x08 - HOST_PIPE Pipe Interrupt Flag Clear"]
    pub pintenclr: PINTENCLR,
    #[doc = "0x09 - HOST_PIPE Pipe Interrupt Flag Set"]
    pub pintenset: PINTENSET,
}
#[doc = "PCFG (rw) register accessor: an alias for `Reg<PCFG_SPEC>`"]
pub type PCFG = crate::Reg<pcfg::PCFG_SPEC>;
#[doc = "HOST_PIPE End Point Configuration"]
pub mod pcfg;
#[doc = "BINTERVAL (rw) register accessor: an alias for `Reg<BINTERVAL_SPEC>`"]
pub type BINTERVAL = crate::Reg<binterval::BINTERVAL_SPEC>;
#[doc = "HOST_PIPE Bus Access Period of Pipe"]
pub mod binterval;
#[doc = "PSTATUSCLR (w) register accessor: an alias for `Reg<PSTATUSCLR_SPEC>`"]
pub type PSTATUSCLR = crate::Reg<pstatusclr::PSTATUSCLR_SPEC>;
#[doc = "HOST_PIPE End Point Pipe Status Clear"]
pub mod pstatusclr;
#[doc = "PSTATUSSET (w) register accessor: an alias for `Reg<PSTATUSSET_SPEC>`"]
pub type PSTATUSSET = crate::Reg<pstatusset::PSTATUSSET_SPEC>;
#[doc = "HOST_PIPE End Point Pipe Status Set"]
pub mod pstatusset;
#[doc = "PSTATUS (r) register accessor: an alias for `Reg<PSTATUS_SPEC>`"]
pub type PSTATUS = crate::Reg<pstatus::PSTATUS_SPEC>;
#[doc = "HOST_PIPE End Point Pipe Status"]
pub mod pstatus;
#[doc = "PINTFLAG (rw) register accessor: an alias for `Reg<PINTFLAG_SPEC>`"]
pub type PINTFLAG = crate::Reg<pintflag::PINTFLAG_SPEC>;
#[doc = "HOST_PIPE Pipe Interrupt Flag"]
pub mod pintflag;
#[doc = "PINTENCLR (rw) register accessor: an alias for `Reg<PINTENCLR_SPEC>`"]
pub type PINTENCLR = crate::Reg<pintenclr::PINTENCLR_SPEC>;
#[doc = "HOST_PIPE Pipe Interrupt Flag Clear"]
pub mod pintenclr;
#[doc = "PINTENSET (rw) register accessor: an alias for `Reg<PINTENSET_SPEC>`"]
pub type PINTENSET = crate::Reg<pintenset::PINTENSET_SPEC>;
#[doc = "HOST_PIPE Pipe Interrupt Flag Set"]
pub mod pintenset;
