#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    rtcctrl: Rtcctrl,
    _reserved5: [u8; 0x03],
    xosc32k: Xosc32k,
    cfdctrl: Cfdctrl,
    evctrl: Evctrl,
    _reserved8: [u8; 0x04],
    osculp32k: Osculp32k,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x04 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x0c - Power and Clocks Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - RTC Clock Selection"]
    #[inline(always)]
    pub const fn rtcctrl(&self) -> &Rtcctrl {
        &self.rtcctrl
    }
    #[doc = "0x14 - 32kHz External Crystal Oscillator (XOSC32K) Control"]
    #[inline(always)]
    pub const fn xosc32k(&self) -> &Xosc32k {
        &self.xosc32k
    }
    #[doc = "0x16 - Clock Failure Detector Control"]
    #[inline(always)]
    pub const fn cfdctrl(&self) -> &Cfdctrl {
        &self.cfdctrl
    }
    #[doc = "0x17 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x1c - 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
    #[inline(always)]
    pub const fn osculp32k(&self) -> &Osculp32k {
        &self.osculp32k
    }
}
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
#[doc = "STATUS (r) register accessor: Power and Clocks Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Power and Clocks Status"]
pub mod status;
#[doc = "RTCCTRL (rw) register accessor: RTC Clock Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcctrl`] module"]
#[doc(alias = "RTCCTRL")]
pub type Rtcctrl = crate::Reg<rtcctrl::RtcctrlSpec>;
#[doc = "RTC Clock Selection"]
pub mod rtcctrl;
#[doc = "XOSC32K (rw) register accessor: 32kHz External Crystal Oscillator (XOSC32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc32k`] module"]
#[doc(alias = "XOSC32K")]
pub type Xosc32k = crate::Reg<xosc32k::Xosc32kSpec>;
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub mod xosc32k;
#[doc = "CFDCTRL (rw) register accessor: Clock Failure Detector Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cfdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfdctrl`] module"]
#[doc(alias = "CFDCTRL")]
pub type Cfdctrl = crate::Reg<cfdctrl::CfdctrlSpec>;
#[doc = "Clock Failure Detector Control"]
pub mod cfdctrl;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`] module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "OSCULP32K (rw) register accessor: 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osculp32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osculp32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osculp32k`] module"]
#[doc(alias = "OSCULP32K")]
pub type Osculp32k = crate::Reg<osculp32k::Osculp32kSpec>;
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub mod osculp32k;
