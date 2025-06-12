#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_ssar: [u8; 0x04],
    bsr: Bsr,
    bcr: Bcr,
    arg1r: Arg1r,
    tmr: Tmr,
    cr: Cr,
    rr: [Rr; 4],
    bdpr: Bdpr,
    psr: Psr,
    _reserved_9_hc1: [u8; 0x01],
    pcr: Pcr,
    _reserved_11_bgcr: [u8; 0x01],
    wcr: Wcr,
    ccr: Ccr,
    tcr: Tcr,
    srr: Srr,
    _reserved_16_nistr: [u8; 0x02],
    _reserved_17_eistr: [u8; 0x02],
    _reserved_18_nister: [u8; 0x02],
    _reserved_19_eister: [u8; 0x02],
    _reserved_20_nisier: [u8; 0x02],
    _reserved_21_eisier: [u8; 0x02],
    acesr: Acesr,
    _reserved_23_hc2: [u8; 0x02],
    ca0r: Ca0r,
    ca1r: Ca1r,
    mccar: Mccar,
    _reserved27: [u8; 0x04],
    feraces: Feraces,
    fereis: Fereis,
    aesr: Aesr,
    _reserved30: [u8; 0x03],
    asar: [Asar; 1],
    _reserved31: [u8; 0x04],
    pvr: [Pvr; 8],
    _reserved32: [u8; 0x8c],
    sisr: Sisr,
    hcvr: Hcvr,
    _reserved34: [u8; 0x0100],
    apsr: Apsr,
    mc1r: Mc1r,
    mc2r: Mc2r,
    _reserved37: [u8; 0x02],
    acr: Acr,
    cc2r: Cc2r,
    _reserved39: [u8; 0x20],
    cacr: Cacr,
    dbgr: Dbgr,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub const fn ssar_cmd23_mode(&self) -> &SsarCmd23Mode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - SDMA System Address / Argument 2"]
    #[inline(always)]
    pub const fn ssar(&self) -> &Ssar {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Block Size"]
    #[inline(always)]
    pub const fn bsr(&self) -> &Bsr {
        &self.bsr
    }
    #[doc = "0x06 - Block Count"]
    #[inline(always)]
    pub const fn bcr(&self) -> &Bcr {
        &self.bcr
    }
    #[doc = "0x08 - Argument 1"]
    #[inline(always)]
    pub const fn arg1r(&self) -> &Arg1r {
        &self.arg1r
    }
    #[doc = "0x0c - Transfer Mode"]
    #[inline(always)]
    pub const fn tmr(&self) -> &Tmr {
        &self.tmr
    }
    #[doc = "0x0e - Command"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x10..0x20 - Response"]
    #[inline(always)]
    pub const fn rr(&self, n: usize) -> &Rr {
        &self.rr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Response"]
    #[inline(always)]
    pub fn rr_iter(&self) -> impl Iterator<Item = &Rr> {
        self.rr.iter()
    }
    #[doc = "0x20 - Buffer Data Port"]
    #[inline(always)]
    pub const fn bdpr(&self) -> &Bdpr {
        &self.bdpr
    }
    #[doc = "0x24 - Present State"]
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        &self.psr
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub const fn hc1r_emmc_mode(&self) -> &Hc1rEmmcMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - Host Control 1"]
    #[inline(always)]
    pub const fn hc1r(&self) -> &Hc1r {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x29 - Power Control"]
    #[inline(always)]
    pub const fn pcr(&self) -> &Pcr {
        &self.pcr
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub const fn bgcr_emmc_mode(&self) -> &BgcrEmmcMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2a - Block Gap Control"]
    #[inline(always)]
    pub const fn bgcr(&self) -> &Bgcr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2b - Wakeup Control"]
    #[inline(always)]
    pub const fn wcr(&self) -> &Wcr {
        &self.wcr
    }
    #[doc = "0x2c - Clock Control"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x2e - Timeout Control"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x2f - Software Reset"]
    #[inline(always)]
    pub const fn srr(&self) -> &Srr {
        &self.srr
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub const fn nistr_emmc_mode(&self) -> &NistrEmmcMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - Normal Interrupt Status"]
    #[inline(always)]
    pub const fn nistr(&self) -> &Nistr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub const fn eistr_emmc_mode(&self) -> &EistrEmmcMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(50).cast() }
    }
    #[doc = "0x32 - Error Interrupt Status"]
    #[inline(always)]
    pub const fn eistr(&self) -> &Eistr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(50).cast() }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub const fn nister_emmc_mode(&self) -> &NisterEmmcMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - Normal Interrupt Status Enable"]
    #[inline(always)]
    pub const fn nister(&self) -> &Nister {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn eister_emmc_mode(&self) -> &EisterEmmcMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(54).cast() }
    }
    #[doc = "0x36 - Error Interrupt Status Enable"]
    #[inline(always)]
    pub const fn eister(&self) -> &Eister {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(54).cast() }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn nisier_emmc_mode(&self) -> &NisierEmmcMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn nisier(&self) -> &Nisier {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn eisier_emmc_mode(&self) -> &EisierEmmcMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(58).cast() }
    }
    #[doc = "0x3a - Error Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn eisier(&self) -> &Eisier {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(58).cast() }
    }
    #[doc = "0x3c - Auto CMD Error Status"]
    #[inline(always)]
    pub const fn acesr(&self) -> &Acesr {
        &self.acesr
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub const fn hc2r_emmc_mode(&self) -> &Hc2rEmmcMode {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(62).cast() }
    }
    #[doc = "0x3e - Host Control 2"]
    #[inline(always)]
    pub const fn hc2r(&self) -> &Hc2r {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(62).cast() }
    }
    #[doc = "0x40 - Capabilities 0"]
    #[inline(always)]
    pub const fn ca0r(&self) -> &Ca0r {
        &self.ca0r
    }
    #[doc = "0x44 - Capabilities 1"]
    #[inline(always)]
    pub const fn ca1r(&self) -> &Ca1r {
        &self.ca1r
    }
    #[doc = "0x48 - Maximum Current Capabilities"]
    #[inline(always)]
    pub const fn mccar(&self) -> &Mccar {
        &self.mccar
    }
    #[doc = "0x50 - Force Event for Auto CMD Error Status"]
    #[inline(always)]
    pub const fn feraces(&self) -> &Feraces {
        &self.feraces
    }
    #[doc = "0x52 - Force Event for Error Interrupt Status"]
    #[inline(always)]
    pub const fn fereis(&self) -> &Fereis {
        &self.fereis
    }
    #[doc = "0x54 - ADMA Error Status"]
    #[inline(always)]
    pub const fn aesr(&self) -> &Aesr {
        &self.aesr
    }
    #[doc = "0x58 - ADMA System Address"]
    #[inline(always)]
    pub const fn asar(&self, n: usize) -> &Asar {
        &self.asar[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58 - ADMA System Address"]
    #[inline(always)]
    pub fn asar_iter(&self) -> impl Iterator<Item = &Asar> {
        self.asar.iter()
    }
    #[doc = "0x60..0x70 - Preset Value n"]
    #[inline(always)]
    pub const fn pvr(&self, n: usize) -> &Pvr {
        &self.pvr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x70 - Preset Value n"]
    #[inline(always)]
    pub fn pvr_iter(&self) -> impl Iterator<Item = &Pvr> {
        self.pvr.iter()
    }
    #[doc = "0xfc - Slot Interrupt Status"]
    #[inline(always)]
    pub const fn sisr(&self) -> &Sisr {
        &self.sisr
    }
    #[doc = "0xfe - Host Controller Version"]
    #[inline(always)]
    pub const fn hcvr(&self) -> &Hcvr {
        &self.hcvr
    }
    #[doc = "0x200 - Additional Present State Register"]
    #[inline(always)]
    pub const fn apsr(&self) -> &Apsr {
        &self.apsr
    }
    #[doc = "0x204 - MMC Control 1"]
    #[inline(always)]
    pub const fn mc1r(&self) -> &Mc1r {
        &self.mc1r
    }
    #[doc = "0x205 - MMC Control 2"]
    #[inline(always)]
    pub const fn mc2r(&self) -> &Mc2r {
        &self.mc2r
    }
    #[doc = "0x208 - AHB Control"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x20c - Clock Control 2"]
    #[inline(always)]
    pub const fn cc2r(&self) -> &Cc2r {
        &self.cc2r
    }
    #[doc = "0x230 - Capabilities Control"]
    #[inline(always)]
    pub const fn cacr(&self) -> &Cacr {
        &self.cacr
    }
    #[doc = "0x234 - Debug"]
    #[inline(always)]
    pub const fn dbgr(&self) -> &Dbgr {
        &self.dbgr
    }
}
#[doc = "SSAR (rw) register accessor: SDMA System Address / Argument 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ssar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssar`] module"]
#[doc(alias = "SSAR")]
pub type Ssar = crate::Reg<ssar::SsarSpec>;
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar;
#[doc = "SSAR_CMD23_MODE (rw) register accessor: SDMA System Address / Argument 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ssar_cmd23_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssar_cmd23_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssar_cmd23_mode`] module"]
#[doc(alias = "SSAR_CMD23_MODE")]
pub type SsarCmd23Mode = crate::Reg<ssar_cmd23_mode::SsarCmd23ModeSpec>;
#[doc = "SDMA System Address / Argument 2"]
pub mod ssar_cmd23_mode;
#[doc = "BSR (rw) register accessor: Block Size\n\nYou can [`read`](crate::Reg::read) this register and get [`bsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsr`] module"]
#[doc(alias = "BSR")]
pub type Bsr = crate::Reg<bsr::BsrSpec>;
#[doc = "Block Size"]
pub mod bsr;
#[doc = "BCR (rw) register accessor: Block Count\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`] module"]
#[doc(alias = "BCR")]
pub type Bcr = crate::Reg<bcr::BcrSpec>;
#[doc = "Block Count"]
pub mod bcr;
#[doc = "ARG1R (rw) register accessor: Argument 1\n\nYou can [`read`](crate::Reg::read) this register and get [`arg1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg1r`] module"]
#[doc(alias = "ARG1R")]
pub type Arg1r = crate::Reg<arg1r::Arg1rSpec>;
#[doc = "Argument 1"]
pub mod arg1r;
#[doc = "TMR (rw) register accessor: Transfer Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr`] module"]
#[doc(alias = "TMR")]
pub type Tmr = crate::Reg<tmr::TmrSpec>;
#[doc = "Transfer Mode"]
pub mod tmr;
#[doc = "CR (rw) register accessor: Command\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Command"]
pub mod cr;
#[doc = "RR (r) register accessor: Response\n\nYou can [`read`](crate::Reg::read) this register and get [`rr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rr`] module"]
#[doc(alias = "RR")]
pub type Rr = crate::Reg<rr::RrSpec>;
#[doc = "Response"]
pub mod rr;
#[doc = "BDPR (rw) register accessor: Buffer Data Port\n\nYou can [`read`](crate::Reg::read) this register and get [`bdpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdpr`] module"]
#[doc(alias = "BDPR")]
pub type Bdpr = crate::Reg<bdpr::BdprSpec>;
#[doc = "Buffer Data Port"]
pub mod bdpr;
#[doc = "PSR (r) register accessor: Present State\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`] module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "Present State"]
pub mod psr;
#[doc = "HC1R (rw) register accessor: Host Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1r`] module"]
#[doc(alias = "HC1R")]
pub type Hc1r = crate::Reg<hc1r::Hc1rSpec>;
#[doc = "Host Control 1"]
pub mod hc1r;
#[doc = "HC1R_EMMC_MODE (rw) register accessor: Host Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hc1r_emmc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc1r_emmc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc1r_emmc_mode`] module"]
#[doc(alias = "HC1R_EMMC_MODE")]
pub type Hc1rEmmcMode = crate::Reg<hc1r_emmc_mode::Hc1rEmmcModeSpec>;
#[doc = "Host Control 1"]
pub mod hc1r_emmc_mode;
#[doc = "PCR (rw) register accessor: Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`] module"]
#[doc(alias = "PCR")]
pub type Pcr = crate::Reg<pcr::PcrSpec>;
#[doc = "Power Control"]
pub mod pcr;
#[doc = "BGCR (rw) register accessor: Block Gap Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcr`] module"]
#[doc(alias = "BGCR")]
pub type Bgcr = crate::Reg<bgcr::BgcrSpec>;
#[doc = "Block Gap Control"]
pub mod bgcr;
#[doc = "BGCR_EMMC_MODE (rw) register accessor: Block Gap Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bgcr_emmc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcr_emmc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcr_emmc_mode`] module"]
#[doc(alias = "BGCR_EMMC_MODE")]
pub type BgcrEmmcMode = crate::Reg<bgcr_emmc_mode::BgcrEmmcModeSpec>;
#[doc = "Block Gap Control"]
pub mod bgcr_emmc_mode;
#[doc = "WCR (rw) register accessor: Wakeup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcr`] module"]
#[doc(alias = "WCR")]
pub type Wcr = crate::Reg<wcr::WcrSpec>;
#[doc = "Wakeup Control"]
pub mod wcr;
#[doc = "CCR (rw) register accessor: Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Clock Control"]
pub mod ccr;
#[doc = "TCR (rw) register accessor: Timeout Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`] module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Timeout Control"]
pub mod tcr;
#[doc = "SRR (rw) register accessor: Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`srr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srr`] module"]
#[doc(alias = "SRR")]
pub type Srr = crate::Reg<srr::SrrSpec>;
#[doc = "Software Reset"]
pub mod srr;
#[doc = "NISTR (rw) register accessor: Normal Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`nistr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nistr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nistr`] module"]
#[doc(alias = "NISTR")]
pub type Nistr = crate::Reg<nistr::NistrSpec>;
#[doc = "Normal Interrupt Status"]
pub mod nistr;
#[doc = "NISTR_EMMC_MODE (rw) register accessor: Normal Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`nistr_emmc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nistr_emmc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nistr_emmc_mode`] module"]
#[doc(alias = "NISTR_EMMC_MODE")]
pub type NistrEmmcMode = crate::Reg<nistr_emmc_mode::NistrEmmcModeSpec>;
#[doc = "Normal Interrupt Status"]
pub mod nistr_emmc_mode;
#[doc = "EISTR (rw) register accessor: Error Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`eistr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eistr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eistr`] module"]
#[doc(alias = "EISTR")]
pub type Eistr = crate::Reg<eistr::EistrSpec>;
#[doc = "Error Interrupt Status"]
pub mod eistr;
#[doc = "EISTR_EMMC_MODE (rw) register accessor: Error Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`eistr_emmc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eistr_emmc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eistr_emmc_mode`] module"]
#[doc(alias = "EISTR_EMMC_MODE")]
pub type EistrEmmcMode = crate::Reg<eistr_emmc_mode::EistrEmmcModeSpec>;
#[doc = "Error Interrupt Status"]
pub mod eistr_emmc_mode;
#[doc = "NISTER (rw) register accessor: Normal Interrupt Status Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`nister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nister`] module"]
#[doc(alias = "NISTER")]
pub type Nister = crate::Reg<nister::NisterSpec>;
#[doc = "Normal Interrupt Status Enable"]
pub mod nister;
#[doc = "NISTER_EMMC_MODE (rw) register accessor: Normal Interrupt Status Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`nister_emmc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nister_emmc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nister_emmc_mode`] module"]
#[doc(alias = "NISTER_EMMC_MODE")]
pub type NisterEmmcMode = crate::Reg<nister_emmc_mode::NisterEmmcModeSpec>;
#[doc = "Normal Interrupt Status Enable"]
pub mod nister_emmc_mode;
#[doc = "EISTER (rw) register accessor: Error Interrupt Status Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eister`] module"]
#[doc(alias = "EISTER")]
pub type Eister = crate::Reg<eister::EisterSpec>;
#[doc = "Error Interrupt Status Enable"]
pub mod eister;
#[doc = "EISTER_EMMC_MODE (rw) register accessor: Error Interrupt Status Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eister_emmc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eister_emmc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eister_emmc_mode`] module"]
#[doc(alias = "EISTER_EMMC_MODE")]
pub type EisterEmmcMode = crate::Reg<eister_emmc_mode::EisterEmmcModeSpec>;
#[doc = "Error Interrupt Status Enable"]
pub mod eister_emmc_mode;
#[doc = "NISIER (rw) register accessor: Normal Interrupt Signal Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`nisier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nisier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nisier`] module"]
#[doc(alias = "NISIER")]
pub type Nisier = crate::Reg<nisier::NisierSpec>;
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier;
#[doc = "NISIER_EMMC_MODE (rw) register accessor: Normal Interrupt Signal Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`nisier_emmc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nisier_emmc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nisier_emmc_mode`] module"]
#[doc(alias = "NISIER_EMMC_MODE")]
pub type NisierEmmcMode = crate::Reg<nisier_emmc_mode::NisierEmmcModeSpec>;
#[doc = "Normal Interrupt Signal Enable"]
pub mod nisier_emmc_mode;
#[doc = "EISIER (rw) register accessor: Error Interrupt Signal Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eisier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eisier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eisier`] module"]
#[doc(alias = "EISIER")]
pub type Eisier = crate::Reg<eisier::EisierSpec>;
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier;
#[doc = "EISIER_EMMC_MODE (rw) register accessor: Error Interrupt Signal Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eisier_emmc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eisier_emmc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eisier_emmc_mode`] module"]
#[doc(alias = "EISIER_EMMC_MODE")]
pub type EisierEmmcMode = crate::Reg<eisier_emmc_mode::EisierEmmcModeSpec>;
#[doc = "Error Interrupt Signal Enable"]
pub mod eisier_emmc_mode;
#[doc = "ACESR (r) register accessor: Auto CMD Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`acesr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acesr`] module"]
#[doc(alias = "ACESR")]
pub type Acesr = crate::Reg<acesr::AcesrSpec>;
#[doc = "Auto CMD Error Status"]
pub mod acesr;
#[doc = "HC2R (rw) register accessor: Host Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hc2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2r`] module"]
#[doc(alias = "HC2R")]
pub type Hc2r = crate::Reg<hc2r::Hc2rSpec>;
#[doc = "Host Control 2"]
pub mod hc2r;
#[doc = "HC2R_EMMC_MODE (rw) register accessor: Host Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hc2r_emmc_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc2r_emmc_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc2r_emmc_mode`] module"]
#[doc(alias = "HC2R_EMMC_MODE")]
pub type Hc2rEmmcMode = crate::Reg<hc2r_emmc_mode::Hc2rEmmcModeSpec>;
#[doc = "Host Control 2"]
pub mod hc2r_emmc_mode;
#[doc = "CA0R (r) register accessor: Capabilities 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ca0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ca0r`] module"]
#[doc(alias = "CA0R")]
pub type Ca0r = crate::Reg<ca0r::Ca0rSpec>;
#[doc = "Capabilities 0"]
pub mod ca0r;
#[doc = "CA1R (r) register accessor: Capabilities 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ca1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ca1r`] module"]
#[doc(alias = "CA1R")]
pub type Ca1r = crate::Reg<ca1r::Ca1rSpec>;
#[doc = "Capabilities 1"]
pub mod ca1r;
#[doc = "MCCAR (r) register accessor: Maximum Current Capabilities\n\nYou can [`read`](crate::Reg::read) this register and get [`mccar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mccar`] module"]
#[doc(alias = "MCCAR")]
pub type Mccar = crate::Reg<mccar::MccarSpec>;
#[doc = "Maximum Current Capabilities"]
pub mod mccar;
#[doc = "FERACES (w) register accessor: Force Event for Auto CMD Error Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feraces::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feraces`] module"]
#[doc(alias = "FERACES")]
pub type Feraces = crate::Reg<feraces::FeracesSpec>;
#[doc = "Force Event for Auto CMD Error Status"]
pub mod feraces;
#[doc = "FEREIS (w) register accessor: Force Event for Error Interrupt Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fereis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fereis`] module"]
#[doc(alias = "FEREIS")]
pub type Fereis = crate::Reg<fereis::FereisSpec>;
#[doc = "Force Event for Error Interrupt Status"]
pub mod fereis;
#[doc = "AESR (r) register accessor: ADMA Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`aesr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesr`] module"]
#[doc(alias = "AESR")]
pub type Aesr = crate::Reg<aesr::AesrSpec>;
#[doc = "ADMA Error Status"]
pub mod aesr;
#[doc = "ASAR (rw) register accessor: ADMA System Address\n\nYou can [`read`](crate::Reg::read) this register and get [`asar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asar`] module"]
#[doc(alias = "ASAR")]
pub type Asar = crate::Reg<asar::AsarSpec>;
#[doc = "ADMA System Address"]
pub mod asar;
#[doc = "PVR (rw) register accessor: Preset Value n\n\nYou can [`read`](crate::Reg::read) this register and get [`pvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pvr`] module"]
#[doc(alias = "PVR")]
pub type Pvr = crate::Reg<pvr::PvrSpec>;
#[doc = "Preset Value n"]
pub mod pvr;
#[doc = "SISR (r) register accessor: Slot Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sisr`] module"]
#[doc(alias = "SISR")]
pub type Sisr = crate::Reg<sisr::SisrSpec>;
#[doc = "Slot Interrupt Status"]
pub mod sisr;
#[doc = "HCVR (r) register accessor: Host Controller Version\n\nYou can [`read`](crate::Reg::read) this register and get [`hcvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcvr`] module"]
#[doc(alias = "HCVR")]
pub type Hcvr = crate::Reg<hcvr::HcvrSpec>;
#[doc = "Host Controller Version"]
pub mod hcvr;
#[doc = "APSR (r) register accessor: Additional Present State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apsr`] module"]
#[doc(alias = "APSR")]
pub type Apsr = crate::Reg<apsr::ApsrSpec>;
#[doc = "Additional Present State Register"]
pub mod apsr;
#[doc = "MC1R (rw) register accessor: MMC Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc1r`] module"]
#[doc(alias = "MC1R")]
pub type Mc1r = crate::Reg<mc1r::Mc1rSpec>;
#[doc = "MMC Control 1"]
pub mod mc1r;
#[doc = "MC2R (w) register accessor: MMC Control 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc2r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc2r`] module"]
#[doc(alias = "MC2R")]
pub type Mc2r = crate::Reg<mc2r::Mc2rSpec>;
#[doc = "MMC Control 2"]
pub mod mc2r;
#[doc = "ACR (rw) register accessor: AHB Control\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "AHB Control"]
pub mod acr;
#[doc = "CC2R (rw) register accessor: Clock Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2r`] module"]
#[doc(alias = "CC2R")]
pub type Cc2r = crate::Reg<cc2r::Cc2rSpec>;
#[doc = "Clock Control 2"]
pub mod cc2r;
#[doc = "CACR (rw) register accessor: Capabilities Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cacr`] module"]
#[doc(alias = "CACR")]
pub type Cacr = crate::Reg<cacr::CacrSpec>;
#[doc = "Capabilities Control"]
pub mod cacr;
#[doc = "DBGR (rw) register accessor: Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgr`] module"]
#[doc(alias = "DBGR")]
pub type Dbgr = crate::Reg<dbgr::DbgrSpec>;
#[doc = "Debug"]
pub mod dbgr;
