#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache Type Register"]
    pub type_: TYPE,
    #[doc = "0x04 - Cache Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x08 - Cache Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Cache Status Register"]
    pub sr: SR,
    #[doc = "0x10 - Cache Lock per Way Register"]
    pub lckway: LCKWAY,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Cache Maintenance Register 0"]
    pub maint0: MAINT0,
    #[doc = "0x24 - Cache Maintenance Register 1"]
    pub maint1: MAINT1,
    #[doc = "0x28 - Cache Monitor Configuration Register"]
    pub mcfg: MCFG,
    #[doc = "0x2c - Cache Monitor Enable Register"]
    pub men: MEN,
    #[doc = "0x30 - Cache Monitor Control Register"]
    pub mctrl: MCTRL,
    #[doc = "0x34 - Cache Monitor Status Register"]
    pub msr: MSR,
}
#[doc = "TYPE (r) register accessor: Cache Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type_`]
module"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "Cache Type Register"]
pub mod type_;
#[doc = "CFG (rw) register accessor: Cache Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Cache Configuration Register"]
pub mod cfg;
#[doc = "CTRL (w) register accessor: Cache Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Cache Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: Cache Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Cache Status Register"]
pub mod sr;
#[doc = "LCKWAY (rw) register accessor: Cache Lock per Way Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckway::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckway::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckway`]
module"]
pub type LCKWAY = crate::Reg<lckway::LCKWAY_SPEC>;
#[doc = "Cache Lock per Way Register"]
pub mod lckway;
#[doc = "MAINT0 (w) register accessor: Cache Maintenance Register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maint0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maint0`]
module"]
pub type MAINT0 = crate::Reg<maint0::MAINT0_SPEC>;
#[doc = "Cache Maintenance Register 0"]
pub mod maint0;
#[doc = "MAINT1 (w) register accessor: Cache Maintenance Register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maint1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maint1`]
module"]
pub type MAINT1 = crate::Reg<maint1::MAINT1_SPEC>;
#[doc = "Cache Maintenance Register 1"]
pub mod maint1;
#[doc = "MCFG (rw) register accessor: Cache Monitor Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg`]
module"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Cache Monitor Configuration Register"]
pub mod mcfg;
#[doc = "MEN (rw) register accessor: Cache Monitor Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`men::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`men::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@men`]
module"]
pub type MEN = crate::Reg<men::MEN_SPEC>;
#[doc = "Cache Monitor Enable Register"]
pub mod men;
#[doc = "MCTRL (w) register accessor: Cache Monitor Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctrl`]
module"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Cache Monitor Control Register"]
pub mod mctrl;
#[doc = "MSR (r) register accessor: Cache Monitor Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Cache Monitor Status Register"]
pub mod msr;
