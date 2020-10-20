#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x02 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x03 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x04 - HS Clock Division"]
    pub hsdiv: HSDIV,
    #[doc = "0x05 - CPU Clock Division"]
    pub cpudiv: CPUDIV,
    _reserved5: [u8; 10usize],
    #[doc = "0x10 - AHB Mask"]
    pub ahbmask: AHBMASK,
    #[doc = "0x14 - APBA Mask"]
    pub apbamask: APBAMASK,
    #[doc = "0x18 - APBB Mask"]
    pub apbbmask: APBBMASK,
    #[doc = "0x1c - APBC Mask"]
    pub apbcmask: APBCMASK,
    #[doc = "0x20 - APBD Mask"]
    pub apbdmask: APBDMASK,
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u8, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "HS Clock Division\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsdiv](hsdiv) module"]
pub type HSDIV = crate::Reg<u8, _HSDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSDIV;
#[doc = "`read()` method returns [hsdiv::R](hsdiv::R) reader structure"]
impl crate::Readable for HSDIV {}
#[doc = "HS Clock Division"]
pub mod hsdiv;
#[doc = "CPU Clock Division\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpudiv](cpudiv) module"]
pub type CPUDIV = crate::Reg<u8, _CPUDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUDIV;
#[doc = "`read()` method returns [cpudiv::R](cpudiv::R) reader structure"]
impl crate::Readable for CPUDIV {}
#[doc = "`write(|w| ..)` method takes [cpudiv::W](cpudiv::W) writer structure"]
impl crate::Writable for CPUDIV {}
#[doc = "CPU Clock Division"]
pub mod cpudiv;
#[doc = "AHB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbmask](ahbmask) module"]
pub type AHBMASK = crate::Reg<u32, _AHBMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBMASK;
#[doc = "`read()` method returns [ahbmask::R](ahbmask::R) reader structure"]
impl crate::Readable for AHBMASK {}
#[doc = "`write(|w| ..)` method takes [ahbmask::W](ahbmask::W) writer structure"]
impl crate::Writable for AHBMASK {}
#[doc = "AHB Mask"]
pub mod ahbmask;
#[doc = "APBA Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbamask](apbamask) module"]
pub type APBAMASK = crate::Reg<u32, _APBAMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBAMASK;
#[doc = "`read()` method returns [apbamask::R](apbamask::R) reader structure"]
impl crate::Readable for APBAMASK {}
#[doc = "`write(|w| ..)` method takes [apbamask::W](apbamask::W) writer structure"]
impl crate::Writable for APBAMASK {}
#[doc = "APBA Mask"]
pub mod apbamask;
#[doc = "APBB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbbmask](apbbmask) module"]
pub type APBBMASK = crate::Reg<u32, _APBBMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBBMASK;
#[doc = "`read()` method returns [apbbmask::R](apbbmask::R) reader structure"]
impl crate::Readable for APBBMASK {}
#[doc = "`write(|w| ..)` method takes [apbbmask::W](apbbmask::W) writer structure"]
impl crate::Writable for APBBMASK {}
#[doc = "APBB Mask"]
pub mod apbbmask;
#[doc = "APBC Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbcmask](apbcmask) module"]
pub type APBCMASK = crate::Reg<u32, _APBCMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBCMASK;
#[doc = "`read()` method returns [apbcmask::R](apbcmask::R) reader structure"]
impl crate::Readable for APBCMASK {}
#[doc = "`write(|w| ..)` method takes [apbcmask::W](apbcmask::W) writer structure"]
impl crate::Writable for APBCMASK {}
#[doc = "APBC Mask"]
pub mod apbcmask;
#[doc = "APBD Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbdmask](apbdmask) module"]
pub type APBDMASK = crate::Reg<u32, _APBDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBDMASK;
#[doc = "`read()` method returns [apbdmask::R](apbdmask::R) reader structure"]
impl crate::Readable for APBDMASK {}
#[doc = "`write(|w| ..)` method takes [apbdmask::W](apbdmask::W) writer structure"]
impl crate::Writable for APBDMASK {}
#[doc = "APBD Mask"]
pub mod apbdmask;
