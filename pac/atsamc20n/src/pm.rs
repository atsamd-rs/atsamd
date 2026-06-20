#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: crate::Reg<sleepcfg::SLEEPCFG_SPEC>,
    _reserved1: [u8; 0x06],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: crate::Reg<stdbycfg::STDBYCFG_SPEC>,
}
#[doc = "SLEEPCFG register accessor: an alias for `Reg<SLEEPCFG_SPEC>`"]
pub type SLEEPCFG = crate::Reg<sleepcfg::SLEEPCFG_SPEC>;
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "STDBYCFG register accessor: an alias for `Reg<STDBYCFG_SPEC>`"]
pub type STDBYCFG = crate::Reg<stdbycfg::STDBYCFG_SPEC>;
#[doc = "Standby Configuration"]
pub mod stdbycfg;
