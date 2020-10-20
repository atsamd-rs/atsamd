#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Sleep Mode"]
    pub sleep: SLEEP,
    _reserved2: [u8; 6usize],
    #[doc = "0x08 - CPU Clock Select"]
    pub cpusel: CPUSEL,
    #[doc = "0x09 - APBA Clock Select"]
    pub apbasel: APBASEL,
    #[doc = "0x0a - APBB Clock Select"]
    pub apbbsel: APBBSEL,
    #[doc = "0x0b - APBC Clock Select"]
    pub apbcsel: APBCSEL,
    _reserved6: [u8; 8usize],
    #[doc = "0x14 - AHB Mask"]
    pub ahbmask: AHBMASK,
    #[doc = "0x18 - APBA Mask"]
    pub apbamask: APBAMASK,
    #[doc = "0x1c - APBB Mask"]
    pub apbbmask: APBBMASK,
    #[doc = "0x20 - APBC Mask"]
    pub apbcmask: APBCMASK,
    _reserved10: [u8; 16usize],
    #[doc = "0x34 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x35 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x36 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved13: [u8; 1usize],
    #[doc = "0x38 - Reset Cause"]
    pub rcause: RCAUSE,
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u8, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleep](sleep) module"]
pub type SLEEP = crate::Reg<u8, _SLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEP;
#[doc = "`read()` method returns [sleep::R](sleep::R) reader structure"]
impl crate::Readable for SLEEP {}
#[doc = "`write(|w| ..)` method takes [sleep::W](sleep::W) writer structure"]
impl crate::Writable for SLEEP {}
#[doc = "Sleep Mode"]
pub mod sleep;
#[doc = "CPU Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpusel](cpusel) module"]
pub type CPUSEL = crate::Reg<u8, _CPUSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUSEL;
#[doc = "`read()` method returns [cpusel::R](cpusel::R) reader structure"]
impl crate::Readable for CPUSEL {}
#[doc = "`write(|w| ..)` method takes [cpusel::W](cpusel::W) writer structure"]
impl crate::Writable for CPUSEL {}
#[doc = "CPU Clock Select"]
pub mod cpusel;
#[doc = "APBA Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbasel](apbasel) module"]
pub type APBASEL = crate::Reg<u8, _APBASEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBASEL;
#[doc = "`read()` method returns [apbasel::R](apbasel::R) reader structure"]
impl crate::Readable for APBASEL {}
#[doc = "`write(|w| ..)` method takes [apbasel::W](apbasel::W) writer structure"]
impl crate::Writable for APBASEL {}
#[doc = "APBA Clock Select"]
pub mod apbasel;
#[doc = "APBB Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbbsel](apbbsel) module"]
pub type APBBSEL = crate::Reg<u8, _APBBSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBBSEL;
#[doc = "`read()` method returns [apbbsel::R](apbbsel::R) reader structure"]
impl crate::Readable for APBBSEL {}
#[doc = "`write(|w| ..)` method takes [apbbsel::W](apbbsel::W) writer structure"]
impl crate::Writable for APBBSEL {}
#[doc = "APBB Clock Select"]
pub mod apbbsel;
#[doc = "APBC Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbcsel](apbcsel) module"]
pub type APBCSEL = crate::Reg<u8, _APBCSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APBCSEL;
#[doc = "`read()` method returns [apbcsel::R](apbcsel::R) reader structure"]
impl crate::Readable for APBCSEL {}
#[doc = "`write(|w| ..)` method takes [apbcsel::W](apbcsel::W) writer structure"]
impl crate::Writable for APBCSEL {}
#[doc = "APBC Clock Select"]
pub mod apbcsel;
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
#[doc = "Reset Cause\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcause](rcause) module"]
pub type RCAUSE = crate::Reg<u8, _RCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCAUSE;
#[doc = "`read()` method returns [rcause::R](rcause::R) reader structure"]
impl crate::Readable for RCAUSE {}
#[doc = "Reset Cause"]
pub mod rcause;
