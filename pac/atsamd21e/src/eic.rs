#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    nmictrl: Nmictrl,
    nmiflag: Nmiflag,
    evctrl: Evctrl,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    wakeup: Wakeup,
    config: [Config; 2],
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
    #[doc = "0x02 - Non-Maskable Interrupt Control"]
    #[inline(always)]
    pub const fn nmictrl(&self) -> &Nmictrl {
        &self.nmictrl
    }
    #[doc = "0x03 - Non-Maskable Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn nmiflag(&self) -> &Nmiflag {
        &self.nmiflag
    }
    #[doc = "0x04 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x08 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x0c - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x10 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x14 - Wake-Up Enable"]
    #[inline(always)]
    pub const fn wakeup(&self) -> &Wakeup {
        &self.wakeup
    }
    #[doc = "0x18..0x20 - Configuration n"]
    #[inline(always)]
    pub const fn config(&self, n: usize) -> &Config {
        &self.config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x20 - Configuration n"]
    #[inline(always)]
    pub fn config_iter(&self) -> impl Iterator<Item = &Config> {
        self.config.iter()
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
#[doc = "NMICTRL (rw) register accessor: Non-Maskable Interrupt Control\n\nYou can [`read`](crate::Reg::read) this register and get [`nmictrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmictrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmictrl`]
module"]
#[doc(alias = "NMICTRL")]
pub type Nmictrl = crate::Reg<nmictrl::NmictrlSpec>;
#[doc = "Non-Maskable Interrupt Control"]
pub mod nmictrl;
#[doc = "NMIFLAG (rw) register accessor: Non-Maskable Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiflag`]
module"]
#[doc(alias = "NMIFLAG")]
pub type Nmiflag = crate::Reg<nmiflag::NmiflagSpec>;
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub mod nmiflag;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "WAKEUP (rw) register accessor: Wake-Up Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup`]
module"]
#[doc(alias = "WAKEUP")]
pub type Wakeup = crate::Reg<wakeup::WakeupSpec>;
#[doc = "Wake-Up Enable"]
pub mod wakeup;
#[doc = "CONFIG (rw) register accessor: Configuration n\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration n"]
pub mod config;
