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
#[doc = "CTRLA (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "GENCTRL (rw) register accessor: Generic Clock Generator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`genctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`genctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@genctrl`]
module"]
pub type GENCTRL = crate::Reg<genctrl::GENCTRL_SPEC>;
#[doc = "Generic Clock Generator Control"]
pub mod genctrl;
#[doc = "PCHCTRL (rw) register accessor: Peripheral Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pchctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pchctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pchctrl`]
module"]
pub type PCHCTRL = crate::Reg<pchctrl::PCHCTRL_SPEC>;
#[doc = "Peripheral Clock Control"]
pub mod pchctrl;
