#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Channel"]
    pub channel: CHANNEL,
    #[doc = "0x08 - User Multiplexer"]
    pub user: USER,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - Channel Status"]
    pub chstatus: CHSTATUS,
    #[doc = "0x10 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x14 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x18 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
}
#[doc = "CTRL (w) register accessor: Control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "CHANNEL (rw) register accessor: Channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel`]
module"]
pub type CHANNEL = crate::Reg<channel::CHANNEL_SPEC>;
#[doc = "Channel"]
pub mod channel;
#[doc = "USER (rw) register accessor: User Multiplexer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`]
module"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "User Multiplexer"]
pub mod user;
#[doc = "CHSTATUS (r) register accessor: Channel Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatus`]
module"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = "Channel Status"]
pub mod chstatus;
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
