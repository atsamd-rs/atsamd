#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x08 - Status"]
    pub sr: SR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Interrupt Enable"]
    pub ier: IER,
    #[doc = "0x14 - Interrupt Disable"]
    pub idr: IDR,
    #[doc = "0x18 - Interrupt Mask"]
    pub imr: IMR,
    #[doc = "0x1c - Interrupt Status"]
    pub isr: ISR,
    #[doc = "0x20 - Undefined Access Status"]
    pub uasr: UASR,
    _reserved8: [u8; 12usize],
    #[doc = "0x30 - Region Descriptor Area Start Address"]
    pub dscr: DSCR,
    #[doc = "0x34 - Region Hash Area Start Address"]
    pub hash: HASH,
    #[doc = "0x38 - User Initial Hash Value n"]
    pub uihval: [UIHVAL; 8],
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration"]
pub mod cfg;
#[doc = "Control\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status"]
pub mod sr;
#[doc = "Interrupt Enable\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "Interrupt Disable\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable"]
pub mod idr;
#[doc = "Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask"]
pub mod imr;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status"]
pub mod isr;
#[doc = "Undefined Access Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uasr](uasr) module"]
pub type UASR = crate::Reg<u32, _UASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UASR;
#[doc = "`read()` method returns [uasr::R](uasr::R) reader structure"]
impl crate::Readable for UASR {}
#[doc = "Undefined Access Status"]
pub mod uasr;
#[doc = "Region Descriptor Area Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscr](dscr) module"]
pub type DSCR = crate::Reg<u32, _DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCR;
#[doc = "`read()` method returns [dscr::R](dscr::R) reader structure"]
impl crate::Readable for DSCR {}
#[doc = "`write(|w| ..)` method takes [dscr::W](dscr::W) writer structure"]
impl crate::Writable for DSCR {}
#[doc = "Region Descriptor Area Start Address"]
pub mod dscr;
#[doc = "Region Hash Area Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash](hash) module"]
pub type HASH = crate::Reg<u32, _HASH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASH;
#[doc = "`read()` method returns [hash::R](hash::R) reader structure"]
impl crate::Readable for HASH {}
#[doc = "`write(|w| ..)` method takes [hash::W](hash::W) writer structure"]
impl crate::Writable for HASH {}
#[doc = "Region Hash Area Start Address"]
pub mod hash;
#[doc = "User Initial Hash Value n\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uihval](uihval) module"]
pub type UIHVAL = crate::Reg<u32, _UIHVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIHVAL;
#[doc = "`write(|w| ..)` method takes [uihval::W](uihval::W) writer structure"]
impl crate::Writable for UIHVAL {}
#[doc = "User Initial Hash Value n"]
pub mod uihval;
