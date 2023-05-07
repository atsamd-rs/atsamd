#[doc = r"Register block"]
#[repr(C)]
pub struct PRS {
    #[doc = "0x00 - Priority A for Slave"]
    pub pras: PRAS,
    #[doc = "0x04 - Priority B for Slave"]
    pub prbs: PRBS,
}
#[doc = "PRAS (rw) register accessor: an alias for `Reg<PRAS_SPEC>`"]
pub type PRAS = crate::Reg<pras::PRAS_SPEC>;
#[doc = "Priority A for Slave"]
pub mod pras;
#[doc = "PRBS (rw) register accessor: an alias for `Reg<PRBS_SPEC>`"]
pub type PRBS = crate::Reg<prbs::PRBS_SPEC>;
#[doc = "Priority B for Slave"]
pub mod prbs;
