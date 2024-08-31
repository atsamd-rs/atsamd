#[repr(C)]
#[doc = "32-bit Counter with Single 32-bit Compare"]
#[doc(alias = "MODE0")]
pub struct Mode0 {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    evctrl: Evctrl,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    dbgctrl: Dbgctrl,
    _reserved7: [u8; 0x01],
    syncbusy: Syncbusy,
    freqcorr: Freqcorr,
    _reserved9: [u8; 0x03],
    count: Count,
    _reserved10: [u8; 0x04],
    comp: [Comp; 2],
    _reserved11: [u8; 0x18],
    gp: [Gp; 4],
    _reserved12: [u8; 0x10],
    tampctrl: Tampctrl,
    timestamp: Timestamp,
    tampid: Tampid,
    _reserved15: [u8; 0x14],
    bkup: [Bkup; 8],
}
impl Mode0 {
    #[doc = "0x00 - MODE0 Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x02 - MODE0 Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x04 - MODE0 Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x08 - MODE0 Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x0a - MODE0 Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0c - MODE0 Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x0e - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x10 - MODE0 Synchronization Busy Status"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x14 - Frequency Correction"]
    #[inline(always)]
    pub const fn freqcorr(&self) -> &Freqcorr {
        &self.freqcorr
    }
    #[doc = "0x18 - MODE0 Counter Value"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x20..0x28 - MODE0 Compare n Value"]
    #[inline(always)]
    pub const fn comp(&self, n: usize) -> &Comp {
        &self.comp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - MODE0 Compare n Value"]
    #[inline(always)]
    pub fn comp_iter(&self) -> impl Iterator<Item = &Comp> {
        self.comp.iter()
    }
    #[doc = "0x40..0x50 - General Purpose"]
    #[inline(always)]
    pub const fn gp(&self, n: usize) -> &Gp {
        &self.gp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - General Purpose"]
    #[inline(always)]
    pub fn gp_iter(&self) -> impl Iterator<Item = &Gp> {
        self.gp.iter()
    }
    #[doc = "0x60 - Tamper Control"]
    #[inline(always)]
    pub const fn tampctrl(&self) -> &Tampctrl {
        &self.tampctrl
    }
    #[doc = "0x64 - MODE0 Timestamp"]
    #[inline(always)]
    pub const fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
    #[doc = "0x68 - Tamper ID"]
    #[inline(always)]
    pub const fn tampid(&self) -> &Tampid {
        &self.tampid
    }
    #[doc = "0x80..0xa0 - Backup"]
    #[inline(always)]
    pub const fn bkup(&self, n: usize) -> &Bkup {
        &self.bkup[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xa0 - Backup"]
    #[inline(always)]
    pub fn bkup_iter(&self) -> impl Iterator<Item = &Bkup> {
        self.bkup.iter()
    }
}
#[doc = "CTRLA (rw) register accessor: MODE0 Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "MODE0 Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: MODE0 Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "MODE0 Control B"]
pub mod ctrlb;
#[doc = "EVCTRL (rw) register accessor: MODE0 Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "MODE0 Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: MODE0 Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "MODE0 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: MODE0 Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "MODE0 Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: MODE0 Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "MODE0 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SYNCBUSY (r) register accessor: MODE0 Synchronization Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`]
module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "MODE0 Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "FREQCORR (rw) register accessor: Frequency Correction\n\nYou can [`read`](crate::Reg::read) this register and get [`freqcorr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqcorr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqcorr`]
module"]
#[doc(alias = "FREQCORR")]
pub type Freqcorr = crate::Reg<freqcorr::FreqcorrSpec>;
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "COUNT (rw) register accessor: MODE0 Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "MODE0 Counter Value"]
pub mod count;
#[doc = "COMP (rw) register accessor: MODE0 Compare n Value\n\nYou can [`read`](crate::Reg::read) this register and get [`comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp`]
module"]
#[doc(alias = "COMP")]
pub type Comp = crate::Reg<comp::CompSpec>;
#[doc = "MODE0 Compare n Value"]
pub mod comp;
#[doc = "GP (rw) register accessor: General Purpose\n\nYou can [`read`](crate::Reg::read) this register and get [`gp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp`]
module"]
#[doc(alias = "GP")]
pub type Gp = crate::Reg<gp::GpSpec>;
#[doc = "General Purpose"]
pub mod gp;
#[doc = "TAMPCTRL (rw) register accessor: Tamper Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tampctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampctrl`]
module"]
#[doc(alias = "TAMPCTRL")]
pub type Tampctrl = crate::Reg<tampctrl::TampctrlSpec>;
#[doc = "Tamper Control"]
pub mod tampctrl;
#[doc = "TIMESTAMP (r) register accessor: MODE0 Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp`]
module"]
#[doc(alias = "TIMESTAMP")]
pub type Timestamp = crate::Reg<timestamp::TimestampSpec>;
#[doc = "MODE0 Timestamp"]
pub mod timestamp;
#[doc = "TAMPID (rw) register accessor: Tamper ID\n\nYou can [`read`](crate::Reg::read) this register and get [`tampid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampid`]
module"]
#[doc(alias = "TAMPID")]
pub type Tampid = crate::Reg<tampid::TampidSpec>;
#[doc = "Tamper ID"]
pub mod tampid;
#[doc = "BKUP (rw) register accessor: Backup\n\nYou can [`read`](crate::Reg::read) this register and get [`bkup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkup`]
module"]
#[doc(alias = "BKUP")]
pub type Bkup = crate::Reg<bkup::BkupSpec>;
#[doc = "Backup"]
pub mod bkup;
