#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Software Event"]
    pub swevt: SWEVT,
    #[doc = "0x08 - Priority Control"]
    pub prictrl: PRICTRL,
    _reserved3: [u8; 7usize],
    #[doc = "0x10 - Channel Pending Interrupt"]
    pub intpend: INTPEND,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x18 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x1c - Ready Users"]
    pub readyusr: READYUSR,
    #[doc = "0x20 - CHANNEL\\[%s\\]"]
    pub channel: [CHANNEL; 32],
    #[doc = "0x120 - User Multiplexer n"]
    pub user: [USER; 67],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Channel n Control"]
    pub channel: self::channel::CHANNEL,
    #[doc = "0x04 - Channel n Interrupt Enable Clear"]
    pub chintenclr: self::channel::CHINTENCLR,
    #[doc = "0x05 - Channel n Interrupt Enable Set"]
    pub chintenset: self::channel::CHINTENSET,
    #[doc = "0x06 - Channel n Interrupt Flag Status and Clear"]
    pub chintflag: self::channel::CHINTFLAG,
    #[doc = "0x07 - Channel n Status"]
    pub chstatus: self::channel::CHSTATUS,
}
#[doc = r"Register block"]
#[doc = "CHANNEL\\[%s\\]"]
pub mod channel;
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](ctrla) module"]
pub type CTRLA = crate::Reg<u8, _CTRLA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA;
#[doc = "`read()` method returns [ctrla::R](ctrla::R) reader structure"]
impl crate::Readable for CTRLA {}
#[doc = "`write(|w| ..)` method takes [ctrla::W](ctrla::W) writer structure"]
impl crate::Writable for CTRLA {}
#[doc = "Control"]
pub mod ctrla;
#[doc = "Software Event\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevt](swevt) module"]
pub type SWEVT = crate::Reg<u32, _SWEVT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWEVT;
#[doc = "`write(|w| ..)` method takes [swevt::W](swevt::W) writer structure"]
impl crate::Writable for SWEVT {}
#[doc = "Software Event"]
pub mod swevt;
#[doc = "Priority Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prictrl](prictrl) module"]
pub type PRICTRL = crate::Reg<u8, _PRICTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRICTRL;
#[doc = "`read()` method returns [prictrl::R](prictrl::R) reader structure"]
impl crate::Readable for PRICTRL {}
#[doc = "`write(|w| ..)` method takes [prictrl::W](prictrl::W) writer structure"]
impl crate::Writable for PRICTRL {}
#[doc = "Priority Control"]
pub mod prictrl;
#[doc = "Channel Pending Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpend](intpend) module"]
pub type INTPEND = crate::Reg<u16, _INTPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPEND;
#[doc = "`read()` method returns [intpend::R](intpend::R) reader structure"]
impl crate::Readable for INTPEND {}
#[doc = "`write(|w| ..)` method takes [intpend::W](intpend::W) writer structure"]
impl crate::Writable for INTPEND {}
#[doc = "Channel Pending Interrupt"]
pub mod intpend;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u32, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "Busy Channels\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busych](busych) module"]
pub type BUSYCH = crate::Reg<u32, _BUSYCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSYCH;
#[doc = "`read()` method returns [busych::R](busych::R) reader structure"]
impl crate::Readable for BUSYCH {}
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "Ready Users\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readyusr](readyusr) module"]
pub type READYUSR = crate::Reg<u32, _READYUSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READYUSR;
#[doc = "`read()` method returns [readyusr::R](readyusr::R) reader structure"]
impl crate::Readable for READYUSR {}
#[doc = "Ready Users"]
pub mod readyusr;
#[doc = "User Multiplexer n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user](user) module"]
pub type USER = crate::Reg<u32, _USER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER;
#[doc = "`read()` method returns [user::R](user::R) reader structure"]
impl crate::Readable for USER {}
#[doc = "`write(|w| ..)` method takes [user::W](user::W) writer structure"]
impl crate::Writable for USER {}
#[doc = "User Multiplexer n"]
pub mod user;
