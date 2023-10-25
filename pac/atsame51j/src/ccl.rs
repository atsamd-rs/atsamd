#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - SEQ Control x"]
    pub seqctrl: [SEQCTRL; 2],
    _reserved2: [u8; 0x02],
    #[doc = "0x08..0x18 - LUT Control x"]
    pub lutctrl: [LUTCTRL; 4],
}
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "SEQCTRL (rw) register accessor: SEQ Control x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqctrl`]
module"]
pub type SEQCTRL = crate::Reg<seqctrl::SEQCTRL_SPEC>;
#[doc = "SEQ Control x"]
pub mod seqctrl;
#[doc = "LUTCTRL (rw) register accessor: LUT Control x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lutctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lutctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lutctrl`]
module"]
pub type LUTCTRL = crate::Reg<lutctrl::LUTCTRL_SPEC>;
#[doc = "LUT Control x"]
pub mod lutctrl;
