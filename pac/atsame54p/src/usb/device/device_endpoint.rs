#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE_ENDPOINT {
    #[doc = "0x00 - DEVICE_ENDPOINT End Point Configuration"]
    pub epcfg: EPCFG,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - DEVICE_ENDPOINT End Point Pipe Status Clear"]
    pub epstatusclr: EPSTATUSCLR,
    #[doc = "0x05 - DEVICE_ENDPOINT End Point Pipe Status Set"]
    pub epstatusset: EPSTATUSSET,
    #[doc = "0x06 - DEVICE_ENDPOINT End Point Pipe Status"]
    pub epstatus: EPSTATUS,
    #[doc = "0x07 - DEVICE_ENDPOINT End Point Interrupt Flag"]
    pub epintflag: EPINTFLAG,
    #[doc = "0x08 - DEVICE_ENDPOINT End Point Interrupt Clear Flag"]
    pub epintenclr: EPINTENCLR,
    #[doc = "0x09 - DEVICE_ENDPOINT End Point Interrupt Set Flag"]
    pub epintenset: EPINTENSET,
}
#[doc = "EPCFG (rw) register accessor: an alias for `Reg<EPCFG_SPEC>`"]
pub type EPCFG = crate::Reg<epcfg::EPCFG_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Configuration"]
pub mod epcfg;
#[doc = "EPSTATUSCLR (w) register accessor: an alias for `Reg<EPSTATUSCLR_SPEC>`"]
pub type EPSTATUSCLR = crate::Reg<epstatusclr::EPSTATUSCLR_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Clear"]
pub mod epstatusclr;
#[doc = "EPSTATUSSET (w) register accessor: an alias for `Reg<EPSTATUSSET_SPEC>`"]
pub type EPSTATUSSET = crate::Reg<epstatusset::EPSTATUSSET_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Set"]
pub mod epstatusset;
#[doc = "EPSTATUS (r) register accessor: an alias for `Reg<EPSTATUS_SPEC>`"]
pub type EPSTATUS = crate::Reg<epstatus::EPSTATUS_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status"]
pub mod epstatus;
#[doc = "EPINTFLAG (rw) register accessor: an alias for `Reg<EPINTFLAG_SPEC>`"]
pub type EPINTFLAG = crate::Reg<epintflag::EPINTFLAG_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Flag"]
pub mod epintflag;
#[doc = "EPINTENCLR (rw) register accessor: an alias for `Reg<EPINTENCLR_SPEC>`"]
pub type EPINTENCLR = crate::Reg<epintenclr::EPINTENCLR_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Clear Flag"]
pub mod epintenclr;
#[doc = "EPINTENSET (rw) register accessor: an alias for `Reg<EPINTENSET_SPEC>`"]
pub type EPINTENSET = crate::Reg<epintenset::EPINTENSET_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Set Flag"]
pub mod epintenset;
