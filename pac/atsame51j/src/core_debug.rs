#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug Halting Control and Status Register"]
    pub dhcsr: DHCSR,
    #[doc = "0x04 - Debug Core Register Selector Register"]
    pub dcrsr: DCRSR,
    #[doc = "0x08 - Debug Core Register Data Register"]
    pub dcrdr: DCRDR,
    #[doc = "0x0c - Debug Exception and Monitor Control Register"]
    pub demcr: DEMCR,
}
#[doc = "Debug Halting Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhcsr](dhcsr) module"]
pub type DHCSR = crate::Reg<u32, _DHCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHCSR;
#[doc = "`read()` method returns [dhcsr::R](dhcsr::R) reader structure"]
impl crate::Readable for DHCSR {}
#[doc = "`write(|w| ..)` method takes [dhcsr::W](dhcsr::W) writer structure"]
impl crate::Writable for DHCSR {}
#[doc = "Debug Halting Control and Status Register"]
pub mod dhcsr;
#[doc = "Debug Core Register Selector Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrsr](dcrsr) module"]
pub type DCRSR = crate::Reg<u32, _DCRSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRSR;
#[doc = "`write(|w| ..)` method takes [dcrsr::W](dcrsr::W) writer structure"]
impl crate::Writable for DCRSR {}
#[doc = "Debug Core Register Selector Register"]
pub mod dcrsr;
#[doc = "Debug Core Register Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrdr](dcrdr) module"]
pub type DCRDR = crate::Reg<u32, _DCRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRDR;
#[doc = "`read()` method returns [dcrdr::R](dcrdr::R) reader structure"]
impl crate::Readable for DCRDR {}
#[doc = "`write(|w| ..)` method takes [dcrdr::W](dcrdr::W) writer structure"]
impl crate::Writable for DCRDR {}
#[doc = "Debug Core Register Data Register"]
pub mod dcrdr;
#[doc = "Debug Exception and Monitor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [demcr](demcr) module"]
pub type DEMCR = crate::Reg<u32, _DEMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEMCR;
#[doc = "`read()` method returns [demcr::R](demcr::R) reader structure"]
impl crate::Readable for DEMCR {}
#[doc = "`write(|w| ..)` method takes [demcr::W](demcr::W) writer structure"]
impl crate::Writable for DEMCR {}
#[doc = "Debug Exception and Monitor Control Register"]
pub mod demcr;
