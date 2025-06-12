#[repr(C)]
#[doc = "HOST_PIPE\\[%s\\]"]
#[doc(alias = "HOST_PIPE")]
pub struct HostPipe {
    pcfg: Pcfg,
    _reserved1: [u8; 0x02],
    binterval: Binterval,
    pstatusclr: Pstatusclr,
    pstatusset: Pstatusset,
    pstatus: Pstatus,
    pintflag: Pintflag,
    pintenclr: Pintenclr,
    pintenset: Pintenset,
}
impl HostPipe {
    #[doc = "0x00 - HOST_PIPE End Point Configuration"]
    #[inline(always)]
    pub const fn pcfg(&self) -> &Pcfg {
        &self.pcfg
    }
    #[doc = "0x03 - HOST_PIPE Bus Access Period of Pipe"]
    #[inline(always)]
    pub const fn binterval(&self) -> &Binterval {
        &self.binterval
    }
    #[doc = "0x04 - HOST_PIPE End Point Pipe Status Clear"]
    #[inline(always)]
    pub const fn pstatusclr(&self) -> &Pstatusclr {
        &self.pstatusclr
    }
    #[doc = "0x05 - HOST_PIPE End Point Pipe Status Set"]
    #[inline(always)]
    pub const fn pstatusset(&self) -> &Pstatusset {
        &self.pstatusset
    }
    #[doc = "0x06 - HOST_PIPE End Point Pipe Status"]
    #[inline(always)]
    pub const fn pstatus(&self) -> &Pstatus {
        &self.pstatus
    }
    #[doc = "0x07 - HOST_PIPE Pipe Interrupt Flag"]
    #[inline(always)]
    pub const fn pintflag(&self) -> &Pintflag {
        &self.pintflag
    }
    #[doc = "0x08 - HOST_PIPE Pipe Interrupt Flag Clear"]
    #[inline(always)]
    pub const fn pintenclr(&self) -> &Pintenclr {
        &self.pintenclr
    }
    #[doc = "0x09 - HOST_PIPE Pipe Interrupt Flag Set"]
    #[inline(always)]
    pub const fn pintenset(&self) -> &Pintenset {
        &self.pintenset
    }
}
#[doc = "PCFG (rw) register accessor: HOST_PIPE End Point Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcfg`] module"]
#[doc(alias = "PCFG")]
pub type Pcfg = crate::Reg<pcfg::PcfgSpec>;
#[doc = "HOST_PIPE End Point Configuration"]
pub mod pcfg;
#[doc = "BINTERVAL (rw) register accessor: HOST_PIPE Bus Access Period of Pipe\n\nYou can [`read`](crate::Reg::read) this register and get [`binterval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`binterval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@binterval`] module"]
#[doc(alias = "BINTERVAL")]
pub type Binterval = crate::Reg<binterval::BintervalSpec>;
#[doc = "HOST_PIPE Bus Access Period of Pipe"]
pub mod binterval;
#[doc = "PSTATUSCLR (w) register accessor: HOST_PIPE End Point Pipe Status Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pstatusclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstatusclr`] module"]
#[doc(alias = "PSTATUSCLR")]
pub type Pstatusclr = crate::Reg<pstatusclr::PstatusclrSpec>;
#[doc = "HOST_PIPE End Point Pipe Status Clear"]
pub mod pstatusclr;
#[doc = "PSTATUSSET (w) register accessor: HOST_PIPE End Point Pipe Status Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pstatusset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstatusset`] module"]
#[doc(alias = "PSTATUSSET")]
pub type Pstatusset = crate::Reg<pstatusset::PstatussetSpec>;
#[doc = "HOST_PIPE End Point Pipe Status Set"]
pub mod pstatusset;
#[doc = "PSTATUS (r) register accessor: HOST_PIPE End Point Pipe Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstatus`] module"]
#[doc(alias = "PSTATUS")]
pub type Pstatus = crate::Reg<pstatus::PstatusSpec>;
#[doc = "HOST_PIPE End Point Pipe Status"]
pub mod pstatus;
#[doc = "PINTFLAG (rw) register accessor: HOST_PIPE Pipe Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintflag`] module"]
#[doc(alias = "PINTFLAG")]
pub type Pintflag = crate::Reg<pintflag::PintflagSpec>;
#[doc = "HOST_PIPE Pipe Interrupt Flag"]
pub mod pintflag;
#[doc = "PINTENCLR (rw) register accessor: HOST_PIPE Pipe Interrupt Flag Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`pintenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintenclr`] module"]
#[doc(alias = "PINTENCLR")]
pub type Pintenclr = crate::Reg<pintenclr::PintenclrSpec>;
#[doc = "HOST_PIPE Pipe Interrupt Flag Clear"]
pub mod pintenclr;
#[doc = "PINTENSET (rw) register accessor: HOST_PIPE Pipe Interrupt Flag Set\n\nYou can [`read`](crate::Reg::read) this register and get [`pintenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintenset`] module"]
#[doc(alias = "PINTENSET")]
pub type Pintenset = crate::Reg<pintenset::PintensetSpec>;
#[doc = "HOST_PIPE Pipe Interrupt Flag Set"]
pub mod pintenset;
