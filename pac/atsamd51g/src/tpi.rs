#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supported Parallel Port Size Register"]
    pub sspsr: crate::Reg<sspsr::SSPSR_SPEC>,
    #[doc = "0x04 - Current Parallel Port Size Register"]
    pub cspsr: crate::Reg<cspsr::CSPSR_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Asynchronous Clock Prescaler Register"]
    pub acpr: crate::Reg<acpr::ACPR_SPEC>,
    _reserved3: [u8; 0xdc],
    #[doc = "0xf0 - Selected Pin Protocol Register"]
    pub sppr: crate::Reg<sppr::SPPR_SPEC>,
    _reserved4: [u8; 0x020c],
    #[doc = "0x300 - Formatter and Flush Status Register"]
    pub ffsr: crate::Reg<ffsr::FFSR_SPEC>,
    #[doc = "0x304 - Formatter and Flush Control Register"]
    pub ffcr: crate::Reg<ffcr::FFCR_SPEC>,
    #[doc = "0x308 - Formatter Synchronization Counter Register"]
    pub fscr: crate::Reg<fscr::FSCR_SPEC>,
    _reserved7: [u8; 0x0bdc],
    #[doc = "0xee8 - TRIGGER"]
    pub trigger: crate::Reg<trigger::TRIGGER_SPEC>,
    #[doc = "0xeec - Integration ETM Data"]
    pub fifo0: crate::Reg<fifo0::FIFO0_SPEC>,
    #[doc = "0xef0 - ITATBCTR2"]
    pub itatbctr2: crate::Reg<itatbctr2::ITATBCTR2_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0xef8 - ITATBCTR0"]
    pub itatbctr0: crate::Reg<itatbctr0::ITATBCTR0_SPEC>,
    #[doc = "0xefc - Integration ITM Data"]
    pub fifo1: crate::Reg<fifo1::FIFO1_SPEC>,
    #[doc = "0xf00 - Integration Mode Control"]
    pub itctrl: crate::Reg<itctrl::ITCTRL_SPEC>,
    _reserved13: [u8; 0x9c],
    #[doc = "0xfa0 - Claim tag set"]
    pub claimset: crate::Reg<claimset::CLAIMSET_SPEC>,
    #[doc = "0xfa4 - Claim tag clear"]
    pub claimclr: crate::Reg<claimclr::CLAIMCLR_SPEC>,
    _reserved15: [u8; 0x20],
    #[doc = "0xfc8 - TPIU_DEVID"]
    pub devid: crate::Reg<devid::DEVID_SPEC>,
    #[doc = "0xfcc - TPIU_DEVTYPE"]
    pub devtype: crate::Reg<devtype::DEVTYPE_SPEC>,
}
#[doc = "SSPSR register accessor: an alias for `Reg<SSPSR_SPEC>`"]
pub type SSPSR = crate::Reg<sspsr::SSPSR_SPEC>;
#[doc = "Supported Parallel Port Size Register"]
pub mod sspsr;
#[doc = "CSPSR register accessor: an alias for `Reg<CSPSR_SPEC>`"]
pub type CSPSR = crate::Reg<cspsr::CSPSR_SPEC>;
#[doc = "Current Parallel Port Size Register"]
pub mod cspsr;
#[doc = "ACPR register accessor: an alias for `Reg<ACPR_SPEC>`"]
pub type ACPR = crate::Reg<acpr::ACPR_SPEC>;
#[doc = "Asynchronous Clock Prescaler Register"]
pub mod acpr;
#[doc = "SPPR register accessor: an alias for `Reg<SPPR_SPEC>`"]
pub type SPPR = crate::Reg<sppr::SPPR_SPEC>;
#[doc = "Selected Pin Protocol Register"]
pub mod sppr;
#[doc = "FFSR register accessor: an alias for `Reg<FFSR_SPEC>`"]
pub type FFSR = crate::Reg<ffsr::FFSR_SPEC>;
#[doc = "Formatter and Flush Status Register"]
pub mod ffsr;
#[doc = "FFCR register accessor: an alias for `Reg<FFCR_SPEC>`"]
pub type FFCR = crate::Reg<ffcr::FFCR_SPEC>;
#[doc = "Formatter and Flush Control Register"]
pub mod ffcr;
#[doc = "FSCR register accessor: an alias for `Reg<FSCR_SPEC>`"]
pub type FSCR = crate::Reg<fscr::FSCR_SPEC>;
#[doc = "Formatter Synchronization Counter Register"]
pub mod fscr;
#[doc = "TRIGGER register accessor: an alias for `Reg<TRIGGER_SPEC>`"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "TRIGGER"]
pub mod trigger;
#[doc = "FIFO0 register accessor: an alias for `Reg<FIFO0_SPEC>`"]
pub type FIFO0 = crate::Reg<fifo0::FIFO0_SPEC>;
#[doc = "Integration ETM Data"]
pub mod fifo0;
#[doc = "ITATBCTR2 register accessor: an alias for `Reg<ITATBCTR2_SPEC>`"]
pub type ITATBCTR2 = crate::Reg<itatbctr2::ITATBCTR2_SPEC>;
#[doc = "ITATBCTR2"]
pub mod itatbctr2;
#[doc = "ITATBCTR0 register accessor: an alias for `Reg<ITATBCTR0_SPEC>`"]
pub type ITATBCTR0 = crate::Reg<itatbctr0::ITATBCTR0_SPEC>;
#[doc = "ITATBCTR0"]
pub mod itatbctr0;
#[doc = "FIFO1 register accessor: an alias for `Reg<FIFO1_SPEC>`"]
pub type FIFO1 = crate::Reg<fifo1::FIFO1_SPEC>;
#[doc = "Integration ITM Data"]
pub mod fifo1;
#[doc = "ITCTRL register accessor: an alias for `Reg<ITCTRL_SPEC>`"]
pub type ITCTRL = crate::Reg<itctrl::ITCTRL_SPEC>;
#[doc = "Integration Mode Control"]
pub mod itctrl;
#[doc = "CLAIMSET register accessor: an alias for `Reg<CLAIMSET_SPEC>`"]
pub type CLAIMSET = crate::Reg<claimset::CLAIMSET_SPEC>;
#[doc = "Claim tag set"]
pub mod claimset;
#[doc = "CLAIMCLR register accessor: an alias for `Reg<CLAIMCLR_SPEC>`"]
pub type CLAIMCLR = crate::Reg<claimclr::CLAIMCLR_SPEC>;
#[doc = "Claim tag clear"]
pub mod claimclr;
#[doc = "DEVID register accessor: an alias for `Reg<DEVID_SPEC>`"]
pub type DEVID = crate::Reg<devid::DEVID_SPEC>;
#[doc = "TPIU_DEVID"]
pub mod devid;
#[doc = "DEVTYPE register accessor: an alias for `Reg<DEVTYPE_SPEC>`"]
pub type DEVTYPE = crate::Reg<devtype::DEVTYPE_SPEC>;
#[doc = "TPIU_DEVTYPE"]
pub mod devtype;
