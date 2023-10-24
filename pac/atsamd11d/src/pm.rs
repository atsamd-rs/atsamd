#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Sleep Mode"]
    pub sleep: SLEEP,
    #[doc = "0x02 - External Reset Controller"]
    pub extctrl: EXTCTRL,
    _reserved3: [u8; 0x05],
    #[doc = "0x08 - CPU Clock Select"]
    pub cpusel: CPUSEL,
    #[doc = "0x09 - APBA Clock Select"]
    pub apbasel: APBASEL,
    #[doc = "0x0a - APBB Clock Select"]
    pub apbbsel: APBBSEL,
    #[doc = "0x0b - APBC Clock Select"]
    pub apbcsel: APBCSEL,
    _reserved7: [u8; 0x08],
    #[doc = "0x14 - AHB Mask"]
    pub ahbmask: AHBMASK,
    #[doc = "0x18 - APBA Mask"]
    pub apbamask: APBAMASK,
    #[doc = "0x1c - APBB Mask"]
    pub apbbmask: APBBMASK,
    #[doc = "0x20 - APBC Mask"]
    pub apbcmask: APBCMASK,
    _reserved11: [u8; 0x10],
    #[doc = "0x34 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x35 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x36 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved14: [u8; 0x01],
    #[doc = "0x38 - Reset Cause"]
    pub rcause: RCAUSE,
}
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "SLEEP (rw) register accessor: Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleep::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleep::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleep`]
module"]
pub type SLEEP = crate::Reg<sleep::SLEEP_SPEC>;
#[doc = "Sleep Mode"]
pub mod sleep;
#[doc = "EXTCTRL (rw) register accessor: External Reset Controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extctrl`]
module"]
pub type EXTCTRL = crate::Reg<extctrl::EXTCTRL_SPEC>;
#[doc = "External Reset Controller"]
pub mod extctrl;
#[doc = "CPUSEL (rw) register accessor: CPU Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpusel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpusel`]
module"]
pub type CPUSEL = crate::Reg<cpusel::CPUSEL_SPEC>;
#[doc = "CPU Clock Select"]
pub mod cpusel;
#[doc = "APBASEL (rw) register accessor: APBA Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbasel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbasel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbasel`]
module"]
pub type APBASEL = crate::Reg<apbasel::APBASEL_SPEC>;
#[doc = "APBA Clock Select"]
pub mod apbasel;
#[doc = "APBBSEL (rw) register accessor: APBB Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbbsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbbsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbbsel`]
module"]
pub type APBBSEL = crate::Reg<apbbsel::APBBSEL_SPEC>;
#[doc = "APBB Clock Select"]
pub mod apbbsel;
#[doc = "APBCSEL (rw) register accessor: APBC Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbcsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbcsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbcsel`]
module"]
pub type APBCSEL = crate::Reg<apbcsel::APBCSEL_SPEC>;
#[doc = "APBC Clock Select"]
pub mod apbcsel;
#[doc = "AHBMASK (rw) register accessor: AHB Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbmask`]
module"]
pub type AHBMASK = crate::Reg<ahbmask::AHBMASK_SPEC>;
#[doc = "AHB Mask"]
pub mod ahbmask;
#[doc = "APBAMASK (rw) register accessor: APBA Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbamask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbamask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbamask`]
module"]
pub type APBAMASK = crate::Reg<apbamask::APBAMASK_SPEC>;
#[doc = "APBA Mask"]
pub mod apbamask;
#[doc = "APBBMASK (rw) register accessor: APBB Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbbmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbbmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbbmask`]
module"]
pub type APBBMASK = crate::Reg<apbbmask::APBBMASK_SPEC>;
#[doc = "APBB Mask"]
pub mod apbbmask;
#[doc = "APBCMASK (rw) register accessor: APBC Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbcmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbcmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbcmask`]
module"]
pub type APBCMASK = crate::Reg<apbcmask::APBCMASK_SPEC>;
#[doc = "APBC Mask"]
pub mod apbcmask;
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
#[doc = "RCAUSE (r) register accessor: Reset Cause\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcause`]
module"]
pub type RCAUSE = crate::Reg<rcause::RCAUSE_SPEC>;
#[doc = "Reset Cause"]
pub mod rcause;
