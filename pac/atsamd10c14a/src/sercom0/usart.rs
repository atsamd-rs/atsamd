#[doc = "USART Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u32, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "USART Control A"]
pub mod ctrla;
#[doc = "USART Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](ctrlb) module"]
pub type CTRLB = crate::Reg<u32, _CTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB;
#[doc = "`read()` method returns [ctrlb::R](ctrlb::R) reader structure"]
impl crate::Readable for CTRLB {}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](ctrlb::W) writer structure"]
impl crate::Writable for CTRLB {}
#[doc = "USART Control B"]
pub mod ctrlb;
#[doc = "USART Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](baud) module"]
pub type BAUD = crate::Reg<u16, _BAUD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD;
#[doc = "`read()` method returns [baud::R](baud::R) reader structure"]
impl crate::Readable for BAUD {}
#[doc = "`write(|w| ..)` method takes [baud::W](baud::W) writer structure"]
impl crate::Writable for BAUD {}
#[doc = "USART Baud Rate"]
pub mod baud;
#[doc = "USART Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud_frac_mode](baud_frac_mode) module"]
pub type BAUD_FRAC_MODE = crate::Reg<u16, _BAUD_FRAC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD_FRAC_MODE;
#[doc = "`read()` method returns [baud_frac_mode::R](baud_frac_mode::R) reader structure"]
impl crate::Readable for BAUD_FRAC_MODE {}
#[doc = "`write(|w| ..)` method takes [baud_frac_mode::W](baud_frac_mode::W) writer structure"]
impl crate::Writable for BAUD_FRAC_MODE {}
#[doc = "USART Baud Rate"]
pub mod baud_frac_mode;
#[doc = "USART Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud_fracfp_mode](baud_fracfp_mode) module"]
pub type BAUD_FRACFP_MODE = crate::Reg<u16, _BAUD_FRACFP_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD_FRACFP_MODE;
#[doc = "`read()` method returns [baud_fracfp_mode::R](baud_fracfp_mode::R) reader structure"]
impl crate::Readable for BAUD_FRACFP_MODE {}
#[doc = "`write(|w| ..)` method takes [baud_fracfp_mode::W](baud_fracfp_mode::W) writer structure"]
impl crate::Writable for BAUD_FRACFP_MODE {}
#[doc = "USART Baud Rate"]
pub mod baud_fracfp_mode;
#[doc = "USART Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud_usartfp_mode](baud_usartfp_mode) module"]
pub type BAUD_USARTFP_MODE = crate::Reg<u16, _BAUD_USARTFP_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUD_USARTFP_MODE;
#[doc = "`read()` method returns [baud_usartfp_mode::R](baud_usartfp_mode::R) reader structure"]
impl crate::Readable for BAUD_USARTFP_MODE {}
#[doc = "`write(|w| ..)` method takes [baud_usartfp_mode::W](baud_usartfp_mode::W) writer structure"]
impl crate::Writable for BAUD_USARTFP_MODE {}
#[doc = "USART Baud Rate"]
pub mod baud_usartfp_mode;
#[doc = "USART Receive Pulse Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxpl](rxpl) module"]
pub type RXPL = crate::Reg<u8, _RXPL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXPL;
#[doc = "`read()` method returns [rxpl::R](rxpl::R) reader structure"]
impl crate::Readable for RXPL {}
#[doc = "`write(|w| ..)` method takes [rxpl::W](rxpl::W) writer structure"]
impl crate::Writable for RXPL {}
#[doc = "USART Receive Pulse Length"]
pub mod rxpl;
#[doc = "USART Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u8, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "USART Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "USART Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u8, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "USART Interrupt Enable Set"]
pub mod intenset;
#[doc = "USART Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](intflag) module"]
pub type INTFLAG = crate::Reg<u8, _INTFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTFLAG;
#[doc = "`read()` method returns [intflag::R](intflag::R) reader structure"]
impl crate::Readable for INTFLAG {}
#[doc = "`write(|w| ..)` method takes [intflag::W](intflag::W) writer structure"]
impl crate::Writable for INTFLAG {}
#[doc = "USART Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "USART Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u16, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "USART Status"]
pub mod status;
#[doc = "USART Syncbusy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "USART Syncbusy"]
pub mod syncbusy;
#[doc = "USART Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u16, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "USART Data"]
pub mod data;
#[doc = "USART Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgctrl](dbgctrl) module"]
pub type DBGCTRL = crate::Reg<u8, _DBGCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGCTRL;
#[doc = "`read()` method returns [dbgctrl::R](dbgctrl::R) reader structure"]
impl crate::Readable for DBGCTRL {}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](dbgctrl::W) writer structure"]
impl crate::Writable for DBGCTRL {}
#[doc = "USART Debug Control"]
pub mod dbgctrl;
