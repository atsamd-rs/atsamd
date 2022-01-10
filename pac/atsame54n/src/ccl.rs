#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - SEQ Control x"]
    pub seqctrl: [crate::Reg<seqctrl::SEQCTRL_SPEC>; 2],
    _reserved2: [u8; 0x02],
    #[doc = "0x08..0x18 - LUT Control x"]
    pub lutctrl: [crate::Reg<lutctrl::LUTCTRL_SPEC>; 4],
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "SEQCTRL register accessor: an alias for `Reg<SEQCTRL_SPEC>`"]
pub type SEQCTRL = crate::Reg<seqctrl::SEQCTRL_SPEC>;
#[doc = "SEQ Control x"]
pub mod seqctrl;
#[doc = "LUTCTRL register accessor: an alias for `Reg<LUTCTRL_SPEC>`"]
pub type LUTCTRL = crate::Reg<lutctrl::LUTCTRL_SPEC>;
#[doc = "LUT Control x"]
pub mod lutctrl;
