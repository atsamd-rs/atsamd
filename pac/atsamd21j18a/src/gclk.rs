#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Status"]
    pub status: STATUS,
    #[doc = "0x02 - Generic Clock Control"]
    pub clkctrl: CLKCTRL,
    #[doc = "0x04 - Generic Clock Generator Control"]
    pub genctrl: GENCTRL,
    #[doc = "0x08 - Generic Clock Generator Division"]
    pub gendiv: GENDIV,
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u8, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control"]
pub mod ctrl;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u8, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Generic Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctrl](clkctrl) module"]
pub type CLKCTRL = crate::Reg<u16, _CLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCTRL;
#[doc = "`read()` method returns [clkctrl::R](clkctrl::R) reader structure"]
impl crate::Readable for CLKCTRL {}
#[doc = "`write(|w| ..)` method takes [clkctrl::W](clkctrl::W) writer structure"]
impl crate::Writable for CLKCTRL {}
#[doc = "Generic Clock Control"]
pub mod clkctrl;
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
#[doc = "Generic Clock Generator Division\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gendiv](gendiv) module"]
pub type GENDIV = crate::Reg<u32, _GENDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GENDIV;
#[doc = "`read()` method returns [gendiv::R](gendiv::R) reader structure"]
impl crate::Readable for GENDIV {}
#[doc = "`write(|w| ..)` method takes [gendiv::W](gendiv::W) writer structure"]
impl crate::Writable for GENDIV {}
#[doc = "Generic Clock Generator Division"]
pub mod gendiv;
