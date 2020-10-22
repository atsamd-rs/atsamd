#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Core Release"]
    pub crel: CREL,
    #[doc = "0x04 - Endian"]
    pub endn: ENDN,
    #[doc = "0x08 - Message RAM Configuration"]
    pub mrcfg: MRCFG,
    #[doc = "0x0c - Fast Bit Timing and Prescaler"]
    pub dbtp: DBTP,
    #[doc = "0x10 - Test"]
    pub test: TEST,
    #[doc = "0x14 - RAM Watchdog"]
    pub rwd: RWD,
    #[doc = "0x18 - CC Control"]
    pub cccr: CCCR,
    #[doc = "0x1c - Nominal Bit Timing and Prescaler"]
    pub nbtp: NBTP,
    #[doc = "0x20 - Timestamp Counter Configuration"]
    pub tscc: TSCC,
    #[doc = "0x24 - Timestamp Counter Value"]
    pub tscv: TSCV,
    #[doc = "0x28 - Timeout Counter Configuration"]
    pub tocc: TOCC,
    #[doc = "0x2c - Timeout Counter Value"]
    pub tocv: TOCV,
    _reserved12: [u8; 16usize],
    #[doc = "0x40 - Error Counter"]
    pub ecr: ECR,
    #[doc = "0x44 - Protocol Status"]
    pub psr: PSR,
    #[doc = "0x48 - Extended ID Filter Configuration"]
    pub tdcr: TDCR,
    _reserved15: [u8; 4usize],
    #[doc = "0x50 - Interrupt"]
    pub ir: IR,
    #[doc = "0x54 - Interrupt Enable"]
    pub ie: IE,
    #[doc = "0x58 - Interrupt Line Select"]
    pub ils: ILS,
    #[doc = "0x5c - Interrupt Line Enable"]
    pub ile: ILE,
    _reserved19: [u8; 32usize],
    #[doc = "0x80 - Global Filter Configuration"]
    pub gfc: GFC,
    #[doc = "0x84 - Standard ID Filter Configuration"]
    pub sidfc: SIDFC,
    #[doc = "0x88 - Extended ID Filter Configuration"]
    pub xidfc: XIDFC,
    _reserved22: [u8; 4usize],
    #[doc = "0x90 - Extended ID AND Mask"]
    pub xidam: XIDAM,
    #[doc = "0x94 - High Priority Message Status"]
    pub hpms: HPMS,
    #[doc = "0x98 - New Data 1"]
    pub ndat1: NDAT1,
    #[doc = "0x9c - New Data 2"]
    pub ndat2: NDAT2,
    #[doc = "0xa0 - Rx FIFO 0 Configuration"]
    pub rxf0c: RXF0C,
    #[doc = "0xa4 - Rx FIFO 0 Status"]
    pub rxf0s: RXF0S,
    #[doc = "0xa8 - Rx FIFO 0 Acknowledge"]
    pub rxf0a: RXF0A,
    #[doc = "0xac - Rx Buffer Configuration"]
    pub rxbc: RXBC,
    #[doc = "0xb0 - Rx FIFO 1 Configuration"]
    pub rxf1c: RXF1C,
    #[doc = "0xb4 - Rx FIFO 1 Status"]
    pub rxf1s: RXF1S,
    #[doc = "0xb8 - Rx FIFO 1 Acknowledge"]
    pub rxf1a: RXF1A,
    #[doc = "0xbc - Rx Buffer / FIFO Element Size Configuration"]
    pub rxesc: RXESC,
    #[doc = "0xc0 - Tx Buffer Configuration"]
    pub txbc: TXBC,
    #[doc = "0xc4 - Tx FIFO / Queue Status"]
    pub txfqs: TXFQS,
    #[doc = "0xc8 - Tx Buffer Element Size Configuration"]
    pub txesc: TXESC,
    #[doc = "0xcc - Tx Buffer Request Pending"]
    pub txbrp: TXBRP,
    #[doc = "0xd0 - Tx Buffer Add Request"]
    pub txbar: TXBAR,
    #[doc = "0xd4 - Tx Buffer Cancellation Request"]
    pub txbcr: TXBCR,
    #[doc = "0xd8 - Tx Buffer Transmission Occurred"]
    pub txbto: TXBTO,
    #[doc = "0xdc - Tx Buffer Cancellation Finished"]
    pub txbcf: TXBCF,
    #[doc = "0xe0 - Tx Buffer Transmission Interrupt Enable"]
    pub txbtie: TXBTIE,
    #[doc = "0xe4 - Tx Buffer Cancellation Finished Interrupt Enable"]
    pub txbcie: TXBCIE,
    _reserved44: [u8; 8usize],
    #[doc = "0xf0 - Tx Event FIFO Configuration"]
    pub txefc: TXEFC,
    #[doc = "0xf4 - Tx Event FIFO Status"]
    pub txefs: TXEFS,
    #[doc = "0xf8 - Tx Event FIFO Acknowledge"]
    pub txefa: TXEFA,
}
#[doc = "Core Release\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crel](crel) module"]
pub type CREL = crate::Reg<u32, _CREL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CREL;
#[doc = "`read()` method returns [crel::R](crel::R) reader structure"]
impl crate::Readable for CREL {}
#[doc = "Core Release"]
pub mod crel;
#[doc = "Endian\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endn](endn) module"]
pub type ENDN = crate::Reg<u32, _ENDN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDN;
#[doc = "`read()` method returns [endn::R](endn::R) reader structure"]
impl crate::Readable for ENDN {}
#[doc = "Endian"]
pub mod endn;
#[doc = "Message RAM Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrcfg](mrcfg) module"]
pub type MRCFG = crate::Reg<u32, _MRCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRCFG;
#[doc = "`read()` method returns [mrcfg::R](mrcfg::R) reader structure"]
impl crate::Readable for MRCFG {}
#[doc = "`write(|w| ..)` method takes [mrcfg::W](mrcfg::W) writer structure"]
impl crate::Writable for MRCFG {}
#[doc = "Message RAM Configuration"]
pub mod mrcfg;
#[doc = "Fast Bit Timing and Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbtp](dbtp) module"]
pub type DBTP = crate::Reg<u32, _DBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBTP;
#[doc = "`read()` method returns [dbtp::R](dbtp::R) reader structure"]
impl crate::Readable for DBTP {}
#[doc = "`write(|w| ..)` method takes [dbtp::W](dbtp::W) writer structure"]
impl crate::Writable for DBTP {}
#[doc = "Fast Bit Timing and Prescaler"]
pub mod dbtp;
#[doc = "Test\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](test) module"]
pub type TEST = crate::Reg<u32, _TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST;
#[doc = "`read()` method returns [test::R](test::R) reader structure"]
impl crate::Readable for TEST {}
#[doc = "`write(|w| ..)` method takes [test::W](test::W) writer structure"]
impl crate::Writable for TEST {}
#[doc = "Test"]
pub mod test;
#[doc = "RAM Watchdog\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwd](rwd) module"]
pub type RWD = crate::Reg<u32, _RWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RWD;
#[doc = "`read()` method returns [rwd::R](rwd::R) reader structure"]
impl crate::Readable for RWD {}
#[doc = "`write(|w| ..)` method takes [rwd::W](rwd::W) writer structure"]
impl crate::Writable for RWD {}
#[doc = "RAM Watchdog"]
pub mod rwd;
#[doc = "CC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](cccr) module"]
pub type CCCR = crate::Reg<u32, _CCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCCR;
#[doc = "`read()` method returns [cccr::R](cccr::R) reader structure"]
impl crate::Readable for CCCR {}
#[doc = "`write(|w| ..)` method takes [cccr::W](cccr::W) writer structure"]
impl crate::Writable for CCCR {}
#[doc = "CC Control"]
pub mod cccr;
#[doc = "Nominal Bit Timing and Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nbtp](nbtp) module"]
pub type NBTP = crate::Reg<u32, _NBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NBTP;
#[doc = "`read()` method returns [nbtp::R](nbtp::R) reader structure"]
impl crate::Readable for NBTP {}
#[doc = "`write(|w| ..)` method takes [nbtp::W](nbtp::W) writer structure"]
impl crate::Writable for NBTP {}
#[doc = "Nominal Bit Timing and Prescaler"]
pub mod nbtp;
#[doc = "Timestamp Counter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscc](tscc) module"]
pub type TSCC = crate::Reg<u32, _TSCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCC;
#[doc = "`read()` method returns [tscc::R](tscc::R) reader structure"]
impl crate::Readable for TSCC {}
#[doc = "`write(|w| ..)` method takes [tscc::W](tscc::W) writer structure"]
impl crate::Writable for TSCC {}
#[doc = "Timestamp Counter Configuration"]
pub mod tscc;
#[doc = "Timestamp Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscv](tscv) module"]
pub type TSCV = crate::Reg<u32, _TSCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSCV;
#[doc = "`read()` method returns [tscv::R](tscv::R) reader structure"]
impl crate::Readable for TSCV {}
#[doc = "Timestamp Counter Value"]
pub mod tscv;
#[doc = "Timeout Counter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocc](tocc) module"]
pub type TOCC = crate::Reg<u32, _TOCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOCC;
#[doc = "`read()` method returns [tocc::R](tocc::R) reader structure"]
impl crate::Readable for TOCC {}
#[doc = "`write(|w| ..)` method takes [tocc::W](tocc::W) writer structure"]
impl crate::Writable for TOCC {}
#[doc = "Timeout Counter Configuration"]
pub mod tocc;
#[doc = "Timeout Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocv](tocv) module"]
pub type TOCV = crate::Reg<u32, _TOCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOCV;
#[doc = "`read()` method returns [tocv::R](tocv::R) reader structure"]
impl crate::Readable for TOCV {}
#[doc = "`write(|w| ..)` method takes [tocv::W](tocv::W) writer structure"]
impl crate::Writable for TOCV {}
#[doc = "Timeout Counter Value"]
pub mod tocv;
#[doc = "Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "Protocol Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](psr) module"]
pub type PSR = crate::Reg<u32, _PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR;
#[doc = "`read()` method returns [psr::R](psr::R) reader structure"]
impl crate::Readable for PSR {}
#[doc = "Protocol Status"]
pub mod psr;
#[doc = "Extended ID Filter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdcr](tdcr) module"]
pub type TDCR = crate::Reg<u32, _TDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDCR;
#[doc = "`read()` method returns [tdcr::R](tdcr::R) reader structure"]
impl crate::Readable for TDCR {}
#[doc = "`write(|w| ..)` method takes [tdcr::W](tdcr::W) writer structure"]
impl crate::Writable for TDCR {}
#[doc = "Extended ID Filter Configuration"]
pub mod tdcr;
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "Interrupt"]
pub mod ir;
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "Interrupt Enable"]
pub mod ie;
#[doc = "Interrupt Line Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ils](ils) module"]
pub type ILS = crate::Reg<u32, _ILS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILS;
#[doc = "`read()` method returns [ils::R](ils::R) reader structure"]
impl crate::Readable for ILS {}
#[doc = "`write(|w| ..)` method takes [ils::W](ils::W) writer structure"]
impl crate::Writable for ILS {}
#[doc = "Interrupt Line Select"]
pub mod ils;
#[doc = "Interrupt Line Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ile](ile) module"]
pub type ILE = crate::Reg<u32, _ILE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILE;
#[doc = "`read()` method returns [ile::R](ile::R) reader structure"]
impl crate::Readable for ILE {}
#[doc = "`write(|w| ..)` method takes [ile::W](ile::W) writer structure"]
impl crate::Writable for ILE {}
#[doc = "Interrupt Line Enable"]
pub mod ile;
#[doc = "Global Filter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfc](gfc) module"]
pub type GFC = crate::Reg<u32, _GFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GFC;
#[doc = "`read()` method returns [gfc::R](gfc::R) reader structure"]
impl crate::Readable for GFC {}
#[doc = "`write(|w| ..)` method takes [gfc::W](gfc::W) writer structure"]
impl crate::Writable for GFC {}
#[doc = "Global Filter Configuration"]
pub mod gfc;
#[doc = "Standard ID Filter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidfc](sidfc) module"]
pub type SIDFC = crate::Reg<u32, _SIDFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIDFC;
#[doc = "`read()` method returns [sidfc::R](sidfc::R) reader structure"]
impl crate::Readable for SIDFC {}
#[doc = "`write(|w| ..)` method takes [sidfc::W](sidfc::W) writer structure"]
impl crate::Writable for SIDFC {}
#[doc = "Standard ID Filter Configuration"]
pub mod sidfc;
#[doc = "Extended ID Filter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xidfc](xidfc) module"]
pub type XIDFC = crate::Reg<u32, _XIDFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIDFC;
#[doc = "`read()` method returns [xidfc::R](xidfc::R) reader structure"]
impl crate::Readable for XIDFC {}
#[doc = "`write(|w| ..)` method takes [xidfc::W](xidfc::W) writer structure"]
impl crate::Writable for XIDFC {}
#[doc = "Extended ID Filter Configuration"]
pub mod xidfc;
#[doc = "Extended ID AND Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xidam](xidam) module"]
pub type XIDAM = crate::Reg<u32, _XIDAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIDAM;
#[doc = "`read()` method returns [xidam::R](xidam::R) reader structure"]
impl crate::Readable for XIDAM {}
#[doc = "`write(|w| ..)` method takes [xidam::W](xidam::W) writer structure"]
impl crate::Writable for XIDAM {}
#[doc = "Extended ID AND Mask"]
pub mod xidam;
#[doc = "High Priority Message Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpms](hpms) module"]
pub type HPMS = crate::Reg<u32, _HPMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPMS;
#[doc = "`read()` method returns [hpms::R](hpms::R) reader structure"]
impl crate::Readable for HPMS {}
#[doc = "High Priority Message Status"]
pub mod hpms;
#[doc = "New Data 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat1](ndat1) module"]
pub type NDAT1 = crate::Reg<u32, _NDAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDAT1;
#[doc = "`read()` method returns [ndat1::R](ndat1::R) reader structure"]
impl crate::Readable for NDAT1 {}
#[doc = "`write(|w| ..)` method takes [ndat1::W](ndat1::W) writer structure"]
impl crate::Writable for NDAT1 {}
#[doc = "New Data 1"]
pub mod ndat1;
#[doc = "New Data 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat2](ndat2) module"]
pub type NDAT2 = crate::Reg<u32, _NDAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDAT2;
#[doc = "`read()` method returns [ndat2::R](ndat2::R) reader structure"]
impl crate::Readable for NDAT2 {}
#[doc = "`write(|w| ..)` method takes [ndat2::W](ndat2::W) writer structure"]
impl crate::Writable for NDAT2 {}
#[doc = "New Data 2"]
pub mod ndat2;
#[doc = "Rx FIFO 0 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0c](rxf0c) module"]
pub type RXF0C = crate::Reg<u32, _RXF0C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0C;
#[doc = "`read()` method returns [rxf0c::R](rxf0c::R) reader structure"]
impl crate::Readable for RXF0C {}
#[doc = "`write(|w| ..)` method takes [rxf0c::W](rxf0c::W) writer structure"]
impl crate::Writable for RXF0C {}
#[doc = "Rx FIFO 0 Configuration"]
pub mod rxf0c;
#[doc = "Rx FIFO 0 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0s](rxf0s) module"]
pub type RXF0S = crate::Reg<u32, _RXF0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0S;
#[doc = "`read()` method returns [rxf0s::R](rxf0s::R) reader structure"]
impl crate::Readable for RXF0S {}
#[doc = "Rx FIFO 0 Status"]
pub mod rxf0s;
#[doc = "Rx FIFO 0 Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0a](rxf0a) module"]
pub type RXF0A = crate::Reg<u32, _RXF0A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF0A;
#[doc = "`read()` method returns [rxf0a::R](rxf0a::R) reader structure"]
impl crate::Readable for RXF0A {}
#[doc = "`write(|w| ..)` method takes [rxf0a::W](rxf0a::W) writer structure"]
impl crate::Writable for RXF0A {}
#[doc = "Rx FIFO 0 Acknowledge"]
pub mod rxf0a;
#[doc = "Rx Buffer Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbc](rxbc) module"]
pub type RXBC = crate::Reg<u32, _RXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBC;
#[doc = "`read()` method returns [rxbc::R](rxbc::R) reader structure"]
impl crate::Readable for RXBC {}
#[doc = "`write(|w| ..)` method takes [rxbc::W](rxbc::W) writer structure"]
impl crate::Writable for RXBC {}
#[doc = "Rx Buffer Configuration"]
pub mod rxbc;
#[doc = "Rx FIFO 1 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1c](rxf1c) module"]
pub type RXF1C = crate::Reg<u32, _RXF1C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1C;
#[doc = "`read()` method returns [rxf1c::R](rxf1c::R) reader structure"]
impl crate::Readable for RXF1C {}
#[doc = "`write(|w| ..)` method takes [rxf1c::W](rxf1c::W) writer structure"]
impl crate::Writable for RXF1C {}
#[doc = "Rx FIFO 1 Configuration"]
pub mod rxf1c;
#[doc = "Rx FIFO 1 Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1s](rxf1s) module"]
pub type RXF1S = crate::Reg<u32, _RXF1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1S;
#[doc = "`read()` method returns [rxf1s::R](rxf1s::R) reader structure"]
impl crate::Readable for RXF1S {}
#[doc = "Rx FIFO 1 Status"]
pub mod rxf1s;
#[doc = "Rx FIFO 1 Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1a](rxf1a) module"]
pub type RXF1A = crate::Reg<u32, _RXF1A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXF1A;
#[doc = "`read()` method returns [rxf1a::R](rxf1a::R) reader structure"]
impl crate::Readable for RXF1A {}
#[doc = "`write(|w| ..)` method takes [rxf1a::W](rxf1a::W) writer structure"]
impl crate::Writable for RXF1A {}
#[doc = "Rx FIFO 1 Acknowledge"]
pub mod rxf1a;
#[doc = "Rx Buffer / FIFO Element Size Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxesc](rxesc) module"]
pub type RXESC = crate::Reg<u32, _RXESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXESC;
#[doc = "`read()` method returns [rxesc::R](rxesc::R) reader structure"]
impl crate::Readable for RXESC {}
#[doc = "`write(|w| ..)` method takes [rxesc::W](rxesc::W) writer structure"]
impl crate::Writable for RXESC {}
#[doc = "Rx Buffer / FIFO Element Size Configuration"]
pub mod rxesc;
#[doc = "Tx Buffer Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbc](txbc) module"]
pub type TXBC = crate::Reg<u32, _TXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBC;
#[doc = "`read()` method returns [txbc::R](txbc::R) reader structure"]
impl crate::Readable for TXBC {}
#[doc = "`write(|w| ..)` method takes [txbc::W](txbc::W) writer structure"]
impl crate::Writable for TXBC {}
#[doc = "Tx Buffer Configuration"]
pub mod txbc;
#[doc = "Tx FIFO / Queue Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfqs](txfqs) module"]
pub type TXFQS = crate::Reg<u32, _TXFQS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFQS;
#[doc = "`read()` method returns [txfqs::R](txfqs::R) reader structure"]
impl crate::Readable for TXFQS {}
#[doc = "Tx FIFO / Queue Status"]
pub mod txfqs;
#[doc = "Tx Buffer Element Size Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txesc](txesc) module"]
pub type TXESC = crate::Reg<u32, _TXESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXESC;
#[doc = "`read()` method returns [txesc::R](txesc::R) reader structure"]
impl crate::Readable for TXESC {}
#[doc = "`write(|w| ..)` method takes [txesc::W](txesc::W) writer structure"]
impl crate::Writable for TXESC {}
#[doc = "Tx Buffer Element Size Configuration"]
pub mod txesc;
#[doc = "Tx Buffer Request Pending\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbrp](txbrp) module"]
pub type TXBRP = crate::Reg<u32, _TXBRP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBRP;
#[doc = "`read()` method returns [txbrp::R](txbrp::R) reader structure"]
impl crate::Readable for TXBRP {}
#[doc = "Tx Buffer Request Pending"]
pub mod txbrp;
#[doc = "Tx Buffer Add Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbar](txbar) module"]
pub type TXBAR = crate::Reg<u32, _TXBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBAR;
#[doc = "`read()` method returns [txbar::R](txbar::R) reader structure"]
impl crate::Readable for TXBAR {}
#[doc = "`write(|w| ..)` method takes [txbar::W](txbar::W) writer structure"]
impl crate::Writable for TXBAR {}
#[doc = "Tx Buffer Add Request"]
pub mod txbar;
#[doc = "Tx Buffer Cancellation Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcr](txbcr) module"]
pub type TXBCR = crate::Reg<u32, _TXBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCR;
#[doc = "`read()` method returns [txbcr::R](txbcr::R) reader structure"]
impl crate::Readable for TXBCR {}
#[doc = "`write(|w| ..)` method takes [txbcr::W](txbcr::W) writer structure"]
impl crate::Writable for TXBCR {}
#[doc = "Tx Buffer Cancellation Request"]
pub mod txbcr;
#[doc = "Tx Buffer Transmission Occurred\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbto](txbto) module"]
pub type TXBTO = crate::Reg<u32, _TXBTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBTO;
#[doc = "`read()` method returns [txbto::R](txbto::R) reader structure"]
impl crate::Readable for TXBTO {}
#[doc = "Tx Buffer Transmission Occurred"]
pub mod txbto;
#[doc = "Tx Buffer Cancellation Finished\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcf](txbcf) module"]
pub type TXBCF = crate::Reg<u32, _TXBCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCF;
#[doc = "`read()` method returns [txbcf::R](txbcf::R) reader structure"]
impl crate::Readable for TXBCF {}
#[doc = "Tx Buffer Cancellation Finished"]
pub mod txbcf;
#[doc = "Tx Buffer Transmission Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbtie](txbtie) module"]
pub type TXBTIE = crate::Reg<u32, _TXBTIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBTIE;
#[doc = "`read()` method returns [txbtie::R](txbtie::R) reader structure"]
impl crate::Readable for TXBTIE {}
#[doc = "`write(|w| ..)` method takes [txbtie::W](txbtie::W) writer structure"]
impl crate::Writable for TXBTIE {}
#[doc = "Tx Buffer Transmission Interrupt Enable"]
pub mod txbtie;
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcie](txbcie) module"]
pub type TXBCIE = crate::Reg<u32, _TXBCIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBCIE;
#[doc = "`read()` method returns [txbcie::R](txbcie::R) reader structure"]
impl crate::Readable for TXBCIE {}
#[doc = "`write(|w| ..)` method takes [txbcie::W](txbcie::W) writer structure"]
impl crate::Writable for TXBCIE {}
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
pub mod txbcie;
#[doc = "Tx Event FIFO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefc](txefc) module"]
pub type TXEFC = crate::Reg<u32, _TXEFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFC;
#[doc = "`read()` method returns [txefc::R](txefc::R) reader structure"]
impl crate::Readable for TXEFC {}
#[doc = "`write(|w| ..)` method takes [txefc::W](txefc::W) writer structure"]
impl crate::Writable for TXEFC {}
#[doc = "Tx Event FIFO Configuration"]
pub mod txefc;
#[doc = "Tx Event FIFO Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefs](txefs) module"]
pub type TXEFS = crate::Reg<u32, _TXEFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFS;
#[doc = "`read()` method returns [txefs::R](txefs::R) reader structure"]
impl crate::Readable for TXEFS {}
#[doc = "Tx Event FIFO Status"]
pub mod txefs;
#[doc = "Tx Event FIFO Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefa](txefa) module"]
pub type TXEFA = crate::Reg<u32, _TXEFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXEFA;
#[doc = "`read()` method returns [txefa::R](txefa::R) reader structure"]
impl crate::Readable for TXEFA {}
#[doc = "`write(|w| ..)` method takes [txefa::W](txefa::W) writer structure"]
impl crate::Writable for TXEFA {}
#[doc = "Tx Event FIFO Acknowledge"]
pub mod txefa;
