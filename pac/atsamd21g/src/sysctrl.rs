#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    pclksr: Pclksr,
    xosc: Xosc,
    _reserved5: [u8; 0x02],
    xosc32k: Xosc32k,
    _reserved6: [u8; 0x02],
    osc32k: Osc32k,
    osculp32k: Osculp32k,
    _reserved8: [u8; 0x03],
    osc8m: Osc8m,
    dfllctrl: Dfllctrl,
    _reserved10: [u8; 0x02],
    dfllval: Dfllval,
    dfllmul: Dfllmul,
    dfllsync: Dfllsync,
    _reserved13: [u8; 0x03],
    bod33: Bod33,
    _reserved14: [u8; 0x04],
    vreg: Vreg,
    _reserved15: [u8; 0x02],
    vref: Vref,
    dpllctrla: Dpllctrla,
    _reserved17: [u8; 0x03],
    dpllratio: Dpllratio,
    dpllctrlb: Dpllctrlb,
    dpllstatus: Dpllstatus,
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
    pub const fn pclksr(&self) -> &Pclksr {
        &self.pclksr
    }
    #[doc = "0x10 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    #[inline(always)]
    pub const fn xosc(&self) -> &Xosc {
        &self.xosc
    }
    #[doc = "0x14 - 32kHz External Crystal Oscillator (XOSC32K) Control"]
    #[inline(always)]
    pub const fn xosc32k(&self) -> &Xosc32k {
        &self.xosc32k
    }
    #[doc = "0x18 - 32kHz Internal Oscillator (OSC32K) Control"]
    #[inline(always)]
    pub const fn osc32k(&self) -> &Osc32k {
        &self.osc32k
    }
    #[doc = "0x1c - 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
    #[inline(always)]
    pub const fn osculp32k(&self) -> &Osculp32k {
        &self.osculp32k
    }
    #[doc = "0x20 - 8MHz Internal Oscillator (OSC8M) Control"]
    #[inline(always)]
    pub const fn osc8m(&self) -> &Osc8m {
        &self.osc8m
    }
    #[doc = "0x24 - DFLL48M Control"]
    #[inline(always)]
    pub const fn dfllctrl(&self) -> &Dfllctrl {
        &self.dfllctrl
    }
    #[doc = "0x28 - DFLL48M Value"]
    #[inline(always)]
    pub const fn dfllval(&self) -> &Dfllval {
        &self.dfllval
    }
    #[doc = "0x2c - DFLL48M Multiplier"]
    #[inline(always)]
    pub const fn dfllmul(&self) -> &Dfllmul {
        &self.dfllmul
    }
    #[doc = "0x30 - DFLL48M Synchronization"]
    #[inline(always)]
    pub const fn dfllsync(&self) -> &Dfllsync {
        &self.dfllsync
    }
    #[doc = "0x34 - 3.3V Brown-Out Detector (BOD33) Control"]
    #[inline(always)]
    pub const fn bod33(&self) -> &Bod33 {
        &self.bod33
    }
    #[doc = "0x3c - Voltage Regulator System (VREG) Control"]
    #[inline(always)]
    pub const fn vreg(&self) -> &Vreg {
        &self.vreg
    }
    #[doc = "0x40 - Voltage References System (VREF) Control"]
    #[inline(always)]
    pub const fn vref(&self) -> &Vref {
        &self.vref
    }
    #[doc = "0x44 - DPLL Control A"]
    #[inline(always)]
    pub const fn dpllctrla(&self) -> &Dpllctrla {
        &self.dpllctrla
    }
    #[doc = "0x48 - DPLL Ratio Control"]
    #[inline(always)]
    pub const fn dpllratio(&self) -> &Dpllratio {
        &self.dpllratio
    }
    #[doc = "0x4c - DPLL Control B"]
    #[inline(always)]
    pub const fn dpllctrlb(&self) -> &Dpllctrlb {
        &self.dpllctrlb
    }
    #[doc = "0x50 - DPLL Status"]
    #[inline(always)]
    pub const fn dpllstatus(&self) -> &Dpllstatus {
        &self.dpllstatus
    }
}
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
#[doc = "PCLKSR (r) register accessor: Power and Clocks Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pclksr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclksr`]
module"]
#[doc(alias = "PCLKSR")]
pub type Pclksr = crate::Reg<pclksr::PclksrSpec>;
#[doc = "Power and Clocks Status"]
pub mod pclksr;
#[doc = "XOSC (rw) register accessor: External Multipurpose Crystal Oscillator (XOSC) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc`]
module"]
#[doc(alias = "XOSC")]
pub type Xosc = crate::Reg<xosc::XoscSpec>;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xosc;
#[doc = "XOSC32K (rw) register accessor: 32kHz External Crystal Oscillator (XOSC32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xosc32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc32k`]
module"]
#[doc(alias = "XOSC32K")]
pub type Xosc32k = crate::Reg<xosc32k::Xosc32kSpec>;
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub mod xosc32k;
#[doc = "OSC32K (rw) register accessor: 32kHz Internal Oscillator (OSC32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osc32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc32k`]
module"]
#[doc(alias = "OSC32K")]
pub type Osc32k = crate::Reg<osc32k::Osc32kSpec>;
#[doc = "32kHz Internal Oscillator (OSC32K) Control"]
pub mod osc32k;
#[doc = "OSCULP32K (rw) register accessor: 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osculp32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osculp32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osculp32k`]
module"]
#[doc(alias = "OSCULP32K")]
pub type Osculp32k = crate::Reg<osculp32k::Osculp32kSpec>;
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub mod osculp32k;
#[doc = "OSC8M (rw) register accessor: 8MHz Internal Oscillator (OSC8M) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osc8m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc8m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc8m`]
module"]
#[doc(alias = "OSC8M")]
pub type Osc8m = crate::Reg<osc8m::Osc8mSpec>;
#[doc = "8MHz Internal Oscillator (OSC8M) Control"]
pub mod osc8m;
#[doc = "DFLLCTRL (rw) register accessor: DFLL48M Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllctrl`]
module"]
#[doc(alias = "DFLLCTRL")]
pub type Dfllctrl = crate::Reg<dfllctrl::DfllctrlSpec>;
#[doc = "DFLL48M Control"]
pub mod dfllctrl;
#[doc = "DFLLVAL (rw) register accessor: DFLL48M Value\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllval`]
module"]
#[doc(alias = "DFLLVAL")]
pub type Dfllval = crate::Reg<dfllval::DfllvalSpec>;
#[doc = "DFLL48M Value"]
pub mod dfllval;
#[doc = "DFLLMUL (rw) register accessor: DFLL48M Multiplier\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllmul::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllmul::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllmul`]
module"]
#[doc(alias = "DFLLMUL")]
pub type Dfllmul = crate::Reg<dfllmul::DfllmulSpec>;
#[doc = "DFLL48M Multiplier"]
pub mod dfllmul;
#[doc = "DFLLSYNC (rw) register accessor: DFLL48M Synchronization\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllsync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllsync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllsync`]
module"]
#[doc(alias = "DFLLSYNC")]
pub type Dfllsync = crate::Reg<dfllsync::DfllsyncSpec>;
#[doc = "DFLL48M Synchronization"]
pub mod dfllsync;
#[doc = "BOD33 (rw) register accessor: 3.3V Brown-Out Detector (BOD33) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bod33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bod33`]
module"]
#[doc(alias = "BOD33")]
pub type Bod33 = crate::Reg<bod33::Bod33Spec>;
#[doc = "3.3V Brown-Out Detector (BOD33) Control"]
pub mod bod33;
#[doc = "VREG (rw) register accessor: Voltage Regulator System (VREG) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vreg`]
module"]
#[doc(alias = "VREG")]
pub type Vreg = crate::Reg<vreg::VregSpec>;
#[doc = "Voltage Regulator System (VREG) Control"]
pub mod vreg;
#[doc = "VREF (rw) register accessor: Voltage References System (VREF) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vref::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref`]
module"]
#[doc(alias = "VREF")]
pub type Vref = crate::Reg<vref::VrefSpec>;
#[doc = "Voltage References System (VREF) Control"]
pub mod vref;
#[doc = "DPLLCTRLA (rw) register accessor: DPLL Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllctrla`]
module"]
#[doc(alias = "DPLLCTRLA")]
pub type Dpllctrla = crate::Reg<dpllctrla::DpllctrlaSpec>;
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLLRATIO (rw) register accessor: DPLL Ratio Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllratio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllratio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllratio`]
module"]
#[doc(alias = "DPLLRATIO")]
pub type Dpllratio = crate::Reg<dpllratio::DpllratioSpec>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB (rw) register accessor: DPLL Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllctrlb`]
module"]
#[doc(alias = "DPLLCTRLB")]
pub type Dpllctrlb = crate::Reg<dpllctrlb::DpllctrlbSpec>;
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLLSTATUS (r) register accessor: DPLL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllstatus`]
module"]
#[doc(alias = "DPLLSTATUS")]
pub type Dpllstatus = crate::Reg<dpllstatus::DpllstatusSpec>;
#[doc = "DPLL Status"]
pub mod dpllstatus;
