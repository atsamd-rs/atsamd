#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: SLEEPCFG,
    _reserved2: [u8; 0x02],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved5: [u8; 0x01],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: STDBYCFG,
    #[doc = "0x09 - Hibernate Configuration"]
    pub hibcfg: HIBCFG,
    #[doc = "0x0a - Backup Configuration"]
    pub bkupcfg: BKUPCFG,
    _reserved8: [u8; 0x07],
    #[doc = "0x12 - Power Switch Acknowledge Delay"]
    pub pwsakdly: PWSAKDLY,
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "SLEEPCFG (rw) register accessor: Sleep Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepcfg`]
module"]
pub type SLEEPCFG = crate::Reg<sleepcfg::SLEEPCFG_SPEC>;
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
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
#[doc = "STDBYCFG (rw) register accessor: Standby Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stdbycfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stdbycfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stdbycfg`]
module"]
pub type STDBYCFG = crate::Reg<stdbycfg::STDBYCFG_SPEC>;
#[doc = "Standby Configuration"]
pub mod stdbycfg;
#[doc = "HIBCFG (rw) register accessor: Hibernate Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hibcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hibcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hibcfg`]
module"]
pub type HIBCFG = crate::Reg<hibcfg::HIBCFG_SPEC>;
#[doc = "Hibernate Configuration"]
pub mod hibcfg;
#[doc = "BKUPCFG (rw) register accessor: Backup Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkupcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkupcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkupcfg`]
module"]
pub type BKUPCFG = crate::Reg<bkupcfg::BKUPCFG_SPEC>;
#[doc = "Backup Configuration"]
pub mod bkupcfg;
#[doc = "PWSAKDLY (rw) register accessor: Power Switch Acknowledge Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwsakdly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwsakdly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwsakdly`]
module"]
pub type PWSAKDLY = crate::Reg<pwsakdly::PWSAKDLY_SPEC>;
#[doc = "Power Switch Acknowledge Delay"]
pub mod pwsakdly;
