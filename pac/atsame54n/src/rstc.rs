#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Cause"]
    pub rcause: RCAUSE,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Backup Exit Source"]
    pub bkupexit: BKUPEXIT,
}
#[doc = "RCAUSE (r) register accessor: Reset Cause\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcause`]
module"]
pub type RCAUSE = crate::Reg<rcause::RCAUSE_SPEC>;
#[doc = "Reset Cause"]
pub mod rcause;
#[doc = "BKUPEXIT (r) register accessor: Backup Exit Source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkupexit::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkupexit`]
module"]
pub type BKUPEXIT = crate::Reg<bkupexit::BKUPEXIT_SPEC>;
#[doc = "Backup Exit Source"]
pub mod bkupexit;
