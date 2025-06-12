#[repr(C)]
#[doc = "USB is Device"]
#[doc(alias = "DEVICE")]
pub struct Device {
    ctrla: Ctrla,
    _reserved1: [u8; 0x01],
    syncbusy: Syncbusy,
    qosctrl: Qosctrl,
    _reserved3: [u8; 0x04],
    ctrlb: Ctrlb,
    dadd: Dadd,
    _reserved5: [u8; 0x01],
    status: Status,
    fsmstatus: Fsmstatus,
    _reserved7: [u8; 0x02],
    fnum: Fnum,
    _reserved8: [u8; 0x02],
    intenclr: Intenclr,
    _reserved9: [u8; 0x02],
    intenset: Intenset,
    _reserved10: [u8; 0x02],
    intflag: Intflag,
    _reserved11: [u8; 0x02],
    epintsmry: Epintsmry,
    _reserved12: [u8; 0x02],
    descadd: Descadd,
    padcal: Padcal,
    _reserved14: [u8; 0xd6],
    device_endpoint: (),
}
impl Device {
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
    #[doc = "0x08 - DEVICE Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x0a - DEVICE Device Address"]
    #[inline(always)]
    pub const fn dadd(&self) -> &Dadd {
        &self.dadd
    }
    #[doc = "0x0c - DEVICE Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0d - Finite State Machine Status"]
    #[inline(always)]
    pub const fn fsmstatus(&self) -> &Fsmstatus {
        &self.fsmstatus
    }
    #[doc = "0x10 - DEVICE Device Frame Number"]
    #[inline(always)]
    pub const fn fnum(&self) -> &Fnum {
        &self.fnum
    }
    #[doc = "0x14 - DEVICE Device Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x18 - DEVICE Device Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x1c - DEVICE Device Interrupt Flag"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x20 - DEVICE End Point Interrupt Summary"]
    #[inline(always)]
    pub const fn epintsmry(&self) -> &Epintsmry {
        &self.epintsmry
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
    #[doc = "0x100..0x150 - DEVICE_ENDPOINT\\[%s\\]"]
    #[inline(always)]
    pub const fn device_endpoint(&self, n: usize) -> &DeviceEndpoint {
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
    #[doc = "0x100..0x150 - DEVICE_ENDPOINT\\[%s\\]"]
    #[inline(always)]
    pub fn device_endpoint_iter(&self) -> impl Iterator<Item = &DeviceEndpoint> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
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
#[doc = "CTRLB (rw) register accessor: DEVICE Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "DEVICE Control B"]
pub mod ctrlb;
#[doc = "DADD (rw) register accessor: DEVICE Device Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dadd`] module"]
#[doc(alias = "DADD")]
pub type Dadd = crate::Reg<dadd::DaddSpec>;
#[doc = "DEVICE Device Address"]
pub mod dadd;
#[doc = "STATUS (r) register accessor: DEVICE Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "DEVICE Status"]
pub mod status;
#[doc = "FSMSTATUS (r) register accessor: Finite State Machine Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsmstatus`] module"]
#[doc(alias = "FSMSTATUS")]
pub type Fsmstatus = crate::Reg<fsmstatus::FsmstatusSpec>;
#[doc = "Finite State Machine Status"]
pub mod fsmstatus;
#[doc = "FNUM (r) register accessor: DEVICE Device Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`fnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fnum`] module"]
#[doc(alias = "FNUM")]
pub type Fnum = crate::Reg<fnum::FnumSpec>;
#[doc = "DEVICE Device Frame Number"]
pub mod fnum;
#[doc = "INTENCLR (rw) register accessor: DEVICE Device Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "DEVICE Device Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: DEVICE Device Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "DEVICE Device Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: DEVICE Device Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "DEVICE Device Interrupt Flag"]
pub mod intflag;
#[doc = "EPINTSMRY (r) register accessor: DEVICE End Point Interrupt Summary\n\nYou can [`read`](crate::Reg::read) this register and get [`epintsmry::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epintsmry`] module"]
#[doc(alias = "EPINTSMRY")]
pub type Epintsmry = crate::Reg<epintsmry::EpintsmrySpec>;
#[doc = "DEVICE End Point Interrupt Summary"]
pub mod epintsmry;
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
#[doc = "DEVICE_ENDPOINT\\[%s\\]"]
pub use self::device_endpoint::DeviceEndpoint;
#[doc = r"Cluster"]
#[doc = "DEVICE_ENDPOINT\\[%s\\]"]
pub mod device_endpoint;
