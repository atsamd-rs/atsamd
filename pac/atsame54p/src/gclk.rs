#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    _reserved2: [u8; 24usize],
    #[doc = "0x20 - Generic Clock Generator Control"]
    pub genctrl: [GENCTRL; 12],
    _reserved3: [u8; 48usize],
    #[doc = "0x80 - Peripheral Clock Control"]
    pub pchctrl: [PCHCTRL; 48],
}
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
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "Generic Clock Generator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [genctrl](genctrl) module"]
pub type GENCTRL = crate::Reg<u32, _GENCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GENCTRL;
#[doc = "`read()` method returns [genctrl::R](genctrl::R) reader structure"]
impl crate::Readable for GENCTRL {}
#[doc = "`write(|w| ..)` method takes [genctrl::W](genctrl::W) writer structure"]
impl crate::Writable for GENCTRL {}
#[doc = "Generic Clock Generator Control"]
pub mod genctrl;
#[doc = "Peripheral Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pchctrl](pchctrl) module"]
pub type PCHCTRL = crate::Reg<u32, _PCHCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCHCTRL;
#[doc = "`read()` method returns [pchctrl::R](pchctrl::R) reader structure"]
impl crate::Readable for PCHCTRL {}
#[doc = "`write(|w| ..)` method takes [pchctrl::W](pchctrl::W) writer structure"]
impl crate::Writable for PCHCTRL {}
#[doc = "Peripheral Clock Control"]
pub mod pchctrl;
