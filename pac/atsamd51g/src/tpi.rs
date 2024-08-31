#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sspsr: Sspsr,
    cspsr: Cspsr,
    _reserved2: [u8; 0x08],
    acpr: Acpr,
    _reserved3: [u8; 0xdc],
    sppr: Sppr,
    _reserved4: [u8; 0x020c],
    ffsr: Ffsr,
    ffcr: Ffcr,
    fscr: Fscr,
    _reserved7: [u8; 0x0bdc],
    trigger: Trigger,
    fifo0: Fifo0,
    itatbctr2: Itatbctr2,
    _reserved10: [u8; 0x04],
    itatbctr0: Itatbctr0,
    fifo1: Fifo1,
    itctrl: Itctrl,
    _reserved13: [u8; 0x9c],
    claimset: Claimset,
    claimclr: Claimclr,
    _reserved15: [u8; 0x20],
    devid: Devid,
    devtype: Devtype,
}
impl RegisterBlock {
    #[doc = "0x00 - Supported Parallel Port Size Register"]
    #[inline(always)]
    pub const fn sspsr(&self) -> &Sspsr {
        &self.sspsr
    }
    #[doc = "0x04 - Current Parallel Port Size Register"]
    #[inline(always)]
    pub const fn cspsr(&self) -> &Cspsr {
        &self.cspsr
    }
    #[doc = "0x10 - Asynchronous Clock Prescaler Register"]
    #[inline(always)]
    pub const fn acpr(&self) -> &Acpr {
        &self.acpr
    }
    #[doc = "0xf0 - Selected Pin Protocol Register"]
    #[inline(always)]
    pub const fn sppr(&self) -> &Sppr {
        &self.sppr
    }
    #[doc = "0x300 - Formatter and Flush Status Register"]
    #[inline(always)]
    pub const fn ffsr(&self) -> &Ffsr {
        &self.ffsr
    }
    #[doc = "0x304 - Formatter and Flush Control Register"]
    #[inline(always)]
    pub const fn ffcr(&self) -> &Ffcr {
        &self.ffcr
    }
    #[doc = "0x308 - Formatter Synchronization Counter Register"]
    #[inline(always)]
    pub const fn fscr(&self) -> &Fscr {
        &self.fscr
    }
    #[doc = "0xee8 - TRIGGER"]
    #[inline(always)]
    pub const fn trigger(&self) -> &Trigger {
        &self.trigger
    }
    #[doc = "0xeec - Integration ETM Data"]
    #[inline(always)]
    pub const fn fifo0(&self) -> &Fifo0 {
        &self.fifo0
    }
    #[doc = "0xef0 - ITATBCTR2"]
    #[inline(always)]
    pub const fn itatbctr2(&self) -> &Itatbctr2 {
        &self.itatbctr2
    }
    #[doc = "0xef8 - ITATBCTR0"]
    #[inline(always)]
    pub const fn itatbctr0(&self) -> &Itatbctr0 {
        &self.itatbctr0
    }
    #[doc = "0xefc - Integration ITM Data"]
    #[inline(always)]
    pub const fn fifo1(&self) -> &Fifo1 {
        &self.fifo1
    }
    #[doc = "0xf00 - Integration Mode Control"]
    #[inline(always)]
    pub const fn itctrl(&self) -> &Itctrl {
        &self.itctrl
    }
    #[doc = "0xfa0 - Claim tag set"]
    #[inline(always)]
    pub const fn claimset(&self) -> &Claimset {
        &self.claimset
    }
    #[doc = "0xfa4 - Claim tag clear"]
    #[inline(always)]
    pub const fn claimclr(&self) -> &Claimclr {
        &self.claimclr
    }
    #[doc = "0xfc8 - TPIU_DEVID"]
    #[inline(always)]
    pub const fn devid(&self) -> &Devid {
        &self.devid
    }
    #[doc = "0xfcc - TPIU_DEVTYPE"]
    #[inline(always)]
    pub const fn devtype(&self) -> &Devtype {
        &self.devtype
    }
}
#[doc = "SSPSR (r) register accessor: Supported Parallel Port Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sspsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspsr`]
module"]
#[doc(alias = "SSPSR")]
pub type Sspsr = crate::Reg<sspsr::SspsrSpec>;
#[doc = "Supported Parallel Port Size Register"]
pub mod sspsr;
#[doc = "CSPSR (rw) register accessor: Current Parallel Port Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cspsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cspsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspsr`]
module"]
#[doc(alias = "CSPSR")]
pub type Cspsr = crate::Reg<cspsr::CspsrSpec>;
#[doc = "Current Parallel Port Size Register"]
pub mod cspsr;
#[doc = "ACPR (rw) register accessor: Asynchronous Clock Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`acpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acpr`]
module"]
#[doc(alias = "ACPR")]
pub type Acpr = crate::Reg<acpr::AcprSpec>;
#[doc = "Asynchronous Clock Prescaler Register"]
pub mod acpr;
#[doc = "SPPR (rw) register accessor: Selected Pin Protocol Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sppr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sppr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sppr`]
module"]
#[doc(alias = "SPPR")]
pub type Sppr = crate::Reg<sppr::SpprSpec>;
#[doc = "Selected Pin Protocol Register"]
pub mod sppr;
#[doc = "FFSR (r) register accessor: Formatter and Flush Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffsr`]
module"]
#[doc(alias = "FFSR")]
pub type Ffsr = crate::Reg<ffsr::FfsrSpec>;
#[doc = "Formatter and Flush Status Register"]
pub mod ffsr;
#[doc = "FFCR (rw) register accessor: Formatter and Flush Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ffcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffcr`]
module"]
#[doc(alias = "FFCR")]
pub type Ffcr = crate::Reg<ffcr::FfcrSpec>;
#[doc = "Formatter and Flush Control Register"]
pub mod ffcr;
#[doc = "FSCR (r) register accessor: Formatter Synchronization Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fscr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fscr`]
module"]
#[doc(alias = "FSCR")]
pub type Fscr = crate::Reg<fscr::FscrSpec>;
#[doc = "Formatter Synchronization Counter Register"]
pub mod fscr;
#[doc = "TRIGGER (r) register accessor: TRIGGER\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`]
module"]
#[doc(alias = "TRIGGER")]
pub type Trigger = crate::Reg<trigger::TriggerSpec>;
#[doc = "TRIGGER"]
pub mod trigger;
#[doc = "FIFO0 (r) register accessor: Integration ETM Data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo0`]
module"]
#[doc(alias = "FIFO0")]
pub type Fifo0 = crate::Reg<fifo0::Fifo0Spec>;
#[doc = "Integration ETM Data"]
pub mod fifo0;
#[doc = "ITATBCTR2 (r) register accessor: ITATBCTR2\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr2`]
module"]
#[doc(alias = "ITATBCTR2")]
pub type Itatbctr2 = crate::Reg<itatbctr2::Itatbctr2Spec>;
#[doc = "ITATBCTR2"]
pub mod itatbctr2;
#[doc = "ITATBCTR0 (r) register accessor: ITATBCTR0\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr0`]
module"]
#[doc(alias = "ITATBCTR0")]
pub type Itatbctr0 = crate::Reg<itatbctr0::Itatbctr0Spec>;
#[doc = "ITATBCTR0"]
pub mod itatbctr0;
#[doc = "FIFO1 (r) register accessor: Integration ITM Data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo1`]
module"]
#[doc(alias = "FIFO1")]
pub type Fifo1 = crate::Reg<fifo1::Fifo1Spec>;
#[doc = "Integration ITM Data"]
pub mod fifo1;
#[doc = "ITCTRL (rw) register accessor: Integration Mode Control\n\nYou can [`read`](crate::Reg::read) this register and get [`itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itctrl`]
module"]
#[doc(alias = "ITCTRL")]
pub type Itctrl = crate::Reg<itctrl::ItctrlSpec>;
#[doc = "Integration Mode Control"]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: Claim tag set\n\nYou can [`read`](crate::Reg::read) this register and get [`claimset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimset`]
module"]
#[doc(alias = "CLAIMSET")]
pub type Claimset = crate::Reg<claimset::ClaimsetSpec>;
#[doc = "Claim tag set"]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: Claim tag clear\n\nYou can [`read`](crate::Reg::read) this register and get [`claimclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimclr`]
module"]
#[doc(alias = "CLAIMCLR")]
pub type Claimclr = crate::Reg<claimclr::ClaimclrSpec>;
#[doc = "Claim tag clear"]
pub mod claimclr;
#[doc = "DEVID (r) register accessor: TPIU_DEVID\n\nYou can [`read`](crate::Reg::read) this register and get [`devid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devid`]
module"]
#[doc(alias = "DEVID")]
pub type Devid = crate::Reg<devid::DevidSpec>;
#[doc = "TPIU_DEVID"]
pub mod devid;
#[doc = "DEVTYPE (r) register accessor: TPIU_DEVTYPE\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`]
module"]
#[doc(alias = "DEVTYPE")]
pub type Devtype = crate::Reg<devtype::DevtypeSpec>;
#[doc = "TPIU_DEVTYPE"]
pub mod devtype;
