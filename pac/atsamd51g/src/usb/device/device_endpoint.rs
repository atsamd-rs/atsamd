#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE_ENDPOINT {
    #[doc = "0x00 - DEVICE_ENDPOINT End Point Configuration"]
    pub epcfg: EPCFG,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - DEVICE_ENDPOINT End Point Pipe Status Clear"]
    pub epstatusclr: EPSTATUSCLR,
    #[doc = "0x05 - DEVICE_ENDPOINT End Point Pipe Status Set"]
    pub epstatusset: EPSTATUSSET,
    #[doc = "0x06 - DEVICE_ENDPOINT End Point Pipe Status"]
    pub epstatus: EPSTATUS,
    #[doc = "0x07 - DEVICE_ENDPOINT End Point Interrupt Flag"]
    pub epintflag: EPINTFLAG,
    #[doc = "0x08 - DEVICE_ENDPOINT End Point Interrupt Clear Flag"]
    pub epintenclr: EPINTENCLR,
    #[doc = "0x09 - DEVICE_ENDPOINT End Point Interrupt Set Flag"]
    pub epintenset: EPINTENSET,
}
#[doc = "EPCFG (rw) register accessor: DEVICE_ENDPOINT End Point Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epcfg`]
module"]
pub type EPCFG = crate::Reg<epcfg::EPCFG_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Configuration"]
pub mod epcfg;
#[doc = "EPSTATUSCLR (w) register accessor: DEVICE_ENDPOINT End Point Pipe Status Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epstatusclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstatusclr`]
module"]
pub type EPSTATUSCLR = crate::Reg<epstatusclr::EPSTATUSCLR_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Clear"]
pub mod epstatusclr;
#[doc = "EPSTATUSSET (w) register accessor: DEVICE_ENDPOINT End Point Pipe Status Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epstatusset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstatusset`]
module"]
pub type EPSTATUSSET = crate::Reg<epstatusset::EPSTATUSSET_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Set"]
pub mod epstatusset;
#[doc = "EPSTATUS (r) register accessor: DEVICE_ENDPOINT End Point Pipe Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstatus`]
module"]
pub type EPSTATUS = crate::Reg<epstatus::EPSTATUS_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status"]
pub mod epstatus;
#[doc = "EPINTFLAG (rw) register accessor: DEVICE_ENDPOINT End Point Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epintflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epintflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintflag`]
module"]
pub type EPINTFLAG = crate::Reg<epintflag::EPINTFLAG_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Flag"]
pub mod epintflag;
#[doc = "EPINTENCLR (rw) register accessor: DEVICE_ENDPOINT End Point Interrupt Clear Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epintenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epintenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintenclr`]
module"]
pub type EPINTENCLR = crate::Reg<epintenclr::EPINTENCLR_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Clear Flag"]
pub mod epintenclr;
#[doc = "EPINTENSET (rw) register accessor: DEVICE_ENDPOINT End Point Interrupt Set Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epintenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epintenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintenset`]
module"]
pub type EPINTENSET = crate::Reg<epintenset::EPINTENSET_SPEC>;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Set Flag"]
pub mod epintenset;
