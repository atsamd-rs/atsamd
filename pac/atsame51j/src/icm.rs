#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x08 - Status"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Interrupt Enable"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x14 - Interrupt Disable"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x18 - Interrupt Mask"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x1c - Interrupt Status"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x20 - Undefined Access Status"]
    pub uasr: crate::Reg<uasr::UASR_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - Region Descriptor Area Start Address"]
    pub dscr: crate::Reg<dscr::DSCR_SPEC>,
    #[doc = "0x34 - Region Hash Area Start Address"]
    pub hash: crate::Reg<hash::HASH_SPEC>,
    #[doc = "0x38..0x58 - User Initial Hash Value n"]
    pub uihval: [crate::Reg<uihval::UIHVAL_SPEC>; 8],
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration"]
pub mod cfg;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status"]
pub mod sr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status"]
pub mod isr;
#[doc = "UASR register accessor: an alias for `Reg<UASR_SPEC>`"]
pub type UASR = crate::Reg<uasr::UASR_SPEC>;
#[doc = "Undefined Access Status"]
pub mod uasr;
#[doc = "DSCR register accessor: an alias for `Reg<DSCR_SPEC>`"]
pub type DSCR = crate::Reg<dscr::DSCR_SPEC>;
#[doc = "Region Descriptor Area Start Address"]
pub mod dscr;
#[doc = "HASH register accessor: an alias for `Reg<HASH_SPEC>`"]
pub type HASH = crate::Reg<hash::HASH_SPEC>;
#[doc = "Region Hash Area Start Address"]
pub mod hash;
#[doc = "UIHVAL register accessor: an alias for `Reg<UIHVAL_SPEC>`"]
pub type UIHVAL = crate::Reg<uihval::UIHVAL_SPEC>;
#[doc = "User Initial Hash Value n"]
pub mod uihval;
