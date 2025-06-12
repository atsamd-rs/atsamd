#[repr(C)]
#[doc = "Clock/Calendar with Alarm"]
#[doc(alias = "MODE2")]
pub struct Mode2 {
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
    clock: Clock,
    _reserved10: [u8; 0x04],
    alarm0: Alarm0,
    mask0: Mask0,
    _reserved12: [u8; 0x03],
    alarm1: Alarm1,
    mask1: Mask1,
    _reserved14: [u8; 0x13],
    gp: [Gp; 4],
    _reserved15: [u8; 0x10],
    tampctrl: Tampctrl,
    timestamp: Timestamp,
    tampid: Tampid,
    _reserved18: [u8; 0x14],
    bkup: [Bkup; 8],
}
impl Mode2 {
    #[doc = "0x00 - MODE2 Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x02 - MODE2 Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x04 - MODE2 Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x08 - MODE2 Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x0a - MODE2 Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0c - MODE2 Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x0e - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
    #[doc = "0x10 - MODE2 Synchronization Busy Status"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x14 - Frequency Correction"]
    #[inline(always)]
    pub const fn freqcorr(&self) -> &Freqcorr {
        &self.freqcorr
    }
    #[doc = "0x18 - MODE2 Clock Value"]
    #[inline(always)]
    pub const fn clock(&self) -> &Clock {
        &self.clock
    }
    #[doc = "0x20 - MODE2_ALARM Alarm n Value"]
    #[inline(always)]
    pub const fn alarm0(&self) -> &Alarm0 {
        &self.alarm0
    }
    #[doc = "0x24 - MODE2_ALARM Alarm n Mask"]
    #[inline(always)]
    pub const fn mask0(&self) -> &Mask0 {
        &self.mask0
    }
    #[doc = "0x28 - MODE2_ALARM Alarm n Value"]
    #[inline(always)]
    pub const fn alarm1(&self) -> &Alarm1 {
        &self.alarm1
    }
    #[doc = "0x2c - MODE2_ALARM Alarm n Mask"]
    #[inline(always)]
    pub const fn mask1(&self) -> &Mask1 {
        &self.mask1
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
    #[doc = "0x64 - MODE2 Timestamp"]
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
#[doc = "CTRLA (rw) register accessor: MODE2 Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "MODE2 Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: MODE2 Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "MODE2 Control B"]
pub mod ctrlb;
#[doc = "EVCTRL (rw) register accessor: MODE2 Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "MODE2 Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: MODE2 Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "MODE2 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: MODE2 Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "MODE2 Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: MODE2 Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`] module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "MODE2 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`] module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SYNCBUSY (r) register accessor: MODE2 Synchronization Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "MODE2 Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "FREQCORR (rw) register accessor: Frequency Correction\n\nYou can [`read`](crate::Reg::read) this register and get [`freqcorr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqcorr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqcorr`] module"]
#[doc(alias = "FREQCORR")]
pub type Freqcorr = crate::Reg<freqcorr::FreqcorrSpec>;
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "CLOCK (rw) register accessor: MODE2 Clock Value\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
#[doc(alias = "CLOCK")]
pub type Clock = crate::Reg<clock::ClockSpec>;
#[doc = "MODE2 Clock Value"]
pub mod clock;
#[doc = "GP (rw) register accessor: General Purpose\n\nYou can [`read`](crate::Reg::read) this register and get [`gp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp`] module"]
#[doc(alias = "GP")]
pub type Gp = crate::Reg<gp::GpSpec>;
#[doc = "General Purpose"]
pub mod gp;
#[doc = "ALARM0 (rw) register accessor: MODE2_ALARM Alarm n Value\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0`] module"]
#[doc(alias = "ALARM0")]
pub type Alarm0 = crate::Reg<alarm0::Alarm0Spec>;
#[doc = "MODE2_ALARM Alarm n Value"]
pub mod alarm0;
#[doc = "MASK0 (rw) register accessor: MODE2_ALARM Alarm n Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask0`] module"]
#[doc(alias = "MASK0")]
pub type Mask0 = crate::Reg<mask0::Mask0Spec>;
#[doc = "MODE2_ALARM Alarm n Mask"]
pub mod mask0;
#[doc = "ALARM1 (rw) register accessor: MODE2_ALARM Alarm n Value\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm1`] module"]
#[doc(alias = "ALARM1")]
pub type Alarm1 = crate::Reg<alarm1::Alarm1Spec>;
#[doc = "MODE2_ALARM Alarm n Value"]
pub mod alarm1;
#[doc = "MASK1 (rw) register accessor: MODE2_ALARM Alarm n Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask1`] module"]
#[doc(alias = "MASK1")]
pub type Mask1 = crate::Reg<mask1::Mask1Spec>;
#[doc = "MODE2_ALARM Alarm n Mask"]
pub mod mask1;
#[doc = "TAMPCTRL (rw) register accessor: Tamper Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tampctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampctrl`] module"]
#[doc(alias = "TAMPCTRL")]
pub type Tampctrl = crate::Reg<tampctrl::TampctrlSpec>;
#[doc = "Tamper Control"]
pub mod tampctrl;
#[doc = "TIMESTAMP (r) register accessor: MODE2 Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp`] module"]
#[doc(alias = "TIMESTAMP")]
pub type Timestamp = crate::Reg<timestamp::TimestampSpec>;
#[doc = "MODE2 Timestamp"]
pub mod timestamp;
#[doc = "TAMPID (rw) register accessor: Tamper ID\n\nYou can [`read`](crate::Reg::read) this register and get [`tampid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tampid`] module"]
#[doc(alias = "TAMPID")]
pub type Tampid = crate::Reg<tampid::TampidSpec>;
#[doc = "Tamper ID"]
pub mod tampid;
#[doc = "BKUP (rw) register accessor: Backup\n\nYou can [`read`](crate::Reg::read) this register and get [`bkup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkup`] module"]
#[doc(alias = "BKUP")]
pub type Bkup = crate::Reg<bkup::BkupSpec>;
#[doc = "Backup"]
pub mod bkup;
