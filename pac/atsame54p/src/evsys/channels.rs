#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNELS {
    #[doc = "0x00 - Channel n Control"]
    pub channel: CHANNEL,
    #[doc = "0x04 - Channel n Interrupt Enable Clear"]
    pub chintenclr: CHINTENCLR,
    #[doc = "0x05 - Channel n Interrupt Enable Set"]
    pub chintenset: CHINTENSET,
    #[doc = "0x06 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: CHINTFLAG,
    #[doc = "0x07 - Channel n Status"]
    pub chstatus: CHSTATUS,
}
#[doc = "CHANNEL (rw) register accessor: Channel n Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel`]
module"]
pub type CHANNEL = crate::Reg<channel::CHANNEL_SPEC>;
#[doc = "Channel n Control"]
pub mod channel;
#[doc = "CHINTENCLR (rw) register accessor: Channel n Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chintenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chintenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintenclr`]
module"]
pub type CHINTENCLR = crate::Reg<chintenclr::CHINTENCLR_SPEC>;
#[doc = "Channel n Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "CHINTENSET (rw) register accessor: Channel n Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chintenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chintenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintenset`]
module"]
pub type CHINTENSET = crate::Reg<chintenset::CHINTENSET_SPEC>;
#[doc = "Channel n Interrupt Enable Set"]
pub mod chintenset;
#[doc = "CHINTFLAG (rw) register accessor: Channel n Interrupt Flag Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chintflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chintflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintflag`]
module"]
pub type CHINTFLAG = crate::Reg<chintflag::CHINTFLAG_SPEC>;
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "CHSTATUS (r) register accessor: Channel n Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatus`]
module"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = "Channel n Status"]
pub mod chstatus;
