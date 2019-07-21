#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ssar: [u8; 4usize],
    #[doc = "0x04 - Block Size"]
    pub bsr: BSR,
    #[doc = "0x06 - Block Count"]
    pub bcr: BCR,
    #[doc = "0x08 - Argument 1"]
    pub arg1r: ARG1R,
    #[doc = "0x0c - Transfer Mode"]
    pub tmr: TMR,
    #[doc = "0x0e - Command"]
    pub cr: CR,
    #[doc = "0x10 - Response"]
    pub rr: [RR; 4],
    #[doc = "0x20 - Buffer Data Port"]
    pub bdpr: BDPR,
    #[doc = "0x24 - Present State"]
    pub psr: PSR,
    _reserved_9_hc1: [u8; 1usize],
    #[doc = "0x29 - Power Control"]
    pub pcr: PCR,
    _reserved_11_bgcr: [u8; 1usize],
    #[doc = "0x2b - Wakeup Control"]
    pub wcr: WCR,
    #[doc = "0x2c - Clock Control"]
    pub ccr: CCR,
    #[doc = "0x2e - Timeout Control"]
    pub tcr: TCR,
    #[doc = "0x2f - Software Reset"]
    pub srr: SRR,
    _reserved_16_nistr: [u8; 2usize],
    _reserved_17_eistr: [u8; 2usize],
    _reserved_18_nister: [u8; 2usize],
    _reserved_19_eister: [u8; 2usize],
    _reserved_20_nisier: [u8; 2usize],
    _reserved_21_eisier: [u8; 2usize],
    #[doc = "0x3c - Auto CMD Error Status"]
    pub acesr: ACESR,
    _reserved_23_hc2: [u8; 2usize],
    #[doc = "0x40 - Capabilities 0"]
    pub ca0r: CA0R,
    #[doc = "0x44 - Capabilities 1"]
    pub ca1r: CA1R,
    #[doc = "0x48 - Maximum Current Capabilities"]
    pub mccar: MCCAR,
    _reserved27: [u8; 4usize],
    #[doc = "0x50 - Force Event for Auto CMD Error Status"]
    pub feraces: FERACES,
    #[doc = "0x52 - Force Event for Error Interrupt Status"]
    pub fereis: FEREIS,
    #[doc = "0x54 - ADMA Error Status"]
    pub aesr: AESR,
    _reserved30: [u8; 3usize],
    #[doc = "0x58 - ADMA System Address n"]
    pub asar: [ASAR; 1],
    _reserved31: [u8; 4usize],
    #[doc = "0x60 - Preset Value n"]
    pub pvr: [PVR; 8],
    _reserved32: [u8; 140usize],
    #[doc = "0xfc - Slot Interrupt Status"]
    pub sisr: SISR,
    #[doc = "0xfe - Host Controller Version"]
    pub hcvr: HCVR,
    _reserved34: [u8; 260usize],
    #[doc = "0x204 - MMC Control 1"]
    pub mc1r: MC1R,
    #[doc = "0x205 - MMC Control 2"]
    pub mc2r: MC2R,
    _reserved36: [u8; 2usize],
    #[doc = "0x208 - AHB Control"]
    pub acr: ACR,
    #[doc = "0x20c - Clock Control 2"]
    pub cc2r: CC2R,
    _reserved38: [u8; 32usize],
    #[doc = "0x230 - Capabilities Control"]
    pub cacr: CACR,
    #[doc = "0x234 - Debug"]
    pub dbgr: DBGR,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub fn ssar_cmd23(&self) -> &SSAR_CMD23 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SSAR_CMD23) }
    }
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub fn ssar_cmd23_mut(&self) -> &mut SSAR_CMD23 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut SSAR_CMD23) }
    }
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub fn ssar(&self) -> &SSAR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SSAR) }
    }
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub fn ssar_mut(&self) -> &mut SSAR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut SSAR) }
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub fn hc1r_emmc(&self) -> &HC1R_EMMC {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const HC1R_EMMC) }
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub fn hc1r_emmc_mut(&self) -> &mut HC1R_EMMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut HC1R_EMMC) }
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub fn hc1r(&self) -> &HC1R {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const HC1R) }
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub fn hc1r_mut(&self) -> &mut HC1R {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut HC1R) }
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub fn bgcr_emmc(&self) -> &BGCR_EMMC {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const BGCR_EMMC) }
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub fn bgcr_emmc_mut(&self) -> &mut BGCR_EMMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut BGCR_EMMC) }
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub fn bgcr(&self) -> &BGCR {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const BGCR) }
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub fn bgcr_mut(&self) -> &mut BGCR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut BGCR) }
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub fn nistr_emmc(&self) -> &NISTR_EMMC {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const NISTR_EMMC) }
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub fn nistr_emmc_mut(&self) -> &mut NISTR_EMMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut NISTR_EMMC) }
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub fn nistr(&self) -> &NISTR {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const NISTR) }
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub fn nistr_mut(&self) -> &mut NISTR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut NISTR) }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub fn eistr_emmc(&self) -> &EISTR_EMMC {
        unsafe { &*(((self as *const Self) as *const u8).add(50usize) as *const EISTR_EMMC) }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub fn eistr_emmc_mut(&self) -> &mut EISTR_EMMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(50usize) as *mut EISTR_EMMC) }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub fn eistr(&self) -> &EISTR {
        unsafe { &*(((self as *const Self) as *const u8).add(50usize) as *const EISTR) }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub fn eistr_mut(&self) -> &mut EISTR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(50usize) as *mut EISTR) }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub fn nister_emmc(&self) -> &NISTER_EMMC {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const NISTER_EMMC) }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub fn nister_emmc_mut(&self) -> &mut NISTER_EMMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut NISTER_EMMC) }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub fn nister(&self) -> &NISTER {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const NISTER) }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub fn nister_mut(&self) -> &mut NISTER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut NISTER) }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub fn eister_emmc(&self) -> &EISTER_EMMC {
        unsafe { &*(((self as *const Self) as *const u8).add(54usize) as *const EISTER_EMMC) }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub fn eister_emmc_mut(&self) -> &mut EISTER_EMMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(54usize) as *mut EISTER_EMMC) }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub fn eister(&self) -> &EISTER {
        unsafe { &*(((self as *const Self) as *const u8).add(54usize) as *const EISTER) }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub fn eister_mut(&self) -> &mut EISTER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(54usize) as *mut EISTER) }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub fn nisier_emmc(&self) -> &NISIER_EMMC {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const NISIER_EMMC) }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub fn nisier_emmc_mut(&self) -> &mut NISIER_EMMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut NISIER_EMMC) }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub fn nisier(&self) -> &NISIER {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const NISIER) }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub fn nisier_mut(&self) -> &mut NISIER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut NISIER) }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub fn eisier_emmc(&self) -> &EISIER_EMMC {
        unsafe { &*(((self as *const Self) as *const u8).add(58usize) as *const EISIER_EMMC) }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub fn eisier_emmc_mut(&self) -> &mut EISIER_EMMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(58usize) as *mut EISIER_EMMC) }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub fn eisier(&self) -> &EISIER {
        unsafe { &*(((self as *const Self) as *const u8).add(58usize) as *const EISIER) }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub fn eisier_mut(&self) -> &mut EISIER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(58usize) as *mut EISIER) }
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub fn hc2r_emmc(&self) -> &HC2R_EMMC {
        unsafe { &*(((self as *const Self) as *const u8).add(62usize) as *const HC2R_EMMC) }
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub fn hc2r_emmc_mut(&self) -> &mut HC2R_EMMC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(62usize) as *mut HC2R_EMMC) }
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub fn hc2r(&self) -> &HC2R {
        unsafe { &*(((self as *const Self) as *const u8).add(62usize) as *const HC2R) }
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub fn hc2r_mut(&self) -> &mut HC2R {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(62usize) as *mut HC2R) }
    }
}
#[doc = "SDMA System Address / Argument 2"]
pub struct SSAR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar;
#[doc = "SDMA System Address / Argument 2"]
pub struct SSAR_CMD23 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar_cmd23;
#[doc = "Block Size"]
pub struct BSR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Block Size"]
pub mod bsr;
#[doc = "Block Count"]
pub struct BCR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Block Count"]
pub mod bcr;
#[doc = "Argument 1"]
pub struct ARG1R {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Argument 1"]
pub mod arg1r;
#[doc = "Transfer Mode"]
pub struct TMR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Transfer Mode"]
pub mod tmr;
#[doc = "Command"]
pub struct CR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Command"]
pub mod cr;
#[doc = "Response"]
pub struct RR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Response"]
pub mod rr;
#[doc = "Buffer Data Port"]
pub struct BDPR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Buffer Data Port"]
pub mod bdpr;
#[doc = "Present State"]
pub struct PSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Present State"]
pub mod psr;
#[doc = "Host Control 1"]
pub struct HC1R {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Host Control 1"]
pub mod hc1r;
#[doc = "Host Control 1"]
pub struct HC1R_EMMC {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Host Control 1"]
pub mod hc1r_emmc;
#[doc = "Power Control"]
pub struct PCR {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Power Control"]
pub mod pcr;
#[doc = "Block Gap Control"]
pub struct BGCR {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Block Gap Control"]
pub mod bgcr;
#[doc = "Block Gap Control"]
pub struct BGCR_EMMC {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Block Gap Control"]
pub mod bgcr_emmc;
#[doc = "Wakeup Control"]
pub struct WCR {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Wakeup Control"]
pub mod wcr;
#[doc = "Clock Control"]
pub struct CCR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Clock Control"]
pub mod ccr;
#[doc = "Timeout Control"]
pub struct TCR {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Timeout Control"]
pub mod tcr;
#[doc = "Software Reset"]
pub struct SRR {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Software Reset"]
pub mod srr;
#[doc = "Normal Interrupt Status"]
pub struct NISTR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Status"]
pub mod nistr;
#[doc = "Normal Interrupt Status"]
pub struct NISTR_EMMC {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Status"]
pub mod nistr_emmc;
#[doc = "Error Interrupt Status"]
pub struct EISTR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Status"]
pub mod eistr;
#[doc = "Error Interrupt Status"]
pub struct EISTR_EMMC {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Status"]
pub mod eistr_emmc;
#[doc = "Normal Interrupt Status Enable"]
pub struct NISTER {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Status Enable"]
pub mod nister;
#[doc = "Normal Interrupt Status Enable"]
pub struct NISTER_EMMC {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Status Enable"]
pub mod nister_emmc;
#[doc = "Error Interrupt Status Enable"]
pub struct EISTER {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Status Enable"]
pub mod eister;
#[doc = "Error Interrupt Status Enable"]
pub struct EISTER_EMMC {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Status Enable"]
pub mod eister_emmc;
#[doc = "Normal Interrupt Signal Enable"]
pub struct NISIER {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier;
#[doc = "Normal Interrupt Signal Enable"]
pub struct NISIER_EMMC {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier_emmc;
#[doc = "Error Interrupt Signal Enable"]
pub struct EISIER {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier;
#[doc = "Error Interrupt Signal Enable"]
pub struct EISIER_EMMC {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier_emmc;
#[doc = "Auto CMD Error Status"]
pub struct ACESR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Auto CMD Error Status"]
pub mod acesr;
#[doc = "Host Control 2"]
pub struct HC2R {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Host Control 2"]
pub mod hc2r;
#[doc = "Host Control 2"]
pub struct HC2R_EMMC {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Host Control 2"]
pub mod hc2r_emmc;
#[doc = "Capabilities 0"]
pub struct CA0R {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Capabilities 0"]
pub mod ca0r;
#[doc = "Capabilities 1"]
pub struct CA1R {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Capabilities 1"]
pub mod ca1r;
#[doc = "Maximum Current Capabilities"]
pub struct MCCAR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Maximum Current Capabilities"]
pub mod mccar;
#[doc = "Force Event for Auto CMD Error Status"]
pub struct FERACES {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Force Event for Auto CMD Error Status"]
pub mod feraces;
#[doc = "Force Event for Error Interrupt Status"]
pub struct FEREIS {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Force Event for Error Interrupt Status"]
pub mod fereis;
#[doc = "ADMA Error Status"]
pub struct AESR {
    register: vcell::VolatileCell<u8>,
}
#[doc = "ADMA Error Status"]
pub mod aesr;
#[doc = "ADMA System Address n"]
pub struct ASAR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "ADMA System Address n"]
pub mod asar;
#[doc = "Preset Value n"]
pub struct PVR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Preset Value n"]
pub mod pvr;
#[doc = "Slot Interrupt Status"]
pub struct SISR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Slot Interrupt Status"]
pub mod sisr;
#[doc = "Host Controller Version"]
pub struct HCVR {
    register: vcell::VolatileCell<u16>,
}
#[doc = "Host Controller Version"]
pub mod hcvr;
#[doc = "MMC Control 1"]
pub struct MC1R {
    register: vcell::VolatileCell<u8>,
}
#[doc = "MMC Control 1"]
pub mod mc1r;
#[doc = "MMC Control 2"]
pub struct MC2R {
    register: vcell::VolatileCell<u8>,
}
#[doc = "MMC Control 2"]
pub mod mc2r;
#[doc = "AHB Control"]
pub struct ACR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AHB Control"]
pub mod acr;
#[doc = "Clock Control 2"]
pub struct CC2R {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Clock Control 2"]
pub mod cc2r;
#[doc = "Capabilities Control"]
pub struct CACR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Capabilities Control"]
pub mod cacr;
#[doc = "Debug"]
pub struct DBGR {
    register: vcell::VolatileCell<u8>,
}
#[doc = "Debug"]
pub mod dbgr;
