#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x08 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    #[doc = "0x14 - External Multipurpose Crystal Oscillator Control"]
    pub xoscctrl: [XOSCCTRL; 2],
    #[doc = "0x1c - DFLL48M Control A"]
    pub dfllctrla: DFLLCTRLA,
    _reserved7: [u8; 3usize],
    #[doc = "0x20 - DFLL48M Control B"]
    pub dfllctrlb: DFLLCTRLB,
    _reserved8: [u8; 3usize],
    #[doc = "0x24 - DFLL48M Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x28 - DFLL48M Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x2c - DFLL48M Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved11: [u8; 3usize],
    #[doc = "0x30 - DPLL\\[%s\\]"]
    pub dpll: [DPLL; 2],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct DPLL {
    #[doc = "0x00 - DPLL Control A"]
    pub dpllctrla: self::dpll::DPLLCTRLA,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - DPLL Ratio Control"]
    pub dpllratio: self::dpll::DPLLRATIO,
    #[doc = "0x08 - DPLL Control B"]
    pub dpllctrlb: self::dpll::DPLLCTRLB,
    #[doc = "0x0c - DPLL Synchronization Busy"]
    pub dpllsyncbusy: self::dpll::DPLLSYNCBUSY,
    #[doc = "0x10 - DPLL Status"]
    pub dpllstatus: self::dpll::DPLLSTATUS,
}
#[doc = r"Register block"]
#[doc = "DPLL\\[%s\\]"]
pub mod dpll;
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u8, _EVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCTRL;
#[doc = "`read()` method returns [evctrl::R](evctrl::R) reader structure"]
impl crate::Readable for EVCTRL {}
#[doc = "`write(|w| ..)` method takes [evctrl::W](evctrl::W) writer structure"]
impl crate::Writable for EVCTRL {}
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
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
pub type INTENSET = crate::Reg<u32, _INTENSET>;
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
pub type INTFLAG = crate::Reg<u32, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "External Multipurpose Crystal Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xoscctrl](xoscctrl) module"]
pub type XOSCCTRL = crate::Reg<u32, _XOSCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSCCTRL;
#[doc = "`read()` method returns [xoscctrl::R](xoscctrl::R) reader structure"]
impl crate::Readable for XOSCCTRL {}
#[doc = "`write(|w| ..)` method takes [xoscctrl::W](xoscctrl::W) writer structure"]
impl crate::Writable for XOSCCTRL {}
#[doc = "External Multipurpose Crystal Oscillator Control"]
pub mod xoscctrl;
#[doc = "DFLL48M Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllctrla](dfllctrla) module"]
pub type DFLLCTRLA = crate::Reg<u8, _DFLLCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLCTRLA;
#[doc = "`read()` method returns [dfllctrla::R](dfllctrla::R) reader structure"]
impl crate::Readable for DFLLCTRLA {}
#[doc = "`write(|w| ..)` method takes [dfllctrla::W](dfllctrla::W) writer structure"]
impl crate::Writable for DFLLCTRLA {}
#[doc = "DFLL48M Control A"]
pub mod dfllctrla;
#[doc = "DFLL48M Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllctrlb](dfllctrlb) module"]
pub type DFLLCTRLB = crate::Reg<u8, _DFLLCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLCTRLB;
#[doc = "`read()` method returns [dfllctrlb::R](dfllctrlb::R) reader structure"]
impl crate::Readable for DFLLCTRLB {}
#[doc = "`write(|w| ..)` method takes [dfllctrlb::W](dfllctrlb::W) writer structure"]
impl crate::Writable for DFLLCTRLB {}
#[doc = "DFLL48M Control B"]
pub mod dfllctrlb;
#[doc = "DFLL48M Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllval](dfllval) module"]
pub type DFLLVAL = crate::Reg<u32, _DFLLVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLVAL;
#[doc = "`read()` method returns [dfllval::R](dfllval::R) reader structure"]
impl crate::Readable for DFLLVAL {}
#[doc = "`write(|w| ..)` method takes [dfllval::W](dfllval::W) writer structure"]
impl crate::Writable for DFLLVAL {}
#[doc = "DFLL48M Value"]
pub mod dfllval;
#[doc = "DFLL48M Multiplier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllmul](dfllmul) module"]
pub type DFLLMUL = crate::Reg<u32, _DFLLMUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLMUL;
#[doc = "`read()` method returns [dfllmul::R](dfllmul::R) reader structure"]
impl crate::Readable for DFLLMUL {}
#[doc = "`write(|w| ..)` method takes [dfllmul::W](dfllmul::W) writer structure"]
impl crate::Writable for DFLLMUL {}
#[doc = "DFLL48M Multiplier"]
pub mod dfllmul;
#[doc = "DFLL48M Synchronization\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllsync](dfllsync) module"]
pub type DFLLSYNC = crate::Reg<u8, _DFLLSYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLSYNC;
#[doc = "`read()` method returns [dfllsync::R](dfllsync::R) reader structure"]
impl crate::Readable for DFLLSYNC {}
#[doc = "`write(|w| ..)` method takes [dfllsync::W](dfllsync::W) writer structure"]
impl crate::Writable for DFLLSYNC {}
#[doc = "DFLL48M Synchronization"]
pub mod dfllsync;
