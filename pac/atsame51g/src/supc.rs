#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0c - Power and Clocks Status"]
    pub status: STATUS,
    #[doc = "0x10 - BOD33 Control"]
    pub bod33: BOD33,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - VREG Control"]
    pub vreg: VREG,
    #[doc = "0x1c - VREF Control"]
    pub vref: VREF,
    #[doc = "0x20 - Battery Backup Power Switch"]
    pub bbps: BBPS,
    #[doc = "0x24 - Backup Output Control"]
    pub bkout: BKOUT,
    #[doc = "0x28 - Backup Input Control"]
    pub bkin: BKIN,
}
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
#[doc = "STATUS (r) register accessor: Power and Clocks Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Power and Clocks Status"]
pub mod status;
#[doc = "BOD33 (rw) register accessor: BOD33 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bod33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bod33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bod33`]
module"]
pub type BOD33 = crate::Reg<bod33::BOD33_SPEC>;
#[doc = "BOD33 Control"]
pub mod bod33;
#[doc = "VREG (rw) register accessor: VREG Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vreg`]
module"]
pub type VREG = crate::Reg<vreg::VREG_SPEC>;
#[doc = "VREG Control"]
pub mod vreg;
#[doc = "VREF (rw) register accessor: VREF Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref`]
module"]
pub type VREF = crate::Reg<vref::VREF_SPEC>;
#[doc = "VREF Control"]
pub mod vref;
#[doc = "BBPS (rw) register accessor: Battery Backup Power Switch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bbps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bbps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bbps`]
module"]
pub type BBPS = crate::Reg<bbps::BBPS_SPEC>;
#[doc = "Battery Backup Power Switch"]
pub mod bbps;
#[doc = "BKOUT (rw) register accessor: Backup Output Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkout`]
module"]
pub type BKOUT = crate::Reg<bkout::BKOUT_SPEC>;
#[doc = "Backup Output Control"]
pub mod bkout;
#[doc = "BKIN (r) register accessor: Backup Input Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkin`]
module"]
pub type BKIN = crate::Reg<bkin::BKIN_SPEC>;
#[doc = "Backup Input Control"]
pub mod bkin;
