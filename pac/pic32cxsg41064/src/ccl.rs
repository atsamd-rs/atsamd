#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x03],
    seqctrl: [Seqctrl; 2],
    _reserved2: [u8; 0x02],
    lutctrl: [Lutctrl; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - SEQ Control x"]
    #[inline(always)]
    pub const fn seqctrl(&self, n: usize) -> &Seqctrl {
        &self.seqctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04 - SEQ Control x"]
    #[inline(always)]
    pub fn seqctrl_iter(&self) -> impl Iterator<Item = &Seqctrl> {
        self.seqctrl.iter()
    }
    #[doc = "0x08..0x18 - LUT Control x"]
    #[inline(always)]
    pub const fn lutctrl(&self, n: usize) -> &Lutctrl {
        &self.lutctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x18 - LUT Control x"]
    #[inline(always)]
    pub fn lutctrl_iter(&self) -> impl Iterator<Item = &Lutctrl> {
        self.lutctrl.iter()
    }
}
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "SEQCTRL (rw) register accessor: SEQ Control x\n\nYou can [`read`](crate::Reg::read) this register and get [`seqctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqctrl`]
module"]
#[doc(alias = "SEQCTRL")]
pub type Seqctrl = crate::Reg<seqctrl::SeqctrlSpec>;
#[doc = "SEQ Control x"]
pub mod seqctrl;
#[doc = "LUTCTRL (rw) register accessor: LUT Control x\n\nYou can [`read`](crate::Reg::read) this register and get [`lutctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lutctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lutctrl`]
module"]
#[doc(alias = "LUTCTRL")]
pub type Lutctrl = crate::Reg<lutctrl::LutctrlSpec>;
#[doc = "LUT Control x"]
pub mod lutctrl;
