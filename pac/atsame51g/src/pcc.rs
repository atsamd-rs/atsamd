#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x08 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x0c - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x10 - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x14 - Reception Holding Register"]
    pub rhr: crate::Reg<rhr::RHR_SPEC>,
    _reserved6: [u8; 0xc8],
    #[doc = "0xe0 - Write Protection Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe4 - Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
}
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "RHR register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Reception Holding Register"]
pub mod rhr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
