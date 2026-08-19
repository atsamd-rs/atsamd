#[repr(C)]
#[doc = "PRS\\[%s\\]"]
#[doc(alias = "PRS")]
pub struct Prs {
    pras: Pras,
    prbs: Prbs,
}
impl Prs {
    #[doc = "0x00 - Priority A for Slave"]
    #[inline(always)]
    pub const fn pras(&self) -> &Pras {
        &self.pras
    }
    #[doc = "0x04 - Priority B for Slave"]
    #[inline(always)]
    pub const fn prbs(&self) -> &Prbs {
        &self.prbs
    }
}
#[doc = "PRAS (rw) register accessor: Priority A for Slave\n\nYou can [`read`](crate::Reg::read) this register and get [`pras::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras`]
module"]
#[doc(alias = "PRAS")]
pub type Pras = crate::Reg<pras::PrasSpec>;
#[doc = "Priority A for Slave"]
pub mod pras;
#[doc = "PRBS (rw) register accessor: Priority B for Slave\n\nYou can [`read`](crate::Reg::read) this register and get [`prbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prbs`]
module"]
#[doc(alias = "PRBS")]
pub type Prbs = crate::Reg<prbs::PrbsSpec>;
#[doc = "Priority B for Slave"]
pub mod prbs;
