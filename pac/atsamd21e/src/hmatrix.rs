#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    pras: (),
    _reserved1: [u8; 0x04],
    prbs: (),
    _reserved2: [u8; 0x8c],
    sfr: [Sfr; 16],
}
impl RegisterBlock {
    #[doc = "0x80..0xc0 - Priority A for Slave"]
    #[inline(always)]
    pub const fn pras(&self, n: usize) -> &Pras {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(128)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - Priority A for Slave"]
    #[inline(always)]
    pub fn pras_iter(&self) -> impl Iterator<Item = &Pras> {
        (0..16).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(128)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x84..0xc4 - Priority B for Slave"]
    #[inline(always)]
    pub const fn prbs(&self, n: usize) -> &Prbs {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(132)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x84..0xc4 - Priority B for Slave"]
    #[inline(always)]
    pub fn prbs_iter(&self) -> impl Iterator<Item = &Prbs> {
        (0..16).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(132)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x110..0x150 - Special Function"]
    #[inline(always)]
    pub const fn sfr(&self, n: usize) -> &Sfr {
        &self.sfr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x150 - Special Function"]
    #[inline(always)]
    pub fn sfr_iter(&self) -> impl Iterator<Item = &Sfr> {
        self.sfr.iter()
    }
}
#[doc = "PRAS (rw) register accessor: Priority A for Slave\n\nYou can [`read`](crate::Reg::read) this register and get [`pras::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pras`] module"]
#[doc(alias = "PRAS")]
pub type Pras = crate::Reg<pras::PrasSpec>;
#[doc = "Priority A for Slave"]
pub mod pras;
#[doc = "PRBS (rw) register accessor: Priority B for Slave\n\nYou can [`read`](crate::Reg::read) this register and get [`prbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prbs`] module"]
#[doc(alias = "PRBS")]
pub type Prbs = crate::Reg<prbs::PrbsSpec>;
#[doc = "Priority B for Slave"]
pub mod prbs;
#[doc = "SFR (rw) register accessor: Special Function\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr`] module"]
#[doc(alias = "SFR")]
pub type Sfr = crate::Reg<sfr::SfrSpec>;
#[doc = "Special Function"]
pub mod sfr;
