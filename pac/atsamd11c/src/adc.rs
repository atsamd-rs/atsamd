#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    refctrl: Refctrl,
    avgctrl: Avgctrl,
    sampctrl: Sampctrl,
    ctrlb: Ctrlb,
    _reserved5: [u8; 0x02],
    winctrl: Winctrl,
    _reserved6: [u8; 0x03],
    swtrig: Swtrig,
    _reserved7: [u8; 0x03],
    inputctrl: Inputctrl,
    evctrl: Evctrl,
    _reserved9: [u8; 0x01],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    result: Result,
    winlt: Winlt,
    _reserved15: [u8; 0x02],
    winut: Winut,
    _reserved16: [u8; 0x02],
    gaincorr: Gaincorr,
    offsetcorr: Offsetcorr,
    calib: Calib,
    dbgctrl: Dbgctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x01 - Reference Control"]
    #[inline(always)]
    pub const fn refctrl(&self) -> &Refctrl {
        &self.refctrl
    }
    #[doc = "0x02 - Average Control"]
    #[inline(always)]
    pub const fn avgctrl(&self) -> &Avgctrl {
        &self.avgctrl
    }
    #[doc = "0x03 - Sampling Time Control"]
    #[inline(always)]
    pub const fn sampctrl(&self) -> &Sampctrl {
        &self.sampctrl
    }
    #[doc = "0x04 - Control B"]
    #[inline(always)]
    pub const fn ctrlb(&self) -> &Ctrlb {
        &self.ctrlb
    }
    #[doc = "0x08 - Window Monitor Control"]
    #[inline(always)]
    pub const fn winctrl(&self) -> &Winctrl {
        &self.winctrl
    }
    #[doc = "0x0c - Software Trigger"]
    #[inline(always)]
    pub const fn swtrig(&self) -> &Swtrig {
        &self.swtrig
    }
    #[doc = "0x10 - Input Control"]
    #[inline(always)]
    pub const fn inputctrl(&self) -> &Inputctrl {
        &self.inputctrl
    }
    #[doc = "0x14 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x16 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x17 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x18 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x19 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1a - Result"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
    #[doc = "0x1c - Window Monitor Lower Threshold"]
    #[inline(always)]
    pub const fn winlt(&self) -> &Winlt {
        &self.winlt
    }
    #[doc = "0x20 - Window Monitor Upper Threshold"]
    #[inline(always)]
    pub const fn winut(&self) -> &Winut {
        &self.winut
    }
    #[doc = "0x24 - Gain Correction"]
    #[inline(always)]
    pub const fn gaincorr(&self) -> &Gaincorr {
        &self.gaincorr
    }
    #[doc = "0x26 - Offset Correction"]
    #[inline(always)]
    pub const fn offsetcorr(&self) -> &Offsetcorr {
        &self.offsetcorr
    }
    #[doc = "0x28 - Calibration"]
    #[inline(always)]
    pub const fn calib(&self) -> &Calib {
        &self.calib
    }
    #[doc = "0x2a - Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(&self) -> &Dbgctrl {
        &self.dbgctrl
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "REFCTRL (rw) register accessor: Reference Control\n\nYou can [`read`](crate::Reg::read) this register and get [`refctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refctrl`]
module"]
#[doc(alias = "REFCTRL")]
pub type Refctrl = crate::Reg<refctrl::RefctrlSpec>;
#[doc = "Reference Control"]
pub mod refctrl;
#[doc = "AVGCTRL (rw) register accessor: Average Control\n\nYou can [`read`](crate::Reg::read) this register and get [`avgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avgctrl`]
module"]
#[doc(alias = "AVGCTRL")]
pub type Avgctrl = crate::Reg<avgctrl::AvgctrlSpec>;
#[doc = "Average Control"]
pub mod avgctrl;
#[doc = "SAMPCTRL (rw) register accessor: Sampling Time Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sampctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampctrl`]
module"]
#[doc(alias = "SAMPCTRL")]
pub type Sampctrl = crate::Reg<sampctrl::SampctrlSpec>;
#[doc = "Sampling Time Control"]
pub mod sampctrl;
#[doc = "CTRLB (rw) register accessor: Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlb`]
module"]
#[doc(alias = "CTRLB")]
pub type Ctrlb = crate::Reg<ctrlb::CtrlbSpec>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "WINCTRL (rw) register accessor: Window Monitor Control\n\nYou can [`read`](crate::Reg::read) this register and get [`winctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winctrl`]
module"]
#[doc(alias = "WINCTRL")]
pub type Winctrl = crate::Reg<winctrl::WinctrlSpec>;
#[doc = "Window Monitor Control"]
pub mod winctrl;
#[doc = "SWTRIG (rw) register accessor: Software Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrig`]
module"]
#[doc(alias = "SWTRIG")]
pub type Swtrig = crate::Reg<swtrig::SwtrigSpec>;
#[doc = "Software Trigger"]
pub mod swtrig;
#[doc = "INPUTCTRL (rw) register accessor: Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`inputctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputctrl`]
module"]
#[doc(alias = "INPUTCTRL")]
pub type Inputctrl = crate::Reg<inputctrl::InputctrlSpec>;
#[doc = "Input Control"]
pub mod inputctrl;
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
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "RESULT (r) register accessor: Result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`]
module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "Result"]
pub mod result;
#[doc = "WINLT (rw) register accessor: Window Monitor Lower Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`winlt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winlt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winlt`]
module"]
#[doc(alias = "WINLT")]
pub type Winlt = crate::Reg<winlt::WinltSpec>;
#[doc = "Window Monitor Lower Threshold"]
pub mod winlt;
#[doc = "WINUT (rw) register accessor: Window Monitor Upper Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`winut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winut`]
module"]
#[doc(alias = "WINUT")]
pub type Winut = crate::Reg<winut::WinutSpec>;
#[doc = "Window Monitor Upper Threshold"]
pub mod winut;
#[doc = "GAINCORR (rw) register accessor: Gain Correction\n\nYou can [`read`](crate::Reg::read) this register and get [`gaincorr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gaincorr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gaincorr`]
module"]
#[doc(alias = "GAINCORR")]
pub type Gaincorr = crate::Reg<gaincorr::GaincorrSpec>;
#[doc = "Gain Correction"]
pub mod gaincorr;
#[doc = "OFFSETCORR (rw) register accessor: Offset Correction\n\nYou can [`read`](crate::Reg::read) this register and get [`offsetcorr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offsetcorr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offsetcorr`]
module"]
#[doc(alias = "OFFSETCORR")]
pub type Offsetcorr = crate::Reg<offsetcorr::OffsetcorrSpec>;
#[doc = "Offset Correction"]
pub mod offsetcorr;
#[doc = "CALIB (rw) register accessor: Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`calib::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calib::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib`]
module"]
#[doc(alias = "CALIB")]
pub type Calib = crate::Reg<calib::CalibSpec>;
#[doc = "Calibration"]
pub mod calib;
#[doc = "DBGCTRL (rw) register accessor: Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctrl`]
module"]
#[doc(alias = "DBGCTRL")]
pub type Dbgctrl = crate::Reg<dbgctrl::DbgctrlSpec>;
#[doc = "Debug Control"]
pub mod dbgctrl;
