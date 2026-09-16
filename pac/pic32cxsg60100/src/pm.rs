#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    sleepcfg: Sleepcfg,
    _reserved2: [u8; 0x02],
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    _reserved5: [u8; 0x01],
    stdbycfg: Stdbycfg,
    hibcfg: Hibcfg,
    bkupcfg: Bkupcfg,
    _reserved8: [u8; 0x07],
    pwsakdly: Pwsakdly,
}
impl RegisterBlock {
    #[doc = "0x00 - Control A"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x01 - Sleep Configuration"]
    #[inline(always)]
    pub const fn sleepcfg(&self) -> &Sleepcfg {
        &self.sleepcfg
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
    #[doc = "0x08 - Standby Configuration"]
    #[inline(always)]
    pub const fn stdbycfg(&self) -> &Stdbycfg {
        &self.stdbycfg
    }
    #[doc = "0x09 - Hibernate Configuration"]
    #[inline(always)]
    pub const fn hibcfg(&self) -> &Hibcfg {
        &self.hibcfg
    }
    #[doc = "0x0a - Backup Configuration"]
    #[inline(always)]
    pub const fn bkupcfg(&self) -> &Bkupcfg {
        &self.bkupcfg
    }
    #[doc = "0x12 - Power Switch Acknowledge Delay"]
    #[inline(always)]
    pub const fn pwsakdly(&self) -> &Pwsakdly {
        &self.pwsakdly
    }
}
#[doc = "CTRLA (rw) register accessor: Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "SLEEPCFG (rw) register accessor: Sleep Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sleepcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleepcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepcfg`]
module"]
#[doc(alias = "SLEEPCFG")]
pub type Sleepcfg = crate::Reg<sleepcfg::SleepcfgSpec>;
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
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
#[doc = "STDBYCFG (rw) register accessor: Standby Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`stdbycfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stdbycfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stdbycfg`]
module"]
#[doc(alias = "STDBYCFG")]
pub type Stdbycfg = crate::Reg<stdbycfg::StdbycfgSpec>;
#[doc = "Standby Configuration"]
pub mod stdbycfg;
#[doc = "HIBCFG (rw) register accessor: Hibernate Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hibcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hibcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hibcfg`]
module"]
#[doc(alias = "HIBCFG")]
pub type Hibcfg = crate::Reg<hibcfg::HibcfgSpec>;
#[doc = "Hibernate Configuration"]
pub mod hibcfg;
#[doc = "BKUPCFG (rw) register accessor: Backup Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bkupcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkupcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bkupcfg`]
module"]
#[doc(alias = "BKUPCFG")]
pub type Bkupcfg = crate::Reg<bkupcfg::BkupcfgSpec>;
#[doc = "Backup Configuration"]
pub mod bkupcfg;
#[doc = "PWSAKDLY (rw) register accessor: Power Switch Acknowledge Delay\n\nYou can [`read`](crate::Reg::read) this register and get [`pwsakdly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwsakdly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwsakdly`]
module"]
#[doc(alias = "PWSAKDLY")]
pub type Pwsakdly = crate::Reg<pwsakdly::PwsakdlySpec>;
#[doc = "Power Switch Acknowledge Delay"]
pub mod pwsakdly;
