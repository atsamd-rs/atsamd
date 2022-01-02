#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Synchronization Busy"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
    _reserved2: [u8; 0x18],
    #[doc = "0x20..0x50 - Generic Clock Generator Control"]
    pub genctrl: [crate::Reg<genctrl::GENCTRL_SPEC>; 12],
    _reserved3: [u8; 0x30],
    #[doc = "0x80..0x140 - Peripheral Clock Control"]
    pub pchctrl: [crate::Reg<pchctrl::PCHCTRL_SPEC>; 48],
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "GENCTRL register accessor: an alias for `Reg<GENCTRL_SPEC>`"]
pub type GENCTRL = crate::Reg<genctrl::GENCTRL_SPEC>;
#[doc = "Generic Clock Generator Control"]
pub mod genctrl;
#[doc = "PCHCTRL register accessor: an alias for `Reg<PCHCTRL_SPEC>`"]
pub type PCHCTRL = crate::Reg<pchctrl::PCHCTRL_SPEC>;
#[doc = "Peripheral Clock Control"]
pub mod pchctrl;
