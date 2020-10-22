#[doc = r"Register block"]
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
    pub fn ssar_cmd23_mode(&self) -> &SSAR_CMD23_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SSAR_CMD23_MODE) }
    }
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub fn ssar_cmd23_mode_mut(&self) -> &mut SSAR_CMD23_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut SSAR_CMD23_MODE) }
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
    pub fn hc1r_emmc_mode(&self) -> &HC1R_EMMC_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const HC1R_EMMC_MODE) }
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub fn hc1r_emmc_mode_mut(&self) -> &mut HC1R_EMMC_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut HC1R_EMMC_MODE) }
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
    pub fn bgcr_emmc_mode(&self) -> &BGCR_EMMC_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const BGCR_EMMC_MODE) }
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub fn bgcr_emmc_mode_mut(&self) -> &mut BGCR_EMMC_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut BGCR_EMMC_MODE) }
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
    pub fn nistr_emmc_mode(&self) -> &NISTR_EMMC_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const NISTR_EMMC_MODE) }
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub fn nistr_emmc_mode_mut(&self) -> &mut NISTR_EMMC_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut NISTR_EMMC_MODE) }
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
    pub fn eistr_emmc_mode(&self) -> &EISTR_EMMC_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(50usize) as *const EISTR_EMMC_MODE) }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub fn eistr_emmc_mode_mut(&self) -> &mut EISTR_EMMC_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(50usize) as *mut EISTR_EMMC_MODE) }
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
    pub fn nister_emmc_mode(&self) -> &NISTER_EMMC_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const NISTER_EMMC_MODE) }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub fn nister_emmc_mode_mut(&self) -> &mut NISTER_EMMC_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut NISTER_EMMC_MODE) }
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
    pub fn eister_emmc_mode(&self) -> &EISTER_EMMC_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(54usize) as *const EISTER_EMMC_MODE) }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub fn eister_emmc_mode_mut(&self) -> &mut EISTER_EMMC_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(54usize) as *mut EISTER_EMMC_MODE) }
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
    pub fn nisier_emmc_mode(&self) -> &NISIER_EMMC_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const NISIER_EMMC_MODE) }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub fn nisier_emmc_mode_mut(&self) -> &mut NISIER_EMMC_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut NISIER_EMMC_MODE) }
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
    pub fn eisier_emmc_mode(&self) -> &EISIER_EMMC_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(58usize) as *const EISIER_EMMC_MODE) }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub fn eisier_emmc_mode_mut(&self) -> &mut EISIER_EMMC_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(58usize) as *mut EISIER_EMMC_MODE) }
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
    pub fn hc2r_emmc_mode(&self) -> &HC2R_EMMC_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(62usize) as *const HC2R_EMMC_MODE) }
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub fn hc2r_emmc_mode_mut(&self) -> &mut HC2R_EMMC_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(62usize) as *mut HC2R_EMMC_MODE) }
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
#[doc = "SDMA System Address / Argument 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssar](ssar) module"]
pub type SSAR = crate::Reg<u32, _SSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSAR;
#[doc = "`read()` method returns [ssar::R](ssar::R) reader structure"]
impl crate::Readable for SSAR {}
#[doc = "`write(|w| ..)` method takes [ssar::W](ssar::W) writer structure"]
impl crate::Writable for SSAR {}
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar;
#[doc = "SDMA System Address / Argument 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssar_cmd23_mode](ssar_cmd23_mode) module"]
pub type SSAR_CMD23_MODE = crate::Reg<u32, _SSAR_CMD23_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSAR_CMD23_MODE;
#[doc = "`read()` method returns [ssar_cmd23_mode::R](ssar_cmd23_mode::R) reader structure"]
impl crate::Readable for SSAR_CMD23_MODE {}
#[doc = "`write(|w| ..)` method takes [ssar_cmd23_mode::W](ssar_cmd23_mode::W) writer structure"]
impl crate::Writable for SSAR_CMD23_MODE {}
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar_cmd23_mode;
#[doc = "Block Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsr](bsr) module"]
pub type BSR = crate::Reg<u16, _BSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSR;
#[doc = "`read()` method returns [bsr::R](bsr::R) reader structure"]
impl crate::Readable for BSR {}
#[doc = "`write(|w| ..)` method takes [bsr::W](bsr::W) writer structure"]
impl crate::Writable for BSR {}
#[doc = "Block Size"]
pub mod bsr;
#[doc = "Block Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr](bcr) module"]
pub type BCR = crate::Reg<u16, _BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR;
#[doc = "`read()` method returns [bcr::R](bcr::R) reader structure"]
impl crate::Readable for BCR {}
#[doc = "`write(|w| ..)` method takes [bcr::W](bcr::W) writer structure"]
impl crate::Writable for BCR {}
#[doc = "Block Count"]
pub mod bcr;
#[doc = "Argument 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arg1r](arg1r) module"]
pub type ARG1R = crate::Reg<u32, _ARG1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARG1R;
#[doc = "`read()` method returns [arg1r::R](arg1r::R) reader structure"]
impl crate::Readable for ARG1R {}
#[doc = "`write(|w| ..)` method takes [arg1r::W](arg1r::W) writer structure"]
impl crate::Writable for ARG1R {}
#[doc = "Argument 1"]
pub mod arg1r;
#[doc = "Transfer Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr](tmr) module"]
pub type TMR = crate::Reg<u16, _TMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR;
#[doc = "`read()` method returns [tmr::R](tmr::R) reader structure"]
impl crate::Readable for TMR {}
#[doc = "`write(|w| ..)` method takes [tmr::W](tmr::W) writer structure"]
impl crate::Writable for TMR {}
#[doc = "Transfer Mode"]
pub mod tmr;
#[doc = "Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u16, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Command"]
pub mod cr;
#[doc = "Response\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rr](rr) module"]
pub type RR = crate::Reg<u32, _RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RR;
#[doc = "`read()` method returns [rr::R](rr::R) reader structure"]
impl crate::Readable for RR {}
#[doc = "Response"]
pub mod rr;
#[doc = "Buffer Data Port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdpr](bdpr) module"]
pub type BDPR = crate::Reg<u32, _BDPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDPR;
#[doc = "`read()` method returns [bdpr::R](bdpr::R) reader structure"]
impl crate::Readable for BDPR {}
#[doc = "`write(|w| ..)` method takes [bdpr::W](bdpr::W) writer structure"]
impl crate::Writable for BDPR {}
#[doc = "Buffer Data Port"]
pub mod bdpr;
#[doc = "Present State\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](psr) module"]
pub type PSR = crate::Reg<u32, _PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR;
#[doc = "`read()` method returns [psr::R](psr::R) reader structure"]
impl crate::Readable for PSR {}
#[doc = "Present State"]
pub mod psr;
#[doc = "Host Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc1r](hc1r) module"]
pub type HC1R = crate::Reg<u8, _HC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC1R;
#[doc = "`read()` method returns [hc1r::R](hc1r::R) reader structure"]
impl crate::Readable for HC1R {}
#[doc = "`write(|w| ..)` method takes [hc1r::W](hc1r::W) writer structure"]
impl crate::Writable for HC1R {}
#[doc = "Host Control 1"]
pub mod hc1r;
#[doc = "Host Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc1r_emmc_mode](hc1r_emmc_mode) module"]
pub type HC1R_EMMC_MODE = crate::Reg<u8, _HC1R_EMMC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC1R_EMMC_MODE;
#[doc = "`read()` method returns [hc1r_emmc_mode::R](hc1r_emmc_mode::R) reader structure"]
impl crate::Readable for HC1R_EMMC_MODE {}
#[doc = "`write(|w| ..)` method takes [hc1r_emmc_mode::W](hc1r_emmc_mode::W) writer structure"]
impl crate::Writable for HC1R_EMMC_MODE {}
#[doc = "Host Control 1"]
pub mod hc1r_emmc_mode;
#[doc = "Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u8, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "Power Control"]
pub mod pcr;
#[doc = "Block Gap Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgcr](bgcr) module"]
pub type BGCR = crate::Reg<u8, _BGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGCR;
#[doc = "`read()` method returns [bgcr::R](bgcr::R) reader structure"]
impl crate::Readable for BGCR {}
#[doc = "`write(|w| ..)` method takes [bgcr::W](bgcr::W) writer structure"]
impl crate::Writable for BGCR {}
#[doc = "Block Gap Control"]
pub mod bgcr;
#[doc = "Block Gap Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgcr_emmc_mode](bgcr_emmc_mode) module"]
pub type BGCR_EMMC_MODE = crate::Reg<u8, _BGCR_EMMC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BGCR_EMMC_MODE;
#[doc = "`read()` method returns [bgcr_emmc_mode::R](bgcr_emmc_mode::R) reader structure"]
impl crate::Readable for BGCR_EMMC_MODE {}
#[doc = "`write(|w| ..)` method takes [bgcr_emmc_mode::W](bgcr_emmc_mode::W) writer structure"]
impl crate::Writable for BGCR_EMMC_MODE {}
#[doc = "Block Gap Control"]
pub mod bgcr_emmc_mode;
#[doc = "Wakeup Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](wcr) module"]
pub type WCR = crate::Reg<u8, _WCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCR;
#[doc = "`read()` method returns [wcr::R](wcr::R) reader structure"]
impl crate::Readable for WCR {}
#[doc = "`write(|w| ..)` method takes [wcr::W](wcr::W) writer structure"]
impl crate::Writable for WCR {}
#[doc = "Wakeup Control"]
pub mod wcr;
#[doc = "Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u16, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Clock Control"]
pub mod ccr;
#[doc = "Timeout Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u8, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "Timeout Control"]
pub mod tcr;
#[doc = "Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srr](srr) module"]
pub type SRR = crate::Reg<u8, _SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRR;
#[doc = "`read()` method returns [srr::R](srr::R) reader structure"]
impl crate::Readable for SRR {}
#[doc = "`write(|w| ..)` method takes [srr::W](srr::W) writer structure"]
impl crate::Writable for SRR {}
#[doc = "Software Reset"]
pub mod srr;
#[doc = "Normal Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nistr](nistr) module"]
pub type NISTR = crate::Reg<u16, _NISTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NISTR;
#[doc = "`read()` method returns [nistr::R](nistr::R) reader structure"]
impl crate::Readable for NISTR {}
#[doc = "`write(|w| ..)` method takes [nistr::W](nistr::W) writer structure"]
impl crate::Writable for NISTR {}
#[doc = "Normal Interrupt Status"]
pub mod nistr;
#[doc = "Normal Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nistr_emmc_mode](nistr_emmc_mode) module"]
pub type NISTR_EMMC_MODE = crate::Reg<u16, _NISTR_EMMC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NISTR_EMMC_MODE;
#[doc = "`read()` method returns [nistr_emmc_mode::R](nistr_emmc_mode::R) reader structure"]
impl crate::Readable for NISTR_EMMC_MODE {}
#[doc = "`write(|w| ..)` method takes [nistr_emmc_mode::W](nistr_emmc_mode::W) writer structure"]
impl crate::Writable for NISTR_EMMC_MODE {}
#[doc = "Normal Interrupt Status"]
pub mod nistr_emmc_mode;
#[doc = "Error Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eistr](eistr) module"]
pub type EISTR = crate::Reg<u16, _EISTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EISTR;
#[doc = "`read()` method returns [eistr::R](eistr::R) reader structure"]
impl crate::Readable for EISTR {}
#[doc = "`write(|w| ..)` method takes [eistr::W](eistr::W) writer structure"]
impl crate::Writable for EISTR {}
#[doc = "Error Interrupt Status"]
pub mod eistr;
#[doc = "Error Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eistr_emmc_mode](eistr_emmc_mode) module"]
pub type EISTR_EMMC_MODE = crate::Reg<u16, _EISTR_EMMC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EISTR_EMMC_MODE;
#[doc = "`read()` method returns [eistr_emmc_mode::R](eistr_emmc_mode::R) reader structure"]
impl crate::Readable for EISTR_EMMC_MODE {}
#[doc = "`write(|w| ..)` method takes [eistr_emmc_mode::W](eistr_emmc_mode::W) writer structure"]
impl crate::Writable for EISTR_EMMC_MODE {}
#[doc = "Error Interrupt Status"]
pub mod eistr_emmc_mode;
#[doc = "Normal Interrupt Status Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nister](nister) module"]
pub type NISTER = crate::Reg<u16, _NISTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NISTER;
#[doc = "`read()` method returns [nister::R](nister::R) reader structure"]
impl crate::Readable for NISTER {}
#[doc = "`write(|w| ..)` method takes [nister::W](nister::W) writer structure"]
impl crate::Writable for NISTER {}
#[doc = "Normal Interrupt Status Enable"]
pub mod nister;
#[doc = "Normal Interrupt Status Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nister_emmc_mode](nister_emmc_mode) module"]
pub type NISTER_EMMC_MODE = crate::Reg<u16, _NISTER_EMMC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NISTER_EMMC_MODE;
#[doc = "`read()` method returns [nister_emmc_mode::R](nister_emmc_mode::R) reader structure"]
impl crate::Readable for NISTER_EMMC_MODE {}
#[doc = "`write(|w| ..)` method takes [nister_emmc_mode::W](nister_emmc_mode::W) writer structure"]
impl crate::Writable for NISTER_EMMC_MODE {}
#[doc = "Normal Interrupt Status Enable"]
pub mod nister_emmc_mode;
#[doc = "Error Interrupt Status Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eister](eister) module"]
pub type EISTER = crate::Reg<u16, _EISTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EISTER;
#[doc = "`read()` method returns [eister::R](eister::R) reader structure"]
impl crate::Readable for EISTER {}
#[doc = "`write(|w| ..)` method takes [eister::W](eister::W) writer structure"]
impl crate::Writable for EISTER {}
#[doc = "Error Interrupt Status Enable"]
pub mod eister;
#[doc = "Error Interrupt Status Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eister_emmc_mode](eister_emmc_mode) module"]
pub type EISTER_EMMC_MODE = crate::Reg<u16, _EISTER_EMMC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EISTER_EMMC_MODE;
#[doc = "`read()` method returns [eister_emmc_mode::R](eister_emmc_mode::R) reader structure"]
impl crate::Readable for EISTER_EMMC_MODE {}
#[doc = "`write(|w| ..)` method takes [eister_emmc_mode::W](eister_emmc_mode::W) writer structure"]
impl crate::Writable for EISTER_EMMC_MODE {}
#[doc = "Error Interrupt Status Enable"]
pub mod eister_emmc_mode;
#[doc = "Normal Interrupt Signal Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nisier](nisier) module"]
pub type NISIER = crate::Reg<u16, _NISIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NISIER;
#[doc = "`read()` method returns [nisier::R](nisier::R) reader structure"]
impl crate::Readable for NISIER {}
#[doc = "`write(|w| ..)` method takes [nisier::W](nisier::W) writer structure"]
impl crate::Writable for NISIER {}
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier;
#[doc = "Normal Interrupt Signal Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nisier_emmc_mode](nisier_emmc_mode) module"]
pub type NISIER_EMMC_MODE = crate::Reg<u16, _NISIER_EMMC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NISIER_EMMC_MODE;
#[doc = "`read()` method returns [nisier_emmc_mode::R](nisier_emmc_mode::R) reader structure"]
impl crate::Readable for NISIER_EMMC_MODE {}
#[doc = "`write(|w| ..)` method takes [nisier_emmc_mode::W](nisier_emmc_mode::W) writer structure"]
impl crate::Writable for NISIER_EMMC_MODE {}
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier_emmc_mode;
#[doc = "Error Interrupt Signal Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eisier](eisier) module"]
pub type EISIER = crate::Reg<u16, _EISIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EISIER;
#[doc = "`read()` method returns [eisier::R](eisier::R) reader structure"]
impl crate::Readable for EISIER {}
#[doc = "`write(|w| ..)` method takes [eisier::W](eisier::W) writer structure"]
impl crate::Writable for EISIER {}
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier;
#[doc = "Error Interrupt Signal Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eisier_emmc_mode](eisier_emmc_mode) module"]
pub type EISIER_EMMC_MODE = crate::Reg<u16, _EISIER_EMMC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EISIER_EMMC_MODE;
#[doc = "`read()` method returns [eisier_emmc_mode::R](eisier_emmc_mode::R) reader structure"]
impl crate::Readable for EISIER_EMMC_MODE {}
#[doc = "`write(|w| ..)` method takes [eisier_emmc_mode::W](eisier_emmc_mode::W) writer structure"]
impl crate::Writable for EISIER_EMMC_MODE {}
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier_emmc_mode;
#[doc = "Auto CMD Error Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acesr](acesr) module"]
pub type ACESR = crate::Reg<u16, _ACESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACESR;
#[doc = "`read()` method returns [acesr::R](acesr::R) reader structure"]
impl crate::Readable for ACESR {}
#[doc = "Auto CMD Error Status"]
pub mod acesr;
#[doc = "Host Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc2r](hc2r) module"]
pub type HC2R = crate::Reg<u16, _HC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC2R;
#[doc = "`read()` method returns [hc2r::R](hc2r::R) reader structure"]
impl crate::Readable for HC2R {}
#[doc = "`write(|w| ..)` method takes [hc2r::W](hc2r::W) writer structure"]
impl crate::Writable for HC2R {}
#[doc = "Host Control 2"]
pub mod hc2r;
#[doc = "Host Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc2r_emmc_mode](hc2r_emmc_mode) module"]
pub type HC2R_EMMC_MODE = crate::Reg<u16, _HC2R_EMMC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC2R_EMMC_MODE;
#[doc = "`read()` method returns [hc2r_emmc_mode::R](hc2r_emmc_mode::R) reader structure"]
impl crate::Readable for HC2R_EMMC_MODE {}
#[doc = "`write(|w| ..)` method takes [hc2r_emmc_mode::W](hc2r_emmc_mode::W) writer structure"]
impl crate::Writable for HC2R_EMMC_MODE {}
#[doc = "Host Control 2"]
pub mod hc2r_emmc_mode;
#[doc = "Capabilities 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ca0r](ca0r) module"]
pub type CA0R = crate::Reg<u32, _CA0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CA0R;
#[doc = "`read()` method returns [ca0r::R](ca0r::R) reader structure"]
impl crate::Readable for CA0R {}
#[doc = "Capabilities 0"]
pub mod ca0r;
#[doc = "Capabilities 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ca1r](ca1r) module"]
pub type CA1R = crate::Reg<u32, _CA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CA1R;
#[doc = "`read()` method returns [ca1r::R](ca1r::R) reader structure"]
impl crate::Readable for CA1R {}
#[doc = "Capabilities 1"]
pub mod ca1r;
#[doc = "Maximum Current Capabilities\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mccar](mccar) module"]
pub type MCCAR = crate::Reg<u32, _MCCAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCCAR;
#[doc = "`read()` method returns [mccar::R](mccar::R) reader structure"]
impl crate::Readable for MCCAR {}
#[doc = "Maximum Current Capabilities"]
pub mod mccar;
#[doc = "Force Event for Auto CMD Error Status\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feraces](feraces) module"]
pub type FERACES = crate::Reg<u16, _FERACES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FERACES;
#[doc = "`write(|w| ..)` method takes [feraces::W](feraces::W) writer structure"]
impl crate::Writable for FERACES {}
#[doc = "Force Event for Auto CMD Error Status"]
pub mod feraces;
#[doc = "Force Event for Error Interrupt Status\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fereis](fereis) module"]
pub type FEREIS = crate::Reg<u16, _FEREIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEREIS;
#[doc = "`write(|w| ..)` method takes [fereis::W](fereis::W) writer structure"]
impl crate::Writable for FEREIS {}
#[doc = "Force Event for Error Interrupt Status"]
pub mod fereis;
#[doc = "ADMA Error Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesr](aesr) module"]
pub type AESR = crate::Reg<u8, _AESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESR;
#[doc = "`read()` method returns [aesr::R](aesr::R) reader structure"]
impl crate::Readable for AESR {}
#[doc = "ADMA Error Status"]
pub mod aesr;
#[doc = "ADMA System Address n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asar](asar) module"]
pub type ASAR = crate::Reg<u32, _ASAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASAR;
#[doc = "`read()` method returns [asar::R](asar::R) reader structure"]
impl crate::Readable for ASAR {}
#[doc = "`write(|w| ..)` method takes [asar::W](asar::W) writer structure"]
impl crate::Writable for ASAR {}
#[doc = "ADMA System Address n"]
pub mod asar;
#[doc = "Preset Value n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pvr](pvr) module"]
pub type PVR = crate::Reg<u16, _PVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PVR;
#[doc = "`read()` method returns [pvr::R](pvr::R) reader structure"]
impl crate::Readable for PVR {}
#[doc = "`write(|w| ..)` method takes [pvr::W](pvr::W) writer structure"]
impl crate::Writable for PVR {}
#[doc = "Preset Value n"]
pub mod pvr;
#[doc = "Slot Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sisr](sisr) module"]
pub type SISR = crate::Reg<u16, _SISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SISR;
#[doc = "`read()` method returns [sisr::R](sisr::R) reader structure"]
impl crate::Readable for SISR {}
#[doc = "Slot Interrupt Status"]
pub mod sisr;
#[doc = "Host Controller Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcvr](hcvr) module"]
pub type HCVR = crate::Reg<u16, _HCVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCVR;
#[doc = "`read()` method returns [hcvr::R](hcvr::R) reader structure"]
impl crate::Readable for HCVR {}
#[doc = "Host Controller Version"]
pub mod hcvr;
#[doc = "MMC Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mc1r](mc1r) module"]
pub type MC1R = crate::Reg<u8, _MC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MC1R;
#[doc = "`read()` method returns [mc1r::R](mc1r::R) reader structure"]
impl crate::Readable for MC1R {}
#[doc = "`write(|w| ..)` method takes [mc1r::W](mc1r::W) writer structure"]
impl crate::Writable for MC1R {}
#[doc = "MMC Control 1"]
pub mod mc1r;
#[doc = "MMC Control 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mc2r](mc2r) module"]
pub type MC2R = crate::Reg<u8, _MC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MC2R;
#[doc = "`write(|w| ..)` method takes [mc2r::W](mc2r::W) writer structure"]
impl crate::Writable for MC2R {}
#[doc = "MMC Control 2"]
pub mod mc2r;
#[doc = "AHB Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](acr) module"]
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
#[doc = "`read()` method returns [acr::R](acr::R) reader structure"]
impl crate::Readable for ACR {}
#[doc = "`write(|w| ..)` method takes [acr::W](acr::W) writer structure"]
impl crate::Writable for ACR {}
#[doc = "AHB Control"]
pub mod acr;
#[doc = "Clock Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc2r](cc2r) module"]
pub type CC2R = crate::Reg<u32, _CC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC2R;
#[doc = "`read()` method returns [cc2r::R](cc2r::R) reader structure"]
impl crate::Readable for CC2R {}
#[doc = "`write(|w| ..)` method takes [cc2r::W](cc2r::W) writer structure"]
impl crate::Writable for CC2R {}
#[doc = "Clock Control 2"]
pub mod cc2r;
#[doc = "Capabilities Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacr](cacr) module"]
pub type CACR = crate::Reg<u32, _CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACR;
#[doc = "`read()` method returns [cacr::R](cacr::R) reader structure"]
impl crate::Readable for CACR {}
#[doc = "`write(|w| ..)` method takes [cacr::W](cacr::W) writer structure"]
impl crate::Writable for CACR {}
#[doc = "Capabilities Control"]
pub mod cacr;
#[doc = "Debug\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgr](dbgr) module"]
pub type DBGR = crate::Reg<u8, _DBGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGR;
#[doc = "`read()` method returns [dbgr::R](dbgr::R) reader structure"]
impl crate::Readable for DBGR {}
#[doc = "`write(|w| ..)` method takes [dbgr::W](dbgr::W) writer structure"]
impl crate::Writable for DBGR {}
#[doc = "Debug"]
pub mod dbgr;
