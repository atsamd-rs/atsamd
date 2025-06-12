#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    hsdiv: Hsdiv,
    cpudiv: Cpudiv,
    _reserved5: [u8; 0x0a],
    ahbmask: Ahbmask,
    apbamask: Apbamask,
    apbbmask: Apbbmask,
    apbcmask: Apbcmask,
    apbdmask: Apbdmask,
}
impl RegisterBlock {
    #[doc = "0x01 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x02 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x03 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x04 - HS Clock Division"]
    #[inline(always)]
    pub const fn hsdiv(&self) -> &Hsdiv {
        &self.hsdiv
    }
    #[doc = "0x05 - CPU Clock Division"]
    #[inline(always)]
    pub const fn cpudiv(&self) -> &Cpudiv {
        &self.cpudiv
    }
    #[doc = "0x10 - AHB Mask"]
    #[inline(always)]
    pub const fn ahbmask(&self) -> &Ahbmask {
        &self.ahbmask
    }
    #[doc = "0x14 - APBA Mask"]
    #[inline(always)]
    pub const fn apbamask(&self) -> &Apbamask {
        &self.apbamask
    }
    #[doc = "0x18 - APBB Mask"]
    #[inline(always)]
    pub const fn apbbmask(&self) -> &Apbbmask {
        &self.apbbmask
    }
    #[doc = "0x1c - APBC Mask"]
    #[inline(always)]
    pub const fn apbcmask(&self) -> &Apbcmask {
        &self.apbcmask
    }
    #[doc = "0x20 - APBD Mask"]
    #[inline(always)]
    pub const fn apbdmask(&self) -> &Apbdmask {
        &self.apbdmask
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
#[doc = "HSDIV (r) register accessor: HS Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`hsdiv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsdiv`] module"]
#[doc(alias = "HSDIV")]
pub type Hsdiv = crate::Reg<hsdiv::HsdivSpec>;
#[doc = "HS Clock Division"]
pub mod hsdiv;
#[doc = "CPUDIV (rw) register accessor: CPU Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`cpudiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpudiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpudiv`] module"]
#[doc(alias = "CPUDIV")]
pub type Cpudiv = crate::Reg<cpudiv::CpudivSpec>;
#[doc = "CPU Clock Division"]
pub mod cpudiv;
#[doc = "AHBMASK (rw) register accessor: AHB Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbmask`] module"]
#[doc(alias = "AHBMASK")]
pub type Ahbmask = crate::Reg<ahbmask::AhbmaskSpec>;
#[doc = "AHB Mask"]
pub mod ahbmask;
#[doc = "APBAMASK (rw) register accessor: APBA Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbamask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbamask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbamask`] module"]
#[doc(alias = "APBAMASK")]
pub type Apbamask = crate::Reg<apbamask::ApbamaskSpec>;
#[doc = "APBA Mask"]
pub mod apbamask;
#[doc = "APBBMASK (rw) register accessor: APBB Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbbmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbbmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbbmask`] module"]
#[doc(alias = "APBBMASK")]
pub type Apbbmask = crate::Reg<apbbmask::ApbbmaskSpec>;
#[doc = "APBB Mask"]
pub mod apbbmask;
#[doc = "APBCMASK (rw) register accessor: APBC Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbcmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbcmask`] module"]
#[doc(alias = "APBCMASK")]
pub type Apbcmask = crate::Reg<apbcmask::ApbcmaskSpec>;
#[doc = "APBC Mask"]
pub mod apbcmask;
#[doc = "APBDMASK (rw) register accessor: APBD Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbdmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbdmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbdmask`] module"]
#[doc(alias = "APBDMASK")]
pub type Apbdmask = crate::Reg<apbdmask::ApbdmaskSpec>;
#[doc = "APBD Mask"]
pub mod apbdmask;
