#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write control"]
    pub wrctrl: WRCTRL,
    #[doc = "0x04 - Event control"]
    pub evctrl: EVCTRL,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt enable set"]
    pub intenset: INTENSET,
    _reserved4: [u8; 6usize],
    #[doc = "0x10 - Bridge interrupt flag status"]
    pub intflagahb: INTFLAGAHB,
    #[doc = "0x14 - Peripheral interrupt flag status - Bridge A"]
    pub intflaga: INTFLAGA,
    #[doc = "0x18 - Peripheral interrupt flag status - Bridge B"]
    pub intflagb: INTFLAGB,
    #[doc = "0x1c - Peripheral interrupt flag status - Bridge C"]
    pub intflagc: INTFLAGC,
    #[doc = "0x20 - Peripheral interrupt flag status - Bridge D"]
    pub intflagd: INTFLAGD,
    _reserved9: [u8; 16usize],
    #[doc = "0x34 - Peripheral write protection status - Bridge A"]
    pub statusa: STATUSA,
    #[doc = "0x38 - Peripheral write protection status - Bridge B"]
    pub statusb: STATUSB,
    #[doc = "0x3c - Peripheral write protection status - Bridge C"]
    pub statusc: STATUSC,
    #[doc = "0x40 - Peripheral write protection status - Bridge D"]
    pub statusd: STATUSD,
}
#[doc = "Write control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrctrl](wrctrl) module"]
pub type WRCTRL = crate::Reg<u32, _WRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRCTRL;
#[doc = "`read()` method returns [wrctrl::R](wrctrl::R) reader structure"]
impl crate::Readable for WRCTRL {}
#[doc = "`write(|w| ..)` method takes [wrctrl::W](wrctrl::W) writer structure"]
impl crate::Writable for WRCTRL {}
#[doc = "Write control"]
pub mod wrctrl;
#[doc = "Event control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u8, _EVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTRL;
#[doc = "`read()` method returns [evctrl::R](evctrl::R) reader structure"]
impl crate::Readable for EVCTRL {}
#[doc = "`write(|w| ..)` method takes [evctrl::W](evctrl::W) writer structure"]
impl crate::Writable for EVCTRL {}
#[doc = "Event control"]
pub mod evctrl;
#[doc = "Interrupt enable clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "Interrupt enable set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u8, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt enable set"]
pub mod intenset;
#[doc = "Bridge interrupt flag status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagahb](intflagahb) module"]
pub type INTFLAGAHB = crate::Reg<u32, _INTFLAGAHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAGAHB;
#[doc = "`read()` method returns [intflagahb::R](intflagahb::R) reader structure"]
impl crate::Readable for INTFLAGAHB {}
#[doc = "`write(|w| ..)` method takes [intflagahb::W](intflagahb::W) writer structure"]
impl crate::Writable for INTFLAGAHB {}
#[doc = "Bridge interrupt flag status"]
pub mod intflagahb;
#[doc = "Peripheral interrupt flag status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflaga](intflaga) module"]
pub type INTFLAGA = crate::Reg<u32, _INTFLAGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAGA;
#[doc = "`read()` method returns [intflaga::R](intflaga::R) reader structure"]
impl crate::Readable for INTFLAGA {}
#[doc = "`write(|w| ..)` method takes [intflaga::W](intflaga::W) writer structure"]
impl crate::Writable for INTFLAGA {}
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub mod intflaga;
#[doc = "Peripheral interrupt flag status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagb](intflagb) module"]
pub type INTFLAGB = crate::Reg<u32, _INTFLAGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAGB;
#[doc = "`read()` method returns [intflagb::R](intflagb::R) reader structure"]
impl crate::Readable for INTFLAGB {}
#[doc = "`write(|w| ..)` method takes [intflagb::W](intflagb::W) writer structure"]
impl crate::Writable for INTFLAGB {}
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub mod intflagb;
#[doc = "Peripheral interrupt flag status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagc](intflagc) module"]
pub type INTFLAGC = crate::Reg<u32, _INTFLAGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAGC;
#[doc = "`read()` method returns [intflagc::R](intflagc::R) reader structure"]
impl crate::Readable for INTFLAGC {}
#[doc = "`write(|w| ..)` method takes [intflagc::W](intflagc::W) writer structure"]
impl crate::Writable for INTFLAGC {}
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub mod intflagc;
#[doc = "Peripheral interrupt flag status - Bridge D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagd](intflagd) module"]
pub type INTFLAGD = crate::Reg<u32, _INTFLAGD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAGD;
#[doc = "`read()` method returns [intflagd::R](intflagd::R) reader structure"]
impl crate::Readable for INTFLAGD {}
#[doc = "`write(|w| ..)` method takes [intflagd::W](intflagd::W) writer structure"]
impl crate::Writable for INTFLAGD {}
#[doc = "Peripheral interrupt flag status - Bridge D"]
pub mod intflagd;
#[doc = "Peripheral write protection status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusa](statusa) module"]
pub type STATUSA = crate::Reg<u32, _STATUSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSA;
#[doc = "`read()` method returns [statusa::R](statusa::R) reader structure"]
impl crate::Readable for STATUSA {}
#[doc = "Peripheral write protection status - Bridge A"]
pub mod statusa;
#[doc = "Peripheral write protection status - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusb](statusb) module"]
pub type STATUSB = crate::Reg<u32, _STATUSB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSB;
#[doc = "`read()` method returns [statusb::R](statusb::R) reader structure"]
impl crate::Readable for STATUSB {}
#[doc = "Peripheral write protection status - Bridge B"]
pub mod statusb;
#[doc = "Peripheral write protection status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusc](statusc) module"]
pub type STATUSC = crate::Reg<u32, _STATUSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSC;
#[doc = "`read()` method returns [statusc::R](statusc::R) reader structure"]
impl crate::Readable for STATUSC {}
#[doc = "Peripheral write protection status - Bridge C"]
pub mod statusc;
#[doc = "Peripheral write protection status - Bridge D\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusd](statusd) module"]
pub type STATUSD = crate::Reg<u32, _STATUSD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUSD;
#[doc = "`read()` method returns [statusd::R](statusd::R) reader structure"]
impl crate::Readable for STATUSD {}
#[doc = "Peripheral write protection status - Bridge D"]
pub mod statusd;
