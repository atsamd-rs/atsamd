#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x03],
    swevt: Swevt,
    prictrl: Prictrl,
    _reserved3: [u8; 0x07],
    intpend: Intpend,
    _reserved4: [u8; 0x02],
    intstatus: Intstatus,
    busych: Busych,
    readyusr: Readyusr,
    channels: [Channels; 32],
    user: [User; 67],
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Software Event"]
    #[inline(always)]
    pub const fn swevt(&self) -> &Swevt {
        &self.swevt
    }
    #[doc = "0x08 - Priority Control"]
    #[inline(always)]
    pub const fn prictrl(&self) -> &Prictrl {
        &self.prictrl
    }
    #[doc = "0x10 - Channel Pending Interrupt"]
    #[inline(always)]
    pub const fn intpend(&self) -> &Intpend {
        &self.intpend
    }
    #[doc = "0x14 - Interrupt Status"]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        &self.intstatus
    }
    #[doc = "0x18 - Busy Channels"]
    #[inline(always)]
    pub const fn busych(&self) -> &Busych {
        &self.busych
    }
    #[doc = "0x1c - Ready Users"]
    #[inline(always)]
    pub const fn readyusr(&self) -> &Readyusr {
        &self.readyusr
    }
    #[doc = "0x20..0x120 - CHANNELS\\[%s\\]"]
    #[inline(always)]
    pub const fn channels(&self, n: usize) -> &Channels {
        &self.channels[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x120 - CHANNELS\\[%s\\]"]
    #[inline(always)]
    pub fn channels_iter(&self) -> impl Iterator<Item = &Channels> {
        self.channels.iter()
    }
    #[doc = "0x120..0x163 - User Multiplexer n"]
    #[inline(always)]
    pub const fn user(&self, n: usize) -> &User {
        &self.user[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x163 - User Multiplexer n"]
    #[inline(always)]
    pub fn user_iter(&self) -> impl Iterator<Item = &User> {
        self.user.iter()
    }
}
#[doc = "CTRLA (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "SWEVT (w) register accessor: Software Event\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevt::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swevt`]
module"]
#[doc(alias = "SWEVT")]
pub type Swevt = crate::Reg<swevt::SwevtSpec>;
#[doc = "Software Event"]
pub mod swevt;
#[doc = "PRICTRL (rw) register accessor: Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`prictrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prictrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prictrl`]
module"]
#[doc(alias = "PRICTRL")]
pub type Prictrl = crate::Reg<prictrl::PrictrlSpec>;
#[doc = "Priority Control"]
pub mod prictrl;
#[doc = "INTPEND (rw) register accessor: Channel Pending Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intpend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend`]
module"]
#[doc(alias = "INTPEND")]
pub type Intpend = crate::Reg<intpend::IntpendSpec>;
#[doc = "Channel Pending Interrupt"]
pub mod intpend;
#[doc = "INTSTATUS (r) register accessor: Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
#[doc(alias = "INTSTATUS")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "BUSYCH (r) register accessor: Busy Channels\n\nYou can [`read`](crate::Reg::read) this register and get [`busych::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busych`]
module"]
#[doc(alias = "BUSYCH")]
pub type Busych = crate::Reg<busych::BusychSpec>;
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "READYUSR (r) register accessor: Ready Users\n\nYou can [`read`](crate::Reg::read) this register and get [`readyusr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readyusr`]
module"]
#[doc(alias = "READYUSR")]
pub type Readyusr = crate::Reg<readyusr::ReadyusrSpec>;
#[doc = "Ready Users"]
pub mod readyusr;
#[doc = "CHANNELS\\[%s\\]"]
pub use self::channels::Channels;
#[doc = r"Cluster"]
#[doc = "CHANNELS\\[%s\\]"]
pub mod channels;
#[doc = "USER (rw) register accessor: User Multiplexer n\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@user`]
module"]
#[doc(alias = "USER")]
pub type User = crate::Reg<user::UserSpec>;
#[doc = "User Multiplexer n"]
pub mod user;
