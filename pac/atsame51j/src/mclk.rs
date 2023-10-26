#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x02 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x03 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x04 - HS Clock Division"]
    pub hsdiv: HSDIV,
    #[doc = "0x05 - CPU Clock Division"]
    pub cpudiv: CPUDIV,
    _reserved5: [u8; 0x0a],
    #[doc = "0x10 - AHB Mask"]
    pub ahbmask: AHBMASK,
    #[doc = "0x14 - APBA Mask"]
    pub apbamask: APBAMASK,
    #[doc = "0x18 - APBB Mask"]
    pub apbbmask: APBBMASK,
    #[doc = "0x1c - APBC Mask"]
    pub apbcmask: APBCMASK,
    #[doc = "0x20 - APBD Mask"]
    pub apbdmask: APBDMASK,
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
#[doc = "HSDIV (r) register accessor: HS Clock Division\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsdiv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsdiv`]
module"]
pub type HSDIV = crate::Reg<hsdiv::HSDIV_SPEC>;
#[doc = "HS Clock Division"]
pub mod hsdiv;
#[doc = "CPUDIV (rw) register accessor: CPU Clock Division\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpudiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpudiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpudiv`]
module"]
pub type CPUDIV = crate::Reg<cpudiv::CPUDIV_SPEC>;
#[doc = "CPU Clock Division"]
pub mod cpudiv;
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
#[doc = "APBDMASK (rw) register accessor: APBD Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbdmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbdmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbdmask`]
module"]
pub type APBDMASK = crate::Reg<apbdmask::APBDMASK_SPEC>;
#[doc = "APBD Mask"]
pub mod apbdmask;
