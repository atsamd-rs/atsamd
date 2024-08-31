#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    nmictrl: Nmictrl,
    nmiflag: Nmiflag,
    syncbusy: Syncbusy,
    evctrl: Evctrl,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    asynch: Asynch,
    config: [Config; 2],
    _reserved10: [u8; 0x0c],
    debouncen: Debouncen,
    dprescaler: Dprescaler,
    pinstate: Pinstate,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x01 - Non-Maskable Interrupt Control"]
    #[inline(always)]
    pub const fn nmictrl(&self) -> &Nmictrl {
        &self.nmictrl
    }
    #[doc = "0x02 - Non-Maskable Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn nmiflag(&self) -> &Nmiflag {
        &self.nmiflag
    }
    #[doc = "0x04 - Synchronization Busy"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x08 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x0c - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x10 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x18 - External Interrupt Asynchronous Mode"]
    #[inline(always)]
    pub const fn asynch(&self) -> &Asynch {
        &self.asynch
    }
    #[doc = "0x1c..0x24 - External Interrupt Sense Configuration"]
    #[inline(always)]
    pub const fn config(&self, n: usize) -> &Config {
        &self.config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x24 - External Interrupt Sense Configuration"]
    #[inline(always)]
    pub fn config_iter(&self) -> impl Iterator<Item = &Config> {
        self.config.iter()
    }
    #[doc = "0x30 - Debouncer Enable"]
    #[inline(always)]
    pub const fn debouncen(&self) -> &Debouncen {
        &self.debouncen
    }
    #[doc = "0x34 - Debouncer Prescaler"]
    #[inline(always)]
    pub const fn dprescaler(&self) -> &Dprescaler {
        &self.dprescaler
    }
    #[doc = "0x38 - Pin State"]
    #[inline(always)]
    pub const fn pinstate(&self) -> &Pinstate {
        &self.pinstate
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
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
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
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
#[doc = "ASYNCH (rw) register accessor: External Interrupt Asynchronous Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`asynch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asynch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asynch`]
module"]
#[doc(alias = "ASYNCH")]
pub type Asynch = crate::Reg<asynch::AsynchSpec>;
#[doc = "External Interrupt Asynchronous Mode"]
pub mod asynch;
#[doc = "CONFIG (rw) register accessor: External Interrupt Sense Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "External Interrupt Sense Configuration"]
pub mod config;
#[doc = "DEBOUNCEN (rw) register accessor: Debouncer Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`debouncen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debouncen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debouncen`]
module"]
#[doc(alias = "DEBOUNCEN")]
pub type Debouncen = crate::Reg<debouncen::DebouncenSpec>;
#[doc = "Debouncer Enable"]
pub mod debouncen;
#[doc = "DPRESCALER (rw) register accessor: Debouncer Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`dprescaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dprescaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dprescaler`]
module"]
#[doc(alias = "DPRESCALER")]
pub type Dprescaler = crate::Reg<dprescaler::DprescalerSpec>;
#[doc = "Debouncer Prescaler"]
pub mod dprescaler;
#[doc = "PINSTATE (r) register accessor: Pin State\n\nYou can [`read`](crate::Reg::read) this register and get [`pinstate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinstate`]
module"]
#[doc(alias = "PINSTATE")]
pub type Pinstate = crate::Reg<pinstate::PinstateSpec>;
#[doc = "Pin State"]
pub mod pinstate;
