#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rcause: Rcause,
    _reserved1: [u8; 0x01],
    bkupexit: Bkupexit,
}
impl RegisterBlock {
    #[doc = "0x00 - Reset Cause"]
    #[inline(always)]
    pub const fn rcause(&self) -> &Rcause {
        &self.rcause
    }
    #[doc = "0x02 - Backup Exit Source"]
    #[inline(always)]
    pub const fn bkupexit(&self) -> &Bkupexit {
        &self.bkupexit
    }
}
#[doc = "RCAUSE (r) register accessor: Reset Cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rcause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcause`]
module"]
#[doc(alias = "RCAUSE")]
pub type Rcause = crate::Reg<rcause::RcauseSpec>;
#[doc = "Reset Cause"]
pub mod rcause;
#[doc = "BKUPEXIT (r) register accessor: Backup Exit Source\n\nYou can [`read`](crate::Reg::read) this register and get [`bkupexit::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkupexit`]
module"]
#[doc(alias = "BKUPEXIT")]
pub type Bkupexit = crate::Reg<bkupexit::BkupexitSpec>;
#[doc = "Backup Exit Source"]
pub mod bkupexit;
