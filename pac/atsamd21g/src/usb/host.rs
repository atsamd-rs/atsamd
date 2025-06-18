#[repr(C)]
#[doc = "USB is Host"]
#[doc(alias = "HOST")]
pub struct Host {
    ctrla: Ctrla,
    _reserved1: [u8; 0x01],
    syncbusy: Syncbusy,
    qosctrl: Qosctrl,
    _reserved3: [u8; 0x04],
    ctrlb: Ctrlb,
    hsofc: Hsofc,
    _reserved5: [u8; 0x01],
    status: Status,
    fsmstatus: Fsmstatus,
    _reserved7: [u8; 0x02],
    fnum: Fnum,
    flenhigh: Flenhigh,
    _reserved9: [u8; 0x01],
    intenclr: Intenclr,
    _reserved10: [u8; 0x02],
    intenset: Intenset,
    _reserved11: [u8; 0x02],
    intflag: Intflag,
    _reserved12: [u8; 0x02],
    pintsmry: Pintsmry,
    _reserved13: [u8; 0x02],
    descadd: Descadd,
    padcal: Padcal,
    _reserved15: [u8; 0xd6],
    pcfg: (),
    _reserved16: [u8; 0x03],
    binterval: (),
    _reserved17: [u8; 0x01],
    pstatusclr: (),
    _reserved18: [u8; 0x01],
    pstatusset: (),
    _reserved19: [u8; 0x01],
    pstatus: (),
    _reserved20: [u8; 0x01],
    pintflag: (),
    _reserved21: [u8; 0x01],
    pintenclr: (),
    _reserved22: [u8; 0x01],
    pintenset: (),
}
impl Host {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x02 - Synchronization Busy"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x03 - USB Quality Of Service"]
    #[inline(always)]
    pub const fn qosctrl(&self) -> &Qosctrl {
        &self.qosctrl
    }
    #[doc = "0x08 - HOST Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x0a - HOST Host Start Of Frame Control"]
    #[inline(always)]
    pub const fn hsofc(&self) -> &Hsofc {
        &self.hsofc
    }
    #[doc = "0x0c - HOST Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0d - Finite State Machine Status"]
    #[inline(always)]
    pub const fn fsmstatus(&self) -> &Fsmstatus {
        &self.fsmstatus
    }
    #[doc = "0x10 - HOST Host Frame Number"]
    #[inline(always)]
    pub const fn fnum(&self) -> &Fnum {
        &self.fnum
    }
    #[doc = "0x12 - HOST Host Frame Length"]
    #[inline(always)]
    pub const fn flenhigh(&self) -> &Flenhigh {
        &self.flenhigh
    }
    #[doc = "0x14 - HOST Host Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x18 - HOST Host Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x1c - HOST Host Interrupt Flag"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x20 - HOST Pipe Interrupt Summary"]
    #[inline(always)]
    pub const fn pintsmry(&self) -> &Pintsmry {
        &self.pintsmry
    }
    #[doc = "0x24 - Descriptor Address"]
    #[inline(always)]
    pub const fn descadd(&self) -> &Descadd {
        &self.descadd
    }
    #[doc = "0x28 - USB PAD Calibration"]
    #[inline(always)]
    pub const fn padcal(&self) -> &Padcal {
        &self.padcal
    }
    #[doc = "0x100..0x108 - HOST End Point Configuration"]
    #[inline(always)]
    pub const fn pcfg(&self, n: usize) -> &Pcfg {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x108 - HOST End Point Configuration"]
    #[inline(always)]
    pub fn pcfg_iter(&self) -> impl Iterator<Item = &Pcfg> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x103..0x10b - HOST Bus Access Period of Pipe"]
    #[inline(always)]
    pub const fn binterval(&self, n: usize) -> &Binterval {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(259)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x103..0x10b - HOST Bus Access Period of Pipe"]
    #[inline(always)]
    pub fn binterval_iter(&self) -> impl Iterator<Item = &Binterval> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(259)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x104..0x10c - HOST End Point Pipe Status Clear"]
    #[inline(always)]
    pub const fn pstatusclr(&self, n: usize) -> &Pstatusclr {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x10c - HOST End Point Pipe Status Clear"]
    #[inline(always)]
    pub fn pstatusclr_iter(&self) -> impl Iterator<Item = &Pstatusclr> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x105..0x10d - HOST End Point Pipe Status Set"]
    #[inline(always)]
    pub const fn pstatusset(&self, n: usize) -> &Pstatusset {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(261)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x105..0x10d - HOST End Point Pipe Status Set"]
    #[inline(always)]
    pub fn pstatusset_iter(&self) -> impl Iterator<Item = &Pstatusset> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(261)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x106..0x10e - HOST End Point Pipe Status"]
    #[inline(always)]
    pub const fn pstatus(&self, n: usize) -> &Pstatus {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(262)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x106..0x10e - HOST End Point Pipe Status"]
    #[inline(always)]
    pub fn pstatus_iter(&self) -> impl Iterator<Item = &Pstatus> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(262)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x107..0x10f - HOST Pipe Interrupt Flag"]
    #[inline(always)]
    pub const fn pintflag(&self, n: usize) -> &Pintflag {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(263)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x107..0x10f - HOST Pipe Interrupt Flag"]
    #[inline(always)]
    pub fn pintflag_iter(&self) -> impl Iterator<Item = &Pintflag> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(263)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x108..0x110 - HOST Pipe Interrupt Flag Clear"]
    #[inline(always)]
    pub const fn pintenclr(&self, n: usize) -> &Pintenclr {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(264)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x108..0x110 - HOST Pipe Interrupt Flag Clear"]
    #[inline(always)]
    pub fn pintenclr_iter(&self) -> impl Iterator<Item = &Pintenclr> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(264)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x109..0x111 - HOST Pipe Interrupt Flag Set"]
    #[inline(always)]
    pub const fn pintenset(&self, n: usize) -> &Pintenset {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(265)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x109..0x111 - HOST Pipe Interrupt Flag Set"]
    #[inline(always)]
    pub fn pintenset_iter(&self) -> impl Iterator<Item = &Pintenset> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(265)
                .add(32 * n)
                .cast()
        })
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "QOSCTRL (rw) register accessor: USB Quality Of Service\n\nYou can [`read`](crate::Reg::read) this register and get [`qosctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qosctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qosctrl`] module"]
#[doc(alias = "QOSCTRL")]
pub type Qosctrl = crate::Reg<qosctrl::QosctrlSpec>;
#[doc = "USB Quality Of Service"]
pub mod qosctrl;
#[doc = "CTRLB (rw) register accessor: HOST Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "HOST Control B"]
pub mod ctrlb;
#[doc = "HSOFC (rw) register accessor: HOST Host Start Of Frame Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hsofc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsofc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsofc`] module"]
#[doc(alias = "HSOFC")]
pub type Hsofc = crate::Reg<hsofc::HsofcSpec>;
#[doc = "HOST Host Start Of Frame Control"]
pub mod hsofc;
#[doc = "STATUS (rw) register accessor: HOST Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "HOST Status"]
pub mod status;
#[doc = "FSMSTATUS (r) register accessor: Finite State Machine Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsmstatus`] module"]
#[doc(alias = "FSMSTATUS")]
pub type Fsmstatus = crate::Reg<fsmstatus::FsmstatusSpec>;
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "FNUM (rw) register accessor: HOST Host Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`fnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnum`] module"]
#[doc(alias = "FNUM")]
pub type Fnum = crate::Reg<fnum::FnumSpec>;
#[doc = "HOST Host Frame Number"]
pub mod fnum;
#[doc = "FLENHIGH (r) register accessor: HOST Host Frame Length\n\nYou can [`read`](crate::Reg::read) this register and get [`flenhigh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flenhigh`] module"]
#[doc(alias = "FLENHIGH")]
pub type Flenhigh = crate::Reg<flenhigh::FlenhighSpec>;
#[doc = "HOST Host Frame Length"]
pub mod flenhigh;
#[doc = "INTENCLR (rw) register accessor: HOST Host Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "HOST Host Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: HOST Host Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "HOST Host Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: HOST Host Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "HOST Host Interrupt Flag"]
pub mod intflag;
#[doc = "PINTSMRY (r) register accessor: HOST Pipe Interrupt Summary\n\nYou can [`read`](crate::Reg::read) this register and get [`pintsmry::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintsmry`] module"]
#[doc(alias = "PINTSMRY")]
pub type Pintsmry = crate::Reg<pintsmry::PintsmrySpec>;
#[doc = "HOST Pipe Interrupt Summary"]
pub mod pintsmry;
#[doc = "DESCADD (rw) register accessor: Descriptor Address\n\nYou can [`read`](crate::Reg::read) this register and get [`descadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`descadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descadd`] module"]
#[doc(alias = "DESCADD")]
pub type Descadd = crate::Reg<descadd::DescaddSpec>;
#[doc = "Descriptor Address"]
pub mod descadd;
#[doc = "PADCAL (rw) register accessor: USB PAD Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`padcal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padcal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padcal`] module"]
#[doc(alias = "PADCAL")]
pub type Padcal = crate::Reg<padcal::PadcalSpec>;
#[doc = "USB PAD Calibration"]
pub mod padcal;
#[doc = "PCFG (rw) register accessor: HOST End Point Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcfg`] module"]
#[doc(alias = "PCFG")]
pub type Pcfg = crate::Reg<pcfg::PcfgSpec>;
#[doc = "HOST End Point Configuration"]
pub mod pcfg;
#[doc = "BINTERVAL (rw) register accessor: HOST Bus Access Period of Pipe\n\nYou can [`read`](crate::Reg::read) this register and get [`binterval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`binterval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@binterval`] module"]
#[doc(alias = "BINTERVAL")]
pub type Binterval = crate::Reg<binterval::BintervalSpec>;
#[doc = "HOST Bus Access Period of Pipe"]
pub mod binterval;
#[doc = "PSTATUSCLR (rw) register accessor: HOST End Point Pipe Status Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`pstatusclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pstatusclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstatusclr`] module"]
#[doc(alias = "PSTATUSCLR")]
pub type Pstatusclr = crate::Reg<pstatusclr::PstatusclrSpec>;
#[doc = "HOST End Point Pipe Status Clear"]
pub mod pstatusclr;
#[doc = "PSTATUSSET (w) register accessor: HOST End Point Pipe Status Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pstatusset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstatusset`] module"]
#[doc(alias = "PSTATUSSET")]
pub type Pstatusset = crate::Reg<pstatusset::PstatussetSpec>;
#[doc = "HOST End Point Pipe Status Set"]
pub mod pstatusset;
#[doc = "PSTATUS (r) register accessor: HOST End Point Pipe Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstatus`] module"]
#[doc(alias = "PSTATUS")]
pub type Pstatus = crate::Reg<pstatus::PstatusSpec>;
#[doc = "HOST End Point Pipe Status"]
pub mod pstatus;
#[doc = "PINTFLAG (rw) register accessor: HOST Pipe Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`pintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintflag`] module"]
#[doc(alias = "PINTFLAG")]
pub type Pintflag = crate::Reg<pintflag::PintflagSpec>;
#[doc = "HOST Pipe Interrupt Flag"]
pub mod pintflag;
#[doc = "PINTENCLR (rw) register accessor: HOST Pipe Interrupt Flag Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`pintenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintenclr`] module"]
#[doc(alias = "PINTENCLR")]
pub type Pintenclr = crate::Reg<pintenclr::PintenclrSpec>;
#[doc = "HOST Pipe Interrupt Flag Clear"]
pub mod pintenclr;
#[doc = "PINTENSET (rw) register accessor: HOST Pipe Interrupt Flag Set\n\nYou can [`read`](crate::Reg::read) this register and get [`pintenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pintenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pintenset`] module"]
#[doc(alias = "PINTENSET")]
pub type Pintenset = crate::Reg<pintenset::PintensetSpec>;
#[doc = "HOST Pipe Interrupt Flag Set"]
pub mod pintenset;
