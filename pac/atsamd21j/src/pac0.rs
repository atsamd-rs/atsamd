#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write Protection Clear"]
    pub wpclr: WPCLR,
    #[doc = "0x04 - Write Protection Set"]
    pub wpset: WPSET,
}
#[doc = "WPCLR (rw) register accessor: an alias for `Reg<WPCLR_SPEC>`"]
pub type WPCLR = crate::Reg<wpclr::WPCLR_SPEC>;
#[doc = "Write Protection Clear"]
pub mod wpclr;
#[doc = "WPSET (rw) register accessor: an alias for `Reg<WPSET_SPEC>`"]
pub type WPSET = crate::Reg<wpset::WPSET_SPEC>;
#[doc = "Write Protection Set"]
pub mod wpset;
