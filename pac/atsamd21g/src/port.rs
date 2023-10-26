#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Direction"]
    pub dir0: DIR,
    #[doc = "0x04 - Data Direction Clear"]
    pub dirclr0: DIRCLR,
    #[doc = "0x08 - Data Direction Set"]
    pub dirset0: DIRSET,
    #[doc = "0x0c - Data Direction Toggle"]
    pub dirtgl0: DIRTGL,
    #[doc = "0x10 - Data Output Value"]
    pub out0: OUT,
    #[doc = "0x14 - Data Output Value Clear"]
    pub outclr0: OUTCLR,
    #[doc = "0x18 - Data Output Value Set"]
    pub outset0: OUTSET,
    #[doc = "0x1c - Data Output Value Toggle"]
    pub outtgl0: OUTTGL,
    #[doc = "0x20 - Data Input Value"]
    pub in0: IN,
    #[doc = "0x24 - Control"]
    pub ctrl0: CTRL,
    #[doc = "0x28 - Write Configuration"]
    pub wrconfig0: WRCONFIG,
    _reserved11: [u8; 0x04],
    #[doc = "0x30..0x40 - Peripheral Multiplexing n - Group 0"]
    pub pmux0_: [PMUX0_; 16],
    #[doc = "0x40..0x60 - Pin Configuration n - Group 0"]
    pub pincfg0_: [PINCFG0_; 32],
    _reserved13: [u8; 0x20],
    #[doc = "0x80 - Data Direction"]
    pub dir1: DIR,
    #[doc = "0x84 - Data Direction Clear"]
    pub dirclr1: DIRCLR,
    #[doc = "0x88 - Data Direction Set"]
    pub dirset1: DIRSET,
    #[doc = "0x8c - Data Direction Toggle"]
    pub dirtgl1: DIRTGL,
    #[doc = "0x90 - Data Output Value"]
    pub out1: OUT,
    #[doc = "0x94 - Data Output Value Clear"]
    pub outclr1: OUTCLR,
    #[doc = "0x98 - Data Output Value Set"]
    pub outset1: OUTSET,
    #[doc = "0x9c - Data Output Value Toggle"]
    pub outtgl1: OUTTGL,
    #[doc = "0xa0 - Data Input Value"]
    pub in1: IN,
    #[doc = "0xa4 - Control"]
    pub ctrl1: CTRL,
    #[doc = "0xa8 - Write Configuration"]
    pub wrconfig1: WRCONFIG,
    _reserved24: [u8; 0x04],
    #[doc = "0xb0..0xc0 - Peripheral Multiplexing n - Group 1"]
    pub pmux1_0: [PMUX1_; 16],
    #[doc = "0xc0..0xe0 - Pin Configuration n - Group 1"]
    pub pincfg1_0: [PINCFG1_; 32],
}
#[doc = "DIR (rw) register accessor: Data Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`]
module"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Data Direction"]
pub mod dir;
#[doc = "DIRCLR (rw) register accessor: Data Direction Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirclr`]
module"]
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
#[doc = "Data Direction Clear"]
pub mod dirclr;
#[doc = "DIRSET (rw) register accessor: Data Direction Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirset`]
module"]
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
#[doc = "Data Direction Set"]
pub mod dirset;
#[doc = "DIRTGL (rw) register accessor: Data Direction Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirtgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirtgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dirtgl`]
module"]
pub type DIRTGL = crate::Reg<dirtgl::DIRTGL_SPEC>;
#[doc = "Data Direction Toggle"]
pub mod dirtgl;
#[doc = "OUT (rw) register accessor: Data Output Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Data Output Value"]
pub mod out;
#[doc = "OUTCLR (rw) register accessor: Data Output Value Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outclr`]
module"]
pub type OUTCLR = crate::Reg<outclr::OUTCLR_SPEC>;
#[doc = "Data Output Value Clear"]
pub mod outclr;
#[doc = "OUTSET (rw) register accessor: Data Output Value Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outset`]
module"]
pub type OUTSET = crate::Reg<outset::OUTSET_SPEC>;
#[doc = "Data Output Value Set"]
pub mod outset;
#[doc = "OUTTGL (rw) register accessor: Data Output Value Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outtgl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outtgl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outtgl`]
module"]
pub type OUTTGL = crate::Reg<outtgl::OUTTGL_SPEC>;
#[doc = "Data Output Value Toggle"]
pub mod outtgl;
#[doc = "IN (r) register accessor: Data Input Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Data Input Value"]
pub mod in_;
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "WRCONFIG (w) register accessor: Write Configuration\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrconfig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrconfig`]
module"]
pub type WRCONFIG = crate::Reg<wrconfig::WRCONFIG_SPEC>;
#[doc = "Write Configuration"]
pub mod wrconfig;
#[doc = "PMUX0_ (rw) register accessor: Peripheral Multiplexing n - Group 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmux0_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmux0_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmux0_`]
module"]
pub type PMUX0_ = crate::Reg<pmux0_::PMUX0__SPEC>;
#[doc = "Peripheral Multiplexing n - Group 0"]
pub mod pmux0_;
pub use pmux0_ as pmux1_;
pub use PMUX0_ as PMUX1_;
#[doc = "PINCFG0_ (rw) register accessor: Pin Configuration n - Group 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pincfg0_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pincfg0_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pincfg0_`]
module"]
pub type PINCFG0_ = crate::Reg<pincfg0_::PINCFG0__SPEC>;
#[doc = "Pin Configuration n - Group 0"]
pub mod pincfg0_;
pub use pincfg0_ as pincfg1_;
pub use PINCFG0_ as PINCFG1_;
