#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    evctrl: Evctrl,
    _reserved1: [u8; 0x03],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    xoscctrl: [Xoscctrl; 2],
    dfllctrla: Dfllctrla,
    _reserved7: [u8; 0x03],
    dfllctrlb: Dfllctrlb,
    _reserved8: [u8; 0x03],
    dfllval: Dfllval,
    dfllmul: Dfllmul,
    dfllsync: Dfllsync,
    _reserved11: [u8; 0x03],
    dpll: [Dpll; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x04 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x08 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x10 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14..0x1c - External Multipurpose Crystal Oscillator Control"]
    #[inline(always)]
    pub const fn xoscctrl(&self, n: usize) -> &Xoscctrl {
        &self.xoscctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x1c - External Multipurpose Crystal Oscillator Control"]
    #[inline(always)]
    pub fn xoscctrl_iter(&self) -> impl Iterator<Item = &Xoscctrl> {
        self.xoscctrl.iter()
    }
    #[doc = "0x1c - DFLL48M Control A"]
    #[inline(always)]
    pub const fn dfllctrla(&self) -> &Dfllctrla {
        &self.dfllctrla
    }
    #[doc = "0x20 - DFLL48M Control B"]
    #[inline(always)]
    pub const fn dfllctrlb(&self) -> &Dfllctrlb {
        &self.dfllctrlb
    }
    #[doc = "0x24 - DFLL48M Value"]
    #[inline(always)]
    pub const fn dfllval(&self) -> &Dfllval {
        &self.dfllval
    }
    #[doc = "0x28 - DFLL48M Multiplier"]
    #[inline(always)]
    pub const fn dfllmul(&self) -> &Dfllmul {
        &self.dfllmul
    }
    #[doc = "0x2c - DFLL48M Synchronization"]
    #[inline(always)]
    pub const fn dfllsync(&self) -> &Dfllsync {
        &self.dfllsync
    }
    #[doc = "0x30..0x58 - DPLL\\[%s\\]"]
    #[inline(always)]
    pub const fn dpll(&self, n: usize) -> &Dpll {
        &self.dpll[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x58 - DPLL\\[%s\\]"]
    #[inline(always)]
    pub fn dpll_iter(&self) -> impl Iterator<Item = &Dpll> {
        self.dpll.iter()
    }
}
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
#[doc = "XOSCCTRL (rw) register accessor: External Multipurpose Crystal Oscillator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xoscctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xoscctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xoscctrl`]
module"]
#[doc(alias = "XOSCCTRL")]
pub type Xoscctrl = crate::Reg<xoscctrl::XoscctrlSpec>;
#[doc = "External Multipurpose Crystal Oscillator Control"]
pub mod xoscctrl;
#[doc = "DFLLCTRLA (rw) register accessor: DFLL48M Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllctrla`]
module"]
#[doc(alias = "DFLLCTRLA")]
pub type Dfllctrla = crate::Reg<dfllctrla::DfllctrlaSpec>;
#[doc = "DFLL48M Control A"]
pub mod dfllctrla;
#[doc = "DFLLCTRLB (rw) register accessor: DFLL48M Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`dfllctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfllctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfllctrlb`]
module"]
#[doc(alias = "DFLLCTRLB")]
pub type Dfllctrlb = crate::Reg<dfllctrlb::DfllctrlbSpec>;
#[doc = "DFLL48M Control B"]
pub mod dfllctrlb;
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
#[doc = "DPLL\\[%s\\]"]
pub use self::dpll::Dpll;
#[doc = r"Cluster"]
#[doc = "DPLL\\[%s\\]"]
pub mod dpll;
