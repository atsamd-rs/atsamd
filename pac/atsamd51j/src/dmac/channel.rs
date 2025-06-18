#[repr(C)]
#[doc = "CHANNEL\\[%s\\]"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    chctrla: Chctrla,
    chctrlb: Chctrlb,
    chprilvl: Chprilvl,
    chevctrl: Chevctrl,
    _reserved4: [u8; 0x05],
    chintenclr: Chintenclr,
    chintenset: Chintenset,
    chintflag: Chintflag,
    chstatus: Chstatus,
}
impl Channel {
    #[doc = "0x00 - Channel n Control A"]
    #[inline(always)]
    pub const fn chctrla(&self) -> &Chctrla {
        &self.chctrla
    }
    #[doc = "0x04 - Channel n Control B"]
    #[inline(always)]
    pub const fn chctrlb(&self) -> &Chctrlb {
        &self.chctrlb
    }
    #[doc = "0x05 - Channel n Priority Level"]
    #[inline(always)]
    pub const fn chprilvl(&self) -> &Chprilvl {
        &self.chprilvl
    }
    #[doc = "0x06 - Channel n Event Control"]
    #[inline(always)]
    pub const fn chevctrl(&self) -> &Chevctrl {
        &self.chevctrl
    }
    #[doc = "0x0c - Channel n Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn chintenclr(&self) -> &Chintenclr {
        &self.chintenclr
    }
    #[doc = "0x0d - Channel n Interrupt Enable Set"]
    #[inline(always)]
    pub const fn chintenset(&self) -> &Chintenset {
        &self.chintenset
    }
    #[doc = "0x0e - Channel n Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn chintflag(&self) -> &Chintflag {
        &self.chintflag
    }
    #[doc = "0x0f - Channel n Status"]
    #[inline(always)]
    pub const fn chstatus(&self) -> &Chstatus {
        &self.chstatus
    }
}
#[doc = "CHCTRLA (rw) register accessor: Channel n Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctrla`] module"]
#[doc(alias = "CHCTRLA")]
pub type Chctrla = crate::Reg<chctrla::ChctrlaSpec>;
#[doc = "Channel n Control A"]
pub mod chctrla;
#[doc = "CHCTRLB (rw) register accessor: Channel n Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`chctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctrlb`] module"]
#[doc(alias = "CHCTRLB")]
pub type Chctrlb = crate::Reg<chctrlb::ChctrlbSpec>;
#[doc = "Channel n Control B"]
pub mod chctrlb;
#[doc = "CHPRILVL (rw) register accessor: Channel n Priority Level\n\nYou can [`read`](crate::Reg::read) this register and get [`chprilvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chprilvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chprilvl`] module"]
#[doc(alias = "CHPRILVL")]
pub type Chprilvl = crate::Reg<chprilvl::ChprilvlSpec>;
#[doc = "Channel n Priority Level"]
pub mod chprilvl;
#[doc = "CHEVCTRL (rw) register accessor: Channel n Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`chevctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chevctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chevctrl`] module"]
#[doc(alias = "CHEVCTRL")]
pub type Chevctrl = crate::Reg<chevctrl::ChevctrlSpec>;
#[doc = "Channel n Event Control"]
pub mod chevctrl;
#[doc = "CHINTENCLR (rw) register accessor: Channel n Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintenclr`] module"]
#[doc(alias = "CHINTENCLR")]
pub type Chintenclr = crate::Reg<chintenclr::ChintenclrSpec>;
#[doc = "Channel n Interrupt Enable Clear"]
pub mod chintenclr;
#[doc = "CHINTENSET (rw) register accessor: Channel n Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintenset`] module"]
#[doc(alias = "CHINTENSET")]
pub type Chintenset = crate::Reg<chintenset::ChintensetSpec>;
#[doc = "Channel n Interrupt Enable Set"]
pub mod chintenset;
#[doc = "CHINTFLAG (rw) register accessor: Channel n Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`chintflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chintflag`] module"]
#[doc(alias = "CHINTFLAG")]
pub type Chintflag = crate::Reg<chintflag::ChintflagSpec>;
#[doc = "Channel n Interrupt Flag Status and Clear"]
pub mod chintflag;
#[doc = "CHSTATUS (rw) register accessor: Channel n Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatus`] module"]
#[doc(alias = "CHSTATUS")]
pub type Chstatus = crate::Reg<chstatus::ChstatusSpec>;
#[doc = "Channel n Status"]
pub mod chstatus;
