#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x04 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0c - Power and Clocks Status"]
    pub pclksr: PCLKSR,
    #[doc = "0x10 - XOSC Control"]
    pub xosc: XOSC,
    _reserved5: [u8; 2usize],
    #[doc = "0x14 - XOSC32K Control"]
    pub xosc32k: XOSC32K,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - OSC32K Control"]
    pub osc32k: OSC32K,
    #[doc = "0x1c - OSCULP32K Control"]
    pub osculp32k: OSCULP32K,
    _reserved8: [u8; 3usize],
    #[doc = "0x20 - OSC8M Control A"]
    pub osc8m: OSC8M,
    #[doc = "0x24 - DFLL Config"]
    pub dfllctrl: DFLLCTRL,
    _reserved10: [u8; 2usize],
    #[doc = "0x28 - DFLL Calibration Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x2c - DFLL Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x30 - DFLL Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved13: [u8; 3usize],
    #[doc = "0x34 - 3.3V Brown-Out Detector (BOD33) Control"]
    pub bod33: BOD33,
    _reserved14: [u8; 4usize],
    #[doc = "0x3c - VREG Control"]
    pub vreg: VREG,
    _reserved15: [u8; 2usize],
    #[doc = "0x40 - VREF Control A"]
    pub vref: VREF,
}
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
#[doc = "Power and Clocks Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclksr](pclksr) module"]
pub type PCLKSR = crate::Reg<u32, _PCLKSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCLKSR;
#[doc = "`read()` method returns [pclksr::R](pclksr::R) reader structure"]
impl crate::Readable for PCLKSR {}
#[doc = "Power and Clocks Status"]
pub mod pclksr;
#[doc = "XOSC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc](xosc) module"]
pub type XOSC = crate::Reg<u16, _XOSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSC;
#[doc = "`read()` method returns [xosc::R](xosc::R) reader structure"]
impl crate::Readable for XOSC {}
#[doc = "`write(|w| ..)` method takes [xosc::W](xosc::W) writer structure"]
impl crate::Writable for XOSC {}
#[doc = "XOSC Control"]
pub mod xosc;
#[doc = "XOSC32K Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc32k](xosc32k) module"]
pub type XOSC32K = crate::Reg<u16, _XOSC32K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSC32K;
#[doc = "`read()` method returns [xosc32k::R](xosc32k::R) reader structure"]
impl crate::Readable for XOSC32K {}
#[doc = "`write(|w| ..)` method takes [xosc32k::W](xosc32k::W) writer structure"]
impl crate::Writable for XOSC32K {}
#[doc = "XOSC32K Control"]
pub mod xosc32k;
#[doc = "OSC32K Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc32k](osc32k) module"]
pub type OSC32K = crate::Reg<u32, _OSC32K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC32K;
#[doc = "`read()` method returns [osc32k::R](osc32k::R) reader structure"]
impl crate::Readable for OSC32K {}
#[doc = "`write(|w| ..)` method takes [osc32k::W](osc32k::W) writer structure"]
impl crate::Writable for OSC32K {}
#[doc = "OSC32K Control"]
pub mod osc32k;
#[doc = "OSCULP32K Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculp32k](osculp32k) module"]
pub type OSCULP32K = crate::Reg<u8, _OSCULP32K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCULP32K;
#[doc = "`read()` method returns [osculp32k::R](osculp32k::R) reader structure"]
impl crate::Readable for OSCULP32K {}
#[doc = "`write(|w| ..)` method takes [osculp32k::W](osculp32k::W) writer structure"]
impl crate::Writable for OSCULP32K {}
#[doc = "OSCULP32K Control"]
pub mod osculp32k;
#[doc = "OSC8M Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc8m](osc8m) module"]
pub type OSC8M = crate::Reg<u32, _OSC8M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC8M;
#[doc = "`read()` method returns [osc8m::R](osc8m::R) reader structure"]
impl crate::Readable for OSC8M {}
#[doc = "`write(|w| ..)` method takes [osc8m::W](osc8m::W) writer structure"]
impl crate::Writable for OSC8M {}
#[doc = "OSC8M Control A"]
pub mod osc8m;
#[doc = "DFLL Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllctrl](dfllctrl) module"]
pub type DFLLCTRL = crate::Reg<u16, _DFLLCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLCTRL;
#[doc = "`read()` method returns [dfllctrl::R](dfllctrl::R) reader structure"]
impl crate::Readable for DFLLCTRL {}
#[doc = "`write(|w| ..)` method takes [dfllctrl::W](dfllctrl::W) writer structure"]
impl crate::Writable for DFLLCTRL {}
#[doc = "DFLL Config"]
pub mod dfllctrl;
#[doc = "DFLL Calibration Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllval](dfllval) module"]
pub type DFLLVAL = crate::Reg<u32, _DFLLVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLVAL;
#[doc = "`read()` method returns [dfllval::R](dfllval::R) reader structure"]
impl crate::Readable for DFLLVAL {}
#[doc = "`write(|w| ..)` method takes [dfllval::W](dfllval::W) writer structure"]
impl crate::Writable for DFLLVAL {}
#[doc = "DFLL Calibration Value"]
pub mod dfllval;
#[doc = "DFLL Multiplier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllmul](dfllmul) module"]
pub type DFLLMUL = crate::Reg<u32, _DFLLMUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLMUL;
#[doc = "`read()` method returns [dfllmul::R](dfllmul::R) reader structure"]
impl crate::Readable for DFLLMUL {}
#[doc = "`write(|w| ..)` method takes [dfllmul::W](dfllmul::W) writer structure"]
impl crate::Writable for DFLLMUL {}
#[doc = "DFLL Multiplier"]
pub mod dfllmul;
#[doc = "DFLL Synchronization\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllsync](dfllsync) module"]
pub type DFLLSYNC = crate::Reg<u8, _DFLLSYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLSYNC;
#[doc = "`write(|w| ..)` method takes [dfllsync::W](dfllsync::W) writer structure"]
impl crate::Writable for DFLLSYNC {}
#[doc = "DFLL Synchronization"]
pub mod dfllsync;
#[doc = "3.3V Brown-Out Detector (BOD33) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33](bod33) module"]
pub type BOD33 = crate::Reg<u32, _BOD33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOD33;
#[doc = "`read()` method returns [bod33::R](bod33::R) reader structure"]
impl crate::Readable for BOD33 {}
#[doc = "`write(|w| ..)` method takes [bod33::W](bod33::W) writer structure"]
impl crate::Writable for BOD33 {}
#[doc = "3.3V Brown-Out Detector (BOD33) Control"]
pub mod bod33;
#[doc = "VREG Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vreg](vreg) module"]
pub type VREG = crate::Reg<u16, _VREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREG;
#[doc = "`read()` method returns [vreg::R](vreg::R) reader structure"]
impl crate::Readable for VREG {}
#[doc = "`write(|w| ..)` method takes [vreg::W](vreg::W) writer structure"]
impl crate::Writable for VREG {}
#[doc = "VREG Control"]
pub mod vreg;
#[doc = "VREF Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref](vref) module"]
pub type VREF = crate::Reg<u32, _VREF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF;
#[doc = "`read()` method returns [vref::R](vref::R) reader structure"]
impl crate::Readable for VREF {}
#[doc = "`write(|w| ..)` method takes [vref::W](vref::W) writer structure"]
impl crate::Writable for VREF {}
#[doc = "VREF Control A"]
pub mod vref;
