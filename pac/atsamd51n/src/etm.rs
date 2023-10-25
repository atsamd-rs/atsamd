#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ETM Main Control Register"]
    pub cr: CR,
    #[doc = "0x04 - ETM Configuration Code Register"]
    pub ccr: CCR,
    #[doc = "0x08 - ETM Trigger Event Register"]
    pub trigger: TRIGGER,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - ETM Status Register"]
    pub sr: SR,
    #[doc = "0x14 - ETM System Configuration Register"]
    pub scr: SCR,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - ETM TraceEnable Event Register"]
    pub teevr: TEEVR,
    #[doc = "0x24 - ETM TraceEnable Control 1 Register"]
    pub tecr1: TECR1,
    #[doc = "0x28 - ETM FIFO Full Level Register"]
    pub fflr: FFLR,
    _reserved8: [u8; 0x0114],
    #[doc = "0x140 - ETM Free-running Counter Reload Value"]
    pub cntrldvr1: CNTRLDVR1,
    _reserved9: [u8; 0x9c],
    #[doc = "0x1e0 - ETM Synchronization Frequency Register"]
    pub syncfr: SYNCFR,
    #[doc = "0x1e4 - ETM ID Register"]
    pub idr: IDR,
    #[doc = "0x1e8 - ETM Configuration Code Extension Register"]
    pub ccer: CCER,
    _reserved12: [u8; 0x04],
    #[doc = "0x1f0 - ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
    pub tesseicr: TESSEICR,
    _reserved13: [u8; 0x04],
    #[doc = "0x1f8 - ETM TimeStamp Event Register"]
    pub tsevt: TSEVT,
    _reserved14: [u8; 0x04],
    #[doc = "0x200 - ETM CoreSight Trace ID Register"]
    pub traceidr: TRACEIDR,
    _reserved15: [u8; 0x04],
    #[doc = "0x208 - ETM ID Register 2"]
    pub idr2: IDR2,
    _reserved16: [u8; 0x0108],
    #[doc = "0x314 - ETM Device Power-Down Status Register"]
    pub pdsr: PDSR,
    _reserved17: [u8; 0x0bc8],
    #[doc = "0xee0 - ETM Integration Test Miscellaneous Inputs"]
    pub itmiscin: ITMISCIN,
    _reserved18: [u8; 0x04],
    #[doc = "0xee8 - ETM Integration Test Trigger Out"]
    pub ittrigout: ITTRIGOUT,
    _reserved19: [u8; 0x04],
    #[doc = "0xef0 - ETM Integration Test ATB Control 2"]
    pub itatbctr2: ITATBCTR2,
    _reserved20: [u8; 0x04],
    #[doc = "0xef8 - ETM Integration Test ATB Control 0"]
    pub itatbctr0: ITATBCTR0,
    _reserved21: [u8; 0x04],
    #[doc = "0xf00 - ETM Integration Mode Control Register"]
    pub itctrl: ITCTRL,
    _reserved22: [u8; 0x9c],
    #[doc = "0xfa0 - ETM Claim Tag Set Register"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - ETM Claim Tag Clear Register"]
    pub claimclr: CLAIMCLR,
    _reserved24: [u8; 0x08],
    #[doc = "0xfb0 - ETM Lock Access Register"]
    pub lar: LAR,
    #[doc = "0xfb4 - ETM Lock Status Register"]
    pub lsr: LSR,
    #[doc = "0xfb8 - ETM Authentication Status Register"]
    pub authstatus: AUTHSTATUS,
    _reserved27: [u8; 0x10],
    #[doc = "0xfcc - ETM CoreSight Device Type Register"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - ETM Peripheral Identification Register #4"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - ETM Peripheral Identification Register #5"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - ETM Peripheral Identification Register #6"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - ETM Peripheral Identification Register #7"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - ETM Peripheral Identification Register #0"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - ETM Peripheral Identification Register #1"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - ETM Peripheral Identification Register #2"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - ETM Peripheral Identification Register #3"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - ETM Component Identification Register #0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - ETM Component Identification Register #1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - ETM Component Identification Register #2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - ETM Component Identification Register #3"]
    pub cidr3: CIDR3,
}
#[doc = "CR (rw) register accessor: ETM Main Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ETM Main Control Register"]
pub mod cr;
#[doc = "CCR (r) register accessor: ETM Configuration Code Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "ETM Configuration Code Register"]
pub mod ccr;
#[doc = "TRIGGER (rw) register accessor: ETM Trigger Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigger::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`]
module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "ETM Trigger Event Register"]
pub mod trigger;
#[doc = "SR (rw) register accessor: ETM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "ETM Status Register"]
pub mod sr;
#[doc = "SCR (r) register accessor: ETM System Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "ETM System Configuration Register"]
pub mod scr;
#[doc = "TEEVR (rw) register accessor: ETM TraceEnable Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`teevr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`teevr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@teevr`]
module"]
pub type TEEVR = crate::Reg<teevr::TEEVR_SPEC>;
#[doc = "ETM TraceEnable Event Register"]
pub mod teevr;
#[doc = "TECR1 (rw) register accessor: ETM TraceEnable Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tecr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tecr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tecr1`]
module"]
pub type TECR1 = crate::Reg<tecr1::TECR1_SPEC>;
#[doc = "ETM TraceEnable Control 1 Register"]
pub mod tecr1;
#[doc = "FFLR (rw) register accessor: ETM FIFO Full Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fflr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fflr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fflr`]
module"]
pub type FFLR = crate::Reg<fflr::FFLR_SPEC>;
#[doc = "ETM FIFO Full Level Register"]
pub mod fflr;
#[doc = "CNTRLDVR1 (rw) register accessor: ETM Free-running Counter Reload Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntrldvr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntrldvr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntrldvr1`]
module"]
pub type CNTRLDVR1 = crate::Reg<cntrldvr1::CNTRLDVR1_SPEC>;
#[doc = "ETM Free-running Counter Reload Value"]
pub mod cntrldvr1;
#[doc = "SYNCFR (r) register accessor: ETM Synchronization Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncfr`]
module"]
pub type SYNCFR = crate::Reg<syncfr::SYNCFR_SPEC>;
#[doc = "ETM Synchronization Frequency Register"]
pub mod syncfr;
#[doc = "IDR (r) register accessor: ETM ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "ETM ID Register"]
pub mod idr;
#[doc = "CCER (r) register accessor: ETM Configuration Code Extension Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccer::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`]
module"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "ETM Configuration Code Extension Register"]
pub mod ccer;
#[doc = "TESSEICR (rw) register accessor: ETM TraceEnable Start/Stop EmbeddedICE Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tesseicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tesseicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tesseicr`]
module"]
pub type TESSEICR = crate::Reg<tesseicr::TESSEICR_SPEC>;
#[doc = "ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod tesseicr;
#[doc = "TSEVT (rw) register accessor: ETM TimeStamp Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsevt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsevt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsevt`]
module"]
pub type TSEVT = crate::Reg<tsevt::TSEVT_SPEC>;
#[doc = "ETM TimeStamp Event Register"]
pub mod tsevt;
#[doc = "TRACEIDR (rw) register accessor: ETM CoreSight Trace ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`traceidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`traceidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceidr`]
module"]
pub type TRACEIDR = crate::Reg<traceidr::TRACEIDR_SPEC>;
#[doc = "ETM CoreSight Trace ID Register"]
pub mod traceidr;
#[doc = "IDR2 (r) register accessor: ETM ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr2`]
module"]
pub type IDR2 = crate::Reg<idr2::IDR2_SPEC>;
#[doc = "ETM ID Register 2"]
pub mod idr2;
#[doc = "PDSR (r) register accessor: ETM Device Power-Down Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsr`]
module"]
pub type PDSR = crate::Reg<pdsr::PDSR_SPEC>;
#[doc = "ETM Device Power-Down Status Register"]
pub mod pdsr;
#[doc = "ITMISCIN (r) register accessor: ETM Integration Test Miscellaneous Inputs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itmiscin::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itmiscin`]
module"]
pub type ITMISCIN = crate::Reg<itmiscin::ITMISCIN_SPEC>;
#[doc = "ETM Integration Test Miscellaneous Inputs"]
pub mod itmiscin;
#[doc = "ITTRIGOUT (w) register accessor: ETM Integration Test Trigger Out\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ittrigout::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ittrigout`]
module"]
pub type ITTRIGOUT = crate::Reg<ittrigout::ITTRIGOUT_SPEC>;
#[doc = "ETM Integration Test Trigger Out"]
pub mod ittrigout;
#[doc = "ITATBCTR2 (r) register accessor: ETM Integration Test ATB Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itatbctr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr2`]
module"]
pub type ITATBCTR2 = crate::Reg<itatbctr2::ITATBCTR2_SPEC>;
#[doc = "ETM Integration Test ATB Control 2"]
pub mod itatbctr2;
#[doc = "ITATBCTR0 (w) register accessor: ETM Integration Test ATB Control 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itatbctr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itatbctr0`]
module"]
pub type ITATBCTR0 = crate::Reg<itatbctr0::ITATBCTR0_SPEC>;
#[doc = "ETM Integration Test ATB Control 0"]
pub mod itatbctr0;
#[doc = "ITCTRL (rw) register accessor: ETM Integration Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itctrl`]
module"]
pub type ITCTRL = crate::Reg<itctrl::ITCTRL_SPEC>;
#[doc = "ETM Integration Mode Control Register"]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: ETM Claim Tag Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimset`]
module"]
pub type CLAIMSET = crate::Reg<claimset::CLAIMSET_SPEC>;
#[doc = "ETM Claim Tag Set Register"]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: ETM Claim Tag Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimclr`]
module"]
pub type CLAIMCLR = crate::Reg<claimclr::CLAIMCLR_SPEC>;
#[doc = "ETM Claim Tag Clear Register"]
pub mod claimclr;
#[doc = "LAR (w) register accessor: ETM Lock Access Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lar`]
module"]
pub type LAR = crate::Reg<lar::LAR_SPEC>;
#[doc = "ETM Lock Access Register"]
pub mod lar;
#[doc = "LSR (r) register accessor: ETM Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "ETM Lock Status Register"]
pub mod lsr;
#[doc = "AUTHSTATUS (r) register accessor: ETM Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`authstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authstatus`]
module"]
pub type AUTHSTATUS = crate::Reg<authstatus::AUTHSTATUS_SPEC>;
#[doc = "ETM Authentication Status Register"]
pub mod authstatus;
#[doc = "DEVTYPE (r) register accessor: ETM CoreSight Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devtype::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`]
module"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "ETM CoreSight Device Type Register"]
pub mod devtype;
#[doc = "PIDR4 (r) register accessor: ETM Peripheral Identification Register #4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`]
module"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "ETM Peripheral Identification Register #4"]
pub mod pidr4;
#[doc = "PIDR5 (r) register accessor: ETM Peripheral Identification Register #5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr5`]
module"]
pub type PIDR5 = crate::Reg<pidr5::PIDR5_SPEC>;
#[doc = "ETM Peripheral Identification Register #5"]
pub mod pidr5;
#[doc = "PIDR6 (r) register accessor: ETM Peripheral Identification Register #6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr6`]
module"]
pub type PIDR6 = crate::Reg<pidr6::PIDR6_SPEC>;
#[doc = "ETM Peripheral Identification Register #6"]
pub mod pidr6;
#[doc = "PIDR7 (r) register accessor: ETM Peripheral Identification Register #7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr7`]
module"]
pub type PIDR7 = crate::Reg<pidr7::PIDR7_SPEC>;
#[doc = "ETM Peripheral Identification Register #7"]
pub mod pidr7;
#[doc = "PIDR0 (r) register accessor: ETM Peripheral Identification Register #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`]
module"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "ETM Peripheral Identification Register #0"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: ETM Peripheral Identification Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`]
module"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "ETM Peripheral Identification Register #1"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: ETM Peripheral Identification Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`]
module"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "ETM Peripheral Identification Register #2"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: ETM Peripheral Identification Register #3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`]
module"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "ETM Peripheral Identification Register #3"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: ETM Component Identification Register #0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`]
module"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "ETM Component Identification Register #0"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: ETM Component Identification Register #1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`]
module"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "ETM Component Identification Register #1"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: ETM Component Identification Register #2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`]
module"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "ETM Component Identification Register #2"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: ETM Component Identification Register #3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`]
module"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "ETM Component Identification Register #3"]
pub mod cidr3;
