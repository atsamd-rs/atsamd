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
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "NMICTRL (rw) register accessor: an alias for `Reg<NMICTRL_SPEC>`"]
pub type NMICTRL = crate::Reg<nmictrl::NMICTRL_SPEC>;
#[doc = "Non-Maskable Interrupt Control"]
pub mod nmictrl;
#[doc = "NMIFLAG (rw) register accessor: an alias for `Reg<NMIFLAG_SPEC>`"]
pub type NMIFLAG = crate::Reg<nmiflag::NMIFLAG_SPEC>;
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub mod nmiflag;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "ASYNCH (rw) register accessor: an alias for `Reg<ASYNCH_SPEC>`"]
pub type ASYNCH = crate::Reg<asynch::ASYNCH_SPEC>;
#[doc = "External Interrupt Asynchronous Mode"]
pub mod asynch;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "External Interrupt Sense Configuration"]
pub mod config;
#[doc = "DEBOUNCEN (rw) register accessor: an alias for `Reg<DEBOUNCEN_SPEC>`"]
pub type DEBOUNCEN = crate::Reg<debouncen::DEBOUNCEN_SPEC>;
#[doc = "Debouncer Enable"]
pub mod debouncen;
#[doc = "DPRESCALER (rw) register accessor: an alias for `Reg<DPRESCALER_SPEC>`"]
pub type DPRESCALER = crate::Reg<dprescaler::DPRESCALER_SPEC>;
#[doc = "Debouncer Prescaler"]
pub mod dprescaler;
#[doc = "PINSTATE (r) register accessor: an alias for `Reg<PINSTATE_SPEC>`"]
pub type PINSTATE = crate::Reg<pinstate::PINSTATE_SPEC>;
#[doc = "Pin State"]
pub mod pinstate;
