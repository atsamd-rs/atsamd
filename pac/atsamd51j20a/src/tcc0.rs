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
    _reserved_18_cc: [u8; 24usize],
    _reserved19: [u8; 8usize],
    #[doc = "0x64 - Pattern Buffer"]
    pub pattbuf: PATTBUF,
    _reserved20: [u8; 6usize],
    _reserved_20_perbuf: [u8; 4usize],
    _reserved_21_ccbuf: [u8; 24usize],
}
impl RegisterBlock {
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith6_mode(&self) -> &COUNT_DITH6_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH6_MODE) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith6_mode_mut(&self) -> &mut COUNT_DITH6_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT_DITH6_MODE) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith5_mode(&self) -> &COUNT_DITH5_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH5_MODE) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith5_mode_mut(&self) -> &mut COUNT_DITH5_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT_DITH5_MODE) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith4_mode(&self) -> &COUNT_DITH4_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH4_MODE) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith4_mode_mut(&self) -> &mut COUNT_DITH4_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut COUNT_DITH4_MODE) }
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
    pub fn per_dith6_mode(&self) -> &PER_DITH6_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH6_MODE) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith6_mode_mut(&self) -> &mut PER_DITH6_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER_DITH6_MODE) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith5_mode(&self) -> &PER_DITH5_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH5_MODE) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith5_mode_mut(&self) -> &mut PER_DITH5_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER_DITH5_MODE) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith4_mode(&self) -> &PER_DITH4_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH4_MODE) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith4_mode_mut(&self) -> &mut PER_DITH4_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut PER_DITH4_MODE) }
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
    pub fn cc_dith6_mode(&self) -> &[CC_DITH6_MODE; 6] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH6_MODE; 6])
        }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith6_mode_mut(&self) -> &mut [CC_DITH6_MODE; 6] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC_DITH6_MODE; 6])
        }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith5_mode(&self) -> &[CC_DITH5_MODE; 6] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH5_MODE; 6])
        }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith5_mode_mut(&self) -> &mut [CC_DITH5_MODE; 6] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC_DITH5_MODE; 6])
        }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith4_mode(&self) -> &[CC_DITH4_MODE; 6] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH4_MODE; 6])
        }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith4_mode_mut(&self) -> &mut [CC_DITH4_MODE; 6] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC_DITH4_MODE; 6])
        }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc(&self) -> &[CC; 6] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC; 6]) }
    }
    #[doc = "0x44 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_mut(&self) -> &mut [CC; 6] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut [CC; 6]) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perbuf_dith6_mode(&self) -> &PERBUF_DITH6_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize) as *const PERBUF_DITH6_MODE)
        }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perbuf_dith6_mode_mut(&self) -> &mut PERBUF_DITH6_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERBUF_DITH6_MODE)
        }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perbuf_dith5_mode(&self) -> &PERBUF_DITH5_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize) as *const PERBUF_DITH5_MODE)
        }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perbuf_dith5_mode_mut(&self) -> &mut PERBUF_DITH5_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERBUF_DITH5_MODE)
        }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perbuf_dith4_mode(&self) -> &PERBUF_DITH4_MODE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(108usize) as *const PERBUF_DITH4_MODE)
        }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perbuf_dith4_mode_mut(&self) -> &mut PERBUF_DITH4_MODE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERBUF_DITH4_MODE)
        }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perbuf(&self) -> &PERBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERBUF) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perbuf_mut(&self) -> &mut PERBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(108usize) as *mut PERBUF) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_dith6_mode(&self) -> &[CCBUF_DITH6_MODE; 6] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize) as *const [CCBUF_DITH6_MODE; 6])
        }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_dith6_mode_mut(&self) -> &mut [CCBUF_DITH6_MODE; 6] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCBUF_DITH6_MODE; 6])
        }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_dith5_mode(&self) -> &[CCBUF_DITH5_MODE; 6] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize) as *const [CCBUF_DITH5_MODE; 6])
        }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_dith5_mode_mut(&self) -> &mut [CCBUF_DITH5_MODE; 6] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCBUF_DITH5_MODE; 6])
        }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_dith4_mode(&self) -> &[CCBUF_DITH4_MODE; 6] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(112usize) as *const [CCBUF_DITH4_MODE; 6])
        }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_dith4_mode_mut(&self) -> &mut [CCBUF_DITH4_MODE; 6] {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCBUF_DITH4_MODE; 6])
        }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf(&self) -> &[CCBUF; 6] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCBUF; 6]) }
    }
    #[doc = "0x70 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccbuf_mut(&self) -> &mut [CCBUF; 6] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(112usize) as *mut [CCBUF; 6]) }
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
#[doc = "Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_dith4_mode](count_dith4_mode) module"]
pub type COUNT_DITH4_MODE = crate::Reg<u32, _COUNT_DITH4_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT_DITH4_MODE;
#[doc = "`read()` method returns [count_dith4_mode::R](count_dith4_mode::R) reader structure"]
impl crate::Readable for COUNT_DITH4_MODE {}
#[doc = "`write(|w| ..)` method takes [count_dith4_mode::W](count_dith4_mode::W) writer structure"]
impl crate::Writable for COUNT_DITH4_MODE {}
#[doc = "Count"]
pub mod count_dith4_mode;
#[doc = "Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_dith5_mode](count_dith5_mode) module"]
pub type COUNT_DITH5_MODE = crate::Reg<u32, _COUNT_DITH5_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT_DITH5_MODE;
#[doc = "`read()` method returns [count_dith5_mode::R](count_dith5_mode::R) reader structure"]
impl crate::Readable for COUNT_DITH5_MODE {}
#[doc = "`write(|w| ..)` method takes [count_dith5_mode::W](count_dith5_mode::W) writer structure"]
impl crate::Writable for COUNT_DITH5_MODE {}
#[doc = "Count"]
pub mod count_dith5_mode;
#[doc = "Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_dith6_mode](count_dith6_mode) module"]
pub type COUNT_DITH6_MODE = crate::Reg<u32, _COUNT_DITH6_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT_DITH6_MODE;
#[doc = "`read()` method returns [count_dith6_mode::R](count_dith6_mode::R) reader structure"]
impl crate::Readable for COUNT_DITH6_MODE {}
#[doc = "`write(|w| ..)` method takes [count_dith6_mode::W](count_dith6_mode::W) writer structure"]
impl crate::Writable for COUNT_DITH6_MODE {}
#[doc = "Count"]
pub mod count_dith6_mode;
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
#[doc = "Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_dith4_mode](per_dith4_mode) module"]
pub type PER_DITH4_MODE = crate::Reg<u32, _PER_DITH4_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_DITH4_MODE;
#[doc = "`read()` method returns [per_dith4_mode::R](per_dith4_mode::R) reader structure"]
impl crate::Readable for PER_DITH4_MODE {}
#[doc = "`write(|w| ..)` method takes [per_dith4_mode::W](per_dith4_mode::W) writer structure"]
impl crate::Writable for PER_DITH4_MODE {}
#[doc = "Period"]
pub mod per_dith4_mode;
#[doc = "Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_dith5_mode](per_dith5_mode) module"]
pub type PER_DITH5_MODE = crate::Reg<u32, _PER_DITH5_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_DITH5_MODE;
#[doc = "`read()` method returns [per_dith5_mode::R](per_dith5_mode::R) reader structure"]
impl crate::Readable for PER_DITH5_MODE {}
#[doc = "`write(|w| ..)` method takes [per_dith5_mode::W](per_dith5_mode::W) writer structure"]
impl crate::Writable for PER_DITH5_MODE {}
#[doc = "Period"]
pub mod per_dith5_mode;
#[doc = "Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_dith6_mode](per_dith6_mode) module"]
pub type PER_DITH6_MODE = crate::Reg<u32, _PER_DITH6_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER_DITH6_MODE;
#[doc = "`read()` method returns [per_dith6_mode::R](per_dith6_mode::R) reader structure"]
impl crate::Readable for PER_DITH6_MODE {}
#[doc = "`write(|w| ..)` method takes [per_dith6_mode::W](per_dith6_mode::W) writer structure"]
impl crate::Writable for PER_DITH6_MODE {}
#[doc = "Period"]
pub mod per_dith6_mode;
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
#[doc = "Compare and Capture\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_dith4_mode](cc_dith4_mode) module"]
pub type CC_DITH4_MODE = crate::Reg<u32, _CC_DITH4_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_DITH4_MODE;
#[doc = "`read()` method returns [cc_dith4_mode::R](cc_dith4_mode::R) reader structure"]
impl crate::Readable for CC_DITH4_MODE {}
#[doc = "`write(|w| ..)` method takes [cc_dith4_mode::W](cc_dith4_mode::W) writer structure"]
impl crate::Writable for CC_DITH4_MODE {}
#[doc = "Compare and Capture"]
pub mod cc_dith4_mode;
#[doc = "Compare and Capture\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_dith5_mode](cc_dith5_mode) module"]
pub type CC_DITH5_MODE = crate::Reg<u32, _CC_DITH5_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_DITH5_MODE;
#[doc = "`read()` method returns [cc_dith5_mode::R](cc_dith5_mode::R) reader structure"]
impl crate::Readable for CC_DITH5_MODE {}
#[doc = "`write(|w| ..)` method takes [cc_dith5_mode::W](cc_dith5_mode::W) writer structure"]
impl crate::Writable for CC_DITH5_MODE {}
#[doc = "Compare and Capture"]
pub mod cc_dith5_mode;
#[doc = "Compare and Capture\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc_dith6_mode](cc_dith6_mode) module"]
pub type CC_DITH6_MODE = crate::Reg<u32, _CC_DITH6_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC_DITH6_MODE;
#[doc = "`read()` method returns [cc_dith6_mode::R](cc_dith6_mode::R) reader structure"]
impl crate::Readable for CC_DITH6_MODE {}
#[doc = "`write(|w| ..)` method takes [cc_dith6_mode::W](cc_dith6_mode::W) writer structure"]
impl crate::Writable for CC_DITH6_MODE {}
#[doc = "Compare and Capture"]
pub mod cc_dith6_mode;
#[doc = "Pattern Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pattbuf](pattbuf) module"]
pub type PATTBUF = crate::Reg<u16, _PATTBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATTBUF;
#[doc = "`read()` method returns [pattbuf::R](pattbuf::R) reader structure"]
impl crate::Readable for PATTBUF {}
#[doc = "`write(|w| ..)` method takes [pattbuf::W](pattbuf::W) writer structure"]
impl crate::Writable for PATTBUF {}
#[doc = "Pattern Buffer"]
pub mod pattbuf;
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbuf](perbuf) module"]
pub type PERBUF = crate::Reg<u32, _PERBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERBUF;
#[doc = "`read()` method returns [perbuf::R](perbuf::R) reader structure"]
impl crate::Readable for PERBUF {}
#[doc = "`write(|w| ..)` method takes [perbuf::W](perbuf::W) writer structure"]
impl crate::Writable for PERBUF {}
#[doc = "Period Buffer"]
pub mod perbuf;
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbuf_dith4_mode](perbuf_dith4_mode) module"]
pub type PERBUF_DITH4_MODE = crate::Reg<u32, _PERBUF_DITH4_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERBUF_DITH4_MODE;
#[doc = "`read()` method returns [perbuf_dith4_mode::R](perbuf_dith4_mode::R) reader structure"]
impl crate::Readable for PERBUF_DITH4_MODE {}
#[doc = "`write(|w| ..)` method takes [perbuf_dith4_mode::W](perbuf_dith4_mode::W) writer structure"]
impl crate::Writable for PERBUF_DITH4_MODE {}
#[doc = "Period Buffer"]
pub mod perbuf_dith4_mode;
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbuf_dith5_mode](perbuf_dith5_mode) module"]
pub type PERBUF_DITH5_MODE = crate::Reg<u32, _PERBUF_DITH5_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERBUF_DITH5_MODE;
#[doc = "`read()` method returns [perbuf_dith5_mode::R](perbuf_dith5_mode::R) reader structure"]
impl crate::Readable for PERBUF_DITH5_MODE {}
#[doc = "`write(|w| ..)` method takes [perbuf_dith5_mode::W](perbuf_dith5_mode::W) writer structure"]
impl crate::Writable for PERBUF_DITH5_MODE {}
#[doc = "Period Buffer"]
pub mod perbuf_dith5_mode;
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbuf_dith6_mode](perbuf_dith6_mode) module"]
pub type PERBUF_DITH6_MODE = crate::Reg<u32, _PERBUF_DITH6_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERBUF_DITH6_MODE;
#[doc = "`read()` method returns [perbuf_dith6_mode::R](perbuf_dith6_mode::R) reader structure"]
impl crate::Readable for PERBUF_DITH6_MODE {}
#[doc = "`write(|w| ..)` method takes [perbuf_dith6_mode::W](perbuf_dith6_mode::W) writer structure"]
impl crate::Writable for PERBUF_DITH6_MODE {}
#[doc = "Period Buffer"]
pub mod perbuf_dith6_mode;
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccbuf](ccbuf) module"]
pub type CCBUF = crate::Reg<u32, _CCBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCBUF;
#[doc = "`read()` method returns [ccbuf::R](ccbuf::R) reader structure"]
impl crate::Readable for CCBUF {}
#[doc = "`write(|w| ..)` method takes [ccbuf::W](ccbuf::W) writer structure"]
impl crate::Writable for CCBUF {}
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf;
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccbuf_dith4_mode](ccbuf_dith4_mode) module"]
pub type CCBUF_DITH4_MODE = crate::Reg<u32, _CCBUF_DITH4_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCBUF_DITH4_MODE;
#[doc = "`read()` method returns [ccbuf_dith4_mode::R](ccbuf_dith4_mode::R) reader structure"]
impl crate::Readable for CCBUF_DITH4_MODE {}
#[doc = "`write(|w| ..)` method takes [ccbuf_dith4_mode::W](ccbuf_dith4_mode::W) writer structure"]
impl crate::Writable for CCBUF_DITH4_MODE {}
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf_dith4_mode;
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccbuf_dith5_mode](ccbuf_dith5_mode) module"]
pub type CCBUF_DITH5_MODE = crate::Reg<u32, _CCBUF_DITH5_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCBUF_DITH5_MODE;
#[doc = "`read()` method returns [ccbuf_dith5_mode::R](ccbuf_dith5_mode::R) reader structure"]
impl crate::Readable for CCBUF_DITH5_MODE {}
#[doc = "`write(|w| ..)` method takes [ccbuf_dith5_mode::W](ccbuf_dith5_mode::W) writer structure"]
impl crate::Writable for CCBUF_DITH5_MODE {}
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf_dith5_mode;
#[doc = "Compare and Capture Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccbuf_dith6_mode](ccbuf_dith6_mode) module"]
pub type CCBUF_DITH6_MODE = crate::Reg<u32, _CCBUF_DITH6_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCBUF_DITH6_MODE;
#[doc = "`read()` method returns [ccbuf_dith6_mode::R](ccbuf_dith6_mode::R) reader structure"]
impl crate::Readable for CCBUF_DITH6_MODE {}
#[doc = "`write(|w| ..)` method takes [ccbuf_dith6_mode::W](ccbuf_dith6_mode::W) writer structure"]
impl crate::Writable for CCBUF_DITH6_MODE {}
#[doc = "Compare and Capture Buffer"]
pub mod ccbuf_dith6_mode;
