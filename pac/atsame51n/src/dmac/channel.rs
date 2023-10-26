#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control A"]
    pub chctrla: CHCTRLA,
    #[doc = "0x04 - Channel n Control B"]
    pub chctrlb: CHCTRLB,
    #[doc = "0x05 - Channel n Priority Level"]
    pub chprilvl: CHPRILVL,
    #[doc = "0x06 - Channel n Event Control"]
    pub chevctrl: CHEVCTRL,
    _reserved4: [u8; 0x05],
    #[doc = "0x0c - Channel n Interrupt Enable Clear"]
    pub chintenclr: CHINTENCLR,
    #[doc = "0x0d - Channel n Interrupt Enable Set"]
    pub chintenset: CHINTENSET,
    #[doc = "0x0e - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: CHINTFLAG,
    #[doc = "0x0f - Channel n Status"]
    pub chstatus: CHSTATUS,
}
#[doc = "CHCTRLA (rw) register accessor: Channel n Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctrla`]
module"]
pub type CHCTRLA = crate::Reg<chctrla::CHCTRLA_SPEC>;
#[doc = "Channel n Control A"]
pub mod chctrla;
#[doc = "CHCTRLB (rw) register accessor: Channel n Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctrlb`]
module"]
pub type CHCTRLB = crate::Reg<chctrlb::CHCTRLB_SPEC>;
#[doc = "Channel n Control B"]
pub mod chctrlb;
#[doc = "CHPRILVL (rw) register accessor: Channel n Priority Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chprilvl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chprilvl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chprilvl`]
module"]
pub type CHPRILVL = crate::Reg<chprilvl::CHPRILVL_SPEC>;
#[doc = "Channel n Priority Level"]
pub mod chprilvl;
#[doc = "CHEVCTRL (rw) register accessor: Channel n Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chevctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chevctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chevctrl`]
module"]
pub type CHEVCTRL = crate::Reg<chevctrl::CHEVCTRL_SPEC>;
#[doc = "Channel n Event Control"]
pub mod chevctrl;
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
#[doc = "CHSTATUS (rw) register accessor: Channel n Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatus`]
module"]
pub type CHSTATUS = crate::Reg<chstatus::CHSTATUS_SPEC>;
#[doc = "Channel n Status"]
pub mod chstatus;
