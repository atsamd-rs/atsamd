#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    sleep: Sleep,
    _reserved2: [u8; 0x06],
    cpusel: Cpusel,
    apbasel: Apbasel,
    apbbsel: Apbbsel,
    apbcsel: Apbcsel,
    _reserved6: [u8; 0x08],
    ahbmask: Ahbmask,
    apbamask: Apbamask,
    apbbmask: Apbbmask,
    apbcmask: Apbcmask,
    _reserved10: [u8; 0x10],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    _reserved13: [u8; 0x01],
    rcause: Rcause,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x01 - Sleep Mode"]
    #[inline(always)]
    pub const fn sleep(&self) -> &Sleep {
        &self.sleep
    }
    #[doc = "0x08 - CPU Clock Select"]
    #[inline(always)]
    pub const fn cpusel(&self) -> &Cpusel {
        &self.cpusel
    }
    #[doc = "0x09 - APBA Clock Select"]
    #[inline(always)]
    pub const fn apbasel(&self) -> &Apbasel {
        &self.apbasel
    }
    #[doc = "0x0a - APBB Clock Select"]
    #[inline(always)]
    pub const fn apbbsel(&self) -> &Apbbsel {
        &self.apbbsel
    }
    #[doc = "0x0b - APBC Clock Select"]
    #[inline(always)]
    pub const fn apbcsel(&self) -> &Apbcsel {
        &self.apbcsel
    }
    #[doc = "0x14 - AHB Mask"]
    #[inline(always)]
    pub const fn ahbmask(&self) -> &Ahbmask {
        &self.ahbmask
    }
    #[doc = "0x18 - APBA Mask"]
    #[inline(always)]
    pub const fn apbamask(&self) -> &Apbamask {
        &self.apbamask
    }
    #[doc = "0x1c - APBB Mask"]
    #[inline(always)]
    pub const fn apbbmask(&self) -> &Apbbmask {
        &self.apbbmask
    }
    #[doc = "0x20 - APBC Mask"]
    #[inline(always)]
    pub const fn apbcmask(&self) -> &Apbcmask {
        &self.apbcmask
    }
    #[doc = "0x34 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x35 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x36 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x38 - Reset Cause"]
    #[inline(always)]
    pub const fn rcause(&self) -> &Rcause {
        &self.rcause
    }
}
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "SLEEP (rw) register accessor: Sleep Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleep`]
module"]
#[doc(alias = "SLEEP")]
pub type Sleep = crate::Reg<sleep::SleepSpec>;
#[doc = "Sleep Mode"]
pub mod sleep;
#[doc = "CPUSEL (rw) register accessor: CPU Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`cpusel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpusel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpusel`]
module"]
#[doc(alias = "CPUSEL")]
pub type Cpusel = crate::Reg<cpusel::CpuselSpec>;
#[doc = "CPU Clock Select"]
pub mod cpusel;
#[doc = "APBASEL (rw) register accessor: APBA Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`apbasel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbasel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbasel`]
module"]
#[doc(alias = "APBASEL")]
pub type Apbasel = crate::Reg<apbasel::ApbaselSpec>;
#[doc = "APBA Clock Select"]
pub mod apbasel;
#[doc = "APBBSEL (rw) register accessor: APBB Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`apbbsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbbsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbbsel`]
module"]
#[doc(alias = "APBBSEL")]
pub type Apbbsel = crate::Reg<apbbsel::ApbbselSpec>;
#[doc = "APBB Clock Select"]
pub mod apbbsel;
#[doc = "APBCSEL (rw) register accessor: APBC Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`apbcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbcsel`]
module"]
#[doc(alias = "APBCSEL")]
pub type Apbcsel = crate::Reg<apbcsel::ApbcselSpec>;
#[doc = "APBC Clock Select"]
pub mod apbcsel;
#[doc = "AHBMASK (rw) register accessor: AHB Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbmask`]
module"]
#[doc(alias = "AHBMASK")]
pub type Ahbmask = crate::Reg<ahbmask::AhbmaskSpec>;
#[doc = "AHB Mask"]
pub mod ahbmask;
#[doc = "APBAMASK (rw) register accessor: APBA Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbamask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbamask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbamask`]
module"]
#[doc(alias = "APBAMASK")]
pub type Apbamask = crate::Reg<apbamask::ApbamaskSpec>;
#[doc = "APBA Mask"]
pub mod apbamask;
#[doc = "APBBMASK (rw) register accessor: APBB Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbbmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbbmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbbmask`]
module"]
#[doc(alias = "APBBMASK")]
pub type Apbbmask = crate::Reg<apbbmask::ApbbmaskSpec>;
#[doc = "APBB Mask"]
pub mod apbbmask;
#[doc = "APBCMASK (rw) register accessor: APBC Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbcmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbcmask`]
module"]
#[doc(alias = "APBCMASK")]
pub type Apbcmask = crate::Reg<apbcmask::ApbcmaskSpec>;
#[doc = "APBC Mask"]
pub mod apbcmask;
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
#[doc = "RCAUSE (r) register accessor: Reset Cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rcause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcause`]
module"]
#[doc(alias = "RCAUSE")]
pub type Rcause = crate::Reg<rcause::RcauseSpec>;
#[doc = "Reset Cause"]
pub mod rcause;
