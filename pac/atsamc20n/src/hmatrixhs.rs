#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - Master Configuration"]
    pub mcfg: [crate::Reg<mcfg::MCFG_SPEC>; 16],
    #[doc = "0x40..0x80 - Slave Configuration"]
    pub scfg: [crate::Reg<scfg::SCFG_SPEC>; 16],
    #[doc = "0x80..0xa0 - PRS\\[%s\\]"]
    pub prs: [PRS; 4],
    _reserved3: [u8; 0x60],
    #[doc = "0x100 - Master Remap Control"]
    pub mrcr: crate::Reg<mrcr::MRCR_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x110..0x150 - Special Function"]
    pub sfr: [crate::Reg<sfr::SFR_SPEC>; 16],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRS {
    #[doc = "0x00 - Priority A for Slave"]
    pub pras: crate::Reg<self::prs::pras::PRAS_SPEC>,
    #[doc = "0x04 - Priority B for Slave"]
    pub prbs: crate::Reg<self::prs::prbs::PRBS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "PRS\\[%s\\]"]
pub mod prs;
#[doc = "MCFG register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Master Configuration"]
pub mod mcfg;
#[doc = "SCFG register accessor: an alias for `Reg<SCFG_SPEC>`"]
pub type SCFG = crate::Reg<scfg::SCFG_SPEC>;
#[doc = "Slave Configuration"]
pub mod scfg;
#[doc = "MRCR register accessor: an alias for `Reg<MRCR_SPEC>`"]
pub type MRCR = crate::Reg<mrcr::MRCR_SPEC>;
#[doc = "Master Remap Control"]
pub mod mrcr;
#[doc = "SFR register accessor: an alias for `Reg<SFR_SPEC>`"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "Special Function"]
pub mod sfr;
