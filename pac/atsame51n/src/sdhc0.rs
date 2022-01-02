#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ssar: [u8; 0x04],
    #[doc = "0x04 - Block Size"]
    pub bsr: crate::Reg<bsr::BSR_SPEC>,
    #[doc = "0x06 - Block Count"]
    pub bcr: crate::Reg<bcr::BCR_SPEC>,
    #[doc = "0x08 - Argument 1"]
    pub arg1r: crate::Reg<arg1r::ARG1R_SPEC>,
    #[doc = "0x0c - Transfer Mode"]
    pub tmr: crate::Reg<tmr::TMR_SPEC>,
    #[doc = "0x0e - Command"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x10..0x20 - Response"]
    pub rr: [crate::Reg<rr::RR_SPEC>; 4],
    #[doc = "0x20 - Buffer Data Port"]
    pub bdpr: crate::Reg<bdpr::BDPR_SPEC>,
    #[doc = "0x24 - Present State"]
    pub psr: crate::Reg<psr::PSR_SPEC>,
    _reserved_9_hc1: [u8; 0x01],
    #[doc = "0x29 - Power Control"]
    pub pcr: crate::Reg<pcr::PCR_SPEC>,
    _reserved_11_bgcr: [u8; 0x01],
    #[doc = "0x2b - Wakeup Control"]
    pub wcr: crate::Reg<wcr::WCR_SPEC>,
    #[doc = "0x2c - Clock Control"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x2e - Timeout Control"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x2f - Software Reset"]
    pub srr: crate::Reg<srr::SRR_SPEC>,
    _reserved_16_nistr: [u8; 0x02],
    _reserved_17_eistr: [u8; 0x02],
    _reserved_18_nister: [u8; 0x02],
    _reserved_19_eister: [u8; 0x02],
    _reserved_20_nisier: [u8; 0x02],
    _reserved_21_eisier: [u8; 0x02],
    #[doc = "0x3c - Auto CMD Error Status"]
    pub acesr: crate::Reg<acesr::ACESR_SPEC>,
    _reserved_23_hc2: [u8; 0x02],
    #[doc = "0x40 - Capabilities 0"]
    pub ca0r: crate::Reg<ca0r::CA0R_SPEC>,
    #[doc = "0x44 - Capabilities 1"]
    pub ca1r: crate::Reg<ca1r::CA1R_SPEC>,
    #[doc = "0x48 - Maximum Current Capabilities"]
    pub mccar: crate::Reg<mccar::MCCAR_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x50 - Force Event for Auto CMD Error Status"]
    pub feraces: crate::Reg<feraces::FERACES_SPEC>,
    #[doc = "0x52 - Force Event for Error Interrupt Status"]
    pub fereis: crate::Reg<fereis::FEREIS_SPEC>,
    #[doc = "0x54 - ADMA Error Status"]
    pub aesr: crate::Reg<aesr::AESR_SPEC>,
    _reserved30: [u8; 0x03],
    #[doc = "0x58 - ADMA System Address n"]
    pub asar: [crate::Reg<asar::ASAR_SPEC>; 1],
    _reserved31: [u8; 0x04],
    #[doc = "0x60..0x70 - Preset Value n"]
    pub pvr: [crate::Reg<pvr::PVR_SPEC>; 8],
    _reserved32: [u8; 0x8c],
    #[doc = "0xfc - Slot Interrupt Status"]
    pub sisr: crate::Reg<sisr::SISR_SPEC>,
    #[doc = "0xfe - Host Controller Version"]
    pub hcvr: crate::Reg<hcvr::HCVR_SPEC>,
    _reserved34: [u8; 0x0104],
    #[doc = "0x204 - MMC Control 1"]
    pub mc1r: crate::Reg<mc1r::MC1R_SPEC>,
    #[doc = "0x205 - MMC Control 2"]
    pub mc2r: crate::Reg<mc2r::MC2R_SPEC>,
    _reserved36: [u8; 0x02],
    #[doc = "0x208 - AHB Control"]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    #[doc = "0x20c - Clock Control 2"]
    pub cc2r: crate::Reg<cc2r::CC2R_SPEC>,
    _reserved38: [u8; 0x20],
    #[doc = "0x230 - Capabilities Control"]
    pub cacr: crate::Reg<cacr::CACR_SPEC>,
    #[doc = "0x234 - Debug"]
    pub dbgr: crate::Reg<dbgr::DBGR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub fn ssar_cmd23_mode(&self) -> &crate::Reg<ssar_cmd23_mode::SSAR_CMD23_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<ssar_cmd23_mode::SSAR_CMD23_MODE_SPEC>)
        }
    }
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub fn ssar(&self) -> &crate::Reg<ssar::SSAR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<ssar::SSAR_SPEC>)
        }
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub fn hc1r_emmc_mode(&self) -> &crate::Reg<hc1r_emmc_mode::HC1R_EMMC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<hc1r_emmc_mode::HC1R_EMMC_MODE_SPEC>)
        }
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub fn hc1r(&self) -> &crate::Reg<hc1r::HC1R_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<hc1r::HC1R_SPEC>)
        }
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub fn bgcr_emmc_mode(&self) -> &crate::Reg<bgcr_emmc_mode::BGCR_EMMC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(42usize)
                as *const crate::Reg<bgcr_emmc_mode::BGCR_EMMC_MODE_SPEC>)
        }
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub fn bgcr(&self) -> &crate::Reg<bgcr::BGCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(42usize)
                as *const crate::Reg<bgcr::BGCR_SPEC>)
        }
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub fn nistr_emmc_mode(&self) -> &crate::Reg<nistr_emmc_mode::NISTR_EMMC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<nistr_emmc_mode::NISTR_EMMC_MODE_SPEC>)
        }
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub fn nistr(&self) -> &crate::Reg<nistr::NISTR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(48usize)
                as *const crate::Reg<nistr::NISTR_SPEC>)
        }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub fn eistr_emmc_mode(&self) -> &crate::Reg<eistr_emmc_mode::EISTR_EMMC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(50usize)
                as *const crate::Reg<eistr_emmc_mode::EISTR_EMMC_MODE_SPEC>)
        }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub fn eistr(&self) -> &crate::Reg<eistr::EISTR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(50usize)
                as *const crate::Reg<eistr::EISTR_SPEC>)
        }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub fn nister_emmc_mode(&self) -> &crate::Reg<nister_emmc_mode::NISTER_EMMC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<nister_emmc_mode::NISTER_EMMC_MODE_SPEC>)
        }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub fn nister(&self) -> &crate::Reg<nister::NISTER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(52usize)
                as *const crate::Reg<nister::NISTER_SPEC>)
        }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub fn eister_emmc_mode(&self) -> &crate::Reg<eister_emmc_mode::EISTER_EMMC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(54usize)
                as *const crate::Reg<eister_emmc_mode::EISTER_EMMC_MODE_SPEC>)
        }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub fn eister(&self) -> &crate::Reg<eister::EISTER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(54usize)
                as *const crate::Reg<eister::EISTER_SPEC>)
        }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub fn nisier_emmc_mode(&self) -> &crate::Reg<nisier_emmc_mode::NISIER_EMMC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<nisier_emmc_mode::NISIER_EMMC_MODE_SPEC>)
        }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub fn nisier(&self) -> &crate::Reg<nisier::NISIER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(56usize)
                as *const crate::Reg<nisier::NISIER_SPEC>)
        }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub fn eisier_emmc_mode(&self) -> &crate::Reg<eisier_emmc_mode::EISIER_EMMC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(58usize)
                as *const crate::Reg<eisier_emmc_mode::EISIER_EMMC_MODE_SPEC>)
        }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub fn eisier(&self) -> &crate::Reg<eisier::EISIER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(58usize)
                as *const crate::Reg<eisier::EISIER_SPEC>)
        }
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub fn hc2r_emmc_mode(&self) -> &crate::Reg<hc2r_emmc_mode::HC2R_EMMC_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(62usize)
                as *const crate::Reg<hc2r_emmc_mode::HC2R_EMMC_MODE_SPEC>)
        }
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub fn hc2r(&self) -> &crate::Reg<hc2r::HC2R_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(62usize)
                as *const crate::Reg<hc2r::HC2R_SPEC>)
        }
    }
}
#[doc = "SSAR register accessor: an alias for `Reg<SSAR_SPEC>`"]
pub type SSAR = crate::Reg<ssar::SSAR_SPEC>;
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar;
#[doc = "SSAR_CMD23_MODE register accessor: an alias for `Reg<SSAR_CMD23_MODE_SPEC>`"]
pub type SSAR_CMD23_MODE = crate::Reg<ssar_cmd23_mode::SSAR_CMD23_MODE_SPEC>;
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar_cmd23_mode;
#[doc = "BSR register accessor: an alias for `Reg<BSR_SPEC>`"]
pub type BSR = crate::Reg<bsr::BSR_SPEC>;
#[doc = "Block Size"]
pub mod bsr;
#[doc = "BCR register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "Block Count"]
pub mod bcr;
#[doc = "ARG1R register accessor: an alias for `Reg<ARG1R_SPEC>`"]
pub type ARG1R = crate::Reg<arg1r::ARG1R_SPEC>;
#[doc = "Argument 1"]
pub mod arg1r;
#[doc = "TMR register accessor: an alias for `Reg<TMR_SPEC>`"]
pub type TMR = crate::Reg<tmr::TMR_SPEC>;
#[doc = "Transfer Mode"]
pub mod tmr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Command"]
pub mod cr;
#[doc = "RR register accessor: an alias for `Reg<RR_SPEC>`"]
pub type RR = crate::Reg<rr::RR_SPEC>;
#[doc = "Response"]
pub mod rr;
#[doc = "BDPR register accessor: an alias for `Reg<BDPR_SPEC>`"]
pub type BDPR = crate::Reg<bdpr::BDPR_SPEC>;
#[doc = "Buffer Data Port"]
pub mod bdpr;
#[doc = "PSR register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Present State"]
pub mod psr;
#[doc = "HC1R register accessor: an alias for `Reg<HC1R_SPEC>`"]
pub type HC1R = crate::Reg<hc1r::HC1R_SPEC>;
#[doc = "Host Control 1"]
pub mod hc1r;
#[doc = "HC1R_EMMC_MODE register accessor: an alias for `Reg<HC1R_EMMC_MODE_SPEC>`"]
pub type HC1R_EMMC_MODE = crate::Reg<hc1r_emmc_mode::HC1R_EMMC_MODE_SPEC>;
#[doc = "Host Control 1"]
pub mod hc1r_emmc_mode;
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Power Control"]
pub mod pcr;
#[doc = "BGCR register accessor: an alias for `Reg<BGCR_SPEC>`"]
pub type BGCR = crate::Reg<bgcr::BGCR_SPEC>;
#[doc = "Block Gap Control"]
pub mod bgcr;
#[doc = "BGCR_EMMC_MODE register accessor: an alias for `Reg<BGCR_EMMC_MODE_SPEC>`"]
pub type BGCR_EMMC_MODE = crate::Reg<bgcr_emmc_mode::BGCR_EMMC_MODE_SPEC>;
#[doc = "Block Gap Control"]
pub mod bgcr_emmc_mode;
#[doc = "WCR register accessor: an alias for `Reg<WCR_SPEC>`"]
pub type WCR = crate::Reg<wcr::WCR_SPEC>;
#[doc = "Wakeup Control"]
pub mod wcr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Clock Control"]
pub mod ccr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Timeout Control"]
pub mod tcr;
#[doc = "SRR register accessor: an alias for `Reg<SRR_SPEC>`"]
pub type SRR = crate::Reg<srr::SRR_SPEC>;
#[doc = "Software Reset"]
pub mod srr;
#[doc = "NISTR register accessor: an alias for `Reg<NISTR_SPEC>`"]
pub type NISTR = crate::Reg<nistr::NISTR_SPEC>;
#[doc = "Normal Interrupt Status"]
pub mod nistr;
#[doc = "NISTR_EMMC_MODE register accessor: an alias for `Reg<NISTR_EMMC_MODE_SPEC>`"]
pub type NISTR_EMMC_MODE = crate::Reg<nistr_emmc_mode::NISTR_EMMC_MODE_SPEC>;
#[doc = "Normal Interrupt Status"]
pub mod nistr_emmc_mode;
#[doc = "EISTR register accessor: an alias for `Reg<EISTR_SPEC>`"]
pub type EISTR = crate::Reg<eistr::EISTR_SPEC>;
#[doc = "Error Interrupt Status"]
pub mod eistr;
#[doc = "EISTR_EMMC_MODE register accessor: an alias for `Reg<EISTR_EMMC_MODE_SPEC>`"]
pub type EISTR_EMMC_MODE = crate::Reg<eistr_emmc_mode::EISTR_EMMC_MODE_SPEC>;
#[doc = "Error Interrupt Status"]
pub mod eistr_emmc_mode;
#[doc = "NISTER register accessor: an alias for `Reg<NISTER_SPEC>`"]
pub type NISTER = crate::Reg<nister::NISTER_SPEC>;
#[doc = "Normal Interrupt Status Enable"]
pub mod nister;
#[doc = "NISTER_EMMC_MODE register accessor: an alias for `Reg<NISTER_EMMC_MODE_SPEC>`"]
pub type NISTER_EMMC_MODE = crate::Reg<nister_emmc_mode::NISTER_EMMC_MODE_SPEC>;
#[doc = "Normal Interrupt Status Enable"]
pub mod nister_emmc_mode;
#[doc = "EISTER register accessor: an alias for `Reg<EISTER_SPEC>`"]
pub type EISTER = crate::Reg<eister::EISTER_SPEC>;
#[doc = "Error Interrupt Status Enable"]
pub mod eister;
#[doc = "EISTER_EMMC_MODE register accessor: an alias for `Reg<EISTER_EMMC_MODE_SPEC>`"]
pub type EISTER_EMMC_MODE = crate::Reg<eister_emmc_mode::EISTER_EMMC_MODE_SPEC>;
#[doc = "Error Interrupt Status Enable"]
pub mod eister_emmc_mode;
#[doc = "NISIER register accessor: an alias for `Reg<NISIER_SPEC>`"]
pub type NISIER = crate::Reg<nisier::NISIER_SPEC>;
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier;
#[doc = "NISIER_EMMC_MODE register accessor: an alias for `Reg<NISIER_EMMC_MODE_SPEC>`"]
pub type NISIER_EMMC_MODE = crate::Reg<nisier_emmc_mode::NISIER_EMMC_MODE_SPEC>;
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier_emmc_mode;
#[doc = "EISIER register accessor: an alias for `Reg<EISIER_SPEC>`"]
pub type EISIER = crate::Reg<eisier::EISIER_SPEC>;
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier;
#[doc = "EISIER_EMMC_MODE register accessor: an alias for `Reg<EISIER_EMMC_MODE_SPEC>`"]
pub type EISIER_EMMC_MODE = crate::Reg<eisier_emmc_mode::EISIER_EMMC_MODE_SPEC>;
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier_emmc_mode;
#[doc = "ACESR register accessor: an alias for `Reg<ACESR_SPEC>`"]
pub type ACESR = crate::Reg<acesr::ACESR_SPEC>;
#[doc = "Auto CMD Error Status"]
pub mod acesr;
#[doc = "HC2R register accessor: an alias for `Reg<HC2R_SPEC>`"]
pub type HC2R = crate::Reg<hc2r::HC2R_SPEC>;
#[doc = "Host Control 2"]
pub mod hc2r;
#[doc = "HC2R_EMMC_MODE register accessor: an alias for `Reg<HC2R_EMMC_MODE_SPEC>`"]
pub type HC2R_EMMC_MODE = crate::Reg<hc2r_emmc_mode::HC2R_EMMC_MODE_SPEC>;
#[doc = "Host Control 2"]
pub mod hc2r_emmc_mode;
#[doc = "CA0R register accessor: an alias for `Reg<CA0R_SPEC>`"]
pub type CA0R = crate::Reg<ca0r::CA0R_SPEC>;
#[doc = "Capabilities 0"]
pub mod ca0r;
#[doc = "CA1R register accessor: an alias for `Reg<CA1R_SPEC>`"]
pub type CA1R = crate::Reg<ca1r::CA1R_SPEC>;
#[doc = "Capabilities 1"]
pub mod ca1r;
#[doc = "MCCAR register accessor: an alias for `Reg<MCCAR_SPEC>`"]
pub type MCCAR = crate::Reg<mccar::MCCAR_SPEC>;
#[doc = "Maximum Current Capabilities"]
pub mod mccar;
#[doc = "FERACES register accessor: an alias for `Reg<FERACES_SPEC>`"]
pub type FERACES = crate::Reg<feraces::FERACES_SPEC>;
#[doc = "Force Event for Auto CMD Error Status"]
pub mod feraces;
#[doc = "FEREIS register accessor: an alias for `Reg<FEREIS_SPEC>`"]
pub type FEREIS = crate::Reg<fereis::FEREIS_SPEC>;
#[doc = "Force Event for Error Interrupt Status"]
pub mod fereis;
#[doc = "AESR register accessor: an alias for `Reg<AESR_SPEC>`"]
pub type AESR = crate::Reg<aesr::AESR_SPEC>;
#[doc = "ADMA Error Status"]
pub mod aesr;
#[doc = "ASAR register accessor: an alias for `Reg<ASAR_SPEC>`"]
pub type ASAR = crate::Reg<asar::ASAR_SPEC>;
#[doc = "ADMA System Address n"]
pub mod asar;
#[doc = "PVR register accessor: an alias for `Reg<PVR_SPEC>`"]
pub type PVR = crate::Reg<pvr::PVR_SPEC>;
#[doc = "Preset Value n"]
pub mod pvr;
#[doc = "SISR register accessor: an alias for `Reg<SISR_SPEC>`"]
pub type SISR = crate::Reg<sisr::SISR_SPEC>;
#[doc = "Slot Interrupt Status"]
pub mod sisr;
#[doc = "HCVR register accessor: an alias for `Reg<HCVR_SPEC>`"]
pub type HCVR = crate::Reg<hcvr::HCVR_SPEC>;
#[doc = "Host Controller Version"]
pub mod hcvr;
#[doc = "MC1R register accessor: an alias for `Reg<MC1R_SPEC>`"]
pub type MC1R = crate::Reg<mc1r::MC1R_SPEC>;
#[doc = "MMC Control 1"]
pub mod mc1r;
#[doc = "MC2R register accessor: an alias for `Reg<MC2R_SPEC>`"]
pub type MC2R = crate::Reg<mc2r::MC2R_SPEC>;
#[doc = "MMC Control 2"]
pub mod mc2r;
#[doc = "ACR register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "AHB Control"]
pub mod acr;
#[doc = "CC2R register accessor: an alias for `Reg<CC2R_SPEC>`"]
pub type CC2R = crate::Reg<cc2r::CC2R_SPEC>;
#[doc = "Clock Control 2"]
pub mod cc2r;
#[doc = "CACR register accessor: an alias for `Reg<CACR_SPEC>`"]
pub type CACR = crate::Reg<cacr::CACR_SPEC>;
#[doc = "Capabilities Control"]
pub mod cacr;
#[doc = "DBGR register accessor: an alias for `Reg<DBGR_SPEC>`"]
pub type DBGR = crate::Reg<dbgr::DBGR_SPEC>;
#[doc = "Debug"]
pub mod dbgr;
