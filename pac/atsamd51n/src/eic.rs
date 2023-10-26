#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Non-Maskable Interrupt Control"]
    pub nmictrl: NMICTRL,
    #[doc = "0x02 - Non-Maskable Interrupt Flag Status and Clear"]
    pub nmiflag: NMIFLAG,
    #[doc = "0x04 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x08 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x18 - External Interrupt Asynchronous Mode"]
    pub asynch: ASYNCH,
    #[doc = "0x1c..0x24 - External Interrupt Sense Configuration"]
    pub config: [CONFIG; 2],
    _reserved10: [u8; 0x0c],
    #[doc = "0x30 - Debouncer Enable"]
    pub debouncen: DEBOUNCEN,
    #[doc = "0x34 - Debouncer Prescaler"]
    pub dprescaler: DPRESCALER,
    #[doc = "0x38 - Pin State"]
    pub pinstate: PINSTATE,
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "NMICTRL (rw) register accessor: Non-Maskable Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmictrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmictrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmictrl`]
module"]
pub type NMICTRL = crate::Reg<nmictrl::NMICTRL_SPEC>;
#[doc = "Non-Maskable Interrupt Control"]
pub mod nmictrl;
#[doc = "NMIFLAG (rw) register accessor: Non-Maskable Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmiflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmiflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmiflag`]
module"]
pub type NMIFLAG = crate::Reg<nmiflag::NMIFLAG_SPEC>;
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub mod nmiflag;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "ASYNCH (rw) register accessor: External Interrupt Asynchronous Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asynch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asynch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asynch`]
module"]
pub type ASYNCH = crate::Reg<asynch::ASYNCH_SPEC>;
#[doc = "External Interrupt Asynchronous Mode"]
pub mod asynch;
#[doc = "CONFIG (rw) register accessor: External Interrupt Sense Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "External Interrupt Sense Configuration"]
pub mod config;
#[doc = "DEBOUNCEN (rw) register accessor: Debouncer Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debouncen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debouncen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debouncen`]
module"]
pub type DEBOUNCEN = crate::Reg<debouncen::DEBOUNCEN_SPEC>;
#[doc = "Debouncer Enable"]
pub mod debouncen;
#[doc = "DPRESCALER (rw) register accessor: Debouncer Prescaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dprescaler::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dprescaler::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dprescaler`]
module"]
pub type DPRESCALER = crate::Reg<dprescaler::DPRESCALER_SPEC>;
#[doc = "Debouncer Prescaler"]
pub mod dprescaler;
#[doc = "PINSTATE (r) register accessor: Pin State\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinstate::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pinstate`]
module"]
pub type PINSTATE = crate::Reg<pinstate::PINSTATE_SPEC>;
#[doc = "Pin State"]
pub mod pinstate;
