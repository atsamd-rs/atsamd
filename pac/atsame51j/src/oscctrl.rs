#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x08 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    #[doc = "0x14..0x1c - External Multipurpose Crystal Oscillator Control"]
    pub xoscctrl: [XOSCCTRL; 2],
    #[doc = "0x1c - DFLL48M Control A"]
    pub dfllctrla: DFLLCTRLA,
    _reserved7: [u8; 0x03],
    #[doc = "0x20 - DFLL48M Control B"]
    pub dfllctrlb: DFLLCTRLB,
    _reserved8: [u8; 0x03],
    #[doc = "0x24 - DFLL48M Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x28 - DFLL48M Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x2c - DFLL48M Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved11: [u8; 0x03],
    #[doc = "0x30..0x58 - DPLL\\[%s\\]"]
    pub dpll: [DPLL; 2],
}
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
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
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "XOSCCTRL (rw) register accessor: External Multipurpose Crystal Oscillator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xoscctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xoscctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xoscctrl`]
module"]
pub type XOSCCTRL = crate::Reg<xoscctrl::XOSCCTRL_SPEC>;
#[doc = "External Multipurpose Crystal Oscillator Control"]
pub mod xoscctrl;
#[doc = "DFLLCTRLA (rw) register accessor: DFLL48M Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfllctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfllctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllctrla`]
module"]
pub type DFLLCTRLA = crate::Reg<dfllctrla::DFLLCTRLA_SPEC>;
#[doc = "DFLL48M Control A"]
pub mod dfllctrla;
#[doc = "DFLLCTRLB (rw) register accessor: DFLL48M Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfllctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfllctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllctrlb`]
module"]
pub type DFLLCTRLB = crate::Reg<dfllctrlb::DFLLCTRLB_SPEC>;
#[doc = "DFLL48M Control B"]
pub mod dfllctrlb;
#[doc = "DFLLVAL (rw) register accessor: DFLL48M Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfllval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfllval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllval`]
module"]
pub type DFLLVAL = crate::Reg<dfllval::DFLLVAL_SPEC>;
#[doc = "DFLL48M Value"]
pub mod dfllval;
#[doc = "DFLLMUL (rw) register accessor: DFLL48M Multiplier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfllmul::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfllmul::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllmul`]
module"]
pub type DFLLMUL = crate::Reg<dfllmul::DFLLMUL_SPEC>;
#[doc = "DFLL48M Multiplier"]
pub mod dfllmul;
#[doc = "DFLLSYNC (rw) register accessor: DFLL48M Synchronization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfllsync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfllsync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllsync`]
module"]
pub type DFLLSYNC = crate::Reg<dfllsync::DFLLSYNC_SPEC>;
#[doc = "DFLL48M Synchronization"]
pub mod dfllsync;
#[doc = "DPLL\\[%s\\]"]
pub use self::dpll::DPLL;
#[doc = r"Cluster"]
#[doc = "DPLL\\[%s\\]"]
pub mod dpll;
