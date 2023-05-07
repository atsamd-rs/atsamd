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
#[doc = "TYPE (r) register accessor: an alias for `Reg<TYPE_SPEC>`"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "Cache Type Register"]
pub mod type_;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Cache Configuration Register"]
pub mod cfg;
#[doc = "CTRL (w) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Cache Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Cache Status Register"]
pub mod sr;
#[doc = "LCKWAY (rw) register accessor: an alias for `Reg<LCKWAY_SPEC>`"]
pub type LCKWAY = crate::Reg<lckway::LCKWAY_SPEC>;
#[doc = "Cache Lock per Way Register"]
pub mod lckway;
#[doc = "MAINT0 (w) register accessor: an alias for `Reg<MAINT0_SPEC>`"]
pub type MAINT0 = crate::Reg<maint0::MAINT0_SPEC>;
#[doc = "Cache Maintenance Register 0"]
pub mod maint0;
#[doc = "MAINT1 (w) register accessor: an alias for `Reg<MAINT1_SPEC>`"]
pub type MAINT1 = crate::Reg<maint1::MAINT1_SPEC>;
#[doc = "Cache Maintenance Register 1"]
pub mod maint1;
#[doc = "MCFG (rw) register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Cache Monitor Configuration Register"]
pub mod mcfg;
#[doc = "MEN (rw) register accessor: an alias for `Reg<MEN_SPEC>`"]
pub type MEN = crate::Reg<men::MEN_SPEC>;
#[doc = "Cache Monitor Enable Register"]
pub mod men;
#[doc = "MCTRL (w) register accessor: an alias for `Reg<MCTRL_SPEC>`"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Cache Monitor Control Register"]
pub mod mctrl;
#[doc = "MSR (r) register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Cache Monitor Status Register"]
pub mod msr;
