#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: CTRLBSET,
    _reserved3: [u8; 2usize],
    #[doc = "0x08 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - Recoverable Fault A Configuration"]
    pub fctrla: FCTRLA,
    #[doc = "0x10 - Recoverable Fault B Configuration"]
    pub fctrlb: FCTRLB,
    #[doc = "0x14 - Waveform Extension Configuration"]
    pub wexctrl: WEXCTRL,
    #[doc = "0x18 - Driver Control"]
    pub drvctrl: DRVCTRL,
    _reserved8: [u8; 2usize],
    #[doc = "0x1e - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved9: [u8; 1usize],
    #[doc = "0x20 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x24 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x28 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x2c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x30 - Status"]
    pub status: STATUS,
    _reserved_14_count: [u8; 4usize],
    #[doc = "0x38 - Pattern"]
    pub patt: PATT,
    _reserved16: [u8; 2usize],
    #[doc = "0x3c - Waveform Control"]
    pub wave: WAVE,
    _reserved_17_per: [u8; 4usize],
    _reserved_18_cc: [u8; 16usize],
    _reserved19: [u8; 16usize],
    #[doc = "0x64 - Pattern Buffer"]
    pub pattb: PATTB,
    _reserved20: [u8; 2usize],
    #[doc = "0x68 - Waveform Control Buffer"]
    pub waveb: WAVEB,
    _reserved_21_perb: [u8; 4usize],
    _reserved_22_ccb: [u8; 16usize],
}
impl RegisterBlock {
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith6(&self) -> &COUNT_DITH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH6) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith6_mut(&self) -> &mut COUNT_DITH6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT_DITH6) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith5(&self) -> &COUNT_DITH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH5) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith5_mut(&self) -> &mut COUNT_DITH5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT_DITH5) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith4(&self) -> &COUNT_DITH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH4) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith4_mut(&self) -> &mut COUNT_DITH4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT_DITH4) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count(&self) -> &COUNT {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_mut(&self) -> &mut COUNT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith6(&self) -> &PER_DITH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH6) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith6_mut(&self) -> &mut PER_DITH6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER_DITH6) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith5(&self) -> &PER_DITH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH5) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith5_mut(&self) -> &mut PER_DITH5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER_DITH5) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith4(&self) -> &PER_DITH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH4) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith4_mut(&self) -> &mut PER_DITH4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER_DITH4) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per(&self) -> &PER {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_mut(&self) -> &mut PER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith6(&self) -> &[CC_DITH6; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH6; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith6_mut(&self) -> &mut [CC_DITH6; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC_DITH6; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith5(&self) -> &[CC_DITH5; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH5; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith5_mut(&self) -> &mut [CC_DITH5; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC_DITH5; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith4(&self) -> &[CC_DITH4; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH4; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith4_mut(&self) -> &mut [CC_DITH4; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC_DITH4; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc(&self) -> &[CC; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC; 4]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_mut(&self) -> &mut [CC; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC; 4]) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith6(&self) -> &PERB_DITH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB_DITH6) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith6_mut(&self) -> &mut PERB_DITH6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERB_DITH6) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith5(&self) -> &PERB_DITH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB_DITH5) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith5_mut(&self) -> &mut PERB_DITH5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERB_DITH5) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith4(&self) -> &PERB_DITH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB_DITH4) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith4_mut(&self) -> &mut PERB_DITH4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERB_DITH4) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb(&self) -> &PERB {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_mut(&self) -> &mut PERB {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERB) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith6(&self) -> &[CCB_DITH6; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB_DITH6; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith6_mut(&self) -> &mut [CCB_DITH6; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCB_DITH6; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith5(&self) -> &[CCB_DITH5; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB_DITH5; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith5_mut(&self) -> &mut [CCB_DITH5; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCB_DITH5; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith4(&self) -> &[CCB_DITH4; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB_DITH4; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith4_mut(&self) -> &mut [CCB_DITH4; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCB_DITH4; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb(&self) -> &[CCB; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB; 4]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_mut(&self) -> &mut [CCB; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCB; 4]) }
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u32, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlbclr](ctrlbclr) module"]
pub type CTRLBCLR = crate::Reg<u8, _CTRLBCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLBCLR;
#[doc = "`read()` method returns [ctrlbclr::R](ctrlbclr::R) reader structure"]
impl crate::Readable for CTRLBCLR {}
#[doc = "`write(|w| ..)` method takes [ctrlbclr::W](ctrlbclr::W) writer structure"]
impl crate::Writable for CTRLBCLR {}
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "Control B Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlbset](ctrlbset) module"]
pub type CTRLBSET = crate::Reg<u8, _CTRLBSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLBSET;
#[doc = "`read()` method returns [ctrlbset::R](ctrlbset::R) reader structure"]
impl crate::Readable for CTRLBSET {}
#[doc = "`write(|w| ..)` method takes [ctrlbset::W](ctrlbset::W) writer structure"]
impl crate::Writable for CTRLBSET {}
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "Recoverable Fault A Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctrla](fctrla) module"]
pub type FCTRLA = crate::Reg<u32, _FCTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTRLA;
#[doc = "`read()` method returns [fctrla::R](fctrla::R) reader structure"]
impl crate::Readable for FCTRLA {}
#[doc = "`write(|w| ..)` method takes [fctrla::W](fctrla::W) writer structure"]
impl crate::Writable for FCTRLA {}
#[doc = "Recoverable Fault A Configuration"]
pub mod fctrla;
#[doc = "Recoverable Fault B Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctrlb](fctrlb) module"]
pub type FCTRLB = crate::Reg<u32, _FCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTRLB;
#[doc = "`read()` method returns [fctrlb::R](fctrlb::R) reader structure"]
impl crate::Readable for FCTRLB {}
#[doc = "`write(|w| ..)` method takes [fctrlb::W](fctrlb::W) writer structure"]
impl crate::Writable for FCTRLB {}
#[doc = "Recoverable Fault B Configuration"]
pub mod fctrlb;
#[doc = "Waveform Extension Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wexctrl](wexctrl) module"]
pub type WEXCTRL = crate::Reg<u32, _WEXCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WEXCTRL;
#[doc = "`read()` method returns [wexctrl::R](wexctrl::R) reader structure"]
impl crate::Readable for WEXCTRL {}
#[doc = "`write(|w| ..)` method takes [wexctrl::W](wexctrl::W) writer structure"]
impl crate::Writable for WEXCTRL {}
#[doc = "Waveform Extension Configuration"]
pub mod wexctrl;
#[doc = "Driver Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drvctrl](drvctrl) module"]
pub type DRVCTRL = crate::Reg<u32, _DRVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRVCTRL;
#[doc = "`read()` method returns [drvctrl::R](drvctrl::R) reader structure"]
impl crate::Readable for DRVCTRL {}
#[doc = "`write(|w| ..)` method takes [drvctrl::W](drvctrl::W) writer structure"]
impl crate::Writable for DRVCTRL {}
#[doc = "Driver Control"]
pub mod drvctrl;
#[doc = "Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgctrl](dbgctrl) module"]
pub type DBGCTRL = crate::Reg<u8, _DBGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGCTRL;
#[doc = "`read()` method returns [dbgctrl::R](dbgctrl::R) reader structure"]
impl crate::Readable for DBGCTRL {}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](dbgctrl::W) writer structure"]
impl crate::Writable for DBGCTRL {}
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](evctrl) module"]
pub type EVCTRL = crate::Reg<u32, _EVCTRL>;
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
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count](count) module"]
pub type COUNT = crate::Reg<u32, _COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT;
#[doc = "`read()` method returns [count::R](count::R) reader structure"]
impl crate::Readable for COUNT {}
#[doc = "`write(|w| ..)` method takes [count::W](count::W) writer structure"]
impl crate::Writable for COUNT {}
#[doc = "Count"]
pub mod count;
#[doc = "Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_dith4](count_dith4) module"]
pub type COUNT_DITH4 = crate::Reg<u32, _COUNT_DITH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT_DITH4;
#[doc = "`read()` method returns [count_dith4::R](count_dith4::R) reader structure"]
impl crate::Readable for COUNT_DITH4 {}
#[doc = "`write(|w| ..)` method takes [count_dith4::W](count_dith4::W) writer structure"]
impl crate::Writable for COUNT_DITH4 {}
#[doc = "Count"]
pub mod count_dith4;
#[doc = "Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_dith5](count_dith5) module"]
pub type COUNT_DITH5 = crate::Reg<u32, _COUNT_DITH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT_DITH5;
#[doc = "`read()` method returns [count_dith5::R](count_dith5::R) reader structure"]
impl crate::Readable for COUNT_DITH5 {}
#[doc = "`write(|w| ..)` method takes [count_dith5::W](count_dith5::W) writer structure"]
impl crate::Writable for COUNT_DITH5 {}
#[doc = "Count"]
pub mod count_dith5;
#[doc = "Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_dith6](count_dith6) module"]
pub type COUNT_DITH6 = crate::Reg<u32, _COUNT_DITH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT_DITH6;
#[doc = "`read()` method returns [count_dith6::R](count_dith6::R) reader structure"]
impl crate::Readable for COUNT_DITH6 {}
#[doc = "`write(|w| ..)` method takes [count_dith6::W](count_dith6::W) writer structure"]
impl crate::Writable for COUNT_DITH6 {}
#[doc = "Count"]
pub mod count_dith6;
#[doc = "Pattern\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt](patt) module"]
pub type PATT = crate::Reg<u16, _PATT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATT;
#[doc = "`read()` method returns [patt::R](patt::R) reader structure"]
impl crate::Readable for PATT {}
#[doc = "`write(|w| ..)` method takes [patt::W](patt::W) writer structure"]
impl crate::Writable for PATT {}
#[doc = "Pattern"]
pub mod patt;
#[doc = "Waveform Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wave](wave) module"]
pub type WAVE = crate::Reg<u32, _WAVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAVE;
#[doc = "`read()` method returns [wave::R](wave::R) reader structure"]
impl crate::Readable for WAVE {}
#[doc = "`write(|w| ..)` method takes [wave::W](wave::W) writer structure"]
impl crate::Writable for WAVE {}
#[doc = "Waveform Control"]
pub mod wave;
#[doc = "Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per](per) module"]
pub type PER = crate::Reg<u32, _PER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER;
#[doc = "`read()` method returns [per::R](per::R) reader structure"]
impl crate::Readable for PER {}
#[doc = "`write(|w| ..)` method takes [per::W](per::W) writer structure"]
impl crate::Writable for PER {}
#[doc = "Period"]
pub mod per;
#[doc = "Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_dith4](per_dith4) module"]
pub type PER_DITH4 = crate::Reg<u32, _PER_DITH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_DITH4;
#[doc = "`read()` method returns [per_dith4::R](per_dith4::R) reader structure"]
impl crate::Readable for PER_DITH4 {}
#[doc = "`write(|w| ..)` method takes [per_dith4::W](per_dith4::W) writer structure"]
impl crate::Writable for PER_DITH4 {}
#[doc = "Period"]
pub mod per_dith4;
#[doc = "Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_dith5](per_dith5) module"]
pub type PER_DITH5 = crate::Reg<u32, _PER_DITH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_DITH5;
#[doc = "`read()` method returns [per_dith5::R](per_dith5::R) reader structure"]
impl crate::Readable for PER_DITH5 {}
#[doc = "`write(|w| ..)` method takes [per_dith5::W](per_dith5::W) writer structure"]
impl crate::Writable for PER_DITH5 {}
#[doc = "Period"]
pub mod per_dith5;
#[doc = "Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_dith6](per_dith6) module"]
pub type PER_DITH6 = crate::Reg<u32, _PER_DITH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_DITH6;
#[doc = "`read()` method returns [per_dith6::R](per_dith6::R) reader structure"]
impl crate::Readable for PER_DITH6 {}
#[doc = "`write(|w| ..)` method takes [per_dith6::W](per_dith6::W) writer structure"]
impl crate::Writable for PER_DITH6 {}
#[doc = "Period"]
pub mod per_dith6;
#[doc = "Compare and Capture\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "Compare and Capture"]
pub mod cc;
#[doc = "Compare and Capture\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_dith4](cc_dith4) module"]
pub type CC_DITH4 = crate::Reg<u32, _CC_DITH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_DITH4;
#[doc = "`read()` method returns [cc_dith4::R](cc_dith4::R) reader structure"]
impl crate::Readable for CC_DITH4 {}
#[doc = "`write(|w| ..)` method takes [cc_dith4::W](cc_dith4::W) writer structure"]
impl crate::Writable for CC_DITH4 {}
#[doc = "Compare and Capture"]
pub mod cc_dith4;
#[doc = "Compare and Capture\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_dith5](cc_dith5) module"]
pub type CC_DITH5 = crate::Reg<u32, _CC_DITH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_DITH5;
#[doc = "`read()` method returns [cc_dith5::R](cc_dith5::R) reader structure"]
impl crate::Readable for CC_DITH5 {}
#[doc = "`write(|w| ..)` method takes [cc_dith5::W](cc_dith5::W) writer structure"]
impl crate::Writable for CC_DITH5 {}
#[doc = "Compare and Capture"]
pub mod cc_dith5;
#[doc = "Compare and Capture\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_dith6](cc_dith6) module"]
pub type CC_DITH6 = crate::Reg<u32, _CC_DITH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_DITH6;
#[doc = "`read()` method returns [cc_dith6::R](cc_dith6::R) reader structure"]
impl crate::Readable for CC_DITH6 {}
#[doc = "`write(|w| ..)` method takes [cc_dith6::W](cc_dith6::W) writer structure"]
impl crate::Writable for CC_DITH6 {}
#[doc = "Compare and Capture"]
pub mod cc_dith6;
#[doc = "Pattern Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pattb](pattb) module"]
pub type PATTB = crate::Reg<u16, _PATTB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATTB;
#[doc = "`read()` method returns [pattb::R](pattb::R) reader structure"]
impl crate::Readable for PATTB {}
#[doc = "`write(|w| ..)` method takes [pattb::W](pattb::W) writer structure"]
impl crate::Writable for PATTB {}
#[doc = "Pattern Buffer"]
pub mod pattb;
#[doc = "Waveform Control Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waveb](waveb) module"]
pub type WAVEB = crate::Reg<u32, _WAVEB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAVEB;
#[doc = "`read()` method returns [waveb::R](waveb::R) reader structure"]
impl crate::Readable for WAVEB {}
#[doc = "`write(|w| ..)` method takes [waveb::W](waveb::W) writer structure"]
impl crate::Writable for WAVEB {}
#[doc = "Waveform Control Buffer"]
pub mod waveb;
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perb](perb) module"]
pub type PERB = crate::Reg<u32, _PERB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERB;
#[doc = "`read()` method returns [perb::R](perb::R) reader structure"]
impl crate::Readable for PERB {}
#[doc = "`write(|w| ..)` method takes [perb::W](perb::W) writer structure"]
impl crate::Writable for PERB {}
#[doc = "Period Buffer"]
pub mod perb;
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perb_dith4](perb_dith4) module"]
pub type PERB_DITH4 = crate::Reg<u32, _PERB_DITH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERB_DITH4;
#[doc = "`read()` method returns [perb_dith4::R](perb_dith4::R) reader structure"]
impl crate::Readable for PERB_DITH4 {}
#[doc = "`write(|w| ..)` method takes [perb_dith4::W](perb_dith4::W) writer structure"]
impl crate::Writable for PERB_DITH4 {}
#[doc = "Period Buffer"]
pub mod perb_dith4;
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perb_dith5](perb_dith5) module"]
pub type PERB_DITH5 = crate::Reg<u32, _PERB_DITH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERB_DITH5;
#[doc = "`read()` method returns [perb_dith5::R](perb_dith5::R) reader structure"]
impl crate::Readable for PERB_DITH5 {}
#[doc = "`write(|w| ..)` method takes [perb_dith5::W](perb_dith5::W) writer structure"]
impl crate::Writable for PERB_DITH5 {}
#[doc = "Period Buffer"]
pub mod perb_dith5;
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perb_dith6](perb_dith6) module"]
pub type PERB_DITH6 = crate::Reg<u32, _PERB_DITH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERB_DITH6;
#[doc = "`read()` method returns [perb_dith6::R](perb_dith6::R) reader structure"]
impl crate::Readable for PERB_DITH6 {}
#[doc = "`write(|w| ..)` method takes [perb_dith6::W](perb_dith6::W) writer structure"]
impl crate::Writable for PERB_DITH6 {}
#[doc = "Period Buffer"]
pub mod perb_dith6;
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccb](ccb) module"]
pub type CCB = crate::Reg<u32, _CCB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCB;
#[doc = "`read()` method returns [ccb::R](ccb::R) reader structure"]
impl crate::Readable for CCB {}
#[doc = "`write(|w| ..)` method takes [ccb::W](ccb::W) writer structure"]
impl crate::Writable for CCB {}
#[doc = "Compare and Capture Buffer"]
pub mod ccb;
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccb_dith4](ccb_dith4) module"]
pub type CCB_DITH4 = crate::Reg<u32, _CCB_DITH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCB_DITH4;
#[doc = "`read()` method returns [ccb_dith4::R](ccb_dith4::R) reader structure"]
impl crate::Readable for CCB_DITH4 {}
#[doc = "`write(|w| ..)` method takes [ccb_dith4::W](ccb_dith4::W) writer structure"]
impl crate::Writable for CCB_DITH4 {}
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith4;
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccb_dith5](ccb_dith5) module"]
pub type CCB_DITH5 = crate::Reg<u32, _CCB_DITH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCB_DITH5;
#[doc = "`read()` method returns [ccb_dith5::R](ccb_dith5::R) reader structure"]
impl crate::Readable for CCB_DITH5 {}
#[doc = "`write(|w| ..)` method takes [ccb_dith5::W](ccb_dith5::W) writer structure"]
impl crate::Writable for CCB_DITH5 {}
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith5;
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccb_dith6](ccb_dith6) module"]
pub type CCB_DITH6 = crate::Reg<u32, _CCB_DITH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCB_DITH6;
#[doc = "`read()` method returns [ccb_dith6::R](ccb_dith6::R) reader structure"]
impl crate::Readable for CCB_DITH6 {}
#[doc = "`write(|w| ..)` method takes [ccb_dith6::W](ccb_dith6::W) writer structure"]
impl crate::Writable for CCB_DITH6 {}
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith6;
