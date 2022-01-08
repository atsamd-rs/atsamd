#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Interrupt Controller Type Register"]
    pub ictr: crate::Reg<ictr::ICTR_SPEC>,
    #[doc = "0x08 - Auxiliary Control Register"]
    pub actlr: crate::Reg<actlr::ACTLR_SPEC>,
    _reserved2: [u8; 0x0cf4],
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
    #[doc = "0xd18 - System Handler Priority Register 1"]
    pub shpr1: crate::Reg<shpr1::SHPR1_SPEC>,
    #[doc = "0xd1c - System Handler Priority Register 2"]
    pub shpr2: crate::Reg<shpr2::SHPR2_SPEC>,
    #[doc = "0xd20 - System Handler Priority Register 3"]
    pub shpr3: crate::Reg<shpr3::SHPR3_SPEC>,
    #[doc = "0xd24 - System Handler Control and State Register"]
    pub shcsr: crate::Reg<shcsr::SHCSR_SPEC>,
    #[doc = "0xd28 - Configurable Fault Status Register"]
    pub cfsr: crate::Reg<cfsr::CFSR_SPEC>,
    #[doc = "0xd2c - HardFault Status Register"]
    pub hfsr: crate::Reg<hfsr::HFSR_SPEC>,
    #[doc = "0xd30 - Debug Fault Status Register"]
    pub dfsr: crate::Reg<dfsr::DFSR_SPEC>,
    #[doc = "0xd34 - MemManage Fault Address Register"]
    pub mmfar: crate::Reg<mmfar::MMFAR_SPEC>,
    #[doc = "0xd38 - BusFault Address Register"]
    pub bfar: crate::Reg<bfar::BFAR_SPEC>,
    #[doc = "0xd3c - Auxiliary Fault Status Register"]
    pub afsr: crate::Reg<afsr::AFSR_SPEC>,
    #[doc = "0xd40..0xd48 - Processor Feature Register"]
    pub pfr: [crate::Reg<pfr::PFR_SPEC>; 2],
    #[doc = "0xd48 - Debug Feature Register"]
    pub dfr: crate::Reg<dfr::DFR_SPEC>,
    #[doc = "0xd4c - Auxiliary Feature Register"]
    pub adr: crate::Reg<adr::ADR_SPEC>,
    #[doc = "0xd50..0xd60 - Memory Model Feature Register"]
    pub mmfr: [crate::Reg<mmfr::MMFR_SPEC>; 4],
    #[doc = "0xd60..0xd74 - Instruction Set Attributes Register"]
    pub isar: [crate::Reg<isar::ISAR_SPEC>; 5],
    _reserved23: [u8; 0x14],
    #[doc = "0xd88 - Coprocessor Access Control Register"]
    pub cpacr: crate::Reg<cpacr::CPACR_SPEC>,
}
#[doc = "ICTR register accessor: an alias for `Reg<ICTR_SPEC>`"]
pub type ICTR = crate::Reg<ictr::ICTR_SPEC>;
#[doc = "Interrupt Controller Type Register"]
pub mod ictr;
#[doc = "ACTLR register accessor: an alias for `Reg<ACTLR_SPEC>`"]
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
#[doc = "Auxiliary Control Register"]
pub mod actlr;
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
#[doc = "SHPR1 register accessor: an alias for `Reg<SHPR1_SPEC>`"]
pub type SHPR1 = crate::Reg<shpr1::SHPR1_SPEC>;
#[doc = "System Handler Priority Register 1"]
pub mod shpr1;
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
#[doc = "CFSR register accessor: an alias for `Reg<CFSR_SPEC>`"]
pub type CFSR = crate::Reg<cfsr::CFSR_SPEC>;
#[doc = "Configurable Fault Status Register"]
pub mod cfsr;
#[doc = "HFSR register accessor: an alias for `Reg<HFSR_SPEC>`"]
pub type HFSR = crate::Reg<hfsr::HFSR_SPEC>;
#[doc = "HardFault Status Register"]
pub mod hfsr;
#[doc = "DFSR register accessor: an alias for `Reg<DFSR_SPEC>`"]
pub type DFSR = crate::Reg<dfsr::DFSR_SPEC>;
#[doc = "Debug Fault Status Register"]
pub mod dfsr;
#[doc = "MMFAR register accessor: an alias for `Reg<MMFAR_SPEC>`"]
pub type MMFAR = crate::Reg<mmfar::MMFAR_SPEC>;
#[doc = "MemManage Fault Address Register"]
pub mod mmfar;
#[doc = "BFAR register accessor: an alias for `Reg<BFAR_SPEC>`"]
pub type BFAR = crate::Reg<bfar::BFAR_SPEC>;
#[doc = "BusFault Address Register"]
pub mod bfar;
#[doc = "AFSR register accessor: an alias for `Reg<AFSR_SPEC>`"]
pub type AFSR = crate::Reg<afsr::AFSR_SPEC>;
#[doc = "Auxiliary Fault Status Register"]
pub mod afsr;
#[doc = "PFR register accessor: an alias for `Reg<PFR_SPEC>`"]
pub type PFR = crate::Reg<pfr::PFR_SPEC>;
#[doc = "Processor Feature Register"]
pub mod pfr;
#[doc = "DFR register accessor: an alias for `Reg<DFR_SPEC>`"]
pub type DFR = crate::Reg<dfr::DFR_SPEC>;
#[doc = "Debug Feature Register"]
pub mod dfr;
#[doc = "ADR register accessor: an alias for `Reg<ADR_SPEC>`"]
pub type ADR = crate::Reg<adr::ADR_SPEC>;
#[doc = "Auxiliary Feature Register"]
pub mod adr;
#[doc = "MMFR register accessor: an alias for `Reg<MMFR_SPEC>`"]
pub type MMFR = crate::Reg<mmfr::MMFR_SPEC>;
#[doc = "Memory Model Feature Register"]
pub mod mmfr;
#[doc = "ISAR register accessor: an alias for `Reg<ISAR_SPEC>`"]
pub type ISAR = crate::Reg<isar::ISAR_SPEC>;
#[doc = "Instruction Set Attributes Register"]
pub mod isar;
#[doc = "CPACR register accessor: an alias for `Reg<CPACR_SPEC>`"]
pub type CPACR = crate::Reg<cpacr::CPACR_SPEC>;
#[doc = "Coprocessor Access Control Register"]
pub mod cpacr;
