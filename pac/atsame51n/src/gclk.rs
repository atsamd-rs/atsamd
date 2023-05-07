#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    _reserved2: [u8; 0x18],
    #[doc = "0x20..0x50 - Generic Clock Generator Control"]
    pub genctrl: [GENCTRL; 12],
    _reserved3: [u8; 0x30],
    #[doc = "0x80..0x140 - Peripheral Clock Control"]
    pub pchctrl: [PCHCTRL; 48],
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "GENCTRL (rw) register accessor: an alias for `Reg<GENCTRL_SPEC>`"]
pub type GENCTRL = crate::Reg<genctrl::GENCTRL_SPEC>;
#[doc = "Generic Clock Generator Control"]
pub mod genctrl;
#[doc = "PCHCTRL (rw) register accessor: an alias for `Reg<PCHCTRL_SPEC>`"]
pub type PCHCTRL = crate::Reg<pchctrl::PCHCTRL_SPEC>;
#[doc = "Peripheral Clock Control"]
pub mod pchctrl;
