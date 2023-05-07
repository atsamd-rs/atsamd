#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x08 - Status"]
    pub sr: SR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Interrupt Enable"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask"]
    pub imr: IMR,
    #[doc = "0x1c - Interrupt Status"]
    pub isr: ISR,
    #[doc = "0x20 - Undefined Access Status"]
    pub uasr: UASR,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - Region Descriptor Area Start Address"]
    pub dscr: DSCR,
    #[doc = "0x34 - Region Hash Area Start Address"]
    pub hash: HASH,
    #[doc = "0x38..0x58 - User Initial Hash Value n"]
    pub uihval: [UIHVAL; 8],
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration"]
pub mod cfg;
#[doc = "CTRL (w) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status"]
pub mod sr;
#[doc = "IER (w) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "IDR (w) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable"]
pub mod idr;
#[doc = "IMR (r) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask"]
pub mod imr;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status"]
pub mod isr;
#[doc = "UASR (r) register accessor: an alias for `Reg<UASR_SPEC>`"]
pub type UASR = crate::Reg<uasr::UASR_SPEC>;
#[doc = "Undefined Access Status"]
pub mod uasr;
#[doc = "DSCR (rw) register accessor: an alias for `Reg<DSCR_SPEC>`"]
pub type DSCR = crate::Reg<dscr::DSCR_SPEC>;
#[doc = "Region Descriptor Area Start Address"]
pub mod dscr;
#[doc = "HASH (rw) register accessor: an alias for `Reg<HASH_SPEC>`"]
pub type HASH = crate::Reg<hash::HASH_SPEC>;
#[doc = "Region Hash Area Start Address"]
pub mod hash;
#[doc = "UIHVAL (w) register accessor: an alias for `Reg<UIHVAL_SPEC>`"]
pub type UIHVAL = crate::Reg<uihval::UIHVAL_SPEC>;
#[doc = "User Initial Hash Value n"]
pub mod uihval;
