#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Cause"]
    pub rcause: RCAUSE,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Backup Exit Source"]
    pub bkupexit: BKUPEXIT,
}
#[doc = "RCAUSE (r) register accessor: an alias for `Reg<RCAUSE_SPEC>`"]
pub type RCAUSE = crate::Reg<rcause::RCAUSE_SPEC>;
#[doc = "Reset Cause"]
pub mod rcause;
#[doc = "BKUPEXIT (r) register accessor: an alias for `Reg<BKUPEXIT_SPEC>`"]
pub type BKUPEXIT = crate::Reg<bkupexit::BKUPEXIT_SPEC>;
#[doc = "Backup Exit Source"]
pub mod bkupexit;
