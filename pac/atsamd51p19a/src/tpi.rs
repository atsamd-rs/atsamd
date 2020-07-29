#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supported Parallel Port Size Register"]
    pub sspsr: SSPSR,
    #[doc = "0x04 - Current Parallel Port Size Register"]
    pub cspsr: CSPSR,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Asynchronous Clock Prescaler Register"]
    pub acpr: ACPR,
    _reserved3: [u8; 220usize],
    #[doc = "0xf0 - Selected Pin Protocol Register"]
    pub sppr: SPPR,
    _reserved4: [u8; 524usize],
    #[doc = "0x300 - Formatter and Flush Status Register"]
    pub ffsr: FFSR,
    #[doc = "0x304 - Formatter and Flush Control Register"]
    pub ffcr: FFCR,
    #[doc = "0x308 - Formatter Synchronization Counter Register"]
    pub fscr: FSCR,
    _reserved7: [u8; 3036usize],
    #[doc = "0xee8 - TRIGGER"]
    pub trigger: TRIGGER,
    #[doc = "0xeec - Integration ETM Data"]
    pub fifo0: FIFO0,
    #[doc = "0xef0 - ITATBCTR2"]
    pub itatbctr2: ITATBCTR2,
    _reserved10: [u8; 4usize],
    #[doc = "0xef8 - ITATBCTR0"]
    pub itatbctr0: ITATBCTR0,
    #[doc = "0xefc - Integration ITM Data"]
    pub fifo1: FIFO1,
    #[doc = "0xf00 - Integration Mode Control"]
    pub itctrl: ITCTRL,
    _reserved13: [u8; 156usize],
    #[doc = "0xfa0 - Claim tag set"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - Claim tag clear"]
    pub claimclr: CLAIMCLR,
    _reserved15: [u8; 32usize],
    #[doc = "0xfc8 - TPIU_DEVID"]
    pub devid: DEVID,
    #[doc = "0xfcc - TPIU_DEVTYPE"]
    pub devtype: DEVTYPE,
}
#[doc = "Supported Parallel Port Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspsr](sspsr) module"]
pub type SSPSR = crate::Reg<u32, _SSPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPSR;
#[doc = "`read()` method returns [sspsr::R](sspsr::R) reader structure"]
impl crate::Readable for SSPSR {}
#[doc = "Supported Parallel Port Size Register"]
pub mod sspsr;
#[doc = "Current Parallel Port Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspsr](cspsr) module"]
pub type CSPSR = crate::Reg<u32, _CSPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPSR;
#[doc = "`read()` method returns [cspsr::R](cspsr::R) reader structure"]
impl crate::Readable for CSPSR {}
#[doc = "`write(|w| ..)` method takes [cspsr::W](cspsr::W) writer structure"]
impl crate::Writable for CSPSR {}
#[doc = "Current Parallel Port Size Register"]
pub mod cspsr;
#[doc = "Asynchronous Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acpr](acpr) module"]
pub type ACPR = crate::Reg<u32, _ACPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACPR;
#[doc = "`read()` method returns [acpr::R](acpr::R) reader structure"]
impl crate::Readable for ACPR {}
#[doc = "`write(|w| ..)` method takes [acpr::W](acpr::W) writer structure"]
impl crate::Writable for ACPR {}
#[doc = "Asynchronous Clock Prescaler Register"]
pub mod acpr;
#[doc = "Selected Pin Protocol Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sppr](sppr) module"]
pub type SPPR = crate::Reg<u32, _SPPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPPR;
#[doc = "`read()` method returns [sppr::R](sppr::R) reader structure"]
impl crate::Readable for SPPR {}
#[doc = "`write(|w| ..)` method takes [sppr::W](sppr::W) writer structure"]
impl crate::Writable for SPPR {}
#[doc = "Selected Pin Protocol Register"]
pub mod sppr;
#[doc = "Formatter and Flush Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffsr](ffsr) module"]
pub type FFSR = crate::Reg<u32, _FFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFSR;
#[doc = "`read()` method returns [ffsr::R](ffsr::R) reader structure"]
impl crate::Readable for FFSR {}
#[doc = "Formatter and Flush Status Register"]
pub mod ffsr;
#[doc = "Formatter and Flush Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffcr](ffcr) module"]
pub type FFCR = crate::Reg<u32, _FFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFCR;
#[doc = "`read()` method returns [ffcr::R](ffcr::R) reader structure"]
impl crate::Readable for FFCR {}
#[doc = "`write(|w| ..)` method takes [ffcr::W](ffcr::W) writer structure"]
impl crate::Writable for FFCR {}
#[doc = "Formatter and Flush Control Register"]
pub mod ffcr;
#[doc = "Formatter Synchronization Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscr](fscr) module"]
pub type FSCR = crate::Reg<u32, _FSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSCR;
#[doc = "`read()` method returns [fscr::R](fscr::R) reader structure"]
impl crate::Readable for FSCR {}
#[doc = "Formatter Synchronization Counter Register"]
pub mod fscr;
#[doc = "TRIGGER\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger](trigger) module"]
pub type TRIGGER = crate::Reg<u32, _TRIGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIGGER;
#[doc = "`read()` method returns [trigger::R](trigger::R) reader structure"]
impl crate::Readable for TRIGGER {}
#[doc = "TRIGGER"]
pub mod trigger;
#[doc = "Integration ETM Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo0](fifo0) module"]
pub type FIFO0 = crate::Reg<u32, _FIFO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO0;
#[doc = "`read()` method returns [fifo0::R](fifo0::R) reader structure"]
impl crate::Readable for FIFO0 {}
#[doc = "Integration ETM Data"]
pub mod fifo0;
#[doc = "ITATBCTR2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itatbctr2](itatbctr2) module"]
pub type ITATBCTR2 = crate::Reg<u32, _ITATBCTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITATBCTR2;
#[doc = "`read()` method returns [itatbctr2::R](itatbctr2::R) reader structure"]
impl crate::Readable for ITATBCTR2 {}
#[doc = "ITATBCTR2"]
pub mod itatbctr2;
#[doc = "ITATBCTR0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itatbctr0](itatbctr0) module"]
pub type ITATBCTR0 = crate::Reg<u32, _ITATBCTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITATBCTR0;
#[doc = "`read()` method returns [itatbctr0::R](itatbctr0::R) reader structure"]
impl crate::Readable for ITATBCTR0 {}
#[doc = "ITATBCTR0"]
pub mod itatbctr0;
#[doc = "Integration ITM Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo1](fifo1) module"]
pub type FIFO1 = crate::Reg<u32, _FIFO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO1;
#[doc = "`read()` method returns [fifo1::R](fifo1::R) reader structure"]
impl crate::Readable for FIFO1 {}
#[doc = "Integration ITM Data"]
pub mod fifo1;
#[doc = "Integration Mode Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itctrl](itctrl) module"]
pub type ITCTRL = crate::Reg<u32, _ITCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITCTRL;
#[doc = "`read()` method returns [itctrl::R](itctrl::R) reader structure"]
impl crate::Readable for ITCTRL {}
#[doc = "`write(|w| ..)` method takes [itctrl::W](itctrl::W) writer structure"]
impl crate::Writable for ITCTRL {}
#[doc = "Integration Mode Control"]
pub mod itctrl;
#[doc = "Claim tag set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimset](claimset) module"]
pub type CLAIMSET = crate::Reg<u32, _CLAIMSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMSET;
#[doc = "`read()` method returns [claimset::R](claimset::R) reader structure"]
impl crate::Readable for CLAIMSET {}
#[doc = "`write(|w| ..)` method takes [claimset::W](claimset::W) writer structure"]
impl crate::Writable for CLAIMSET {}
#[doc = "Claim tag set"]
pub mod claimset;
#[doc = "Claim tag clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimclr](claimclr) module"]
pub type CLAIMCLR = crate::Reg<u32, _CLAIMCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMCLR;
#[doc = "`read()` method returns [claimclr::R](claimclr::R) reader structure"]
impl crate::Readable for CLAIMCLR {}
#[doc = "`write(|w| ..)` method takes [claimclr::W](claimclr::W) writer structure"]
impl crate::Writable for CLAIMCLR {}
#[doc = "Claim tag clear"]
pub mod claimclr;
#[doc = "TPIU_DEVID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devid](devid) module"]
pub type DEVID = crate::Reg<u32, _DEVID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVID;
#[doc = "`read()` method returns [devid::R](devid::R) reader structure"]
impl crate::Readable for DEVID {}
#[doc = "TPIU_DEVID"]
pub mod devid;
#[doc = "TPIU_DEVTYPE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devtype](devtype) module"]
pub type DEVTYPE = crate::Reg<u32, _DEVTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVTYPE;
#[doc = "`read()` method returns [devtype::R](devtype::R) reader structure"]
impl crate::Readable for DEVTYPE {}
#[doc = "TPIU_DEVTYPE"]
pub mod devtype;
