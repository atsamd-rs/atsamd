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
#[doc = "CSR (rw) register accessor: SysTick Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "SysTick Control and Status Register"]
pub mod csr;
#[doc = "RVR (rw) register accessor: SysTick Reload Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rvr`]
module"]
pub type RVR = crate::Reg<rvr::RVR_SPEC>;
#[doc = "SysTick Reload Value Register"]
pub mod rvr;
#[doc = "CVR (rw) register accessor: SysTick Current Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvr`]
module"]
pub type CVR = crate::Reg<cvr::CVR_SPEC>;
#[doc = "SysTick Current Value Register"]
pub mod cvr;
#[doc = "CALIB (r) register accessor: SysTick Calibration Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib`]
module"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "SysTick Calibration Value Register"]
pub mod calib;
