#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrl: CTRL,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - SEQ Control x"]
    pub seqctrl: [SEQCTRL; 2],
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - LUT Control x"]
    pub lutctrl: [LUTCTRL; 4],
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
#[doc = "SEQ Control x\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqctrl](seqctrl) module"]
pub type SEQCTRL = crate::Reg<u8, _SEQCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQCTRL;
#[doc = "`read()` method returns [seqctrl::R](seqctrl::R) reader structure"]
impl crate::Readable for SEQCTRL {}
#[doc = "`write(|w| ..)` method takes [seqctrl::W](seqctrl::W) writer structure"]
impl crate::Writable for SEQCTRL {}
#[doc = "SEQ Control x"]
pub mod seqctrl;
#[doc = "LUT Control x\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutctrl](lutctrl) module"]
pub type LUTCTRL = crate::Reg<u32, _LUTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUTCTRL;
#[doc = "`read()` method returns [lutctrl::R](lutctrl::R) reader structure"]
impl crate::Readable for LUTCTRL {}
#[doc = "`write(|w| ..)` method takes [lutctrl::W](lutctrl::W) writer structure"]
impl crate::Writable for LUTCTRL {}
#[doc = "LUT Control x"]
pub mod lutctrl;
