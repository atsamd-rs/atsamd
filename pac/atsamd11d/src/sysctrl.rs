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
    #[doc = "0x10 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xosc: XOSC,
    _reserved5: [u8; 2usize],
    #[doc = "0x14 - 32kHz External Crystal Oscillator (XOSC32K) Control"]
    pub xosc32k: XOSC32K,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - 32kHz Internal Oscillator (OSC32K) Control"]
    pub osc32k: OSC32K,
    #[doc = "0x1c - 32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
    pub osculp32k: OSCULP32K,
    _reserved8: [u8; 3usize],
    #[doc = "0x20 - 8MHz Internal Oscillator (OSC8M) Control"]
    pub osc8m: OSC8M,
    #[doc = "0x24 - DFLL48M Control"]
    pub dfllctrl: DFLLCTRL,
    _reserved10: [u8; 2usize],
    #[doc = "0x28 - DFLL48M Value"]
    pub dfllval: DFLLVAL,
    #[doc = "0x2c - DFLL48M Multiplier"]
    pub dfllmul: DFLLMUL,
    #[doc = "0x30 - DFLL48M Synchronization"]
    pub dfllsync: DFLLSYNC,
    _reserved13: [u8; 3usize],
    #[doc = "0x34 - 3.3V Brown-Out Detector (BOD33) Control"]
    pub bod33: BOD33,
    _reserved14: [u8; 8usize],
    #[doc = "0x40 - Voltage References System (VREF) Control"]
    pub vref: VREF,
    #[doc = "0x44 - DPLL Control A"]
    pub dpllctrla: DPLLCTRLA,
    _reserved16: [u8; 3usize],
    #[doc = "0x48 - DPLL Ratio Control"]
    pub dpllratio: DPLLRATIO,
    #[doc = "0x4c - DPLL Control B"]
    pub dpllctrlb: DPLLCTRLB,
    #[doc = "0x50 - DPLL Status"]
    pub dpllstatus: DPLLSTATUS,
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
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc](xosc) module"]
pub type XOSC = crate::Reg<u16, _XOSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSC;
#[doc = "`read()` method returns [xosc::R](xosc::R) reader structure"]
impl crate::Readable for XOSC {}
#[doc = "`write(|w| ..)` method takes [xosc::W](xosc::W) writer structure"]
impl crate::Writable for XOSC {}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xosc;
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc32k](xosc32k) module"]
pub type XOSC32K = crate::Reg<u16, _XOSC32K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSC32K;
#[doc = "`read()` method returns [xosc32k::R](xosc32k::R) reader structure"]
impl crate::Readable for XOSC32K {}
#[doc = "`write(|w| ..)` method takes [xosc32k::W](xosc32k::W) writer structure"]
impl crate::Writable for XOSC32K {}
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control"]
pub mod xosc32k;
#[doc = "32kHz Internal Oscillator (OSC32K) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc32k](osc32k) module"]
pub type OSC32K = crate::Reg<u32, _OSC32K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC32K;
#[doc = "`read()` method returns [osc32k::R](osc32k::R) reader structure"]
impl crate::Readable for OSC32K {}
#[doc = "`write(|w| ..)` method takes [osc32k::W](osc32k::W) writer structure"]
impl crate::Writable for OSC32K {}
#[doc = "32kHz Internal Oscillator (OSC32K) Control"]
pub mod osc32k;
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculp32k](osculp32k) module"]
pub type OSCULP32K = crate::Reg<u8, _OSCULP32K>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCULP32K;
#[doc = "`read()` method returns [osculp32k::R](osculp32k::R) reader structure"]
impl crate::Readable for OSCULP32K {}
#[doc = "`write(|w| ..)` method takes [osculp32k::W](osculp32k::W) writer structure"]
impl crate::Writable for OSCULP32K {}
#[doc = "32kHz Ultra Low Power Internal Oscillator (OSCULP32K) Control"]
pub mod osculp32k;
#[doc = "8MHz Internal Oscillator (OSC8M) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc8m](osc8m) module"]
pub type OSC8M = crate::Reg<u32, _OSC8M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC8M;
#[doc = "`read()` method returns [osc8m::R](osc8m::R) reader structure"]
impl crate::Readable for OSC8M {}
#[doc = "`write(|w| ..)` method takes [osc8m::W](osc8m::W) writer structure"]
impl crate::Writable for OSC8M {}
#[doc = "8MHz Internal Oscillator (OSC8M) Control"]
pub mod osc8m;
#[doc = "DFLL48M Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllctrl](dfllctrl) module"]
pub type DFLLCTRL = crate::Reg<u16, _DFLLCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLCTRL;
#[doc = "`read()` method returns [dfllctrl::R](dfllctrl::R) reader structure"]
impl crate::Readable for DFLLCTRL {}
#[doc = "`write(|w| ..)` method takes [dfllctrl::W](dfllctrl::W) writer structure"]
impl crate::Writable for DFLLCTRL {}
#[doc = "DFLL48M Control"]
pub mod dfllctrl;
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
#[doc = "DFLL48M Synchronization\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllsync](dfllsync) module"]
pub type DFLLSYNC = crate::Reg<u8, _DFLLSYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFLLSYNC;
#[doc = "`write(|w| ..)` method takes [dfllsync::W](dfllsync::W) writer structure"]
impl crate::Writable for DFLLSYNC {}
#[doc = "DFLL48M Synchronization"]
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
#[doc = "Voltage References System (VREF) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref](vref) module"]
pub type VREF = crate::Reg<u32, _VREF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREF;
#[doc = "`read()` method returns [vref::R](vref::R) reader structure"]
impl crate::Readable for VREF {}
#[doc = "`write(|w| ..)` method takes [vref::W](vref::W) writer structure"]
impl crate::Writable for VREF {}
#[doc = "Voltage References System (VREF) Control"]
pub mod vref;
#[doc = "DPLL Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrla](dpllctrla) module"]
pub type DPLLCTRLA = crate::Reg<u8, _DPLLCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLCTRLA;
#[doc = "`read()` method returns [dpllctrla::R](dpllctrla::R) reader structure"]
impl crate::Readable for DPLLCTRLA {}
#[doc = "`write(|w| ..)` method takes [dpllctrla::W](dpllctrla::W) writer structure"]
impl crate::Writable for DPLLCTRLA {}
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLL Ratio Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllratio](dpllratio) module"]
pub type DPLLRATIO = crate::Reg<u32, _DPLLRATIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLRATIO;
#[doc = "`read()` method returns [dpllratio::R](dpllratio::R) reader structure"]
impl crate::Readable for DPLLRATIO {}
#[doc = "`write(|w| ..)` method takes [dpllratio::W](dpllratio::W) writer structure"]
impl crate::Writable for DPLLRATIO {}
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLL Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrlb](dpllctrlb) module"]
pub type DPLLCTRLB = crate::Reg<u32, _DPLLCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLCTRLB;
#[doc = "`read()` method returns [dpllctrlb::R](dpllctrlb::R) reader structure"]
impl crate::Readable for DPLLCTRLB {}
#[doc = "`write(|w| ..)` method takes [dpllctrlb::W](dpllctrlb::W) writer structure"]
impl crate::Writable for DPLLCTRLB {}
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllstatus](dpllstatus) module"]
pub type DPLLSTATUS = crate::Reg<u8, _DPLLSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPLLSTATUS;
#[doc = "`read()` method returns [dpllstatus::R](dpllstatus::R) reader structure"]
impl crate::Readable for DPLLSTATUS {}
#[doc = "DPLL Status"]
pub mod dpllstatus;
