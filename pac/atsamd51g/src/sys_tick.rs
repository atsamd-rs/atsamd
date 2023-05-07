#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SysTick Control and Status Register"]
    pub csr: CSR,
    #[doc = "0x04 - SysTick Reload Value Register"]
    pub rvr: RVR,
    #[doc = "0x08 - SysTick Current Value Register"]
    pub cvr: CVR,
    #[doc = "0x0c - SysTick Calibration Value Register"]
    pub calib: CALIB,
}
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "SysTick Control and Status Register"]
pub mod csr;
#[doc = "RVR (rw) register accessor: an alias for `Reg<RVR_SPEC>`"]
pub type RVR = crate::Reg<rvr::RVR_SPEC>;
#[doc = "SysTick Reload Value Register"]
pub mod rvr;
#[doc = "CVR (rw) register accessor: an alias for `Reg<CVR_SPEC>`"]
pub type CVR = crate::Reg<cvr::CVR_SPEC>;
#[doc = "SysTick Current Value Register"]
pub mod cvr;
#[doc = "CALIB (r) register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "SysTick Calibration Value Register"]
pub mod calib;
