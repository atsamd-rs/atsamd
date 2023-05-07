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
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ETM Main Control Register"]
pub mod cr;
#[doc = "CCR (r) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "ETM Configuration Code Register"]
pub mod ccr;
#[doc = "TRIGGER (rw) register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "ETM Trigger Event Register"]
pub mod trigger;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "ETM Status Register"]
pub mod sr;
#[doc = "SCR (r) register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "ETM System Configuration Register"]
pub mod scr;
#[doc = "TEEVR (rw) register accessor: an alias for `Reg<TEEVR_SPEC>`"]
pub type TEEVR = crate::Reg<teevr::TEEVR_SPEC>;
#[doc = "ETM TraceEnable Event Register"]
pub mod teevr;
#[doc = "TECR1 (rw) register accessor: an alias for `Reg<TECR1_SPEC>`"]
pub type TECR1 = crate::Reg<tecr1::TECR1_SPEC>;
#[doc = "ETM TraceEnable Control 1 Register"]
pub mod tecr1;
#[doc = "FFLR (rw) register accessor: an alias for `Reg<FFLR_SPEC>`"]
pub type FFLR = crate::Reg<fflr::FFLR_SPEC>;
#[doc = "ETM FIFO Full Level Register"]
pub mod fflr;
#[doc = "CNTRLDVR1 (rw) register accessor: an alias for `Reg<CNTRLDVR1_SPEC>`"]
pub type CNTRLDVR1 = crate::Reg<cntrldvr1::CNTRLDVR1_SPEC>;
#[doc = "ETM Free-running Counter Reload Value"]
pub mod cntrldvr1;
#[doc = "SYNCFR (r) register accessor: an alias for `Reg<SYNCFR_SPEC>`"]
pub type SYNCFR = crate::Reg<syncfr::SYNCFR_SPEC>;
#[doc = "ETM Synchronization Frequency Register"]
pub mod syncfr;
#[doc = "IDR (r) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "ETM ID Register"]
pub mod idr;
#[doc = "CCER (r) register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "ETM Configuration Code Extension Register"]
pub mod ccer;
#[doc = "TESSEICR (rw) register accessor: an alias for `Reg<TESSEICR_SPEC>`"]
pub type TESSEICR = crate::Reg<tesseicr::TESSEICR_SPEC>;
#[doc = "ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod tesseicr;
#[doc = "TSEVT (rw) register accessor: an alias for `Reg<TSEVT_SPEC>`"]
pub type TSEVT = crate::Reg<tsevt::TSEVT_SPEC>;
#[doc = "ETM TimeStamp Event Register"]
pub mod tsevt;
#[doc = "TRACEIDR (rw) register accessor: an alias for `Reg<TRACEIDR_SPEC>`"]
pub type TRACEIDR = crate::Reg<traceidr::TRACEIDR_SPEC>;
#[doc = "ETM CoreSight Trace ID Register"]
pub mod traceidr;
#[doc = "IDR2 (r) register accessor: an alias for `Reg<IDR2_SPEC>`"]
pub type IDR2 = crate::Reg<idr2::IDR2_SPEC>;
#[doc = "ETM ID Register 2"]
pub mod idr2;
#[doc = "PDSR (r) register accessor: an alias for `Reg<PDSR_SPEC>`"]
pub type PDSR = crate::Reg<pdsr::PDSR_SPEC>;
#[doc = "ETM Device Power-Down Status Register"]
pub mod pdsr;
#[doc = "ITMISCIN (r) register accessor: an alias for `Reg<ITMISCIN_SPEC>`"]
pub type ITMISCIN = crate::Reg<itmiscin::ITMISCIN_SPEC>;
#[doc = "ETM Integration Test Miscellaneous Inputs"]
pub mod itmiscin;
#[doc = "ITTRIGOUT (w) register accessor: an alias for `Reg<ITTRIGOUT_SPEC>`"]
pub type ITTRIGOUT = crate::Reg<ittrigout::ITTRIGOUT_SPEC>;
#[doc = "ETM Integration Test Trigger Out"]
pub mod ittrigout;
#[doc = "ITATBCTR2 (r) register accessor: an alias for `Reg<ITATBCTR2_SPEC>`"]
pub type ITATBCTR2 = crate::Reg<itatbctr2::ITATBCTR2_SPEC>;
#[doc = "ETM Integration Test ATB Control 2"]
pub mod itatbctr2;
#[doc = "ITATBCTR0 (w) register accessor: an alias for `Reg<ITATBCTR0_SPEC>`"]
pub type ITATBCTR0 = crate::Reg<itatbctr0::ITATBCTR0_SPEC>;
#[doc = "ETM Integration Test ATB Control 0"]
pub mod itatbctr0;
#[doc = "ITCTRL (rw) register accessor: an alias for `Reg<ITCTRL_SPEC>`"]
pub type ITCTRL = crate::Reg<itctrl::ITCTRL_SPEC>;
#[doc = "ETM Integration Mode Control Register"]
pub mod itctrl;
#[doc = "CLAIMSET (rw) register accessor: an alias for `Reg<CLAIMSET_SPEC>`"]
pub type CLAIMSET = crate::Reg<claimset::CLAIMSET_SPEC>;
#[doc = "ETM Claim Tag Set Register"]
pub mod claimset;
#[doc = "CLAIMCLR (rw) register accessor: an alias for `Reg<CLAIMCLR_SPEC>`"]
pub type CLAIMCLR = crate::Reg<claimclr::CLAIMCLR_SPEC>;
#[doc = "ETM Claim Tag Clear Register"]
pub mod claimclr;
#[doc = "LAR (w) register accessor: an alias for `Reg<LAR_SPEC>`"]
pub type LAR = crate::Reg<lar::LAR_SPEC>;
#[doc = "ETM Lock Access Register"]
pub mod lar;
#[doc = "LSR (r) register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "ETM Lock Status Register"]
pub mod lsr;
#[doc = "AUTHSTATUS (r) register accessor: an alias for `Reg<AUTHSTATUS_SPEC>`"]
pub type AUTHSTATUS = crate::Reg<authstatus::AUTHSTATUS_SPEC>;
#[doc = "ETM Authentication Status Register"]
pub mod authstatus;
#[doc = "DEVTYPE (r) register accessor: an alias for `Reg<DEVTYPE_SPEC>`"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "ETM CoreSight Device Type Register"]
pub mod devtype;
#[doc = "PIDR4 (r) register accessor: an alias for `Reg<PIDR4_SPEC>`"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "ETM Peripheral Identification Register #4"]
pub mod pidr4;
#[doc = "PIDR5 (r) register accessor: an alias for `Reg<PIDR5_SPEC>`"]
pub type PIDR5 = crate::Reg<pidr5::PIDR5_SPEC>;
#[doc = "ETM Peripheral Identification Register #5"]
pub mod pidr5;
#[doc = "PIDR6 (r) register accessor: an alias for `Reg<PIDR6_SPEC>`"]
pub type PIDR6 = crate::Reg<pidr6::PIDR6_SPEC>;
#[doc = "ETM Peripheral Identification Register #6"]
pub mod pidr6;
#[doc = "PIDR7 (r) register accessor: an alias for `Reg<PIDR7_SPEC>`"]
pub type PIDR7 = crate::Reg<pidr7::PIDR7_SPEC>;
#[doc = "ETM Peripheral Identification Register #7"]
pub mod pidr7;
#[doc = "PIDR0 (r) register accessor: an alias for `Reg<PIDR0_SPEC>`"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "ETM Peripheral Identification Register #0"]
pub mod pidr0;
#[doc = "PIDR1 (r) register accessor: an alias for `Reg<PIDR1_SPEC>`"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "ETM Peripheral Identification Register #1"]
pub mod pidr1;
#[doc = "PIDR2 (r) register accessor: an alias for `Reg<PIDR2_SPEC>`"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "ETM Peripheral Identification Register #2"]
pub mod pidr2;
#[doc = "PIDR3 (r) register accessor: an alias for `Reg<PIDR3_SPEC>`"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "ETM Peripheral Identification Register #3"]
pub mod pidr3;
#[doc = "CIDR0 (r) register accessor: an alias for `Reg<CIDR0_SPEC>`"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "ETM Component Identification Register #0"]
pub mod cidr0;
#[doc = "CIDR1 (r) register accessor: an alias for `Reg<CIDR1_SPEC>`"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "ETM Component Identification Register #1"]
pub mod cidr1;
#[doc = "CIDR2 (r) register accessor: an alias for `Reg<CIDR2_SPEC>`"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "ETM Component Identification Register #2"]
pub mod cidr2;
#[doc = "CIDR3 (r) register accessor: an alias for `Reg<CIDR3_SPEC>`"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "ETM Component Identification Register #3"]
pub mod cidr3;
