#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80 - Priority A for Slave"]
    pub pras0: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0x84 - Priority B for Slave"]
    pub prbs0: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0x88 - Priority A for Slave"]
    pub pras1: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0x8c - Priority B for Slave"]
    pub prbs1: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0x90 - Priority A for Slave"]
    pub pras2: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0x94 - Priority B for Slave"]
    pub prbs2: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0x98 - Priority A for Slave"]
    pub pras3: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0x9c - Priority B for Slave"]
    pub prbs3: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xa0 - Priority A for Slave"]
    pub pras4: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xa4 - Priority B for Slave"]
    pub prbs4: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xa8 - Priority A for Slave"]
    pub pras5: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xac - Priority B for Slave"]
    pub prbs5: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xb0 - Priority A for Slave"]
    pub pras6: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xb4 - Priority B for Slave"]
    pub prbs6: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xb8 - Priority A for Slave"]
    pub pras7: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xbc - Priority B for Slave"]
    pub prbs7: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xc0 - Priority A for Slave"]
    pub pras8: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xc4 - Priority B for Slave"]
    pub prbs8: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xc8 - Priority A for Slave"]
    pub pras9: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xcc - Priority B for Slave"]
    pub prbs9: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xd0 - Priority A for Slave"]
    pub pras10: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xd4 - Priority B for Slave"]
    pub prbs10: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xd8 - Priority A for Slave"]
    pub pras11: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xdc - Priority B for Slave"]
    pub prbs11: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xe0 - Priority A for Slave"]
    pub pras12: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xe4 - Priority B for Slave"]
    pub prbs12: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xe8 - Priority A for Slave"]
    pub pras13: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xec - Priority B for Slave"]
    pub prbs13: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xf0 - Priority A for Slave"]
    pub pras14: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xf4 - Priority B for Slave"]
    pub prbs14: crate::Reg<prbs::PRBS_SPEC>,
    #[doc = "0xf8 - Priority A for Slave"]
    pub pras15: crate::Reg<pras::PRAS_SPEC>,
    #[doc = "0xfc - Priority B for Slave"]
    pub prbs15: crate::Reg<prbs::PRBS_SPEC>,
    _reserved32: [u8; 0x10],
    #[doc = "0x110..0x150 - Special Function"]
    pub sfr: [crate::Reg<sfr::SFR_SPEC>; 16],
}
#[doc = "PRAS register accessor: an alias for `Reg<PRAS_SPEC>`"]
pub type PRAS = crate::Reg<pras::PRAS_SPEC>;
#[doc = "Priority A for Slave"]
pub mod pras;
#[doc = "PRBS register accessor: an alias for `Reg<PRBS_SPEC>`"]
pub type PRBS = crate::Reg<prbs::PRBS_SPEC>;
#[doc = "Priority B for Slave"]
pub mod prbs;
#[doc = "SFR register accessor: an alias for `Reg<SFR_SPEC>`"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "Special Function"]
pub mod sfr;
