#[repr(C)]
#[doc = "GROUP\\[%s\\]"]
#[doc(alias = "GROUP")]
pub struct Group {
    dir: Dir,
    dirclr: Dirclr,
    dirset: Dirset,
    dirtgl: Dirtgl,
    out: Out,
    outclr: Outclr,
    outset: Outset,
    outtgl: Outtgl,
    in_: In,
    ctrl: Ctrl,
    wrconfig: Wrconfig,
    evctrl: Evctrl,
    pmux: [Pmux; 16],
    pincfg: [Pincfg; 32],
}
impl Group {
    #[doc = "0x00 - Data Direction"]
    #[inline(always)]
    pub const fn dir(&self) -> &Dir {
        &self.dir
    }
    #[doc = "0x04 - Data Direction Clear"]
    #[inline(always)]
    pub const fn dirclr(&self) -> &Dirclr {
        &self.dirclr
    }
    #[doc = "0x08 - Data Direction Set"]
    #[inline(always)]
    pub const fn dirset(&self) -> &Dirset {
        &self.dirset
    }
    #[doc = "0x0c - Data Direction Toggle"]
    #[inline(always)]
    pub const fn dirtgl(&self) -> &Dirtgl {
        &self.dirtgl
    }
    #[doc = "0x10 - Data Output Value"]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
    #[doc = "0x14 - Data Output Value Clear"]
    #[inline(always)]
    pub const fn outclr(&self) -> &Outclr {
        &self.outclr
    }
    #[doc = "0x18 - Data Output Value Set"]
    #[inline(always)]
    pub const fn outset(&self) -> &Outset {
        &self.outset
    }
    #[doc = "0x1c - Data Output Value Toggle"]
    #[inline(always)]
    pub const fn outtgl(&self) -> &Outtgl {
        &self.outtgl
    }
    #[doc = "0x20 - Data Input Value"]
    #[inline(always)]
    pub const fn in_(&self) -> &In {
        &self.in_
    }
    #[doc = "0x24 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x28 - Write Configuration"]
    #[inline(always)]
    pub const fn wrconfig(&self) -> &Wrconfig {
        &self.wrconfig
    }
    #[doc = "0x2c - Event Input Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x30..0x40 - Peripheral Multiplexing"]
    #[inline(always)]
    pub const fn pmux(&self, n: usize) -> &Pmux {
        &self.pmux[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - Peripheral Multiplexing"]
    #[inline(always)]
    pub fn pmux_iter(&self) -> impl Iterator<Item = &Pmux> {
        self.pmux.iter()
    }
    #[doc = "0x40..0x60 - Pin Configuration"]
    #[inline(always)]
    pub const fn pincfg(&self, n: usize) -> &Pincfg {
        &self.pincfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg_iter(&self) -> impl Iterator<Item = &Pincfg> {
        self.pincfg.iter()
    }
}
#[doc = "DIR (rw) register accessor: Data Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`] module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "Data Direction"]
pub mod dir;
#[doc = "DIRCLR (rw) register accessor: Data Direction Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dirclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirclr`] module"]
#[doc(alias = "DIRCLR")]
pub type Dirclr = crate::Reg<dirclr::DirclrSpec>;
#[doc = "Data Direction Clear"]
pub mod dirclr;
#[doc = "DIRSET (rw) register accessor: Data Direction Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dirset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirset`] module"]
#[doc(alias = "DIRSET")]
pub type Dirset = crate::Reg<dirset::DirsetSpec>;
#[doc = "Data Direction Set"]
pub mod dirset;
#[doc = "DIRTGL (rw) register accessor: Data Direction Toggle\n\nYou can [`read`](crate::Reg::read) this register and get [`dirtgl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirtgl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirtgl`] module"]
#[doc(alias = "DIRTGL")]
pub type Dirtgl = crate::Reg<dirtgl::DirtglSpec>;
#[doc = "Data Direction Toggle"]
pub mod dirtgl;
#[doc = "OUT (rw) register accessor: Data Output Value\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
#[doc(alias = "OUT")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "Data Output Value"]
pub mod out;
#[doc = "OUTCLR (rw) register accessor: Data Output Value Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`outclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outclr`] module"]
#[doc(alias = "OUTCLR")]
pub type Outclr = crate::Reg<outclr::OutclrSpec>;
#[doc = "Data Output Value Clear"]
pub mod outclr;
#[doc = "OUTSET (rw) register accessor: Data Output Value Set\n\nYou can [`read`](crate::Reg::read) this register and get [`outset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outset`] module"]
#[doc(alias = "OUTSET")]
pub type Outset = crate::Reg<outset::OutsetSpec>;
#[doc = "Data Output Value Set"]
pub mod outset;
#[doc = "OUTTGL (rw) register accessor: Data Output Value Toggle\n\nYou can [`read`](crate::Reg::read) this register and get [`outtgl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outtgl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outtgl`] module"]
#[doc(alias = "OUTTGL")]
pub type Outtgl = crate::Reg<outtgl::OuttglSpec>;
#[doc = "Data Output Value Toggle"]
pub mod outtgl;
#[doc = "IN (r) register accessor: Data Input Value\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
#[doc(alias = "IN")]
pub type In = crate::Reg<in_::InSpec>;
#[doc = "Data Input Value"]
pub mod in_;
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "WRCONFIG (w) register accessor: Write Configuration\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrconfig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrconfig`] module"]
#[doc(alias = "WRCONFIG")]
pub type Wrconfig = crate::Reg<wrconfig::WrconfigSpec>;
#[doc = "Write Configuration"]
pub mod wrconfig;
#[doc = "EVCTRL (rw) register accessor: Event Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event Input Control"]
pub mod evctrl;
#[doc = "PMUX (rw) register accessor: Peripheral Multiplexing\n\nYou can [`read`](crate::Reg::read) this register and get [`pmux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmux`] module"]
#[doc(alias = "PMUX")]
pub type Pmux = crate::Reg<pmux::PmuxSpec>;
#[doc = "Peripheral Multiplexing"]
pub mod pmux;
#[doc = "PINCFG (rw) register accessor: Pin Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pincfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pincfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg`] module"]
#[doc(alias = "PINCFG")]
pub type Pincfg = crate::Reg<pincfg::PincfgSpec>;
#[doc = "Pin Configuration"]
pub mod pincfg;
