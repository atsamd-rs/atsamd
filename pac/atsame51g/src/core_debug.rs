#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dhcsr: Dhcsr,
    dcrsr: Dcrsr,
    dcrdr: Dcrdr,
    demcr: Demcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Debug Halting Control and Status Register"]
    #[inline(always)]
    pub const fn dhcsr(&self) -> &Dhcsr {
        &self.dhcsr
    }
    #[doc = "0x04 - Debug Core Register Selector Register"]
    #[inline(always)]
    pub const fn dcrsr(&self) -> &Dcrsr {
        &self.dcrsr
    }
    #[doc = "0x08 - Debug Core Register Data Register"]
    #[inline(always)]
    pub const fn dcrdr(&self) -> &Dcrdr {
        &self.dcrdr
    }
    #[doc = "0x0c - Debug Exception and Monitor Control Register"]
    #[inline(always)]
    pub const fn demcr(&self) -> &Demcr {
        &self.demcr
    }
}
#[doc = "DHCSR (rw) register accessor: Debug Halting Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhcsr`] module"]
#[doc(alias = "DHCSR")]
pub type Dhcsr = crate::Reg<dhcsr::DhcsrSpec>;
#[doc = "Debug Halting Control and Status Register"]
pub mod dhcsr;
#[doc = "DCRSR (w) register accessor: Debug Core Register Selector Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrsr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrsr`] module"]
#[doc(alias = "DCRSR")]
pub type Dcrsr = crate::Reg<dcrsr::DcrsrSpec>;
#[doc = "Debug Core Register Selector Register"]
pub mod dcrsr;
#[doc = "DCRDR (rw) register accessor: Debug Core Register Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcrdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcrdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrdr`] module"]
#[doc(alias = "DCRDR")]
pub type Dcrdr = crate::Reg<dcrdr::DcrdrSpec>;
#[doc = "Debug Core Register Data Register"]
pub mod dcrdr;
#[doc = "DEMCR (rw) register accessor: Debug Exception and Monitor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`demcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`demcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@demcr`] module"]
#[doc(alias = "DEMCR")]
pub type Demcr = crate::Reg<demcr::DemcrSpec>;
#[doc = "Debug Exception and Monitor Control Register"]
pub mod demcr;
