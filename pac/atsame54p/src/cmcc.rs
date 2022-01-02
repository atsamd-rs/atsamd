#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache Type Register"]
    pub type_: crate::Reg<type_::TYPE_SPEC>,
    #[doc = "0x04 - Cache Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x08 - Cache Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - Cache Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x10 - Cache Lock per Way Register"]
    pub lckway: crate::Reg<lckway::LCKWAY_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Cache Maintenance Register 0"]
    pub maint0: crate::Reg<maint0::MAINT0_SPEC>,
    #[doc = "0x24 - Cache Maintenance Register 1"]
    pub maint1: crate::Reg<maint1::MAINT1_SPEC>,
    #[doc = "0x28 - Cache Monitor Configuration Register"]
    pub mcfg: crate::Reg<mcfg::MCFG_SPEC>,
    #[doc = "0x2c - Cache Monitor Enable Register"]
    pub men: crate::Reg<men::MEN_SPEC>,
    #[doc = "0x30 - Cache Monitor Control Register"]
    pub mctrl: crate::Reg<mctrl::MCTRL_SPEC>,
    #[doc = "0x34 - Cache Monitor Status Register"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
}
#[doc = "TYPE register accessor: an alias for `Reg<TYPE_SPEC>`"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "Cache Type Register"]
pub mod type_;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Cache Configuration Register"]
pub mod cfg;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Cache Control Register"]
pub mod ctrl;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Cache Status Register"]
pub mod sr;
#[doc = "LCKWAY register accessor: an alias for `Reg<LCKWAY_SPEC>`"]
pub type LCKWAY = crate::Reg<lckway::LCKWAY_SPEC>;
#[doc = "Cache Lock per Way Register"]
pub mod lckway;
#[doc = "MAINT0 register accessor: an alias for `Reg<MAINT0_SPEC>`"]
pub type MAINT0 = crate::Reg<maint0::MAINT0_SPEC>;
#[doc = "Cache Maintenance Register 0"]
pub mod maint0;
#[doc = "MAINT1 register accessor: an alias for `Reg<MAINT1_SPEC>`"]
pub type MAINT1 = crate::Reg<maint1::MAINT1_SPEC>;
#[doc = "Cache Maintenance Register 1"]
pub mod maint1;
#[doc = "MCFG register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Cache Monitor Configuration Register"]
pub mod mcfg;
#[doc = "MEN register accessor: an alias for `Reg<MEN_SPEC>`"]
pub type MEN = crate::Reg<men::MEN_SPEC>;
#[doc = "Cache Monitor Enable Register"]
pub mod men;
#[doc = "MCTRL register accessor: an alias for `Reg<MCTRL_SPEC>`"]
pub type MCTRL = crate::Reg<mctrl::MCTRL_SPEC>;
#[doc = "Cache Monitor Control Register"]
pub mod mctrl;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Cache Monitor Status Register"]
pub mod msr;
