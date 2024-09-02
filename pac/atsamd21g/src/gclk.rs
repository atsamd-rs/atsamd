#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    clkctrl: Clkctrl,
    genctrl: Genctrl,
    gendiv: Gendiv,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x01 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x02 - Generic Clock Control"]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x04 - Generic Clock Generator Control"]
    #[inline(always)]
    pub const fn genctrl(&self) -> &Genctrl {
        &self.genctrl
    }
    #[doc = "0x08 - Generic Clock Generator Division"]
    #[inline(always)]
    pub const fn gendiv(&self) -> &Gendiv {
        &self.gendiv
    }
}
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "CLKCTRL (rw) register accessor: Generic Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Generic Clock Control"]
pub mod clkctrl;
#[doc = "GENCTRL (rw) register accessor: Generic Clock Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`genctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@genctrl`]
module"]
#[doc(alias = "GENCTRL")]
pub type Genctrl = crate::Reg<genctrl::GenctrlSpec>;
#[doc = "Generic Clock Generator Control"]
pub mod genctrl;
#[doc = "GENDIV (rw) register accessor: Generic Clock Generator Division\n\nYou can [`read`](crate::Reg::read) this register and get [`gendiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gendiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gendiv`]
module"]
#[doc(alias = "GENDIV")]
pub type Gendiv = crate::Reg<gendiv::GendivSpec>;
#[doc = "Generic Clock Generator Division"]
pub mod gendiv;
