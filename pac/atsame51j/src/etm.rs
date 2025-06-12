#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    ccr: Ccr,
    trigger: Trigger,
    _reserved3: [u8; 0x04],
    sr: Sr,
    scr: Scr,
    _reserved5: [u8; 0x08],
    teevr: Teevr,
    tecr1: Tecr1,
    fflr: Fflr,
    _reserved8: [u8; 0x0114],
    cntrldvr1: Cntrldvr1,
    _reserved9: [u8; 0x9c],
    syncfr: Syncfr,
    idr: Idr,
    ccer: Ccer,
    _reserved12: [u8; 0x04],
    tesseicr: Tesseicr,
    _reserved13: [u8; 0x04],
    tsevt: Tsevt,
    _reserved14: [u8; 0x04],
    traceidr: Traceidr,
    _reserved15: [u8; 0x04],
    idr2: Idr2,
    _reserved16: [u8; 0x0108],
    pdsr: Pdsr,
    _reserved17: [u8; 0x0bc8],
    itmiscin: Itmiscin,
    _reserved18: [u8; 0x04],
    ittrigout: Ittrigout,
    _reserved19: [u8; 0x04],
    itatbctr2: Itatbctr2,
    _reserved20: [u8; 0x04],
    itatbctr0: Itatbctr0,
    _reserved21: [u8; 0x04],
    itctrl: Itctrl,
    _reserved22: [u8; 0x9c],
    claimset: Claimset,
    claimclr: Claimclr,
    _reserved24: [u8; 0x08],
    lar: Lar,
    lsr: Lsr,
    authstatus: Authstatus,
    _reserved27: [u8; 0x10],
    devtype: Devtype,
    pidr4: Pidr4,
    pidr5: Pidr5,
    pidr6: Pidr6,
    pidr7: Pidr7,
    pidr0: Pidr0,
    pidr1: Pidr1,
    pidr2: Pidr2,
    pidr3: Pidr3,
    cidr0: Cidr0,
    cidr1: Cidr1,
    cidr2: Cidr2,
    cidr3: Cidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - ETM Main Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - ETM Configuration Code Register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x08 - ETM Trigger Event Register"]
    #[inline(always)]
    pub const fn trigger(&self) -> &Trigger {
        &self.trigger
    }
    #[doc = "0x10 - ETM Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - ETM System Configuration Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x20 - ETM TraceEnable Event Register"]
    #[inline(always)]
    pub const fn teevr(&self) -> &Teevr {
        &self.teevr
    }
    #[doc = "0x24 - ETM TraceEnable Control 1 Register"]
    #[inline(always)]
    pub const fn tecr1(&self) -> &Tecr1 {
        &self.tecr1
    }
    #[doc = "0x28 - ETM FIFO Full Level Register"]
    #[inline(always)]
    pub const fn fflr(&self) -> &Fflr {
        &self.fflr
    }
    #[doc = "0x140 - ETM Free-running Counter Reload Value"]
    #[inline(always)]
    pub const fn cntrldvr1(&self) -> &Cntrldvr1 {
        &self.cntrldvr1
    }
    #[doc = "0x1e0 - ETM Synchronization Frequency Register"]
    #[inline(always)]
    pub const fn syncfr(&self) -> &Syncfr {
        &self.syncfr
    }
    #[doc = "0x1e4 - ETM ID Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x1e8 - ETM Configuration Code Extension Register"]
    #[inline(always)]
    pub const fn ccer(&self) -> &Ccer {
        &self.ccer
    }
    #[doc = "0x1f0 - ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
    #[inline(always)]
    pub const fn tesseicr(&self) -> &Tesseicr {
        &self.tesseicr
    }
    #[doc = "0x1f8 - ETM TimeStamp Event Register"]
    #[inline(always)]
    pub const fn tsevt(&self) -> &Tsevt {
        &self.tsevt
    }
    #[doc = "0x200 - ETM CoreSight Trace ID Register"]
    #[inline(always)]
    pub const fn traceidr(&self) -> &Traceidr {
        &self.traceidr
    }
    #[doc = "0x208 - ETM ID Register 2"]
    #[inline(always)]
    pub const fn idr2(&self) -> &Idr2 {
        &self.idr2
    }
    #[doc = "0x314 - ETM Device Power-Down Status Register"]
    #[inline(always)]
    pub const fn pdsr(&self) -> &Pdsr {
        &self.pdsr
    }
    #[doc = "0xee0 - ETM Integration Test Miscellaneous Inputs"]
    #[inline(always)]
    pub const fn itmiscin(&self) -> &Itmiscin {
        &self.itmiscin
    }
    #[doc = "0xee8 - ETM Integration Test Trigger Out"]
    #[inline(always)]
    pub const fn ittrigout(&self) -> &Ittrigout {
        &self.ittrigout
    }
    #[doc = "0xef0 - ETM Integration Test ATB Control 2"]
    #[inline(always)]
    pub const fn itatbctr2(&self) -> &Itatbctr2 {
        &self.itatbctr2
    }
    #[doc = "0xef8 - ETM Integration Test ATB Control 0"]
    #[inline(always)]
    pub const fn itatbctr0(&self) -> &Itatbctr0 {
        &self.itatbctr0
    }
    #[doc = "0xf00 - ETM Integration Mode Control Register"]
    #[inline(always)]
    pub const fn itctrl(&self) -> &Itctrl {
        &self.itctrl
    }
    #[doc = "0xfa0 - ETM Claim Tag Set Register"]
    #[inline(always)]
    pub const fn claimset(&self) -> &Claimset {
        &self.claimset
    }
    #[doc = "0xfa4 - ETM Claim Tag Clear Register"]
    #[inline(always)]
    pub const fn claimclr(&self) -> &Claimclr {
        &self.claimclr
    }
    #[doc = "0xfb0 - ETM Lock Access Register"]
    #[inline(always)]
    pub const fn lar(&self) -> &Lar {
        &self.lar
    }
    #[doc = "0xfb4 - ETM Lock Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0xfb8 - ETM Authentication Status Register"]
    #[inline(always)]
    pub const fn authstatus(&self) -> &Authstatus {
        &self.authstatus
    }
    #[doc = "0xfcc - ETM CoreSight Device Type Register"]
    #[inline(always)]
    pub const fn devtype(&self) -> &Devtype {
        &self.devtype
    }
    #[doc = "0xfd0 - ETM Peripheral Identification Register #4"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfd4 - ETM Peripheral Identification Register #5"]
    #[inline(always)]
    pub const fn pidr5(&self) -> &Pidr5 {
        &self.pidr5
    }
    #[doc = "0xfd8 - ETM Peripheral Identification Register #6"]
    #[inline(always)]
    pub const fn pidr6(&self) -> &Pidr6 {
        &self.pidr6
    }
    #[doc = "0xfdc - ETM Peripheral Identification Register #7"]
    #[inline(always)]
    pub const fn pidr7(&self) -> &Pidr7 {
        &self.pidr7
    }
    #[doc = "0xfe0 - ETM Peripheral Identification Register #0"]
    #[inline(always)]
    pub const fn pidr0(&self) -> &Pidr0 {
        &self.pidr0
    }
    #[doc = "0xfe4 - ETM Peripheral Identification Register #1"]
    #[inline(always)]
    pub const fn pidr1(&self) -> &Pidr1 {
        &self.pidr1
    }
    #[doc = "0xfe8 - ETM Peripheral Identification Register #2"]
    #[inline(always)]
    pub const fn pidr2(&self) -> &Pidr2 {
        &self.pidr2
    }
    #[doc = "0xfec - ETM Peripheral Identification Register #3"]
    #[inline(always)]
    pub const fn pidr3(&self) -> &Pidr3 {
        &self.pidr3
    }
    #[doc = "0xff0 - ETM Component Identification Register #0"]
    #[inline(always)]
    pub const fn cidr0(&self) -> &Cidr0 {
        &self.cidr0
    }
    #[doc = "0xff4 - ETM Component Identification Register #1"]
    #[inline(always)]
    pub const fn cidr1(&self) -> &Cidr1 {
        &self.cidr1
    }
    #[doc = "0xff8 - ETM Component Identification Register #2"]
    #[inline(always)]
    pub const fn cidr2(&self) -> &Cidr2 {
        &self.cidr2
    }
    #[doc = "0xffc - ETM Component Identification Register #3"]
    #[inline(always)]
    pub const fn cidr3(&self) -> &Cidr3 {
        &self.cidr3
    }
}
#[doc = "CR (rw) register accessor: ETM Main Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "ETM Main Control Register"]
pub mod cr;
#[doc = "CCR (r) register accessor: ETM Configuration Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "ETM Configuration Code Register"]
pub mod ccr;
#[doc = "TRIGGER (rw) register accessor: ETM Trigger Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
#[doc(alias = "TRIGGER")]
pub type Trigger = crate::Reg<trigger::TriggerSpec>;
#[doc = "ETM Trigger Event Register"]
pub mod trigger;
#[doc = "SR (rw) register accessor: ETM Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "ETM Status Register"]
pub mod sr;
#[doc = "SCR (r) register accessor: ETM System Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "ETM System Configuration Register"]
pub mod scr;
#[doc = "TEEVR (rw) register accessor: ETM TraceEnable Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`teevr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`teevr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@teevr`] module"]
#[doc(alias = "TEEVR")]
pub type Teevr = crate::Reg<teevr::TeevrSpec>;
#[doc = "ETM TraceEnable Event Register"]
pub mod teevr;
#[doc = "TECR1 (rw) register accessor: ETM TraceEnable Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tecr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tecr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tecr1`] module"]
#[doc(alias = "TECR1")]
pub type Tecr1 = crate::Reg<tecr1::Tecr1Spec>;
#[doc = "ETM TraceEnable Control 1 Register"]
pub mod tecr1;
#[doc = "FFLR (rw) register accessor: ETM FIFO Full Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fflr`] module"]
#[doc(alias = "FFLR")]
pub type Fflr = crate::Reg<fflr::FflrSpec>;
#[doc = "ETM FIFO Full Level Register"]
pub mod fflr;
#[doc = "CNTRLDVR1 (rw) register accessor: ETM Free-running Counter Reload Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cntrldvr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntrldvr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntrldvr1`] module"]
#[doc(alias = "CNTRLDVR1")]
pub type Cntrldvr1 = crate::Reg<cntrldvr1::Cntrldvr1Spec>;
#[doc = "ETM Free-running Counter Reload Value"]
pub mod cntrldvr1;
#[doc = "SYNCFR (r) register accessor: ETM Synchronization Frequency Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncfr`] module"]
#[doc(alias = "SYNCFR")]
pub type Syncfr = crate::Reg<syncfr::SyncfrSpec>;
#[doc = "ETM Synchronization Frequency Register"]
pub mod syncfr;
#[doc = "IDR (r) register accessor: ETM ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "ETM ID Register"]
pub mod idr;
#[doc = "CCER (r) register accessor: ETM Configuration Code Extension Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`] module"]
#[doc(alias = "CCER")]
pub type Ccer = crate::Reg<ccer::CcerSpec>;
#[doc = "ETM Configuration Code Extension Register"]
pub mod ccer;
#[doc = "TESSEICR (rw) register accessor: ETM TraceEnable Start/Stop EmbeddedICE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tesseicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tesseicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tesseicr`] module"]
#[doc(alias = "TESSEICR")]
pub type Tesseicr = crate::Reg<tesseicr::TesseicrSpec>;
#[doc = "ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod tesseicr;
#[doc = "TSEVT (rw) register accessor: ETM TimeStamp Event Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsevt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsevt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsevt`] module"]
#[doc(alias = "TSEVT")]
pub type Tsevt = crate::Reg<tsevt::TsevtSpec>;
#[doc = "ETM TimeStamp Event Register"]
pub mod tsevt;
#[doc = "TRACEIDR (rw) register accessor: ETM CoreSight Trace ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`traceidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceidr`] module"]
#[doc(alias = "TRACEIDR")]
pub type Traceidr = crate::Reg<traceidr::TraceidrSpec>;
#[doc = "ETM CoreSight Trace ID Register"]
pub mod traceidr;
#[doc = "IDR2 (r) register accessor: ETM ID Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`idr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr2`] module"]
#[doc(alias = "IDR2")]
pub type Idr2 = crate::Reg<idr2::Idr2Spec>;
#[doc = "ETM ID Register 2"]
pub mod idr2;
#[doc = "PDSR (r) register accessor: ETM Device Power-Down Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsr`] module"]
#[doc(alias = "PDSR")]
pub type Pdsr = crate::Reg<pdsr::PdsrSpec>;
#[doc = "ETM Device Power-Down Status Register"]
pub mod pdsr;
#[doc = "ITMISCIN (r) register accessor: ETM Integration Test Miscellaneous Inputs\n\nYou can [`read`](crate::Reg::read) this register and get [`itmiscin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itmiscin`] module"]
#[doc(alias = "ITMISCIN")]
pub type Itmiscin = crate::Reg<itmiscin::ItmiscinSpec>;
#[doc = "ETM Integration Test Miscellaneous Inputs"]
pub mod itmiscin;
#[doc = "ITTRIGOUT (w) register accessor: ETM Integration Test Trigger Out\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ittrigout::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ittrigout`] module"]
#[doc(alias = "ITTRIGOUT")]
pub type Ittrigout = crate::Reg<ittrigout::IttrigoutSpec>;
#[doc = "ETM Integration Test Trigger Out"]
pub mod ittrigout;
#[doc = "ITATBCTR2 (r) register accessor: ETM Integration Test ATB Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`itatbctr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr2`] module"]
#[doc(alias = "ITATBCTR2")]
pub type Itatbctr2 = crate::Reg<itatbctr2::Itatbctr2Spec>;
#[doc = "ETM Integration Test ATB Control 2"]
pub mod itatbctr2;
#[doc = "ITATBCTR0 (w) register accessor: ETM Integration Test ATB Control 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itatbctr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr0`] module"]
#[doc(alias = "ITATBCTR0")]
pub type Itatbctr0 = crate::Reg<itatbctr0::Itatbctr0Spec>;
#[doc = "ETM Integration Test ATB Control 0"]
pub mod itatbctr0;
#[doc = "ITCTRL (rw) register accessor: ETM Integration Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`itctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itctrl`] module"]
#[doc(alias = "ITCTRL")]
pub type Itctrl = crate::Reg<itctrl::ItctrlSpec>;
#[doc = "ETM Integration Mode Control Register"]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: ETM Claim Tag Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`claimset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimset`] module"]
#[doc(alias = "CLAIMSET")]
pub type Claimset = crate::Reg<claimset::ClaimsetSpec>;
#[doc = "ETM Claim Tag Set Register"]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: ETM Claim Tag Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`claimclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimclr`] module"]
#[doc(alias = "CLAIMCLR")]
pub type Claimclr = crate::Reg<claimclr::ClaimclrSpec>;
#[doc = "ETM Claim Tag Clear Register"]
pub mod claimclr;
#[doc = "LAR (w) register accessor: ETM Lock Access Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lar`] module"]
#[doc(alias = "LAR")]
pub type Lar = crate::Reg<lar::LarSpec>;
#[doc = "ETM Lock Access Register"]
pub mod lar;
#[doc = "LSR (r) register accessor: ETM Lock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "ETM Lock Status Register"]
pub mod lsr;
#[doc = "AUTHSTATUS (r) register accessor: ETM Authentication Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`authstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authstatus`] module"]
#[doc(alias = "AUTHSTATUS")]
pub type Authstatus = crate::Reg<authstatus::AuthstatusSpec>;
#[doc = "ETM Authentication Status Register"]
pub mod authstatus;
#[doc = "DEVTYPE (r) register accessor: ETM CoreSight Device Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`] module"]
#[doc(alias = "DEVTYPE")]
pub type Devtype = crate::Reg<devtype::DevtypeSpec>;
#[doc = "ETM CoreSight Device Type Register"]
pub mod devtype;
#[doc = "PIDR4 (r) register accessor: ETM Peripheral Identification Register #4\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`] module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "ETM Peripheral Identification Register #4"]
pub mod pidr4;
#[doc = "PIDR5 (r) register accessor: ETM Peripheral Identification Register #5\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr5`] module"]
#[doc(alias = "PIDR5")]
pub type Pidr5 = crate::Reg<pidr5::Pidr5Spec>;
#[doc = "ETM Peripheral Identification Register #5"]
pub mod pidr5;
#[doc = "PIDR6 (r) register accessor: ETM Peripheral Identification Register #6\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr6`] module"]
#[doc(alias = "PIDR6")]
pub type Pidr6 = crate::Reg<pidr6::Pidr6Spec>;
#[doc = "ETM Peripheral Identification Register #6"]
pub mod pidr6;
#[doc = "PIDR7 (r) register accessor: ETM Peripheral Identification Register #7\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr7`] module"]
#[doc(alias = "PIDR7")]
pub type Pidr7 = crate::Reg<pidr7::Pidr7Spec>;
#[doc = "ETM Peripheral Identification Register #7"]
pub mod pidr7;
#[doc = "PIDR0 (r) register accessor: ETM Peripheral Identification Register #0\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`] module"]
#[doc(alias = "PIDR0")]
pub type Pidr0 = crate::Reg<pidr0::Pidr0Spec>;
#[doc = "ETM Peripheral Identification Register #0"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: ETM Peripheral Identification Register #1\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`] module"]
#[doc(alias = "PIDR1")]
pub type Pidr1 = crate::Reg<pidr1::Pidr1Spec>;
#[doc = "ETM Peripheral Identification Register #1"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: ETM Peripheral Identification Register #2\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`] module"]
#[doc(alias = "PIDR2")]
pub type Pidr2 = crate::Reg<pidr2::Pidr2Spec>;
#[doc = "ETM Peripheral Identification Register #2"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: ETM Peripheral Identification Register #3\n\nYou can [`read`](crate::Reg::read) this register and get [`pidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`] module"]
#[doc(alias = "PIDR3")]
pub type Pidr3 = crate::Reg<pidr3::Pidr3Spec>;
#[doc = "ETM Peripheral Identification Register #3"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: ETM Component Identification Register #0\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`] module"]
#[doc(alias = "CIDR0")]
pub type Cidr0 = crate::Reg<cidr0::Cidr0Spec>;
#[doc = "ETM Component Identification Register #0"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: ETM Component Identification Register #1\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`] module"]
#[doc(alias = "CIDR1")]
pub type Cidr1 = crate::Reg<cidr1::Cidr1Spec>;
#[doc = "ETM Component Identification Register #1"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: ETM Component Identification Register #2\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`] module"]
#[doc(alias = "CIDR2")]
pub type Cidr2 = crate::Reg<cidr2::Cidr2Spec>;
#[doc = "ETM Component Identification Register #2"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: ETM Component Identification Register #3\n\nYou can [`read`](crate::Reg::read) this register and get [`cidr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`] module"]
#[doc(alias = "CIDR3")]
pub type Cidr3 = crate::Reg<cidr3::Cidr3Spec>;
#[doc = "ETM Component Identification Register #3"]
pub mod cidr3;
