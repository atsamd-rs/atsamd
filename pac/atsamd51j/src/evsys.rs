#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Software Event"]
    pub swevt: SWEVT,
    #[doc = "0x08 - Priority Control"]
    pub prictrl: PRICTRL,
    _reserved3: [u8; 0x07],
    #[doc = "0x10 - Channel Pending Interrupt"]
    pub intpend: INTPEND,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x18 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x1c - Ready Users"]
    pub readyusr: READYUSR,
    #[doc = "0x20..0x120 - CHANNELS\\[%s\\]"]
    pub channels: [CHANNELS; 32],
    #[doc = "0x120..0x22c - User Multiplexer n"]
    pub user: [USER; 67],
}
#[doc = "CTRLA (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "SWEVT (w) register accessor: Software Event\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swevt`]
module"]
pub type SWEVT = crate::Reg<swevt::SWEVT_SPEC>;
#[doc = "Software Event"]
pub mod swevt;
#[doc = "PRICTRL (rw) register accessor: Priority Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prictrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prictrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prictrl`]
module"]
pub type PRICTRL = crate::Reg<prictrl::PRICTRL_SPEC>;
#[doc = "Priority Control"]
pub mod prictrl;
#[doc = "INTPEND (rw) register accessor: Channel Pending Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend`]
module"]
pub type INTPEND = crate::Reg<intpend::INTPEND_SPEC>;
#[doc = "Channel Pending Interrupt"]
pub mod intpend;
#[doc = "INTSTATUS (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "BUSYCH (r) register accessor: Busy Channels\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busych::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busych`]
module"]
pub type BUSYCH = crate::Reg<busych::BUSYCH_SPEC>;
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "READYUSR (r) register accessor: Ready Users\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readyusr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readyusr`]
module"]
pub type READYUSR = crate::Reg<readyusr::READYUSR_SPEC>;
#[doc = "Ready Users"]
pub mod readyusr;
#[doc = "CHANNELS\\[%s\\]"]
pub use self::channels::CHANNELS;
#[doc = r"Cluster"]
#[doc = "CHANNELS\\[%s\\]"]
pub mod channels;
#[doc = "USER (rw) register accessor: User Multiplexer n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`]
module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "User Multiplexer n"]
pub mod user;
