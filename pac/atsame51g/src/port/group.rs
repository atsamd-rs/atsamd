#[doc = r"Register block"]
#[repr(C)]
pub struct GROUP {
    #[doc = "0x00 - Data Direction"]
    pub dir: DIR,
    #[doc = "0x04 - Data Direction Clear"]
    pub dirclr: DIRCLR,
    #[doc = "0x08 - Data Direction Set"]
    pub dirset: DIRSET,
    #[doc = "0x0c - Data Direction Toggle"]
    pub dirtgl: DIRTGL,
    #[doc = "0x10 - Data Output Value"]
    pub out: OUT,
    #[doc = "0x14 - Data Output Value Clear"]
    pub outclr: OUTCLR,
    #[doc = "0x18 - Data Output Value Set"]
    pub outset: OUTSET,
    #[doc = "0x1c - Data Output Value Toggle"]
    pub outtgl: OUTTGL,
    #[doc = "0x20 - Data Input Value"]
    pub in_: IN,
    #[doc = "0x24 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x28 - Write Configuration"]
    pub wrconfig: WRCONFIG,
    #[doc = "0x2c - Event Input Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x30..0x40 - Peripheral Multiplexing"]
    pub pmux: [PMUX; 16],
    #[doc = "0x40..0x60 - Pin Configuration"]
    pub pincfg: [PINCFG; 32],
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
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Input Control"]
pub mod evctrl;
#[doc = "PMUX (rw) register accessor: an alias for `Reg<PMUX_SPEC>`"]
pub type PMUX = crate::Reg<pmux::PMUX_SPEC>;
#[doc = "Peripheral Multiplexing"]
pub mod pmux;
#[doc = "PINCFG (rw) register accessor: an alias for `Reg<PINCFG_SPEC>`"]
pub type PINCFG = crate::Reg<pincfg::PINCFG_SPEC>;
#[doc = "Pin Configuration"]
pub mod pincfg;
