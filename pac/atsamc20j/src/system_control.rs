#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0d00],
    #[doc = "0xd00 - CPUID Base Register"]
    pub cpuid: crate::Reg<cpuid::CPUID_SPEC>,
    #[doc = "0xd04 - Interrupt Control and State Register"]
    pub icsr: crate::Reg<icsr::ICSR_SPEC>,
    #[doc = "0xd08 - Vector Table Offset Register"]
    pub vtor: crate::Reg<vtor::VTOR_SPEC>,
    #[doc = "0xd0c - Application Interrupt and Reset Control Register"]
    pub aircr: crate::Reg<aircr::AIRCR_SPEC>,
    #[doc = "0xd10 - System Control Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0xd14 - Configuration and Control Register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: crate::Reg<shpr2::SHPR2_SPEC>,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: crate::Reg<shpr3::SHPR3_SPEC>,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: crate::Reg<shcsr::SHCSR_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0xd30 - Debug Fault Status Register"]
    pub dfsr: crate::Reg<dfsr::DFSR_SPEC>,
}
#[doc = "CPUID register accessor: an alias for `Reg<CPUID_SPEC>`"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "CPUID Base Register"]
pub mod cpuid;
#[doc = "ICSR register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "Interrupt Control and State Register"]
pub mod icsr;
#[doc = "VTOR register accessor: an alias for `Reg<VTOR_SPEC>`"]
pub type VTOR = crate::Reg<vtor::VTOR_SPEC>;
#[doc = "Vector Table Offset Register"]
pub mod vtor;
#[doc = "AIRCR register accessor: an alias for `Reg<AIRCR_SPEC>`"]
pub type AIRCR = crate::Reg<aircr::AIRCR_SPEC>;
#[doc = "Application Interrupt and Reset Control Register"]
pub mod aircr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "System Control Register"]
pub mod scr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Configuration and Control Register"]
pub mod ccr;
#[doc = "SHPR2 register accessor: an alias for `Reg<SHPR2_SPEC>`"]
pub type SHPR2 = crate::Reg<shpr2::SHPR2_SPEC>;
#[doc = "System Handler Priority Register 2"]
pub mod shpr2;
#[doc = "SHPR3 register accessor: an alias for `Reg<SHPR3_SPEC>`"]
pub type SHPR3 = crate::Reg<shpr3::SHPR3_SPEC>;
#[doc = "System Handler Priority Register 3"]
pub mod shpr3;
#[doc = "SHCSR register accessor: an alias for `Reg<SHCSR_SPEC>`"]
pub type SHCSR = crate::Reg<shcsr::SHCSR_SPEC>;
#[doc = "System Handler Control and State Register"]
pub mod shcsr;
#[doc = "DFSR register accessor: an alias for `Reg<DFSR_SPEC>`"]
pub type DFSR = crate::Reg<dfsr::DFSR_SPEC>;
#[doc = "Debug Fault Status Register"]
pub mod dfsr;
