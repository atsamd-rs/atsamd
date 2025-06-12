#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    type_: Type,
    cfg: Cfg,
    ctrl: Ctrl,
    sr: Sr,
    lckway: Lckway,
    _reserved5: [u8; 0x0c],
    maint0: Maint0,
    maint1: Maint1,
    mcfg: Mcfg,
    men: Men,
    mctrl: Mctrl,
    msr: Msr,
}
impl RegisterBlock {
    #[doc = "0x00 - Cache Type Register"]
    #[inline(always)]
    pub const fn type_(&self) -> &Type {
        &self.type_
    }
    #[doc = "0x04 - Cache Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - Cache Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - Cache Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - Cache Lock per Way Register"]
    #[inline(always)]
    pub const fn lckway(&self) -> &Lckway {
        &self.lckway
    }
    #[doc = "0x20 - Cache Maintenance Register 0"]
    #[inline(always)]
    pub const fn maint0(&self) -> &Maint0 {
        &self.maint0
    }
    #[doc = "0x24 - Cache Maintenance Register 1"]
    #[inline(always)]
    pub const fn maint1(&self) -> &Maint1 {
        &self.maint1
    }
    #[doc = "0x28 - Cache Monitor Configuration Register"]
    #[inline(always)]
    pub const fn mcfg(&self) -> &Mcfg {
        &self.mcfg
    }
    #[doc = "0x2c - Cache Monitor Enable Register"]
    #[inline(always)]
    pub const fn men(&self) -> &Men {
        &self.men
    }
    #[doc = "0x30 - Cache Monitor Control Register"]
    #[inline(always)]
    pub const fn mctrl(&self) -> &Mctrl {
        &self.mctrl
    }
    #[doc = "0x34 - Cache Monitor Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
}
#[doc = "TYPE (r) register accessor: Cache Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`type_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type_`] module"]
#[doc(alias = "TYPE")]
pub type Type = crate::Reg<type_::TypeSpec>;
#[doc = "Cache Type Register"]
pub mod type_;
#[doc = "CFG (rw) register accessor: Cache Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Cache Configuration Register"]
pub mod cfg;
#[doc = "CTRL (w) register accessor: Cache Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Cache Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: Cache Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Cache Status Register"]
pub mod sr;
#[doc = "LCKWAY (rw) register accessor: Cache Lock per Way Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lckway::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckway::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckway`] module"]
#[doc(alias = "LCKWAY")]
pub type Lckway = crate::Reg<lckway::LckwaySpec>;
#[doc = "Cache Lock per Way Register"]
pub mod lckway;
#[doc = "MAINT0 (w) register accessor: Cache Maintenance Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maint0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maint0`] module"]
#[doc(alias = "MAINT0")]
pub type Maint0 = crate::Reg<maint0::Maint0Spec>;
#[doc = "Cache Maintenance Register 0"]
pub mod maint0;
#[doc = "MAINT1 (w) register accessor: Cache Maintenance Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maint1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maint1`] module"]
#[doc(alias = "MAINT1")]
pub type Maint1 = crate::Reg<maint1::Maint1Spec>;
#[doc = "Cache Maintenance Register 1"]
pub mod maint1;
#[doc = "MCFG (rw) register accessor: Cache Monitor Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg`] module"]
#[doc(alias = "MCFG")]
pub type Mcfg = crate::Reg<mcfg::McfgSpec>;
#[doc = "Cache Monitor Configuration Register"]
pub mod mcfg;
#[doc = "MEN (rw) register accessor: Cache Monitor Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`men::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`men::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@men`] module"]
#[doc(alias = "MEN")]
pub type Men = crate::Reg<men::MenSpec>;
#[doc = "Cache Monitor Enable Register"]
pub mod men;
#[doc = "MCTRL (w) register accessor: Cache Monitor Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctrl`] module"]
#[doc(alias = "MCTRL")]
pub type Mctrl = crate::Reg<mctrl::MctrlSpec>;
#[doc = "Cache Monitor Control Register"]
pub mod mctrl;
#[doc = "MSR (r) register accessor: Cache Monitor Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`] module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Cache Monitor Status Register"]
pub mod msr;
