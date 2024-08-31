#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wrctrl: Wrctrl,
    evctrl: Evctrl,
    _reserved2: [u8; 0x03],
    intenclr: Intenclr,
    intenset: Intenset,
    _reserved4: [u8; 0x06],
    intflagahb: Intflagahb,
    intflaga: Intflaga,
    intflagb: Intflagb,
    intflagc: Intflagc,
    intflagd: Intflagd,
    _reserved9: [u8; 0x10],
    statusa: Statusa,
    statusb: Statusb,
    statusc: Statusc,
    statusd: Statusd,
}
impl RegisterBlock {
    #[doc = "0x00 - Write control"]
    #[inline(always)]
    pub const fn wrctrl(&self) -> &Wrctrl {
        &self.wrctrl
    }
    #[doc = "0x04 - Event control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x08 - Interrupt enable clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x09 - Interrupt enable set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x10 - Bridge interrupt flag status"]
    #[inline(always)]
    pub const fn intflagahb(&self) -> &Intflagahb {
        &self.intflagahb
    }
    #[doc = "0x14 - Peripheral interrupt flag status - Bridge A"]
    #[inline(always)]
    pub const fn intflaga(&self) -> &Intflaga {
        &self.intflaga
    }
    #[doc = "0x18 - Peripheral interrupt flag status - Bridge B"]
    #[inline(always)]
    pub const fn intflagb(&self) -> &Intflagb {
        &self.intflagb
    }
    #[doc = "0x1c - Peripheral interrupt flag status - Bridge C"]
    #[inline(always)]
    pub const fn intflagc(&self) -> &Intflagc {
        &self.intflagc
    }
    #[doc = "0x20 - Peripheral interrupt flag status - Bridge D"]
    #[inline(always)]
    pub const fn intflagd(&self) -> &Intflagd {
        &self.intflagd
    }
    #[doc = "0x34 - Peripheral write protection status - Bridge A"]
    #[inline(always)]
    pub const fn statusa(&self) -> &Statusa {
        &self.statusa
    }
    #[doc = "0x38 - Peripheral write protection status - Bridge B"]
    #[inline(always)]
    pub const fn statusb(&self) -> &Statusb {
        &self.statusb
    }
    #[doc = "0x3c - Peripheral write protection status - Bridge C"]
    #[inline(always)]
    pub const fn statusc(&self) -> &Statusc {
        &self.statusc
    }
    #[doc = "0x40 - Peripheral write protection status - Bridge D"]
    #[inline(always)]
    pub const fn statusd(&self) -> &Statusd {
        &self.statusd
    }
}
#[doc = "WRCTRL (rw) register accessor: Write control\n\nYou can [`read`](crate::Reg::read) this register and get [`wrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrctrl`]
module"]
#[doc(alias = "WRCTRL")]
pub type Wrctrl = crate::Reg<wrctrl::WrctrlSpec>;
#[doc = "Write control"]
pub mod wrctrl;
#[doc = "EVCTRL (rw) register accessor: Event control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: Interrupt enable clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt enable set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt enable set"]
pub mod intenset;
#[doc = "INTFLAGAHB (rw) register accessor: Bridge interrupt flag status\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagahb`]
module"]
#[doc(alias = "INTFLAGAHB")]
pub type Intflagahb = crate::Reg<intflagahb::IntflagahbSpec>;
#[doc = "Bridge interrupt flag status"]
pub mod intflagahb;
#[doc = "INTFLAGA (rw) register accessor: Peripheral interrupt flag status - Bridge A\n\nYou can [`read`](crate::Reg::read) this register and get [`intflaga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflaga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflaga`]
module"]
#[doc(alias = "INTFLAGA")]
pub type Intflaga = crate::Reg<intflaga::IntflagaSpec>;
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub mod intflaga;
#[doc = "INTFLAGB (rw) register accessor: Peripheral interrupt flag status - Bridge B\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagb`]
module"]
#[doc(alias = "INTFLAGB")]
pub type Intflagb = crate::Reg<intflagb::IntflagbSpec>;
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub mod intflagb;
#[doc = "INTFLAGC (rw) register accessor: Peripheral interrupt flag status - Bridge C\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagc`]
module"]
#[doc(alias = "INTFLAGC")]
pub type Intflagc = crate::Reg<intflagc::IntflagcSpec>;
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub mod intflagc;
#[doc = "INTFLAGD (rw) register accessor: Peripheral interrupt flag status - Bridge D\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflagd`]
module"]
#[doc(alias = "INTFLAGD")]
pub type Intflagd = crate::Reg<intflagd::IntflagdSpec>;
#[doc = "Peripheral interrupt flag status - Bridge D"]
pub mod intflagd;
#[doc = "STATUSA (r) register accessor: Peripheral write protection status - Bridge A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusa`]
module"]
#[doc(alias = "STATUSA")]
pub type Statusa = crate::Reg<statusa::StatusaSpec>;
#[doc = "Peripheral write protection status - Bridge A"]
pub mod statusa;
#[doc = "STATUSB (r) register accessor: Peripheral write protection status - Bridge B\n\nYou can [`read`](crate::Reg::read) this register and get [`statusb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusb`]
module"]
#[doc(alias = "STATUSB")]
pub type Statusb = crate::Reg<statusb::StatusbSpec>;
#[doc = "Peripheral write protection status - Bridge B"]
pub mod statusb;
#[doc = "STATUSC (r) register accessor: Peripheral write protection status - Bridge C\n\nYou can [`read`](crate::Reg::read) this register and get [`statusc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusc`]
module"]
#[doc(alias = "STATUSC")]
pub type Statusc = crate::Reg<statusc::StatuscSpec>;
#[doc = "Peripheral write protection status - Bridge C"]
pub mod statusc;
#[doc = "STATUSD (r) register accessor: Peripheral write protection status - Bridge D\n\nYou can [`read`](crate::Reg::read) this register and get [`statusd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusd`]
module"]
#[doc(alias = "STATUSD")]
pub type Statusd = crate::Reg<statusd::StatusdSpec>;
#[doc = "Peripheral write protection status - Bridge D"]
pub mod statusd;
