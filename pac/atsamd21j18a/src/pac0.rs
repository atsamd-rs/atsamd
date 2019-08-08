#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write Protection Clear"]
    pub wpclr: WPCLR,
    #[doc = "0x04 - Write Protection Set"]
    pub wpset: WPSET,
}
#[doc = "Write Protection Clear"]
pub struct WPCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Clear"]
pub mod wpclr;
#[doc = "Write Protection Set"]
pub struct WPSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Set"]
pub mod wpset;
