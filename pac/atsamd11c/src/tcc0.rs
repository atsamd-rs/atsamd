#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlbclr: Ctrlbclr,
    ctrlbset: Ctrlbset,
    _reserved3: [u8; 0x02],
    syncbusy: Syncbusy,
    fctrla: Fctrla,
    fctrlb: Fctrlb,
    wexctrl: Wexctrl,
    drvctrl: Drvctrl,
    _reserved8: [u8; 0x02],
    dbgctrl: Dbgctrl,
    _reserved9: [u8; 0x01],
    evctrl: Evctrl,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    _reserved_14_count: [u8; 0x04],
    patt: Patt,
    _reserved16: [u8; 0x02],
    wave: Wave,
    _reserved_17_per: [u8; 0x04],
    _reserved_18_cc: [u8; 0x10],
    _reserved19: [u8; 0x10],
    pattb: Pattb,
    _reserved20: [u8; 0x02],
    waveb: Waveb,
    _reserved_21_perb: [u8; 0x04],
    _reserved_22_ccb: [u8; 0x10],
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Control B Clear"]
    #[inline(always)]
    pub const fn ctrlbclr(&self) -> &Ctrlbclr {
        &self.ctrlbclr
    }
    #[doc = "0x05 - Control B Set"]
    #[inline(always)]
    pub const fn ctrlbset(&self) -> &Ctrlbset {
        &self.ctrlbset
    }
    #[doc = "0x08 - Synchronization Busy"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x0c - Recoverable Fault A Configuration"]
    #[inline(always)]
    pub const fn fctrla(&self) -> &Fctrla {
        &self.fctrla
    }
    #[doc = "0x10 - Recoverable Fault B Configuration"]
    #[inline(always)]
    pub const fn fctrlb(&self) -> &Fctrlb {
        &self.fctrlb
    }
    #[doc = "0x14 - Waveform Extension Configuration"]
    #[inline(always)]
    pub const fn wexctrl(&self) -> &Wexctrl {
        &self.wexctrl
    }
    #[doc = "0x18 - Driver Control"]
    #[inline(always)]
    pub const fn drvctrl(&self) -> &Drvctrl {
        &self.drvctrl
    }
    #[doc = "0x1e - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x20 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x24 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x28 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x2c - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x30 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub const fn count_dith6(&self) -> &CountDith6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub const fn count_dith5(&self) -> &CountDith5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub const fn count_dith4(&self) -> &CountDith4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x38 - Pattern"]
    #[inline(always)]
    pub const fn patt(&self) -> &Patt {
        &self.patt
    }
    #[doc = "0x3c - Waveform Control"]
    #[inline(always)]
    pub const fn wave(&self) -> &Wave {
        &self.wave
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per_dith6(&self) -> &PerDith6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per_dith5(&self) -> &PerDith5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per_dith4(&self) -> &PerDith4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per(&self) -> &Per {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc_dith6(&self, n: usize) -> &CcDith6 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith6_iter(&self) -> impl Iterator<Item = &CcDith6> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc0_dith6(&self) -> &CcDith6 {
        self.cc_dith6(0)
    }
    #[doc = "0x48 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc1_dith6(&self) -> &CcDith6 {
        self.cc_dith6(1)
    }
    #[doc = "0x4c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc2_dith6(&self) -> &CcDith6 {
        self.cc_dith6(2)
    }
    #[doc = "0x50 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc3_dith6(&self) -> &CcDith6 {
        self.cc_dith6(3)
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc_dith5(&self, n: usize) -> &CcDith5 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith5_iter(&self) -> impl Iterator<Item = &CcDith5> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc0_dith5(&self) -> &CcDith5 {
        self.cc_dith5(0)
    }
    #[doc = "0x48 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc1_dith5(&self) -> &CcDith5 {
        self.cc_dith5(1)
    }
    #[doc = "0x4c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc2_dith5(&self) -> &CcDith5 {
        self.cc_dith5(2)
    }
    #[doc = "0x50 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc3_dith5(&self) -> &CcDith5 {
        self.cc_dith5(3)
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc_dith4(&self, n: usize) -> &CcDith4 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith4_iter(&self) -> impl Iterator<Item = &CcDith4> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc0_dith4(&self) -> &CcDith4 {
        self.cc_dith4(0)
    }
    #[doc = "0x48 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc1_dith4(&self) -> &CcDith4 {
        self.cc_dith4(1)
    }
    #[doc = "0x4c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc2_dith4(&self) -> &CcDith4 {
        self.cc_dith4(2)
    }
    #[doc = "0x50 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc3_dith4(&self) -> &CcDith4 {
        self.cc_dith4(3)
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &Cc {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &Cc> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x64 - Pattern Buffer"]
    #[inline(always)]
    pub const fn pattb(&self) -> &Pattb {
        &self.pattb
    }
    #[doc = "0x68 - Waveform Control Buffer"]
    #[inline(always)]
    pub const fn waveb(&self) -> &Waveb {
        &self.waveb
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perb_dith6(&self) -> &PerbDith6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perb_dith5(&self) -> &PerbDith5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perb_dith4(&self) -> &PerbDith4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perb(&self) -> &Perb {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb_dith6(&self, n: usize) -> &CcbDith6 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith6_iter(&self) -> impl Iterator<Item = &CcbDith6> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb0_dith6(&self) -> &CcbDith6 {
        self.ccb_dith6(0)
    }
    #[doc = "0x74 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb1_dith6(&self) -> &CcbDith6 {
        self.ccb_dith6(1)
    }
    #[doc = "0x78 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb2_dith6(&self) -> &CcbDith6 {
        self.ccb_dith6(2)
    }
    #[doc = "0x7c - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb3_dith6(&self) -> &CcbDith6 {
        self.ccb_dith6(3)
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb_dith5(&self, n: usize) -> &CcbDith5 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith5_iter(&self) -> impl Iterator<Item = &CcbDith5> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb0_dith5(&self) -> &CcbDith5 {
        self.ccb_dith5(0)
    }
    #[doc = "0x74 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb1_dith5(&self) -> &CcbDith5 {
        self.ccb_dith5(1)
    }
    #[doc = "0x78 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb2_dith5(&self) -> &CcbDith5 {
        self.ccb_dith5(2)
    }
    #[doc = "0x7c - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb3_dith5(&self) -> &CcbDith5 {
        self.ccb_dith5(3)
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb_dith4(&self, n: usize) -> &CcbDith4 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith4_iter(&self) -> impl Iterator<Item = &CcbDith4> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb0_dith4(&self) -> &CcbDith4 {
        self.ccb_dith4(0)
    }
    #[doc = "0x74 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb1_dith4(&self) -> &CcbDith4 {
        self.ccb_dith4(1)
    }
    #[doc = "0x78 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb2_dith4(&self) -> &CcbDith4 {
        self.ccb_dith4(2)
    }
    #[doc = "0x7c - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb3_dith4(&self) -> &CcbDith4 {
        self.ccb_dith4(3)
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccb(&self, n: usize) -> &Ccb {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_iter(&self) -> impl Iterator<Item = &Ccb> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        })
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLBCLR (rw) register accessor: Control B Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlbclr`] module"]
#[doc(alias = "CTRLBCLR")]
pub type Ctrlbclr = crate::Reg<ctrlbclr::CtrlbclrSpec>;
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "CTRLBSET (rw) register accessor: Control B Set\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlbset`] module"]
#[doc(alias = "CTRLBSET")]
pub type Ctrlbset = crate::Reg<ctrlbset::CtrlbsetSpec>;
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "FCTRLA (rw) register accessor: Recoverable Fault A Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrla`] module"]
#[doc(alias = "FCTRLA")]
pub type Fctrla = crate::Reg<fctrla::FctrlaSpec>;
#[doc = "Recoverable Fault A Configuration"]
pub mod fctrla;
#[doc = "FCTRLB (rw) register accessor: Recoverable Fault B Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrlb`] module"]
#[doc(alias = "FCTRLB")]
pub type Fctrlb = crate::Reg<fctrlb::FctrlbSpec>;
#[doc = "Recoverable Fault B Configuration"]
pub mod fctrlb;
#[doc = "WEXCTRL (rw) register accessor: Waveform Extension Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wexctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wexctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wexctrl`] module"]
#[doc(alias = "WEXCTRL")]
pub type Wexctrl = crate::Reg<wexctrl::WexctrlSpec>;
#[doc = "Waveform Extension Configuration"]
pub mod wexctrl;
#[doc = "DRVCTRL (rw) register accessor: Driver Control\n\nYou can [`read`](crate::Reg::read) this register and get [`drvctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drvctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drvctrl`] module"]
#[doc(alias = "DRVCTRL")]
pub type Drvctrl = crate::Reg<drvctrl::DrvctrlSpec>;
#[doc = "Driver Control"]
pub mod drvctrl;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "COUNT (rw) register accessor: Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`] module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "Count"]
pub mod count;
#[doc = "COUNT_DITH4 (rw) register accessor: Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count_dith4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count_dith4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count_dith4`] module"]
#[doc(alias = "COUNT_DITH4")]
pub type CountDith4 = crate::Reg<count_dith4::CountDith4Spec>;
#[doc = "Count"]
pub mod count_dith4;
#[doc = "COUNT_DITH5 (rw) register accessor: Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count_dith5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count_dith5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count_dith5`] module"]
#[doc(alias = "COUNT_DITH5")]
pub type CountDith5 = crate::Reg<count_dith5::CountDith5Spec>;
#[doc = "Count"]
pub mod count_dith5;
#[doc = "COUNT_DITH6 (rw) register accessor: Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count_dith6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count_dith6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count_dith6`] module"]
#[doc(alias = "COUNT_DITH6")]
pub type CountDith6 = crate::Reg<count_dith6::CountDith6Spec>;
#[doc = "Count"]
pub mod count_dith6;
#[doc = "PATT (rw) register accessor: Pattern\n\nYou can [`read`](crate::Reg::read) this register and get [`patt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patt`] module"]
#[doc(alias = "PATT")]
pub type Patt = crate::Reg<patt::PattSpec>;
#[doc = "Pattern"]
pub mod patt;
#[doc = "WAVE (rw) register accessor: Waveform Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wave::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave`] module"]
#[doc(alias = "WAVE")]
pub type Wave = crate::Reg<wave::WaveSpec>;
#[doc = "Waveform Control"]
pub mod wave;
#[doc = "PER (rw) register accessor: Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per`] module"]
#[doc(alias = "PER")]
pub type Per = crate::Reg<per::PerSpec>;
#[doc = "Period"]
pub mod per;
#[doc = "PER_DITH4 (rw) register accessor: Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_dith4`] module"]
#[doc(alias = "PER_DITH4")]
pub type PerDith4 = crate::Reg<per_dith4::PerDith4Spec>;
#[doc = "Period"]
pub mod per_dith4;
#[doc = "PER_DITH5 (rw) register accessor: Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_dith5`] module"]
#[doc(alias = "PER_DITH5")]
pub type PerDith5 = crate::Reg<per_dith5::PerDith5Spec>;
#[doc = "Period"]
pub mod per_dith5;
#[doc = "PER_DITH6 (rw) register accessor: Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_dith6`] module"]
#[doc(alias = "PER_DITH6")]
pub type PerDith6 = crate::Reg<per_dith6::PerDith6Spec>;
#[doc = "Period"]
pub mod per_dith6;
#[doc = "CC (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`] module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Compare and Capture"]
pub mod cc;
#[doc = "CC_DITH4 (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_dith4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_dith4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_dith4`] module"]
#[doc(alias = "CC_DITH4")]
pub type CcDith4 = crate::Reg<cc_dith4::CcDith4Spec>;
#[doc = "Compare and Capture"]
pub mod cc_dith4;
#[doc = "CC_DITH5 (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_dith5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_dith5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_dith5`] module"]
#[doc(alias = "CC_DITH5")]
pub type CcDith5 = crate::Reg<cc_dith5::CcDith5Spec>;
#[doc = "Compare and Capture"]
pub mod cc_dith5;
#[doc = "CC_DITH6 (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_dith6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_dith6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_dith6`] module"]
#[doc(alias = "CC_DITH6")]
pub type CcDith6 = crate::Reg<cc_dith6::CcDith6Spec>;
#[doc = "Compare and Capture"]
pub mod cc_dith6;
#[doc = "PATTB (rw) register accessor: Pattern Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`pattb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pattb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pattb`] module"]
#[doc(alias = "PATTB")]
pub type Pattb = crate::Reg<pattb::PattbSpec>;
#[doc = "Pattern Buffer"]
pub mod pattb;
#[doc = "WAVEB (rw) register accessor: Waveform Control Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`waveb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`waveb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waveb`] module"]
#[doc(alias = "WAVEB")]
pub type Waveb = crate::Reg<waveb::WavebSpec>;
#[doc = "Waveform Control Buffer"]
pub mod waveb;
#[doc = "PERB (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perb`] module"]
#[doc(alias = "PERB")]
pub type Perb = crate::Reg<perb::PerbSpec>;
#[doc = "Period Buffer"]
pub mod perb;
#[doc = "PERB_DITH4 (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perb_dith4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perb_dith4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perb_dith4`] module"]
#[doc(alias = "PERB_DITH4")]
pub type PerbDith4 = crate::Reg<perb_dith4::PerbDith4Spec>;
#[doc = "Period Buffer"]
pub mod perb_dith4;
#[doc = "PERB_DITH5 (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perb_dith5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perb_dith5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perb_dith5`] module"]
#[doc(alias = "PERB_DITH5")]
pub type PerbDith5 = crate::Reg<perb_dith5::PerbDith5Spec>;
#[doc = "Period Buffer"]
pub mod perb_dith5;
#[doc = "PERB_DITH6 (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perb_dith6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perb_dith6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perb_dith6`] module"]
#[doc(alias = "PERB_DITH6")]
pub type PerbDith6 = crate::Reg<perb_dith6::PerbDith6Spec>;
#[doc = "Period Buffer"]
pub mod perb_dith6;
#[doc = "CCB (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccb`] module"]
#[doc(alias = "CCB")]
pub type Ccb = crate::Reg<ccb::CcbSpec>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb;
#[doc = "CCB_DITH4 (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccb_dith4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccb_dith4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccb_dith4`] module"]
#[doc(alias = "CCB_DITH4")]
pub type CcbDith4 = crate::Reg<ccb_dith4::CcbDith4Spec>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith4;
#[doc = "CCB_DITH5 (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccb_dith5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccb_dith5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccb_dith5`] module"]
#[doc(alias = "CCB_DITH5")]
pub type CcbDith5 = crate::Reg<ccb_dith5::CcbDith5Spec>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith5;
#[doc = "CCB_DITH6 (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccb_dith6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccb_dith6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccb_dith6`] module"]
#[doc(alias = "CCB_DITH6")]
pub type CcbDith6 = crate::Reg<ccb_dith6::CcbDith6Spec>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith6;
