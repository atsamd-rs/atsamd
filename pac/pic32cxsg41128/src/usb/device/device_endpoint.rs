#[repr(C)]
#[doc = "DEVICE_ENDPOINT\\[%s\\]"]
#[doc(alias = "DEVICE_ENDPOINT")]
pub struct DeviceEndpoint {
    epcfg: Epcfg,
    _reserved1: [u8; 0x03],
    epstatusclr: Epstatusclr,
    epstatusset: Epstatusset,
    epstatus: Epstatus,
    epintflag: Epintflag,
    epintenclr: Epintenclr,
    epintenset: Epintenset,
}
impl DeviceEndpoint {
    #[doc = "0x00 - DEVICE_ENDPOINT End Point Configuration"]
    #[inline(always)]
    pub const fn epcfg(&self) -> &Epcfg {
        &self.epcfg
    }
    #[doc = "0x04 - DEVICE_ENDPOINT End Point Pipe Status Clear"]
    #[inline(always)]
    pub const fn epstatusclr(&self) -> &Epstatusclr {
        &self.epstatusclr
    }
    #[doc = "0x05 - DEVICE_ENDPOINT End Point Pipe Status Set"]
    #[inline(always)]
    pub const fn epstatusset(&self) -> &Epstatusset {
        &self.epstatusset
    }
    #[doc = "0x06 - DEVICE_ENDPOINT End Point Pipe Status"]
    #[inline(always)]
    pub const fn epstatus(&self) -> &Epstatus {
        &self.epstatus
    }
    #[doc = "0x07 - DEVICE_ENDPOINT End Point Interrupt Flag"]
    #[inline(always)]
    pub const fn epintflag(&self) -> &Epintflag {
        &self.epintflag
    }
    #[doc = "0x08 - DEVICE_ENDPOINT End Point Interrupt Clear Flag"]
    #[inline(always)]
    pub const fn epintenclr(&self) -> &Epintenclr {
        &self.epintenclr
    }
    #[doc = "0x09 - DEVICE_ENDPOINT End Point Interrupt Set Flag"]
    #[inline(always)]
    pub const fn epintenset(&self) -> &Epintenset {
        &self.epintenset
    }
}
#[doc = "EPCFG (rw) register accessor: DEVICE_ENDPOINT End Point Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`epcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epcfg`]
module"]
#[doc(alias = "EPCFG")]
pub type Epcfg = crate::Reg<epcfg::EpcfgSpec>;
#[doc = "DEVICE_ENDPOINT End Point Configuration"]
pub mod epcfg;
#[doc = "EPSTATUSCLR (w) register accessor: DEVICE_ENDPOINT End Point Pipe Status Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epstatusclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstatusclr`]
module"]
#[doc(alias = "EPSTATUSCLR")]
pub type Epstatusclr = crate::Reg<epstatusclr::EpstatusclrSpec>;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Clear"]
pub mod epstatusclr;
#[doc = "EPSTATUSSET (w) register accessor: DEVICE_ENDPOINT End Point Pipe Status Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epstatusset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstatusset`]
module"]
#[doc(alias = "EPSTATUSSET")]
pub type Epstatusset = crate::Reg<epstatusset::EpstatussetSpec>;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status Set"]
pub mod epstatusset;
#[doc = "EPSTATUS (r) register accessor: DEVICE_ENDPOINT End Point Pipe Status\n\nYou can [`read`](crate::Reg::read) this register and get [`epstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstatus`]
module"]
#[doc(alias = "EPSTATUS")]
pub type Epstatus = crate::Reg<epstatus::EpstatusSpec>;
#[doc = "DEVICE_ENDPOINT End Point Pipe Status"]
pub mod epstatus;
#[doc = "EPINTFLAG (rw) register accessor: DEVICE_ENDPOINT End Point Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`epintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintflag`]
module"]
#[doc(alias = "EPINTFLAG")]
pub type Epintflag = crate::Reg<epintflag::EpintflagSpec>;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Flag"]
pub mod epintflag;
#[doc = "EPINTENCLR (rw) register accessor: DEVICE_ENDPOINT End Point Interrupt Clear Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`epintenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epintenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintenclr`]
module"]
#[doc(alias = "EPINTENCLR")]
pub type Epintenclr = crate::Reg<epintenclr::EpintenclrSpec>;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Clear Flag"]
pub mod epintenclr;
#[doc = "EPINTENSET (rw) register accessor: DEVICE_ENDPOINT End Point Interrupt Set Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`epintenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epintenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintenset`]
module"]
#[doc(alias = "EPINTENSET")]
pub type Epintenset = crate::Reg<epintenset::EpintensetSpec>;
#[doc = "DEVICE_ENDPOINT End Point Interrupt Set Flag"]
pub mod epintenset;
