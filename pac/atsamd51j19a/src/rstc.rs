#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Cause"]
    pub rcause: RCAUSE,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Backup Exit Source"]
    pub bkupexit: BKUPEXIT,
}
#[doc = "Reset Cause"]
pub struct RCAUSE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Reset Cause"]
pub mod rcause;
#[doc = "Backup Exit Source"]
pub struct BKUPEXIT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Backup Exit Source"]
pub mod bkupexit;
