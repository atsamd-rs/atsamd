#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ssar: [u8; 0x04],
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
    #[doc = "0x10..0x20 - Response"]
    pub rr: [RR; 4],
    #[doc = "0x20 - Buffer Data Port"]
    pub bdpr: BDPR,
    #[doc = "0x24 - Present State"]
    pub psr: PSR,
    _reserved_9_hc1: [u8; 0x01],
    #[doc = "0x29 - Power Control"]
    pub pcr: PCR,
    _reserved_11_bgcr: [u8; 0x01],
    #[doc = "0x2b - Wakeup Control"]
    pub wcr: WCR,
    #[doc = "0x2c - Clock Control"]
    pub ccr: CCR,
    #[doc = "0x2e - Timeout Control"]
    pub tcr: TCR,
    #[doc = "0x2f - Software Reset"]
    pub srr: SRR,
    _reserved_16_nistr: [u8; 0x02],
    _reserved_17_eistr: [u8; 0x02],
    _reserved_18_nister: [u8; 0x02],
    _reserved_19_eister: [u8; 0x02],
    _reserved_20_nisier: [u8; 0x02],
    _reserved_21_eisier: [u8; 0x02],
    #[doc = "0x3c - Auto CMD Error Status"]
    pub acesr: ACESR,
    _reserved_23_hc2: [u8; 0x02],
    #[doc = "0x40 - Capabilities 0"]
    pub ca0r: CA0R,
    #[doc = "0x44 - Capabilities 1"]
    pub ca1r: CA1R,
    #[doc = "0x48 - Maximum Current Capabilities"]
    pub mccar: MCCAR,
    _reserved27: [u8; 0x04],
    #[doc = "0x50 - Force Event for Auto CMD Error Status"]
    pub feraces: FERACES,
    #[doc = "0x52 - Force Event for Error Interrupt Status"]
    pub fereis: FEREIS,
    #[doc = "0x54 - ADMA Error Status"]
    pub aesr: AESR,
    _reserved30: [u8; 0x03],
    #[doc = "0x58 - ADMA System Address n"]
    pub asar: [ASAR; 1],
    _reserved31: [u8; 0x04],
    #[doc = "0x60..0x70 - Preset Value n"]
    pub pvr: [PVR; 8],
    _reserved32: [u8; 0x8c],
    #[doc = "0xfc - Slot Interrupt Status"]
    pub sisr: SISR,
    #[doc = "0xfe - Host Controller Version"]
    pub hcvr: HCVR,
    _reserved34: [u8; 0x0104],
    #[doc = "0x204 - MMC Control 1"]
    pub mc1r: MC1R,
    #[doc = "0x205 - MMC Control 2"]
    pub mc2r: MC2R,
    _reserved36: [u8; 0x02],
    #[doc = "0x208 - AHB Control"]
    pub acr: ACR,
    #[doc = "0x20c - Clock Control 2"]
    pub cc2r: CC2R,
    _reserved38: [u8; 0x20],
    #[doc = "0x230 - Capabilities Control"]
    pub cacr: CACR,
    #[doc = "0x234 - Debug"]
    pub dbgr: DBGR,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub const fn ssar_cmd23_mode(&self) -> &SSAR_CMD23_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub const fn ssar(&self) -> &SSAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub const fn hc1r_emmc_mode(&self) -> &HC1R_EMMC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub const fn hc1r(&self) -> &HC1R {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub const fn bgcr_emmc_mode(&self) -> &BGCR_EMMC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(42usize).cast() }
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub const fn bgcr(&self) -> &BGCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(42usize).cast() }
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub const fn nistr_emmc_mode(&self) -> &NISTR_EMMC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub const fn nistr(&self) -> &NISTR {
        unsafe { &*(self as *const Self).cast::<u8>().add(48usize).cast() }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub const fn eistr_emmc_mode(&self) -> &EISTR_EMMC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(50usize).cast() }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub const fn eistr(&self) -> &EISTR {
        unsafe { &*(self as *const Self).cast::<u8>().add(50usize).cast() }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub const fn nister_emmc_mode(&self) -> &NISTER_EMMC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub const fn nister(&self) -> &NISTER {
        unsafe { &*(self as *const Self).cast::<u8>().add(52usize).cast() }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn eister_emmc_mode(&self) -> &EISTER_EMMC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(54usize).cast() }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn eister(&self) -> &EISTER {
        unsafe { &*(self as *const Self).cast::<u8>().add(54usize).cast() }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn nisier_emmc_mode(&self) -> &NISIER_EMMC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn nisier(&self) -> &NISIER {
        unsafe { &*(self as *const Self).cast::<u8>().add(56usize).cast() }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn eisier_emmc_mode(&self) -> &EISIER_EMMC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(58usize).cast() }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn eisier(&self) -> &EISIER {
        unsafe { &*(self as *const Self).cast::<u8>().add(58usize).cast() }
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub const fn hc2r_emmc_mode(&self) -> &HC2R_EMMC_MODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(62usize).cast() }
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub const fn hc2r(&self) -> &HC2R {
        unsafe { &*(self as *const Self).cast::<u8>().add(62usize).cast() }
    }
}
#[doc = "SSAR (rw) register accessor: SDMA System Address / Argument 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssar`]
module"]
pub type SSAR = crate::Reg<ssar::SSAR_SPEC>;
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar;
#[doc = "SSAR_CMD23_MODE (rw) register accessor: SDMA System Address / Argument 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssar_cmd23_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssar_cmd23_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssar_cmd23_mode`]
module"]
pub type SSAR_CMD23_MODE = crate::Reg<ssar_cmd23_mode::SSAR_CMD23_MODE_SPEC>;
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar_cmd23_mode;
#[doc = "BSR (rw) register accessor: Block Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsr`]
module"]
pub type BSR = crate::Reg<bsr::BSR_SPEC>;
#[doc = "Block Size"]
pub mod bsr;
#[doc = "BCR (rw) register accessor: Block Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`]
module"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "Block Count"]
pub mod bcr;
#[doc = "ARG1R (rw) register accessor: Argument 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg1r`]
module"]
pub type ARG1R = crate::Reg<arg1r::ARG1R_SPEC>;
#[doc = "Argument 1"]
pub mod arg1r;
#[doc = "TMR (rw) register accessor: Transfer Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr`]
module"]
pub type TMR = crate::Reg<tmr::TMR_SPEC>;
#[doc = "Transfer Mode"]
pub mod tmr;
#[doc = "CR (rw) register accessor: Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Command"]
pub mod cr;
#[doc = "RR (r) register accessor: Response\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rr`]
module"]
pub type RR = crate::Reg<rr::RR_SPEC>;
#[doc = "Response"]
pub mod rr;
#[doc = "BDPR (rw) register accessor: Buffer Data Port\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdpr`]
module"]
pub type BDPR = crate::Reg<bdpr::BDPR_SPEC>;
#[doc = "Buffer Data Port"]
pub mod bdpr;
#[doc = "PSR (r) register accessor: Present State\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Present State"]
pub mod psr;
#[doc = "HC1R (rw) register accessor: Host Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1r`]
module"]
pub type HC1R = crate::Reg<hc1r::HC1R_SPEC>;
#[doc = "Host Control 1"]
pub mod hc1r;
#[doc = "HC1R_EMMC_MODE (rw) register accessor: Host Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc1r_emmc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc1r_emmc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1r_emmc_mode`]
module"]
pub type HC1R_EMMC_MODE = crate::Reg<hc1r_emmc_mode::HC1R_EMMC_MODE_SPEC>;
#[doc = "Host Control 1"]
pub mod hc1r_emmc_mode;
#[doc = "PCR (rw) register accessor: Power Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`]
module"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Power Control"]
pub mod pcr;
#[doc = "BGCR (rw) register accessor: Block Gap Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcr`]
module"]
pub type BGCR = crate::Reg<bgcr::BGCR_SPEC>;
#[doc = "Block Gap Control"]
pub mod bgcr;
#[doc = "BGCR_EMMC_MODE (rw) register accessor: Block Gap Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bgcr_emmc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bgcr_emmc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcr_emmc_mode`]
module"]
pub type BGCR_EMMC_MODE = crate::Reg<bgcr_emmc_mode::BGCR_EMMC_MODE_SPEC>;
#[doc = "Block Gap Control"]
pub mod bgcr_emmc_mode;
#[doc = "WCR (rw) register accessor: Wakeup Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcr`]
module"]
pub type WCR = crate::Reg<wcr::WCR_SPEC>;
#[doc = "Wakeup Control"]
pub mod wcr;
#[doc = "CCR (rw) register accessor: Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Clock Control"]
pub mod ccr;
#[doc = "TCR (rw) register accessor: Timeout Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Timeout Control"]
pub mod tcr;
#[doc = "SRR (rw) register accessor: Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srr`]
module"]
pub type SRR = crate::Reg<srr::SRR_SPEC>;
#[doc = "Software Reset"]
pub mod srr;
#[doc = "NISTR (rw) register accessor: Normal Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nistr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nistr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nistr`]
module"]
pub type NISTR = crate::Reg<nistr::NISTR_SPEC>;
#[doc = "Normal Interrupt Status"]
pub mod nistr;
#[doc = "NISTR_EMMC_MODE (rw) register accessor: Normal Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nistr_emmc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nistr_emmc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nistr_emmc_mode`]
module"]
pub type NISTR_EMMC_MODE = crate::Reg<nistr_emmc_mode::NISTR_EMMC_MODE_SPEC>;
#[doc = "Normal Interrupt Status"]
pub mod nistr_emmc_mode;
#[doc = "EISTR (rw) register accessor: Error Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eistr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eistr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eistr`]
module"]
pub type EISTR = crate::Reg<eistr::EISTR_SPEC>;
#[doc = "Error Interrupt Status"]
pub mod eistr;
#[doc = "EISTR_EMMC_MODE (rw) register accessor: Error Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eistr_emmc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eistr_emmc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eistr_emmc_mode`]
module"]
pub type EISTR_EMMC_MODE = crate::Reg<eistr_emmc_mode::EISTR_EMMC_MODE_SPEC>;
#[doc = "Error Interrupt Status"]
pub mod eistr_emmc_mode;
#[doc = "NISTER (rw) register accessor: Normal Interrupt Status Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nister::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nister::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nister`]
module"]
pub type NISTER = crate::Reg<nister::NISTER_SPEC>;
#[doc = "Normal Interrupt Status Enable"]
pub mod nister;
#[doc = "NISTER_EMMC_MODE (rw) register accessor: Normal Interrupt Status Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nister_emmc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nister_emmc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nister_emmc_mode`]
module"]
pub type NISTER_EMMC_MODE = crate::Reg<nister_emmc_mode::NISTER_EMMC_MODE_SPEC>;
#[doc = "Normal Interrupt Status Enable"]
pub mod nister_emmc_mode;
#[doc = "EISTER (rw) register accessor: Error Interrupt Status Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eister::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eister::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eister`]
module"]
pub type EISTER = crate::Reg<eister::EISTER_SPEC>;
#[doc = "Error Interrupt Status Enable"]
pub mod eister;
#[doc = "EISTER_EMMC_MODE (rw) register accessor: Error Interrupt Status Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eister_emmc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eister_emmc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eister_emmc_mode`]
module"]
pub type EISTER_EMMC_MODE = crate::Reg<eister_emmc_mode::EISTER_EMMC_MODE_SPEC>;
#[doc = "Error Interrupt Status Enable"]
pub mod eister_emmc_mode;
#[doc = "NISIER (rw) register accessor: Normal Interrupt Signal Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nisier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nisier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nisier`]
module"]
pub type NISIER = crate::Reg<nisier::NISIER_SPEC>;
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier;
#[doc = "NISIER_EMMC_MODE (rw) register accessor: Normal Interrupt Signal Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nisier_emmc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nisier_emmc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nisier_emmc_mode`]
module"]
pub type NISIER_EMMC_MODE = crate::Reg<nisier_emmc_mode::NISIER_EMMC_MODE_SPEC>;
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier_emmc_mode;
#[doc = "EISIER (rw) register accessor: Error Interrupt Signal Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eisier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eisier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eisier`]
module"]
pub type EISIER = crate::Reg<eisier::EISIER_SPEC>;
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier;
#[doc = "EISIER_EMMC_MODE (rw) register accessor: Error Interrupt Signal Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eisier_emmc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eisier_emmc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eisier_emmc_mode`]
module"]
pub type EISIER_EMMC_MODE = crate::Reg<eisier_emmc_mode::EISIER_EMMC_MODE_SPEC>;
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier_emmc_mode;
#[doc = "ACESR (r) register accessor: Auto CMD Error Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acesr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acesr`]
module"]
pub type ACESR = crate::Reg<acesr::ACESR_SPEC>;
#[doc = "Auto CMD Error Status"]
pub mod acesr;
#[doc = "HC2R (rw) register accessor: Host Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2r`]
module"]
pub type HC2R = crate::Reg<hc2r::HC2R_SPEC>;
#[doc = "Host Control 2"]
pub mod hc2r;
#[doc = "HC2R_EMMC_MODE (rw) register accessor: Host Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc2r_emmc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc2r_emmc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2r_emmc_mode`]
module"]
pub type HC2R_EMMC_MODE = crate::Reg<hc2r_emmc_mode::HC2R_EMMC_MODE_SPEC>;
#[doc = "Host Control 2"]
pub mod hc2r_emmc_mode;
#[doc = "CA0R (r) register accessor: Capabilities 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ca0r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ca0r`]
module"]
pub type CA0R = crate::Reg<ca0r::CA0R_SPEC>;
#[doc = "Capabilities 0"]
pub mod ca0r;
#[doc = "CA1R (r) register accessor: Capabilities 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ca1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ca1r`]
module"]
pub type CA1R = crate::Reg<ca1r::CA1R_SPEC>;
#[doc = "Capabilities 1"]
pub mod ca1r;
#[doc = "MCCAR (r) register accessor: Maximum Current Capabilities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mccar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mccar`]
module"]
pub type MCCAR = crate::Reg<mccar::MCCAR_SPEC>;
#[doc = "Maximum Current Capabilities"]
pub mod mccar;
#[doc = "FERACES (w) register accessor: Force Event for Auto CMD Error Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feraces::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feraces`]
module"]
pub type FERACES = crate::Reg<feraces::FERACES_SPEC>;
#[doc = "Force Event for Auto CMD Error Status"]
pub mod feraces;
#[doc = "FEREIS (w) register accessor: Force Event for Error Interrupt Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fereis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fereis`]
module"]
pub type FEREIS = crate::Reg<fereis::FEREIS_SPEC>;
#[doc = "Force Event for Error Interrupt Status"]
pub mod fereis;
#[doc = "AESR (r) register accessor: ADMA Error Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesr`]
module"]
pub type AESR = crate::Reg<aesr::AESR_SPEC>;
#[doc = "ADMA Error Status"]
pub mod aesr;
#[doc = "ASAR (rw) register accessor: ADMA System Address n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asar`]
module"]
pub type ASAR = crate::Reg<asar::ASAR_SPEC>;
#[doc = "ADMA System Address n"]
pub mod asar;
#[doc = "PVR (rw) register accessor: Preset Value n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvr`]
module"]
pub type PVR = crate::Reg<pvr::PVR_SPEC>;
#[doc = "Preset Value n"]
pub mod pvr;
#[doc = "SISR (r) register accessor: Slot Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sisr`]
module"]
pub type SISR = crate::Reg<sisr::SISR_SPEC>;
#[doc = "Slot Interrupt Status"]
pub mod sisr;
#[doc = "HCVR (r) register accessor: Host Controller Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcvr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcvr`]
module"]
pub type HCVR = crate::Reg<hcvr::HCVR_SPEC>;
#[doc = "Host Controller Version"]
pub mod hcvr;
#[doc = "MC1R (rw) register accessor: MMC Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc1r`]
module"]
pub type MC1R = crate::Reg<mc1r::MC1R_SPEC>;
#[doc = "MMC Control 1"]
pub mod mc1r;
#[doc = "MC2R (w) register accessor: MMC Control 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc2r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc2r`]
module"]
pub type MC2R = crate::Reg<mc2r::MC2R_SPEC>;
#[doc = "MMC Control 2"]
pub mod mc2r;
#[doc = "ACR (rw) register accessor: AHB Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`]
module"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "AHB Control"]
pub mod acr;
#[doc = "CC2R (rw) register accessor: Clock Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2r`]
module"]
pub type CC2R = crate::Reg<cc2r::CC2R_SPEC>;
#[doc = "Clock Control 2"]
pub mod cc2r;
#[doc = "CACR (rw) register accessor: Capabilities Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cacr`]
module"]
pub type CACR = crate::Reg<cacr::CACR_SPEC>;
#[doc = "Capabilities Control"]
pub mod cacr;
#[doc = "DBGR (rw) register accessor: Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgr`]
module"]
pub type DBGR = crate::Reg<dbgr::DBGR_SPEC>;
#[doc = "Debug"]
pub mod dbgr;
