#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x01 - Non-Maskable Interrupt Control"]
    pub nmictrl: NMICTRL,
    #[doc = "0x02 - Non-Maskable Interrupt Flag Status and Clear"]
    pub nmiflag: NMIFLAG,
    #[doc = "0x04 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x08 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x18 - External Interrupt Asynchronous Mode"]
    pub asynch: ASYNCH,
    #[doc = "0x1c - External Interrupt Sense Configuration"]
    pub config: [CONFIG; 2],
    _reserved10: [u8; 12usize],
    #[doc = "0x30 - Debouncer Enable"]
    pub debouncen: DEBOUNCEN,
    #[doc = "0x34 - Debouncer Prescaler"]
    pub dprescaler: DPRESCALER,
    #[doc = "0x38 - Pin State"]
    pub pinstate: PINSTATE,
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
#[doc = "Non-Maskable Interrupt Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmictrl](nmictrl) module"]
pub type NMICTRL = crate::Reg<u8, _NMICTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMICTRL;
#[doc = "`read()` method returns [nmictrl::R](nmictrl::R) reader structure"]
impl crate::Readable for NMICTRL {}
#[doc = "`write(|w| ..)` method takes [nmictrl::W](nmictrl::W) writer structure"]
impl crate::Writable for NMICTRL {}
#[doc = "Non-Maskable Interrupt Control"]
pub mod nmictrl;
#[doc = "Non-Maskable Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmiflag](nmiflag) module"]
pub type NMIFLAG = crate::Reg<u16, _NMIFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NMIFLAG;
#[doc = "`read()` method returns [nmiflag::R](nmiflag::R) reader structure"]
impl crate::Readable for NMIFLAG {}
#[doc = "`write(|w| ..)` method takes [nmiflag::W](nmiflag::W) writer structure"]
impl crate::Writable for NMIFLAG {}
#[doc = "Non-Maskable Interrupt Flag Status and Clear"]
pub mod nmiflag;
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
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
#[doc = "External Interrupt Asynchronous Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asynch](asynch) module"]
pub type ASYNCH = crate::Reg<u32, _ASYNCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCH;
#[doc = "`read()` method returns [asynch::R](asynch::R) reader structure"]
impl crate::Readable for ASYNCH {}
#[doc = "`write(|w| ..)` method takes [asynch::W](asynch::W) writer structure"]
impl crate::Writable for ASYNCH {}
#[doc = "External Interrupt Asynchronous Mode"]
pub mod asynch;
#[doc = "External Interrupt Sense Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "External Interrupt Sense Configuration"]
pub mod config;
#[doc = "Debouncer Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debouncen](debouncen) module"]
pub type DEBOUNCEN = crate::Reg<u32, _DEBOUNCEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBOUNCEN;
#[doc = "`read()` method returns [debouncen::R](debouncen::R) reader structure"]
impl crate::Readable for DEBOUNCEN {}
#[doc = "`write(|w| ..)` method takes [debouncen::W](debouncen::W) writer structure"]
impl crate::Writable for DEBOUNCEN {}
#[doc = "Debouncer Enable"]
pub mod debouncen;
#[doc = "Debouncer Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dprescaler](dprescaler) module"]
pub type DPRESCALER = crate::Reg<u32, _DPRESCALER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPRESCALER;
#[doc = "`read()` method returns [dprescaler::R](dprescaler::R) reader structure"]
impl crate::Readable for DPRESCALER {}
#[doc = "`write(|w| ..)` method takes [dprescaler::W](dprescaler::W) writer structure"]
impl crate::Writable for DPRESCALER {}
#[doc = "Debouncer Prescaler"]
pub mod dprescaler;
#[doc = "Pin State\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinstate](pinstate) module"]
pub type PINSTATE = crate::Reg<u32, _PINSTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSTATE;
#[doc = "`read()` method returns [pinstate::R](pinstate::R) reader structure"]
impl crate::Readable for PINSTATE {}
#[doc = "Pin State"]
pub mod pinstate;
