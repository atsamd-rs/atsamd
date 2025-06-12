#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    bod33: Bod33,
    _reserved5: [u8; 0x04],
    vreg: Vreg,
    vref: Vref,
    bbps: Bbps,
    bkout: Bkout,
    bkin: Bkin,
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
    #[doc = "0x10 - BOD33 Control"]
    #[inline(always)]
    pub const fn bod33(&self) -> &Bod33 {
        &self.bod33
    }
    #[doc = "0x18 - VREG Control"]
    #[inline(always)]
    pub const fn vreg(&self) -> &Vreg {
        &self.vreg
    }
    #[doc = "0x1c - VREF Control"]
    #[inline(always)]
    pub const fn vref(&self) -> &Vref {
        &self.vref
    }
    #[doc = "0x20 - Battery Backup Power Switch"]
    #[inline(always)]
    pub const fn bbps(&self) -> &Bbps {
        &self.bbps
    }
    #[doc = "0x24 - Backup Output Control"]
    #[inline(always)]
    pub const fn bkout(&self) -> &Bkout {
        &self.bkout
    }
    #[doc = "0x28 - Backup Input Control"]
    #[inline(always)]
    pub const fn bkin(&self) -> &Bkin {
        &self.bkin
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
#[doc = "BOD33 (rw) register accessor: BOD33 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bod33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bod33`] module"]
#[doc(alias = "BOD33")]
pub type Bod33 = crate::Reg<bod33::Bod33Spec>;
#[doc = "BOD33 Control"]
pub mod bod33;
#[doc = "VREG (rw) register accessor: VREG Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vreg`] module"]
#[doc(alias = "VREG")]
pub type Vreg = crate::Reg<vreg::VregSpec>;
#[doc = "VREG Control"]
pub mod vreg;
#[doc = "VREF (rw) register accessor: VREF Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vref::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref`] module"]
#[doc(alias = "VREF")]
pub type Vref = crate::Reg<vref::VrefSpec>;
#[doc = "VREF Control"]
pub mod vref;
#[doc = "BBPS (rw) register accessor: Battery Backup Power Switch\n\nYou can [`read`](crate::Reg::read) this register and get [`bbps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bbps`] module"]
#[doc(alias = "BBPS")]
pub type Bbps = crate::Reg<bbps::BbpsSpec>;
#[doc = "Battery Backup Power Switch"]
pub mod bbps;
#[doc = "BKOUT (rw) register accessor: Backup Output Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bkout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkout`] module"]
#[doc(alias = "BKOUT")]
pub type Bkout = crate::Reg<bkout::BkoutSpec>;
#[doc = "Backup Output Control"]
pub mod bkout;
#[doc = "BKIN (r) register accessor: Backup Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bkin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkin`] module"]
#[doc(alias = "BKIN")]
pub type Bkin = crate::Reg<bkin::BkinSpec>;
#[doc = "Backup Input Control"]
pub mod bkin;
