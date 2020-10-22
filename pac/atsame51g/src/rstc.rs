#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Cause"]
    pub rcause: RCAUSE,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Backup Exit Source"]
    pub bkupexit: BKUPEXIT,
}
#[doc = "Reset Cause\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcause](rcause) module"]
pub type RCAUSE = crate::Reg<u8, _RCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCAUSE;
#[doc = "`read()` method returns [rcause::R](rcause::R) reader structure"]
impl crate::Readable for RCAUSE {}
#[doc = "Reset Cause"]
pub mod rcause;
#[doc = "Backup Exit Source\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkupexit](bkupexit) module"]
pub type BKUPEXIT = crate::Reg<u8, _BKUPEXIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKUPEXIT;
#[doc = "`read()` method returns [bkupexit::R](bkupexit::R) reader structure"]
impl crate::Readable for BKUPEXIT {}
#[doc = "Backup Exit Source"]
pub mod bkupexit;
