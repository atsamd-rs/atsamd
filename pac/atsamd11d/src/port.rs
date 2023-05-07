#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Direction"]
    pub dir: [DIR; 1],
    #[doc = "0x04 - Data Direction Clear"]
    pub dirclr: [DIRCLR; 1],
    #[doc = "0x08 - Data Direction Set"]
    pub dirset: [DIRSET; 1],
    #[doc = "0x0c - Data Direction Toggle"]
    pub dirtgl: [DIRTGL; 1],
    #[doc = "0x10 - Data Output Value"]
    pub out: [OUT; 1],
    #[doc = "0x14 - Data Output Value Clear"]
    pub outclr: [OUTCLR; 1],
    #[doc = "0x18 - Data Output Value Set"]
    pub outset: [OUTSET; 1],
    #[doc = "0x1c - Data Output Value Toggle"]
    pub outtgl: [OUTTGL; 1],
    #[doc = "0x20 - Data Input Value"]
    pub in_: [IN; 1],
    #[doc = "0x24 - Control"]
    pub ctrl: [CTRL; 1],
    #[doc = "0x28 - Write Configuration"]
    pub wrconfig: [WRCONFIG; 1],
    _reserved11: [u8; 0x04],
    #[doc = "0x30..0x40 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_: [PMUX0_; 16],
    #[doc = "0x40..0x60 - Pin Configuration n - Group 0"]
    pub pincfg0_: [PINCFG0_; 32],
}
#[doc = "DIR (rw) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Data Direction"]
pub mod dir;
#[doc = "DIRCLR (rw) register accessor: an alias for `Reg<DIRCLR_SPEC>`"]
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
#[doc = "Data Direction Clear"]
pub mod dirclr;
#[doc = "DIRSET (rw) register accessor: an alias for `Reg<DIRSET_SPEC>`"]
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
#[doc = "Data Direction Set"]
pub mod dirset;
#[doc = "DIRTGL (rw) register accessor: an alias for `Reg<DIRTGL_SPEC>`"]
pub type DIRTGL = crate::Reg<dirtgl::DIRTGL_SPEC>;
#[doc = "Data Direction Toggle"]
pub mod dirtgl;
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Data Output Value"]
pub mod out;
#[doc = "OUTCLR (rw) register accessor: an alias for `Reg<OUTCLR_SPEC>`"]
pub type OUTCLR = crate::Reg<outclr::OUTCLR_SPEC>;
#[doc = "Data Output Value Clear"]
pub mod outclr;
#[doc = "OUTSET (rw) register accessor: an alias for `Reg<OUTSET_SPEC>`"]
pub type OUTSET = crate::Reg<outset::OUTSET_SPEC>;
#[doc = "Data Output Value Set"]
pub mod outset;
#[doc = "OUTTGL (rw) register accessor: an alias for `Reg<OUTTGL_SPEC>`"]
pub type OUTTGL = crate::Reg<outtgl::OUTTGL_SPEC>;
#[doc = "Data Output Value Toggle"]
pub mod outtgl;
#[doc = "IN (r) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Data Input Value"]
pub mod in_;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "WRCONFIG (w) register accessor: an alias for `Reg<WRCONFIG_SPEC>`"]
pub type WRCONFIG = crate::Reg<wrconfig::WRCONFIG_SPEC>;
#[doc = "Write Configuration"]
pub mod wrconfig;
#[doc = "PMUX0_ (rw) register accessor: an alias for `Reg<PMUX0__SPEC>`"]
pub type PMUX0_ = crate::Reg<pmux0_::PMUX0__SPEC>;
#[doc = "Peripheral Multiplexing n - Group 0"]
pub mod pmux0_;
#[doc = "PINCFG0_ (rw) register accessor: an alias for `Reg<PINCFG0__SPEC>`"]
pub type PINCFG0_ = crate::Reg<pincfg0_::PINCFG0__SPEC>;
#[doc = "Pin Configuration n - Group 0"]
pub mod pincfg0_;
