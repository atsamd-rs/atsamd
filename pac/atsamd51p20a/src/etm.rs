#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ETM Main Control Register"]
    pub cr: CR,
    #[doc = "0x04 - ETM Configuration Code Register"]
    pub ccr: CCR,
    #[doc = "0x08 - ETM Trigger Event Register"]
    pub trigger: TRIGGER,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - ETM Status Register"]
    pub sr: SR,
    #[doc = "0x14 - ETM System Configuration Register"]
    pub scr: SCR,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - ETM TraceEnable Event Register"]
    pub teevr: TEEVR,
    #[doc = "0x24 - ETM TraceEnable Control 1 Register"]
    pub tecr1: TECR1,
    #[doc = "0x28 - ETM FIFO Full Level Register"]
    pub fflr: FFLR,
    _reserved8: [u8; 276usize],
    #[doc = "0x140 - ETM Free-running Counter Reload Value"]
    pub cntrldvr1: CNTRLDVR1,
    _reserved9: [u8; 156usize],
    #[doc = "0x1e0 - ETM Synchronization Frequency Register"]
    pub syncfr: SYNCFR,
    #[doc = "0x1e4 - ETM ID Register"]
    pub idr: IDR,
    #[doc = "0x1e8 - ETM Configuration Code Extension Register"]
    pub ccer: CCER,
    _reserved12: [u8; 4usize],
    #[doc = "0x1f0 - ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
    pub tesseicr: TESSEICR,
    _reserved13: [u8; 4usize],
    #[doc = "0x1f8 - ETM TimeStamp Event Register"]
    pub tsevt: TSEVT,
    _reserved14: [u8; 4usize],
    #[doc = "0x200 - ETM CoreSight Trace ID Register"]
    pub traceidr: TRACEIDR,
    _reserved15: [u8; 4usize],
    #[doc = "0x208 - ETM ID Register 2"]
    pub idr2: IDR2,
    _reserved16: [u8; 264usize],
    #[doc = "0x314 - ETM Device Power-Down Status Register"]
    pub pdsr: PDSR,
    _reserved17: [u8; 3016usize],
    #[doc = "0xee0 - ETM Integration Test Miscellaneous Inputs"]
    pub itmiscin: ITMISCIN,
    _reserved18: [u8; 4usize],
    #[doc = "0xee8 - ETM Integration Test Trigger Out"]
    pub ittrigout: ITTRIGOUT,
    _reserved19: [u8; 4usize],
    #[doc = "0xef0 - ETM Integration Test ATB Control 2"]
    pub itatbctr2: ITATBCTR2,
    _reserved20: [u8; 4usize],
    #[doc = "0xef8 - ETM Integration Test ATB Control 0"]
    pub itatbctr0: ITATBCTR0,
    _reserved21: [u8; 4usize],
    #[doc = "0xf00 - ETM Integration Mode Control Register"]
    pub itctrl: ITCTRL,
    _reserved22: [u8; 156usize],
    #[doc = "0xfa0 - ETM Claim Tag Set Register"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - ETM Claim Tag Clear Register"]
    pub claimclr: CLAIMCLR,
    _reserved24: [u8; 8usize],
    #[doc = "0xfb0 - ETM Lock Access Register"]
    pub lar: LAR,
    #[doc = "0xfb4 - ETM Lock Status Register"]
    pub lsr: LSR,
    #[doc = "0xfb8 - ETM Authentication Status Register"]
    pub authstatus: AUTHSTATUS,
    _reserved27: [u8; 16usize],
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
#[doc = "ETM Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "ETM Main Control Register"]
pub mod cr;
#[doc = "ETM Configuration Code Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "ETM Configuration Code Register"]
pub mod ccr;
#[doc = "ETM Trigger Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger](trigger) module"]
pub type TRIGGER = crate::Reg<u32, _TRIGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIGGER;
#[doc = "`read()` method returns [trigger::R](trigger::R) reader structure"]
impl crate::Readable for TRIGGER {}
#[doc = "`write(|w| ..)` method takes [trigger::W](trigger::W) writer structure"]
impl crate::Writable for TRIGGER {}
#[doc = "ETM Trigger Event Register"]
pub mod trigger;
#[doc = "ETM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "ETM Status Register"]
pub mod sr;
#[doc = "ETM System Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "ETM System Configuration Register"]
pub mod scr;
#[doc = "ETM TraceEnable Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [teevr](teevr) module"]
pub type TEEVR = crate::Reg<u32, _TEEVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEEVR;
#[doc = "`read()` method returns [teevr::R](teevr::R) reader structure"]
impl crate::Readable for TEEVR {}
#[doc = "`write(|w| ..)` method takes [teevr::W](teevr::W) writer structure"]
impl crate::Writable for TEEVR {}
#[doc = "ETM TraceEnable Event Register"]
pub mod teevr;
#[doc = "ETM TraceEnable Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tecr1](tecr1) module"]
pub type TECR1 = crate::Reg<u32, _TECR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TECR1;
#[doc = "`read()` method returns [tecr1::R](tecr1::R) reader structure"]
impl crate::Readable for TECR1 {}
#[doc = "`write(|w| ..)` method takes [tecr1::W](tecr1::W) writer structure"]
impl crate::Writable for TECR1 {}
#[doc = "ETM TraceEnable Control 1 Register"]
pub mod tecr1;
#[doc = "ETM FIFO Full Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fflr](fflr) module"]
pub type FFLR = crate::Reg<u32, _FFLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFLR;
#[doc = "`read()` method returns [fflr::R](fflr::R) reader structure"]
impl crate::Readable for FFLR {}
#[doc = "`write(|w| ..)` method takes [fflr::W](fflr::W) writer structure"]
impl crate::Writable for FFLR {}
#[doc = "ETM FIFO Full Level Register"]
pub mod fflr;
#[doc = "ETM Free-running Counter Reload Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntrldvr1](cntrldvr1) module"]
pub type CNTRLDVR1 = crate::Reg<u32, _CNTRLDVR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTRLDVR1;
#[doc = "`read()` method returns [cntrldvr1::R](cntrldvr1::R) reader structure"]
impl crate::Readable for CNTRLDVR1 {}
#[doc = "`write(|w| ..)` method takes [cntrldvr1::W](cntrldvr1::W) writer structure"]
impl crate::Writable for CNTRLDVR1 {}
#[doc = "ETM Free-running Counter Reload Value"]
pub mod cntrldvr1;
#[doc = "ETM Synchronization Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncfr](syncfr) module"]
pub type SYNCFR = crate::Reg<u32, _SYNCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCFR;
#[doc = "`read()` method returns [syncfr::R](syncfr::R) reader structure"]
impl crate::Readable for SYNCFR {}
#[doc = "ETM Synchronization Frequency Register"]
pub mod syncfr;
#[doc = "ETM ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
impl crate::Readable for IDR {}
#[doc = "ETM ID Register"]
pub mod idr;
#[doc = "ETM Configuration Code Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccer](ccer) module"]
pub type CCER = crate::Reg<u32, _CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCER;
#[doc = "`read()` method returns [ccer::R](ccer::R) reader structure"]
impl crate::Readable for CCER {}
#[doc = "ETM Configuration Code Extension Register"]
pub mod ccer;
#[doc = "ETM TraceEnable Start/Stop EmbeddedICE Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tesseicr](tesseicr) module"]
pub type TESSEICR = crate::Reg<u32, _TESSEICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TESSEICR;
#[doc = "`read()` method returns [tesseicr::R](tesseicr::R) reader structure"]
impl crate::Readable for TESSEICR {}
#[doc = "`write(|w| ..)` method takes [tesseicr::W](tesseicr::W) writer structure"]
impl crate::Writable for TESSEICR {}
#[doc = "ETM TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod tesseicr;
#[doc = "ETM TimeStamp Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsevt](tsevt) module"]
pub type TSEVT = crate::Reg<u32, _TSEVT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSEVT;
#[doc = "`read()` method returns [tsevt::R](tsevt::R) reader structure"]
impl crate::Readable for TSEVT {}
#[doc = "`write(|w| ..)` method takes [tsevt::W](tsevt::W) writer structure"]
impl crate::Writable for TSEVT {}
#[doc = "ETM TimeStamp Event Register"]
pub mod tsevt;
#[doc = "ETM CoreSight Trace ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceidr](traceidr) module"]
pub type TRACEIDR = crate::Reg<u32, _TRACEIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRACEIDR;
#[doc = "`read()` method returns [traceidr::R](traceidr::R) reader structure"]
impl crate::Readable for TRACEIDR {}
#[doc = "`write(|w| ..)` method takes [traceidr::W](traceidr::W) writer structure"]
impl crate::Writable for TRACEIDR {}
#[doc = "ETM CoreSight Trace ID Register"]
pub mod traceidr;
#[doc = "ETM ID Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr2](idr2) module"]
pub type IDR2 = crate::Reg<u32, _IDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR2;
#[doc = "`read()` method returns [idr2::R](idr2::R) reader structure"]
impl crate::Readable for IDR2 {}
#[doc = "ETM ID Register 2"]
pub mod idr2;
#[doc = "ETM Device Power-Down Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsr](pdsr) module"]
pub type PDSR = crate::Reg<u32, _PDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSR;
#[doc = "`read()` method returns [pdsr::R](pdsr::R) reader structure"]
impl crate::Readable for PDSR {}
#[doc = "ETM Device Power-Down Status Register"]
pub mod pdsr;
#[doc = "ETM Integration Test Miscellaneous Inputs\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itmiscin](itmiscin) module"]
pub type ITMISCIN = crate::Reg<u32, _ITMISCIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITMISCIN;
#[doc = "`read()` method returns [itmiscin::R](itmiscin::R) reader structure"]
impl crate::Readable for ITMISCIN {}
#[doc = "ETM Integration Test Miscellaneous Inputs"]
pub mod itmiscin;
#[doc = "ETM Integration Test Trigger Out\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ittrigout](ittrigout) module"]
pub type ITTRIGOUT = crate::Reg<u32, _ITTRIGOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITTRIGOUT;
#[doc = "`write(|w| ..)` method takes [ittrigout::W](ittrigout::W) writer structure"]
impl crate::Writable for ITTRIGOUT {}
#[doc = "ETM Integration Test Trigger Out"]
pub mod ittrigout;
#[doc = "ETM Integration Test ATB Control 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itatbctr2](itatbctr2) module"]
pub type ITATBCTR2 = crate::Reg<u32, _ITATBCTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITATBCTR2;
#[doc = "`read()` method returns [itatbctr2::R](itatbctr2::R) reader structure"]
impl crate::Readable for ITATBCTR2 {}
#[doc = "ETM Integration Test ATB Control 2"]
pub mod itatbctr2;
#[doc = "ETM Integration Test ATB Control 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itatbctr0](itatbctr0) module"]
pub type ITATBCTR0 = crate::Reg<u32, _ITATBCTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITATBCTR0;
#[doc = "`write(|w| ..)` method takes [itatbctr0::W](itatbctr0::W) writer structure"]
impl crate::Writable for ITATBCTR0 {}
#[doc = "ETM Integration Test ATB Control 0"]
pub mod itatbctr0;
#[doc = "ETM Integration Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itctrl](itctrl) module"]
pub type ITCTRL = crate::Reg<u32, _ITCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITCTRL;
#[doc = "`read()` method returns [itctrl::R](itctrl::R) reader structure"]
impl crate::Readable for ITCTRL {}
#[doc = "`write(|w| ..)` method takes [itctrl::W](itctrl::W) writer structure"]
impl crate::Writable for ITCTRL {}
#[doc = "ETM Integration Mode Control Register"]
pub mod itctrl;
#[doc = "ETM Claim Tag Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimset](claimset) module"]
pub type CLAIMSET = crate::Reg<u32, _CLAIMSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMSET;
#[doc = "`read()` method returns [claimset::R](claimset::R) reader structure"]
impl crate::Readable for CLAIMSET {}
#[doc = "`write(|w| ..)` method takes [claimset::W](claimset::W) writer structure"]
impl crate::Writable for CLAIMSET {}
#[doc = "ETM Claim Tag Set Register"]
pub mod claimset;
#[doc = "ETM Claim Tag Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimclr](claimclr) module"]
pub type CLAIMCLR = crate::Reg<u32, _CLAIMCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMCLR;
#[doc = "`read()` method returns [claimclr::R](claimclr::R) reader structure"]
impl crate::Readable for CLAIMCLR {}
#[doc = "`write(|w| ..)` method takes [claimclr::W](claimclr::W) writer structure"]
impl crate::Writable for CLAIMCLR {}
#[doc = "ETM Claim Tag Clear Register"]
pub mod claimclr;
#[doc = "ETM Lock Access Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lar](lar) module"]
pub type LAR = crate::Reg<u32, _LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LAR;
#[doc = "`write(|w| ..)` method takes [lar::W](lar::W) writer structure"]
impl crate::Writable for LAR {}
#[doc = "ETM Lock Access Register"]
pub mod lar;
#[doc = "ETM Lock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](lsr) module"]
pub type LSR = crate::Reg<u32, _LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSR;
#[doc = "`read()` method returns [lsr::R](lsr::R) reader structure"]
impl crate::Readable for LSR {}
#[doc = "ETM Lock Status Register"]
pub mod lsr;
#[doc = "ETM Authentication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [authstatus](authstatus) module"]
pub type AUTHSTATUS = crate::Reg<u32, _AUTHSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTHSTATUS;
#[doc = "`read()` method returns [authstatus::R](authstatus::R) reader structure"]
impl crate::Readable for AUTHSTATUS {}
#[doc = "ETM Authentication Status Register"]
pub mod authstatus;
#[doc = "ETM CoreSight Device Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devtype](devtype) module"]
pub type DEVTYPE = crate::Reg<u32, _DEVTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVTYPE;
#[doc = "`read()` method returns [devtype::R](devtype::R) reader structure"]
impl crate::Readable for DEVTYPE {}
#[doc = "ETM CoreSight Device Type Register"]
pub mod devtype;
#[doc = "ETM Peripheral Identification Register #4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr4](pidr4) module"]
pub type PIDR4 = crate::Reg<u32, _PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR4;
#[doc = "`read()` method returns [pidr4::R](pidr4::R) reader structure"]
impl crate::Readable for PIDR4 {}
#[doc = "ETM Peripheral Identification Register #4"]
pub mod pidr4;
#[doc = "ETM Peripheral Identification Register #5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr5](pidr5) module"]
pub type PIDR5 = crate::Reg<u32, _PIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR5;
#[doc = "`read()` method returns [pidr5::R](pidr5::R) reader structure"]
impl crate::Readable for PIDR5 {}
#[doc = "ETM Peripheral Identification Register #5"]
pub mod pidr5;
#[doc = "ETM Peripheral Identification Register #6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr6](pidr6) module"]
pub type PIDR6 = crate::Reg<u32, _PIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR6;
#[doc = "`read()` method returns [pidr6::R](pidr6::R) reader structure"]
impl crate::Readable for PIDR6 {}
#[doc = "ETM Peripheral Identification Register #6"]
pub mod pidr6;
#[doc = "ETM Peripheral Identification Register #7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr7](pidr7) module"]
pub type PIDR7 = crate::Reg<u32, _PIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR7;
#[doc = "`read()` method returns [pidr7::R](pidr7::R) reader structure"]
impl crate::Readable for PIDR7 {}
#[doc = "ETM Peripheral Identification Register #7"]
pub mod pidr7;
#[doc = "ETM Peripheral Identification Register #0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr0](pidr0) module"]
pub type PIDR0 = crate::Reg<u32, _PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR0;
#[doc = "`read()` method returns [pidr0::R](pidr0::R) reader structure"]
impl crate::Readable for PIDR0 {}
#[doc = "ETM Peripheral Identification Register #0"]
pub mod pidr0;
#[doc = "ETM Peripheral Identification Register #1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr1](pidr1) module"]
pub type PIDR1 = crate::Reg<u32, _PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR1;
#[doc = "`read()` method returns [pidr1::R](pidr1::R) reader structure"]
impl crate::Readable for PIDR1 {}
#[doc = "ETM Peripheral Identification Register #1"]
pub mod pidr1;
#[doc = "ETM Peripheral Identification Register #2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr2](pidr2) module"]
pub type PIDR2 = crate::Reg<u32, _PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR2;
#[doc = "`read()` method returns [pidr2::R](pidr2::R) reader structure"]
impl crate::Readable for PIDR2 {}
#[doc = "ETM Peripheral Identification Register #2"]
pub mod pidr2;
#[doc = "ETM Peripheral Identification Register #3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr3](pidr3) module"]
pub type PIDR3 = crate::Reg<u32, _PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR3;
#[doc = "`read()` method returns [pidr3::R](pidr3::R) reader structure"]
impl crate::Readable for PIDR3 {}
#[doc = "ETM Peripheral Identification Register #3"]
pub mod pidr3;
#[doc = "ETM Component Identification Register #0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr0](cidr0) module"]
pub type CIDR0 = crate::Reg<u32, _CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR0;
#[doc = "`read()` method returns [cidr0::R](cidr0::R) reader structure"]
impl crate::Readable for CIDR0 {}
#[doc = "ETM Component Identification Register #0"]
pub mod cidr0;
#[doc = "ETM Component Identification Register #1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr1](cidr1) module"]
pub type CIDR1 = crate::Reg<u32, _CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR1;
#[doc = "`read()` method returns [cidr1::R](cidr1::R) reader structure"]
impl crate::Readable for CIDR1 {}
#[doc = "ETM Component Identification Register #1"]
pub mod cidr1;
#[doc = "ETM Component Identification Register #2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr2](cidr2) module"]
pub type CIDR2 = crate::Reg<u32, _CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR2;
#[doc = "`read()` method returns [cidr2::R](cidr2::R) reader structure"]
impl crate::Readable for CIDR2 {}
#[doc = "ETM Component Identification Register #2"]
pub mod cidr2;
#[doc = "ETM Component Identification Register #3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr3](cidr3) module"]
pub type CIDR3 = crate::Reg<u32, _CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR3;
#[doc = "`read()` method returns [cidr3::R](cidr3::R) reader structure"]
impl crate::Readable for CIDR3 {}
#[doc = "ETM Component Identification Register #3"]
pub mod cidr3;
