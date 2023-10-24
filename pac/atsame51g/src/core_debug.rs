#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug Halting Control and Status Register"]
    pub dhcsr: DHCSR,
    #[doc = "0x04 - Debug Core Register Selector Register"]
    pub dcrsr: DCRSR,
    #[doc = "0x08 - Debug Core Register Data Register"]
    pub dcrdr: DCRDR,
    #[doc = "0x0c - Debug Exception and Monitor Control Register"]
    pub demcr: DEMCR,
}
#[doc = "DHCSR (rw) register accessor: Debug Halting Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhcsr`]
module"]
pub type DHCSR = crate::Reg<dhcsr::DHCSR_SPEC>;
#[doc = "Debug Halting Control and Status Register"]
pub mod dhcsr;
#[doc = "DCRSR (w) register accessor: Debug Core Register Selector Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrsr`]
module"]
pub type DCRSR = crate::Reg<dcrsr::DCRSR_SPEC>;
#[doc = "Debug Core Register Selector Register"]
pub mod dcrsr;
#[doc = "DCRDR (rw) register accessor: Debug Core Register Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcrdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrdr`]
module"]
pub type DCRDR = crate::Reg<dcrdr::DCRDR_SPEC>;
#[doc = "Debug Core Register Data Register"]
pub mod dcrdr;
#[doc = "DEMCR (rw) register accessor: Debug Exception and Monitor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`demcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@demcr`]
module"]
pub type DEMCR = crate::Reg<demcr::DEMCR_SPEC>;
#[doc = "Debug Exception and Monitor Control Register"]
pub mod demcr;
