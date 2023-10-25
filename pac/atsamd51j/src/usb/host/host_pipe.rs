#[doc = r"Register block"]
#[repr(C)]
pub struct HOST_PIPE {
    #[doc = "0x00 - HOST_PIPE End Point Configuration"]
    pub pcfg: PCFG,
    _reserved1: [u8; 0x02],
    #[doc = "0x03 - HOST_PIPE Bus Access Period of Pipe"]
    pub binterval: BINTERVAL,
    #[doc = "0x04 - HOST_PIPE End Point Pipe Status Clear"]
    pub pstatusclr: PSTATUSCLR,
    #[doc = "0x05 - HOST_PIPE End Point Pipe Status Set"]
    pub pstatusset: PSTATUSSET,
    #[doc = "0x06 - HOST_PIPE End Point Pipe Status"]
    pub pstatus: PSTATUS,
    #[doc = "0x07 - HOST_PIPE Pipe Interrupt Flag"]
    pub pintflag: PINTFLAG,
    #[doc = "0x08 - HOST_PIPE Pipe Interrupt Flag Clear"]
    pub pintenclr: PINTENCLR,
    #[doc = "0x09 - HOST_PIPE Pipe Interrupt Flag Set"]
    pub pintenset: PINTENSET,
}
#[doc = "PCFG (rw) register accessor: HOST_PIPE End Point Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcfg`]
module"]
pub type PCFG = crate::Reg<pcfg::PCFG_SPEC>;
#[doc = "HOST_PIPE End Point Configuration"]
pub mod pcfg;
#[doc = "BINTERVAL (rw) register accessor: HOST_PIPE Bus Access Period of Pipe\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`binterval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`binterval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@binterval`]
module"]
pub type BINTERVAL = crate::Reg<binterval::BINTERVAL_SPEC>;
#[doc = "HOST_PIPE Bus Access Period of Pipe"]
pub mod binterval;
#[doc = "PSTATUSCLR (w) register accessor: HOST_PIPE End Point Pipe Status Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pstatusclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstatusclr`]
module"]
pub type PSTATUSCLR = crate::Reg<pstatusclr::PSTATUSCLR_SPEC>;
#[doc = "HOST_PIPE End Point Pipe Status Clear"]
pub mod pstatusclr;
#[doc = "PSTATUSSET (w) register accessor: HOST_PIPE End Point Pipe Status Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pstatusset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstatusset`]
module"]
pub type PSTATUSSET = crate::Reg<pstatusset::PSTATUSSET_SPEC>;
#[doc = "HOST_PIPE End Point Pipe Status Set"]
pub mod pstatusset;
#[doc = "PSTATUS (r) register accessor: HOST_PIPE End Point Pipe Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstatus`]
module"]
pub type PSTATUS = crate::Reg<pstatus::PSTATUS_SPEC>;
#[doc = "HOST_PIPE End Point Pipe Status"]
pub mod pstatus;
#[doc = "PINTFLAG (rw) register accessor: HOST_PIPE Pipe Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pintflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pintflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintflag`]
module"]
pub type PINTFLAG = crate::Reg<pintflag::PINTFLAG_SPEC>;
#[doc = "HOST_PIPE Pipe Interrupt Flag"]
pub mod pintflag;
#[doc = "PINTENCLR (rw) register accessor: HOST_PIPE Pipe Interrupt Flag Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pintenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pintenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintenclr`]
module"]
pub type PINTENCLR = crate::Reg<pintenclr::PINTENCLR_SPEC>;
#[doc = "HOST_PIPE Pipe Interrupt Flag Clear"]
pub mod pintenclr;
#[doc = "PINTENSET (rw) register accessor: HOST_PIPE Pipe Interrupt Flag Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pintenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pintenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintenset`]
module"]
pub type PINTENSET = crate::Reg<pintenset::PINTENSET_SPEC>;
#[doc = "HOST_PIPE Pipe Interrupt Flag Set"]
pub mod pintenset;
