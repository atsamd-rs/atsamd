#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x02 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved3: [u8; 1usize],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x07 - Status"]
    pub status: STATUS,
    #[doc = "0x08 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - DAC n Control"]
    pub dacctrl: [DACCTRL; 2],
    #[doc = "0x10 - DAC n Data"]
    pub data: [DATA; 2],
    #[doc = "0x14 - DAC n Data Buffer"]
    pub databuf: [DATABUF; 2],
    #[doc = "0x18 - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved12: [u8; 3usize],
    #[doc = "0x1c - Filter Result"]
    pub result: [RESULT; 2],
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u8, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control A"]
pub mod ctrla;
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u8, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "Control B"]
pub mod ctrlb;
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
#[doc = "DAC n Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacctrl](dacctrl) module"]
pub type DACCTRL = crate::Reg<u16, _DACCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACCTRL;
#[doc = "`read()` method returns [dacctrl::R](dacctrl::R) reader structure"]
impl crate::Readable for DACCTRL {}
#[doc = "`write(|w| ..)` method takes [dacctrl::W](dacctrl::W) writer structure"]
impl crate::Writable for DACCTRL {}
#[doc = "DAC n Control"]
pub mod dacctrl;
#[doc = "DAC n Data\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u16, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "DAC n Data"]
pub mod data;
#[doc = "DAC n Data Buffer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databuf](databuf) module"]
pub type DATABUF = crate::Reg<u16, _DATABUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATABUF;
#[doc = "`write(|w| ..)` method takes [databuf::W](databuf::W) writer structure"]
impl crate::Writable for DATABUF {}
#[doc = "DAC n Data Buffer"]
pub mod databuf;
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
#[doc = "Filter Result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](result) module"]
pub type RESULT = crate::Reg<u16, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Filter Result"]
pub mod result;
