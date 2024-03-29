#[doc = r"Register block"]
#[repr(C)]
pub struct DEVICE {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x03 - USB Quality Of Service"]
    pub qosctrl: QOSCTRL,
    _reserved3: [u8; 0x04],
    #[doc = "0x08 - DEVICE Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x0a - DEVICE Device Address"]
    pub dadd: DADD,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - DEVICE Status"]
    pub status: STATUS,
    #[doc = "0x0d - Finite State Machine Status"]
    pub fsmstatus: FSMSTATUS,
    _reserved7: [u8; 0x02],
    #[doc = "0x10 - DEVICE Device Frame Number"]
    pub fnum: FNUM,
    _reserved8: [u8; 0x02],
    #[doc = "0x14 - DEVICE Device Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 0x02],
    #[doc = "0x18 - DEVICE Device Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved10: [u8; 0x02],
    #[doc = "0x1c - DEVICE Device Interrupt Flag"]
    pub intflag: INTFLAG,
    _reserved11: [u8; 0x02],
    #[doc = "0x20 - DEVICE End Point Interrupt Summary"]
    pub epintsmry: EPINTSMRY,
    _reserved12: [u8; 0x02],
    #[doc = "0x24 - Descriptor Address"]
    pub descadd: DESCADD,
    #[doc = "0x28 - USB PAD Calibration"]
    pub padcal: PADCAL,
    _reserved14: [u8; 0xd6],
    #[doc = "0x100 - DEVICE End Point Configuration"]
    pub epcfg0: EPCFG,
    _reserved15: [u8; 0x03],
    #[doc = "0x104 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr0: EPSTATUSCLR,
    #[doc = "0x105 - DEVICE End Point Pipe Status Set"]
    pub epstatusset0: EPSTATUSSET,
    #[doc = "0x106 - DEVICE End Point Pipe Status"]
    pub epstatus0: EPSTATUS,
    #[doc = "0x107 - DEVICE End Point Interrupt Flag"]
    pub epintflag0: EPINTFLAG,
    #[doc = "0x108 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr0: EPINTENCLR,
    #[doc = "0x109 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset0: EPINTENSET,
    _reserved21: [u8; 0x16],
    #[doc = "0x120 - DEVICE End Point Configuration"]
    pub epcfg1: EPCFG,
    _reserved22: [u8; 0x03],
    #[doc = "0x124 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr1: EPSTATUSCLR,
    #[doc = "0x125 - DEVICE End Point Pipe Status Set"]
    pub epstatusset1: EPSTATUSSET,
    #[doc = "0x126 - DEVICE End Point Pipe Status"]
    pub epstatus1: EPSTATUS,
    #[doc = "0x127 - DEVICE End Point Interrupt Flag"]
    pub epintflag1: EPINTFLAG,
    #[doc = "0x128 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr1: EPINTENCLR,
    #[doc = "0x129 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset1: EPINTENSET,
    _reserved28: [u8; 0x16],
    #[doc = "0x140 - DEVICE End Point Configuration"]
    pub epcfg2: EPCFG,
    _reserved29: [u8; 0x03],
    #[doc = "0x144 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr2: EPSTATUSCLR,
    #[doc = "0x145 - DEVICE End Point Pipe Status Set"]
    pub epstatusset2: EPSTATUSSET,
    #[doc = "0x146 - DEVICE End Point Pipe Status"]
    pub epstatus2: EPSTATUS,
    #[doc = "0x147 - DEVICE End Point Interrupt Flag"]
    pub epintflag2: EPINTFLAG,
    #[doc = "0x148 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr2: EPINTENCLR,
    #[doc = "0x149 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset2: EPINTENSET,
    _reserved35: [u8; 0x16],
    #[doc = "0x160 - DEVICE End Point Configuration"]
    pub epcfg3: EPCFG,
    _reserved36: [u8; 0x03],
    #[doc = "0x164 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr3: EPSTATUSCLR,
    #[doc = "0x165 - DEVICE End Point Pipe Status Set"]
    pub epstatusset3: EPSTATUSSET,
    #[doc = "0x166 - DEVICE End Point Pipe Status"]
    pub epstatus3: EPSTATUS,
    #[doc = "0x167 - DEVICE End Point Interrupt Flag"]
    pub epintflag3: EPINTFLAG,
    #[doc = "0x168 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr3: EPINTENCLR,
    #[doc = "0x169 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset3: EPINTENSET,
    _reserved42: [u8; 0x16],
    #[doc = "0x180 - DEVICE End Point Configuration"]
    pub epcfg4: EPCFG,
    _reserved43: [u8; 0x03],
    #[doc = "0x184 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr4: EPSTATUSCLR,
    #[doc = "0x185 - DEVICE End Point Pipe Status Set"]
    pub epstatusset4: EPSTATUSSET,
    #[doc = "0x186 - DEVICE End Point Pipe Status"]
    pub epstatus4: EPSTATUS,
    #[doc = "0x187 - DEVICE End Point Interrupt Flag"]
    pub epintflag4: EPINTFLAG,
    #[doc = "0x188 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr4: EPINTENCLR,
    #[doc = "0x189 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset4: EPINTENSET,
    _reserved49: [u8; 0x16],
    #[doc = "0x1a0 - DEVICE End Point Configuration"]
    pub epcfg5: EPCFG,
    _reserved50: [u8; 0x03],
    #[doc = "0x1a4 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr5: EPSTATUSCLR,
    #[doc = "0x1a5 - DEVICE End Point Pipe Status Set"]
    pub epstatusset5: EPSTATUSSET,
    #[doc = "0x1a6 - DEVICE End Point Pipe Status"]
    pub epstatus5: EPSTATUS,
    #[doc = "0x1a7 - DEVICE End Point Interrupt Flag"]
    pub epintflag5: EPINTFLAG,
    #[doc = "0x1a8 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr5: EPINTENCLR,
    #[doc = "0x1a9 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset5: EPINTENSET,
    _reserved56: [u8; 0x16],
    #[doc = "0x1c0 - DEVICE End Point Configuration"]
    pub epcfg6: EPCFG,
    _reserved57: [u8; 0x03],
    #[doc = "0x1c4 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr6: EPSTATUSCLR,
    #[doc = "0x1c5 - DEVICE End Point Pipe Status Set"]
    pub epstatusset6: EPSTATUSSET,
    #[doc = "0x1c6 - DEVICE End Point Pipe Status"]
    pub epstatus6: EPSTATUS,
    #[doc = "0x1c7 - DEVICE End Point Interrupt Flag"]
    pub epintflag6: EPINTFLAG,
    #[doc = "0x1c8 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr6: EPINTENCLR,
    #[doc = "0x1c9 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset6: EPINTENSET,
    _reserved63: [u8; 0x16],
    #[doc = "0x1e0 - DEVICE End Point Configuration"]
    pub epcfg7: EPCFG,
    _reserved64: [u8; 0x03],
    #[doc = "0x1e4 - DEVICE End Point Pipe Status Clear"]
    pub epstatusclr7: EPSTATUSCLR,
    #[doc = "0x1e5 - DEVICE End Point Pipe Status Set"]
    pub epstatusset7: EPSTATUSSET,
    #[doc = "0x1e6 - DEVICE End Point Pipe Status"]
    pub epstatus7: EPSTATUS,
    #[doc = "0x1e7 - DEVICE End Point Interrupt Flag"]
    pub epintflag7: EPINTFLAG,
    #[doc = "0x1e8 - DEVICE End Point Interrupt Clear Flag"]
    pub epintenclr7: EPINTENCLR,
    #[doc = "0x1e9 - DEVICE End Point Interrupt Set Flag"]
    pub epintenset7: EPINTENSET,
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncbusy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "QOSCTRL (rw) register accessor: USB Quality Of Service\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qosctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qosctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qosctrl`]
module"]
pub type QOSCTRL = crate::Reg<qosctrl::QOSCTRL_SPEC>;
#[doc = "USB Quality Of Service"]
pub mod qosctrl;
#[doc = "CTRLB (rw) register accessor: DEVICE Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "DEVICE Control B"]
pub mod ctrlb;
#[doc = "DADD (rw) register accessor: DEVICE Device Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dadd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dadd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadd`]
module"]
pub type DADD = crate::Reg<dadd::DADD_SPEC>;
#[doc = "DEVICE Device Address"]
pub mod dadd;
#[doc = "STATUS (r) register accessor: DEVICE Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "DEVICE Status"]
pub mod status;
#[doc = "FSMSTATUS (r) register accessor: Finite State Machine Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsmstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsmstatus`]
module"]
pub type FSMSTATUS = crate::Reg<fsmstatus::FSMSTATUS_SPEC>;
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "FNUM (r) register accessor: DEVICE Device Frame Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnum`]
module"]
pub type FNUM = crate::Reg<fnum::FNUM_SPEC>;
#[doc = "DEVICE Device Frame Number"]
pub mod fnum;
#[doc = "INTENCLR (rw) register accessor: DEVICE Device Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "DEVICE Device Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: DEVICE Device Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "DEVICE Device Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: DEVICE Device Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "DEVICE Device Interrupt Flag"]
pub mod intflag;
#[doc = "EPINTSMRY (r) register accessor: DEVICE End Point Interrupt Summary\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epintsmry::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintsmry`]
module"]
pub type EPINTSMRY = crate::Reg<epintsmry::EPINTSMRY_SPEC>;
#[doc = "DEVICE End Point Interrupt Summary"]
pub mod epintsmry;
#[doc = "DESCADD (rw) register accessor: Descriptor Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descadd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descadd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descadd`]
module"]
pub type DESCADD = crate::Reg<descadd::DESCADD_SPEC>;
#[doc = "Descriptor Address"]
pub mod descadd;
#[doc = "PADCAL (rw) register accessor: USB PAD Calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padcal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padcal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcal`]
module"]
pub type PADCAL = crate::Reg<padcal::PADCAL_SPEC>;
#[doc = "USB PAD Calibration"]
pub mod padcal;
#[doc = "EPCFG (rw) register accessor: DEVICE End Point Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epcfg`]
module"]
pub type EPCFG = crate::Reg<epcfg::EPCFG_SPEC>;
#[doc = "DEVICE End Point Configuration"]
pub mod epcfg;
#[doc = "EPSTATUSCLR (w) register accessor: DEVICE End Point Pipe Status Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epstatusclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstatusclr`]
module"]
pub type EPSTATUSCLR = crate::Reg<epstatusclr::EPSTATUSCLR_SPEC>;
#[doc = "DEVICE End Point Pipe Status Clear"]
pub mod epstatusclr;
#[doc = "EPSTATUSSET (w) register accessor: DEVICE End Point Pipe Status Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epstatusset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstatusset`]
module"]
pub type EPSTATUSSET = crate::Reg<epstatusset::EPSTATUSSET_SPEC>;
#[doc = "DEVICE End Point Pipe Status Set"]
pub mod epstatusset;
#[doc = "EPSTATUS (r) register accessor: DEVICE End Point Pipe Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epstatus`]
module"]
pub type EPSTATUS = crate::Reg<epstatus::EPSTATUS_SPEC>;
#[doc = "DEVICE End Point Pipe Status"]
pub mod epstatus;
#[doc = "EPINTFLAG (rw) register accessor: DEVICE End Point Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epintflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epintflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintflag`]
module"]
pub type EPINTFLAG = crate::Reg<epintflag::EPINTFLAG_SPEC>;
#[doc = "DEVICE End Point Interrupt Flag"]
pub mod epintflag;
#[doc = "EPINTENCLR (rw) register accessor: DEVICE End Point Interrupt Clear Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epintenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epintenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintenclr`]
module"]
pub type EPINTENCLR = crate::Reg<epintenclr::EPINTENCLR_SPEC>;
#[doc = "DEVICE End Point Interrupt Clear Flag"]
pub mod epintenclr;
#[doc = "EPINTENSET (rw) register accessor: DEVICE End Point Interrupt Set Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epintenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epintenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintenset`]
module"]
pub type EPINTENSET = crate::Reg<epintenset::EPINTENSET_SPEC>;
#[doc = "DEVICE End Point Interrupt Set Flag"]
pub mod epintenset;
