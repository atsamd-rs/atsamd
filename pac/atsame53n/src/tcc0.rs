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
    _reserved_18_cc: [u8; 0x18],
    _reserved19: [u8; 0x08],
    pattbuf: Pattbuf,
    _reserved20: [u8; 0x06],
    _reserved_20_perbuf: [u8; 0x04],
    _reserved_21_ccbuf: [u8; 0x18],
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
    pub const fn count_dith6_mode(&self) -> &CountDith6Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub const fn count_dith5_mode(&self) -> &CountDith5Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub const fn count_dith4_mode(&self) -> &CountDith4Mode {
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
    pub const fn per_dith6_mode(&self) -> &PerDith6Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per_dith5_mode(&self) -> &PerDith5Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per_dith4_mode(&self) -> &PerDith4Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub const fn per(&self) -> &Per {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc_dith6_mode(&self, n: usize) -> &CcDith6Mode {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith6_mode_iter(&self) -> impl Iterator<Item = &CcDith6Mode> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc_dith5_mode(&self, n: usize) -> &CcDith5Mode {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith5_mode_iter(&self) -> impl Iterator<Item = &CcDith5Mode> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc_dith4_mode(&self, n: usize) -> &CcDith4Mode {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith4_mode_iter(&self) -> impl Iterator<Item = &CcDith4Mode> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &Cc {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x5c - Compare and Capture"]
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &Cc> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(68)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x64 - Pattern Buffer"]
    #[inline(always)]
    pub const fn pattbuf(&self) -> &Pattbuf {
        &self.pattbuf
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perbuf_dith6_mode(&self) -> &PerbufDith6Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perbuf_dith5_mode(&self) -> &PerbufDith5Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perbuf_dith4_mode(&self) -> &PerbufDith4Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub const fn perbuf(&self) -> &Perbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(108).cast() }
    }
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccbuf_dith6_mode(&self, n: usize) -> &CcbufDith6Mode {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_dith6_mode_iter(&self) -> impl Iterator<Item = &CcbufDith6Mode> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccbuf_dith5_mode(&self, n: usize) -> &CcbufDith5Mode {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_dith5_mode_iter(&self) -> impl Iterator<Item = &CcbufDith5Mode> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccbuf_dith4_mode(&self, n: usize) -> &CcbufDith4Mode {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_dith4_mode_iter(&self) -> impl Iterator<Item = &CcbufDith4Mode> {
        (0..6).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccbuf(&self, n: usize) -> &Ccbuf {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x88 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_iter(&self) -> impl Iterator<Item = &Ccbuf> {
        (0..6).map(move |n| unsafe {
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
#[doc = "COUNT_DITH4_MODE (rw) register accessor: Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count_dith4_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count_dith4_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count_dith4_mode`] module"]
#[doc(alias = "COUNT_DITH4_MODE")]
pub type CountDith4Mode = crate::Reg<count_dith4_mode::CountDith4ModeSpec>;
#[doc = "Count"]
pub mod count_dith4_mode;
#[doc = "COUNT_DITH5_MODE (rw) register accessor: Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count_dith5_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count_dith5_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count_dith5_mode`] module"]
#[doc(alias = "COUNT_DITH5_MODE")]
pub type CountDith5Mode = crate::Reg<count_dith5_mode::CountDith5ModeSpec>;
#[doc = "Count"]
pub mod count_dith5_mode;
#[doc = "COUNT_DITH6_MODE (rw) register accessor: Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count_dith6_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count_dith6_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count_dith6_mode`] module"]
#[doc(alias = "COUNT_DITH6_MODE")]
pub type CountDith6Mode = crate::Reg<count_dith6_mode::CountDith6ModeSpec>;
#[doc = "Count"]
pub mod count_dith6_mode;
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
#[doc = "PER_DITH4_MODE (rw) register accessor: Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith4_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith4_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_dith4_mode`] module"]
#[doc(alias = "PER_DITH4_MODE")]
pub type PerDith4Mode = crate::Reg<per_dith4_mode::PerDith4ModeSpec>;
#[doc = "Period"]
pub mod per_dith4_mode;
#[doc = "PER_DITH5_MODE (rw) register accessor: Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith5_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith5_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_dith5_mode`] module"]
#[doc(alias = "PER_DITH5_MODE")]
pub type PerDith5Mode = crate::Reg<per_dith5_mode::PerDith5ModeSpec>;
#[doc = "Period"]
pub mod per_dith5_mode;
#[doc = "PER_DITH6_MODE (rw) register accessor: Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per_dith6_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_dith6_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_dith6_mode`] module"]
#[doc(alias = "PER_DITH6_MODE")]
pub type PerDith6Mode = crate::Reg<per_dith6_mode::PerDith6ModeSpec>;
#[doc = "Period"]
pub mod per_dith6_mode;
#[doc = "CC (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`] module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Compare and Capture"]
pub mod cc;
#[doc = "CC_DITH4_MODE (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_dith4_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_dith4_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_dith4_mode`] module"]
#[doc(alias = "CC_DITH4_MODE")]
pub type CcDith4Mode = crate::Reg<cc_dith4_mode::CcDith4ModeSpec>;
#[doc = "Compare and Capture"]
pub mod cc_dith4_mode;
#[doc = "CC_DITH5_MODE (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_dith5_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_dith5_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_dith5_mode`] module"]
#[doc(alias = "CC_DITH5_MODE")]
pub type CcDith5Mode = crate::Reg<cc_dith5_mode::CcDith5ModeSpec>;
#[doc = "Compare and Capture"]
pub mod cc_dith5_mode;
#[doc = "CC_DITH6_MODE (rw) register accessor: Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_dith6_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_dith6_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_dith6_mode`] module"]
#[doc(alias = "CC_DITH6_MODE")]
pub type CcDith6Mode = crate::Reg<cc_dith6_mode::CcDith6ModeSpec>;
#[doc = "Compare and Capture"]
pub mod cc_dith6_mode;
#[doc = "PATTBUF (rw) register accessor: Pattern Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`pattbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pattbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pattbuf`] module"]
#[doc(alias = "PATTBUF")]
pub type Pattbuf = crate::Reg<pattbuf::PattbufSpec>;
#[doc = "Pattern Buffer"]
pub mod pattbuf;
#[doc = "PERBUF (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbuf`] module"]
#[doc(alias = "PERBUF")]
pub type Perbuf = crate::Reg<perbuf::PerbufSpec>;
#[doc = "Period Buffer"]
pub mod perbuf;
#[doc = "PERBUF_DITH4_MODE (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perbuf_dith4_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbuf_dith4_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbuf_dith4_mode`] module"]
#[doc(alias = "PERBUF_DITH4_MODE")]
pub type PerbufDith4Mode = crate::Reg<perbuf_dith4_mode::PerbufDith4ModeSpec>;
#[doc = "Period Buffer"]
pub mod perbuf_dith4_mode;
#[doc = "PERBUF_DITH5_MODE (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perbuf_dith5_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbuf_dith5_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbuf_dith5_mode`] module"]
#[doc(alias = "PERBUF_DITH5_MODE")]
pub type PerbufDith5Mode = crate::Reg<perbuf_dith5_mode::PerbufDith5ModeSpec>;
#[doc = "Period Buffer"]
pub mod perbuf_dith5_mode;
#[doc = "PERBUF_DITH6_MODE (rw) register accessor: Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perbuf_dith6_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbuf_dith6_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbuf_dith6_mode`] module"]
#[doc(alias = "PERBUF_DITH6_MODE")]
pub type PerbufDith6Mode = crate::Reg<perbuf_dith6_mode::PerbufDith6ModeSpec>;
#[doc = "Period Buffer"]
pub mod perbuf_dith6_mode;
#[doc = "CCBUF (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccbuf`] module"]
#[doc(alias = "CCBUF")]
pub type Ccbuf = crate::Reg<ccbuf::CcbufSpec>;
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf;
#[doc = "CCBUF_DITH4_MODE (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf_dith4_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf_dith4_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccbuf_dith4_mode`] module"]
#[doc(alias = "CCBUF_DITH4_MODE")]
pub type CcbufDith4Mode = crate::Reg<ccbuf_dith4_mode::CcbufDith4ModeSpec>;
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf_dith4_mode;
#[doc = "CCBUF_DITH5_MODE (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf_dith5_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf_dith5_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccbuf_dith5_mode`] module"]
#[doc(alias = "CCBUF_DITH5_MODE")]
pub type CcbufDith5Mode = crate::Reg<ccbuf_dith5_mode::CcbufDith5ModeSpec>;
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf_dith5_mode;
#[doc = "CCBUF_DITH6_MODE (rw) register accessor: Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf_dith6_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf_dith6_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccbuf_dith6_mode`] module"]
#[doc(alias = "CCBUF_DITH6_MODE")]
pub type CcbufDith6Mode = crate::Reg<ccbuf_dith6_mode::CcbufDith6ModeSpec>;
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf_dith6_mode;
