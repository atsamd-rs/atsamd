#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    ctrlb: Ctrlb,
    evctrl: Evctrl,
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    _reserved6: [u8; 0x01],
    statusa: Statusa,
    statusb: Statusb,
    statusc: Statusc,
    _reserved9: [u8; 0x01],
    winctrl: Winctrl,
    _reserved10: [u8; 0x03],
    compctrl: [Compctrl; 2],
    _reserved11: [u8; 0x08],
    scaler: [Scaler; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x01 - Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x02 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x04 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x05 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x08 - Status A"]
    #[inline(always)]
    pub const fn statusa(&self) -> &Statusa {
        &self.statusa
    }
    #[doc = "0x09 - Status B"]
    #[inline(always)]
    pub const fn statusb(&self) -> &Statusb {
        &self.statusb
    }
    #[doc = "0x0a - Status C"]
    #[inline(always)]
    pub const fn statusc(&self) -> &Statusc {
        &self.statusc
    }
    #[doc = "0x0c - Window Control"]
    #[inline(always)]
    pub const fn winctrl(&self) -> &Winctrl {
        &self.winctrl
    }
    #[doc = "0x10..0x18 - Comparator Control n"]
    #[inline(always)]
    pub const fn compctrl(&self, n: usize) -> &Compctrl {
        &self.compctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - Comparator Control n"]
    #[inline(always)]
    pub fn compctrl_iter(&self) -> impl Iterator<Item = &Compctrl> {
        self.compctrl.iter()
    }
    #[doc = "0x20 - Scaler n"]
    #[inline(always)]
    pub const fn scaler(&self, n: usize) -> &Scaler {
        &self.scaler[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20 - Scaler n"]
    #[inline(always)]
    pub fn scaler_iter(&self) -> impl Iterator<Item = &Scaler> {
        self.scaler.iter()
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`] module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (w) register accessor: Control B\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`] module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "Control B"]
pub mod ctrlb;
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
#[doc = "STATUSA (r) register accessor: Status A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusa`] module"]
#[doc(alias = "STATUSA")]
pub type Statusa = crate::Reg<statusa::StatusaSpec>;
#[doc = "Status A"]
pub mod statusa;
#[doc = "STATUSB (r) register accessor: Status B\n\nYou can [`read`](crate::Reg::read) this register and get [`statusb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusb`] module"]
#[doc(alias = "STATUSB")]
pub type Statusb = crate::Reg<statusb::StatusbSpec>;
#[doc = "Status B"]
pub mod statusb;
#[doc = "STATUSC (r) register accessor: Status C\n\nYou can [`read`](crate::Reg::read) this register and get [`statusc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusc`] module"]
#[doc(alias = "STATUSC")]
pub type Statusc = crate::Reg<statusc::StatuscSpec>;
#[doc = "Status C"]
pub mod statusc;
#[doc = "WINCTRL (rw) register accessor: Window Control\n\nYou can [`read`](crate::Reg::read) this register and get [`winctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winctrl`] module"]
#[doc(alias = "WINCTRL")]
pub type Winctrl = crate::Reg<winctrl::WinctrlSpec>;
#[doc = "Window Control"]
pub mod winctrl;
#[doc = "COMPCTRL (rw) register accessor: Comparator Control n\n\nYou can [`read`](crate::Reg::read) this register and get [`compctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compctrl`] module"]
#[doc(alias = "COMPCTRL")]
pub type Compctrl = crate::Reg<compctrl::CompctrlSpec>;
#[doc = "Comparator Control n"]
pub mod compctrl;
#[doc = "SCALER (rw) register accessor: Scaler n\n\nYou can [`read`](crate::Reg::read) this register and get [`scaler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scaler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scaler`] module"]
#[doc(alias = "SCALER")]
pub type Scaler = crate::Reg<scaler::ScalerSpec>;
#[doc = "Scaler n"]
pub mod scaler;
