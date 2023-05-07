#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Status"]
    pub status: STATUS,
    #[doc = "0x02 - Generic Clock Control"]
    pub clkctrl: CLKCTRL,
    #[doc = "0x04 - Generic Clock Generator Control"]
    pub genctrl: GENCTRL,
    #[doc = "0x08 - Generic Clock Generator Division"]
    pub gendiv: GENDIV,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "CLKCTRL (rw) register accessor: an alias for `Reg<CLKCTRL_SPEC>`"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Generic Clock Control"]
pub mod clkctrl;
#[doc = "GENCTRL (rw) register accessor: an alias for `Reg<GENCTRL_SPEC>`"]
pub type GENCTRL = crate::Reg<genctrl::GENCTRL_SPEC>;
#[doc = "Generic Clock Generator Control"]
pub mod genctrl;
#[doc = "GENDIV (rw) register accessor: an alias for `Reg<GENDIV_SPEC>`"]
pub type GENDIV = crate::Reg<gendiv::GENDIV_SPEC>;
#[doc = "Generic Clock Generator Division"]
pub mod gendiv;
