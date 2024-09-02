#[repr(C)]
#[doc = "32-bit Counter Mode"]
#[doc(alias = "COUNT32")]
pub struct Count32 {
    ctrla: Ctrla,
    ctrlbclr: Ctrlbclr,
    ctrlbset: Ctrlbset,
    evctrl: Evctrl,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    wave: Wave,
    drvctrl: Drvctrl,
    _reserved10: [u8; 0x01],
    dbgctrl: Dbgctrl,
    syncbusy: Syncbusy,
    count: Count,
    _reserved13: [u8; 0x04],
    cc: [Cc; 2],
    _reserved14: [u8; 0x0c],
    ccbuf: [Ccbuf; 2],
}
impl Count32 {
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
    #[doc = "0x06 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x08 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x09 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0a - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x0b - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x0c - Waveform Generation Control"]
    #[inline(always)]
    pub const fn wave(&self) -> &Wave {
        &self.wave
    }
    #[doc = "0x0d - Control C"]
    #[inline(always)]
    pub const fn drvctrl(&self) -> &Drvctrl {
        &self.drvctrl
    }
    #[doc = "0x0f - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x10 - Synchronization Status"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x14 - COUNT32 Count"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x1c..0x24 - COUNT32 Compare and Capture"]
    #[inline(always)]
    pub const fn cc(&self, n: usize) -> &Cc {
        &self.cc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x24 - COUNT32 Compare and Capture"]
    #[inline(always)]
    pub fn cc_iter(&self) -> impl Iterator<Item = &Cc> {
        self.cc.iter()
    }
    #[doc = "0x30..0x38 - COUNT32 Compare and Capture Buffer"]
    #[inline(always)]
    pub const fn ccbuf(&self, n: usize) -> &Ccbuf {
        &self.ccbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x38 - COUNT32 Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_iter(&self) -> impl Iterator<Item = &Ccbuf> {
        self.ccbuf.iter()
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLBCLR (rw) register accessor: Control B Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlbclr`]
module"]
#[doc(alias = "CTRLBCLR")]
pub type Ctrlbclr = crate::Reg<ctrlbclr::CtrlbclrSpec>;
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "CTRLBSET (rw) register accessor: Control B Set\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlbset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlbset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlbset`]
module"]
#[doc(alias = "CTRLBSET")]
pub type Ctrlbset = crate::Reg<ctrlbset::CtrlbsetSpec>;
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "WAVE (rw) register accessor: Waveform Generation Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wave::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wave`]
module"]
#[doc(alias = "WAVE")]
pub type Wave = crate::Reg<wave::WaveSpec>;
#[doc = "Waveform Generation Control"]
pub mod wave;
#[doc = "DRVCTRL (rw) register accessor: Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`drvctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drvctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drvctrl`]
module"]
#[doc(alias = "DRVCTRL")]
pub type Drvctrl = crate::Reg<drvctrl::DrvctrlSpec>;
#[doc = "Control C"]
pub mod drvctrl;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SYNCBUSY (r) register accessor: Synchronization Status\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Synchronization Status"]
pub mod syncbusy;
#[doc = "COUNT (rw) register accessor: COUNT32 Count\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "COUNT32 Count"]
pub mod count;
#[doc = "CC (rw) register accessor: COUNT32 Compare and Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`]
module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "COUNT32 Compare and Capture"]
pub mod cc;
#[doc = "CCBUF (rw) register accessor: COUNT32 Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccbuf`]
module"]
#[doc(alias = "CCBUF")]
pub type Ccbuf = crate::Reg<ccbuf::CcbufSpec>;
#[doc = "COUNT32 Compare and Capture Buffer"]
pub mod ccbuf;
