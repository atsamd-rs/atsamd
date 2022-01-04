#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ETM Main Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - ETM Configuration Code Register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x08 - ETM Trigger Event Register"]
    pub trigger: crate::Reg<trigger::TRIGGER_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - ETM Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x14 - ETM System Configuration Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - ETM TraceEnable Event Register"]
    pub teevr: crate::Reg<teevr::TEEVR_SPEC>,
    #[doc = "0x24 - ETM TraceEnable Control 1 Register"]
    pub tecr1: crate::Reg<tecr1::TECR1_SPEC>,
    #[doc = "0x28 - ETM FIFO Full Level Register"]
    pub fflr: crate::Reg<fflr::FFLR_SPEC>,
    _reserved8: [u8; 0x0114],
    #[doc = "0x140 - ETM Free-running Counter Reload Value"]
    pub cntrldvr1: crate::Reg<cntrldvr1::CNTRLDVR1_SPEC>,
    _reserved9: [u8; 0x9c],
    #[doc = "0x1e0 - ETM Synchronization Frequency Register"]
    pub syncfr: crate::Reg<syncfr::SYNCFR_SPEC>,
    #[doc = "0x1e4 - ETM ID Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x1e8 - ETM Configuration Code Extension Register"]
    pub ccer: crate::Reg<ccer::CCER_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x1f0 - ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
    pub tesseicr: crate::Reg<tesseicr::TESSEICR_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x1f8 - ETM TimeStamp Event Register"]
    pub tsevt: crate::Reg<tsevt::TSEVT_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x200 - ETM CoreSight Trace ID Register"]
    pub traceidr: crate::Reg<traceidr::TRACEIDR_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x208 - ETM ID Register 2"]
    pub idr2: crate::Reg<idr2::IDR2_SPEC>,
    _reserved16: [u8; 0x0108],
    #[doc = "0x314 - ETM Device Power-Down Status Register"]
    pub pdsr: crate::Reg<pdsr::PDSR_SPEC>,
    _reserved17: [u8; 0x0bc8],
    #[doc = "0xee0 - ETM Integration Test Miscellaneous Inputs"]
    pub itmiscin: crate::Reg<itmiscin::ITMISCIN_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0xee8 - ETM Integration Test Trigger Out"]
    pub ittrigout: crate::Reg<ittrigout::ITTRIGOUT_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0xef0 - ETM Integration Test ATB Control 2"]
    pub itatbctr2: crate::Reg<itatbctr2::ITATBCTR2_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0xef8 - ETM Integration Test ATB Control 0"]
    pub itatbctr0: crate::Reg<itatbctr0::ITATBCTR0_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0xf00 - ETM Integration Mode Control Register"]
    pub itctrl: crate::Reg<itctrl::ITCTRL_SPEC>,
    _reserved22: [u8; 0x9c],
    #[doc = "0xfa0 - ETM Claim Tag Set Register"]
    pub claimset: crate::Reg<claimset::CLAIMSET_SPEC>,
    #[doc = "0xfa4 - ETM Claim Tag Clear Register"]
    pub claimclr: crate::Reg<claimclr::CLAIMCLR_SPEC>,
    _reserved24: [u8; 0x08],
    #[doc = "0xfb0 - ETM Lock Access Register"]
    pub lar: crate::Reg<lar::LAR_SPEC>,
    #[doc = "0xfb4 - ETM Lock Status Register"]
    pub lsr: crate::Reg<lsr::LSR_SPEC>,
    #[doc = "0xfb8 - ETM Authentication Status Register"]
    pub authstatus: crate::Reg<authstatus::AUTHSTATUS_SPEC>,
    _reserved27: [u8; 0x10],
    #[doc = "0xfcc - ETM CoreSight Device Type Register"]
    pub devtype: crate::Reg<devtype::DEVTYPE_SPEC>,
    #[doc = "0xfd0 - ETM Peripheral Identification Register #4"]
    pub pidr4: crate::Reg<pidr4::PIDR4_SPEC>,
    #[doc = "0xfd4 - ETM Peripheral Identification Register #5"]
    pub pidr5: crate::Reg<pidr5::PIDR5_SPEC>,
    #[doc = "0xfd8 - ETM Peripheral Identification Register #6"]
    pub pidr6: crate::Reg<pidr6::PIDR6_SPEC>,
    #[doc = "0xfdc - ETM Peripheral Identification Register #7"]
    pub pidr7: crate::Reg<pidr7::PIDR7_SPEC>,
    #[doc = "0xfe0 - ETM Peripheral Identification Register #0"]
    pub pidr0: crate::Reg<pidr0::PIDR0_SPEC>,
    #[doc = "0xfe4 - ETM Peripheral Identification Register #1"]
    pub pidr1: crate::Reg<pidr1::PIDR1_SPEC>,
    #[doc = "0xfe8 - ETM Peripheral Identification Register #2"]
    pub pidr2: crate::Reg<pidr2::PIDR2_SPEC>,
    #[doc = "0xfec - ETM Peripheral Identification Register #3"]
    pub pidr3: crate::Reg<pidr3::PIDR3_SPEC>,
    #[doc = "0xff0 - ETM Component Identification Register #0"]
    pub cidr0: crate::Reg<cidr0::CIDR0_SPEC>,
    #[doc = "0xff4 - ETM Component Identification Register #1"]
    pub cidr1: crate::Reg<cidr1::CIDR1_SPEC>,
    #[doc = "0xff8 - ETM Component Identification Register #2"]
    pub cidr2: crate::Reg<cidr2::CIDR2_SPEC>,
    #[doc = "0xffc - ETM Component Identification Register #3"]
    pub cidr3: crate::Reg<cidr3::CIDR3_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ETM Main Control Register"]
pub mod cr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "ETM Configuration Code Register"]
pub mod ccr;
#[doc = "TRIGGER register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "ETM Trigger Event Register"]
pub mod trigger;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "ETM Status Register"]
pub mod sr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "ETM System Configuration Register"]
pub mod scr;
#[doc = "TEEVR register accessor: an alias for `Reg<TEEVR_SPEC>`"]
pub type TEEVR = crate::Reg<teevr::TEEVR_SPEC>;
#[doc = "ETM TraceEnable Event Register"]
pub mod teevr;
#[doc = "TECR1 register accessor: an alias for `Reg<TECR1_SPEC>`"]
pub type TECR1 = crate::Reg<tecr1::TECR1_SPEC>;
#[doc = "ETM TraceEnable Control 1 Register"]
pub mod tecr1;
#[doc = "FFLR register accessor: an alias for `Reg<FFLR_SPEC>`"]
pub type FFLR = crate::Reg<fflr::FFLR_SPEC>;
#[doc = "ETM FIFO Full Level Register"]
pub mod fflr;
#[doc = "CNTRLDVR1 register accessor: an alias for `Reg<CNTRLDVR1_SPEC>`"]
pub type CNTRLDVR1 = crate::Reg<cntrldvr1::CNTRLDVR1_SPEC>;
#[doc = "ETM Free-running Counter Reload Value"]
pub mod cntrldvr1;
#[doc = "SYNCFR register accessor: an alias for `Reg<SYNCFR_SPEC>`"]
pub type SYNCFR = crate::Reg<syncfr::SYNCFR_SPEC>;
#[doc = "ETM Synchronization Frequency Register"]
pub mod syncfr;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "ETM ID Register"]
pub mod idr;
#[doc = "CCER register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "ETM Configuration Code Extension Register"]
pub mod ccer;
#[doc = "TESSEICR register accessor: an alias for `Reg<TESSEICR_SPEC>`"]
pub type TESSEICR = crate::Reg<tesseicr::TESSEICR_SPEC>;
#[doc = "ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod tesseicr;
#[doc = "TSEVT register accessor: an alias for `Reg<TSEVT_SPEC>`"]
pub type TSEVT = crate::Reg<tsevt::TSEVT_SPEC>;
#[doc = "ETM TimeStamp Event Register"]
pub mod tsevt;
#[doc = "TRACEIDR register accessor: an alias for `Reg<TRACEIDR_SPEC>`"]
pub type TRACEIDR = crate::Reg<traceidr::TRACEIDR_SPEC>;
#[doc = "ETM CoreSight Trace ID Register"]
pub mod traceidr;
#[doc = "IDR2 register accessor: an alias for `Reg<IDR2_SPEC>`"]
pub type IDR2 = crate::Reg<idr2::IDR2_SPEC>;
#[doc = "ETM ID Register 2"]
pub mod idr2;
#[doc = "PDSR register accessor: an alias for `Reg<PDSR_SPEC>`"]
pub type PDSR = crate::Reg<pdsr::PDSR_SPEC>;
#[doc = "ETM Device Power-Down Status Register"]
pub mod pdsr;
#[doc = "ITMISCIN register accessor: an alias for `Reg<ITMISCIN_SPEC>`"]
pub type ITMISCIN = crate::Reg<itmiscin::ITMISCIN_SPEC>;
#[doc = "ETM Integration Test Miscellaneous Inputs"]
pub mod itmiscin;
#[doc = "ITTRIGOUT register accessor: an alias for `Reg<ITTRIGOUT_SPEC>`"]
pub type ITTRIGOUT = crate::Reg<ittrigout::ITTRIGOUT_SPEC>;
#[doc = "ETM Integration Test Trigger Out"]
pub mod ittrigout;
#[doc = "ITATBCTR2 register accessor: an alias for `Reg<ITATBCTR2_SPEC>`"]
pub type ITATBCTR2 = crate::Reg<itatbctr2::ITATBCTR2_SPEC>;
#[doc = "ETM Integration Test ATB Control 2"]
pub mod itatbctr2;
#[doc = "ITATBCTR0 register accessor: an alias for `Reg<ITATBCTR0_SPEC>`"]
pub type ITATBCTR0 = crate::Reg<itatbctr0::ITATBCTR0_SPEC>;
#[doc = "ETM Integration Test ATB Control 0"]
pub mod itatbctr0;
#[doc = "ITCTRL register accessor: an alias for `Reg<ITCTRL_SPEC>`"]
pub type ITCTRL = crate::Reg<itctrl::ITCTRL_SPEC>;
#[doc = "ETM Integration Mode Control Register"]
pub mod itctrl;
#[doc = "CLAIMSET register accessor: an alias for `Reg<CLAIMSET_SPEC>`"]
pub type CLAIMSET = crate::Reg<claimset::CLAIMSET_SPEC>;
#[doc = "ETM Claim Tag Set Register"]
pub mod claimset;
#[doc = "CLAIMCLR register accessor: an alias for `Reg<CLAIMCLR_SPEC>`"]
pub type CLAIMCLR = crate::Reg<claimclr::CLAIMCLR_SPEC>;
#[doc = "ETM Claim Tag Clear Register"]
pub mod claimclr;
#[doc = "LAR register accessor: an alias for `Reg<LAR_SPEC>`"]
pub type LAR = crate::Reg<lar::LAR_SPEC>;
#[doc = "ETM Lock Access Register"]
pub mod lar;
#[doc = "LSR register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "ETM Lock Status Register"]
pub mod lsr;
#[doc = "AUTHSTATUS register accessor: an alias for `Reg<AUTHSTATUS_SPEC>`"]
pub type AUTHSTATUS = crate::Reg<authstatus::AUTHSTATUS_SPEC>;
#[doc = "ETM Authentication Status Register"]
pub mod authstatus;
#[doc = "DEVTYPE register accessor: an alias for `Reg<DEVTYPE_SPEC>`"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "ETM CoreSight Device Type Register"]
pub mod devtype;
#[doc = "PIDR4 register accessor: an alias for `Reg<PIDR4_SPEC>`"]
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
#[doc = "ETM Peripheral Identification Register #4"]
pub mod pidr4;
#[doc = "PIDR5 register accessor: an alias for `Reg<PIDR5_SPEC>`"]
pub type PIDR5 = crate::Reg<pidr5::PIDR5_SPEC>;
#[doc = "ETM Peripheral Identification Register #5"]
pub mod pidr5;
#[doc = "PIDR6 register accessor: an alias for `Reg<PIDR6_SPEC>`"]
pub type PIDR6 = crate::Reg<pidr6::PIDR6_SPEC>;
#[doc = "ETM Peripheral Identification Register #6"]
pub mod pidr6;
#[doc = "PIDR7 register accessor: an alias for `Reg<PIDR7_SPEC>`"]
pub type PIDR7 = crate::Reg<pidr7::PIDR7_SPEC>;
#[doc = "ETM Peripheral Identification Register #7"]
pub mod pidr7;
#[doc = "PIDR0 register accessor: an alias for `Reg<PIDR0_SPEC>`"]
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
#[doc = "ETM Peripheral Identification Register #0"]
pub mod pidr0;
#[doc = "PIDR1 register accessor: an alias for `Reg<PIDR1_SPEC>`"]
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
#[doc = "ETM Peripheral Identification Register #1"]
pub mod pidr1;
#[doc = "PIDR2 register accessor: an alias for `Reg<PIDR2_SPEC>`"]
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
#[doc = "ETM Peripheral Identification Register #2"]
pub mod pidr2;
#[doc = "PIDR3 register accessor: an alias for `Reg<PIDR3_SPEC>`"]
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
#[doc = "ETM Peripheral Identification Register #3"]
pub mod pidr3;
#[doc = "CIDR0 register accessor: an alias for `Reg<CIDR0_SPEC>`"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "ETM Component Identification Register #0"]
pub mod cidr0;
#[doc = "CIDR1 register accessor: an alias for `Reg<CIDR1_SPEC>`"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "ETM Component Identification Register #1"]
pub mod cidr1;
#[doc = "CIDR2 register accessor: an alias for `Reg<CIDR2_SPEC>`"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "ETM Component Identification Register #2"]
pub mod cidr2;
#[doc = "CIDR3 register accessor: an alias for `Reg<CIDR3_SPEC>`"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "ETM Component Identification Register #3"]
pub mod cidr3;
