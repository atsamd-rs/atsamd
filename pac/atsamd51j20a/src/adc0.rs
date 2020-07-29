#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x02 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x03 - Debug Control"]
    pub dbgctrl: DBGCTRL,
    #[doc = "0x04 - Input Control"]
    pub inputctrl: INPUTCTRL,
    #[doc = "0x06 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - Reference Control"]
    pub refctrl: REFCTRL,
    _reserved6: [u8; 1usize],
    #[doc = "0x0a - Average Control"]
    pub avgctrl: AVGCTRL,
    #[doc = "0x0b - Sample Time Control"]
    pub sampctrl: SAMPCTRL,
    #[doc = "0x0c - Window Monitor Lower Threshold"]
    pub winlt: WINLT,
    #[doc = "0x0e - Window Monitor Upper Threshold"]
    pub winut: WINUT,
    #[doc = "0x10 - Gain Correction"]
    pub gaincorr: GAINCORR,
    #[doc = "0x12 - Offset Correction"]
    pub offsetcorr: OFFSETCORR,
    #[doc = "0x14 - Software Trigger"]
    pub swtrig: SWTRIG,
    _reserved13: [u8; 23usize],
    #[doc = "0x2c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x2d - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x2e - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x2f - Status"]
    pub status: STATUS,
    #[doc = "0x30 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x34 - DMA Sequencial Data"]
    pub dseqdata: DSEQDATA,
    #[doc = "0x38 - DMA Sequential Control"]
    pub dseqctrl: DSEQCTRL,
    #[doc = "0x3c - DMA Sequencial Status"]
    pub dseqstat: DSEQSTAT,
    #[doc = "0x40 - Result Conversion Value"]
    pub result: RESULT,
    _reserved22: [u8; 2usize],
    #[doc = "0x44 - Last Sample Result"]
    pub ress: RESS,
    _reserved23: [u8; 2usize],
    #[doc = "0x48 - Calibration"]
    pub calib: CALIB,
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u16, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
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
#[doc = "Input Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputctrl](inputctrl) module"]
pub type INPUTCTRL = crate::Reg<u16, _INPUTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUTCTRL;
#[doc = "`read()` method returns [inputctrl::R](inputctrl::R) reader structure"]
impl crate::Readable for INPUTCTRL {}
#[doc = "`write(|w| ..)` method takes [inputctrl::W](inputctrl::W) writer structure"]
impl crate::Writable for INPUTCTRL {}
#[doc = "Input Control"]
pub mod inputctrl;
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u16, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "Reference Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refctrl](refctrl) module"]
pub type REFCTRL = crate::Reg<u8, _REFCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFCTRL;
#[doc = "`read()` method returns [refctrl::R](refctrl::R) reader structure"]
impl crate::Readable for REFCTRL {}
#[doc = "`write(|w| ..)` method takes [refctrl::W](refctrl::W) writer structure"]
impl crate::Writable for REFCTRL {}
#[doc = "Reference Control"]
pub mod refctrl;
#[doc = "Average Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [avgctrl](avgctrl) module"]
pub type AVGCTRL = crate::Reg<u8, _AVGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AVGCTRL;
#[doc = "`read()` method returns [avgctrl::R](avgctrl::R) reader structure"]
impl crate::Readable for AVGCTRL {}
#[doc = "`write(|w| ..)` method takes [avgctrl::W](avgctrl::W) writer structure"]
impl crate::Writable for AVGCTRL {}
#[doc = "Average Control"]
pub mod avgctrl;
#[doc = "Sample Time Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampctrl](sampctrl) module"]
pub type SAMPCTRL = crate::Reg<u8, _SAMPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMPCTRL;
#[doc = "`read()` method returns [sampctrl::R](sampctrl::R) reader structure"]
impl crate::Readable for SAMPCTRL {}
#[doc = "`write(|w| ..)` method takes [sampctrl::W](sampctrl::W) writer structure"]
impl crate::Writable for SAMPCTRL {}
#[doc = "Sample Time Control"]
pub mod sampctrl;
#[doc = "Window Monitor Lower Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winlt](winlt) module"]
pub type WINLT = crate::Reg<u16, _WINLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINLT;
#[doc = "`read()` method returns [winlt::R](winlt::R) reader structure"]
impl crate::Readable for WINLT {}
#[doc = "`write(|w| ..)` method takes [winlt::W](winlt::W) writer structure"]
impl crate::Writable for WINLT {}
#[doc = "Window Monitor Lower Threshold"]
pub mod winlt;
#[doc = "Window Monitor Upper Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winut](winut) module"]
pub type WINUT = crate::Reg<u16, _WINUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINUT;
#[doc = "`read()` method returns [winut::R](winut::R) reader structure"]
impl crate::Readable for WINUT {}
#[doc = "`write(|w| ..)` method takes [winut::W](winut::W) writer structure"]
impl crate::Writable for WINUT {}
#[doc = "Window Monitor Upper Threshold"]
pub mod winut;
#[doc = "Gain Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gaincorr](gaincorr) module"]
pub type GAINCORR = crate::Reg<u16, _GAINCORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAINCORR;
#[doc = "`read()` method returns [gaincorr::R](gaincorr::R) reader structure"]
impl crate::Readable for GAINCORR {}
#[doc = "`write(|w| ..)` method takes [gaincorr::W](gaincorr::W) writer structure"]
impl crate::Writable for GAINCORR {}
#[doc = "Gain Correction"]
pub mod gaincorr;
#[doc = "Offset Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offsetcorr](offsetcorr) module"]
pub type OFFSETCORR = crate::Reg<u16, _OFFSETCORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFFSETCORR;
#[doc = "`read()` method returns [offsetcorr::R](offsetcorr::R) reader structure"]
impl crate::Readable for OFFSETCORR {}
#[doc = "`write(|w| ..)` method takes [offsetcorr::W](offsetcorr::W) writer structure"]
impl crate::Writable for OFFSETCORR {}
#[doc = "Offset Correction"]
pub mod offsetcorr;
#[doc = "Software Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrig](swtrig) module"]
pub type SWTRIG = crate::Reg<u8, _SWTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWTRIG;
#[doc = "`read()` method returns [swtrig::R](swtrig::R) reader structure"]
impl crate::Readable for SWTRIG {}
#[doc = "`write(|w| ..)` method takes [swtrig::W](swtrig::W) writer structure"]
impl crate::Writable for SWTRIG {}
#[doc = "Software Trigger"]
pub mod swtrig;
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
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u8, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "DMA Sequencial Data\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dseqdata](dseqdata) module"]
pub type DSEQDATA = crate::Reg<u32, _DSEQDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSEQDATA;
#[doc = "`write(|w| ..)` method takes [dseqdata::W](dseqdata::W) writer structure"]
impl crate::Writable for DSEQDATA {}
#[doc = "DMA Sequencial Data"]
pub mod dseqdata;
#[doc = "DMA Sequential Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dseqctrl](dseqctrl) module"]
pub type DSEQCTRL = crate::Reg<u32, _DSEQCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSEQCTRL;
#[doc = "`read()` method returns [dseqctrl::R](dseqctrl::R) reader structure"]
impl crate::Readable for DSEQCTRL {}
#[doc = "`write(|w| ..)` method takes [dseqctrl::W](dseqctrl::W) writer structure"]
impl crate::Writable for DSEQCTRL {}
#[doc = "DMA Sequential Control"]
pub mod dseqctrl;
#[doc = "DMA Sequencial Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dseqstat](dseqstat) module"]
pub type DSEQSTAT = crate::Reg<u32, _DSEQSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSEQSTAT;
#[doc = "`read()` method returns [dseqstat::R](dseqstat::R) reader structure"]
impl crate::Readable for DSEQSTAT {}
#[doc = "DMA Sequencial Status"]
pub mod dseqstat;
#[doc = "Result Conversion Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](result) module"]
pub type RESULT = crate::Reg<u16, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Result Conversion Value"]
pub mod result;
#[doc = "Last Sample Result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ress](ress) module"]
pub type RESS = crate::Reg<u16, _RESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESS;
#[doc = "`read()` method returns [ress::R](ress::R) reader structure"]
impl crate::Readable for RESS {}
#[doc = "Last Sample Result"]
pub mod ress;
#[doc = "Calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](calib) module"]
pub type CALIB = crate::Reg<u16, _CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALIB;
#[doc = "`read()` method returns [calib::R](calib::R) reader structure"]
impl crate::Readable for CALIB {}
#[doc = "`write(|w| ..)` method takes [calib::W](calib::W) writer structure"]
impl crate::Writable for CALIB {}
#[doc = "Calibration"]
pub mod calib;
