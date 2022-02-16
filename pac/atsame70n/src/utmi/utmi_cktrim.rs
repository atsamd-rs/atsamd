#[doc = "Register `UTMI_CKTRIM` reader"]
pub struct R(crate::R<UTMI_CKTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UTMI_CKTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UTMI_CKTRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UTMI_CKTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UTMI_CKTRIM` writer"]
pub struct W(crate::W<UTMI_CKTRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UTMI_CKTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UTMI_CKTRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UTMI_CKTRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "UTMI Reference Clock Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FREQ_A {
    #[doc = "0: 12 MHz reference clock"]
    XTAL12 = 0,
    #[doc = "1: 16 MHz reference clock"]
    XTAL16 = 1,
}
impl From<FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FREQ` reader - UTMI Reference Clock Frequency"]
pub struct FREQ_R(crate::FieldReader<u8, FREQ_A>);
impl FREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQ_A> {
        match self.bits {
            0 => Some(FREQ_A::XTAL12),
            1 => Some(FREQ_A::XTAL16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL12`"]
    #[inline(always)]
    pub fn is_xtal12(&self) -> bool {
        **self == FREQ_A::XTAL12
    }
    #[doc = "Checks if the value of the field is `XTAL16`"]
    #[inline(always)]
    pub fn is_xtal16(&self) -> bool {
        **self == FREQ_A::XTAL16
    }
}
impl core::ops::Deref for FREQ_R {
    type Target = crate::FieldReader<u8, FREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQ` writer - UTMI Reference Clock Frequency"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "12 MHz reference clock"]
    #[inline(always)]
    pub fn xtal12(self) -> &'a mut W {
        self.variant(FREQ_A::XTAL12)
    }
    #[doc = "16 MHz reference clock"]
    #[inline(always)]
    pub fn xtal16(self) -> &'a mut W {
        self.variant(FREQ_A::XTAL16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - UTMI Reference Clock Frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UTMI Reference Clock Frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UTMI Clock Trimming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utmi_cktrim](index.html) module"]
pub struct UTMI_CKTRIM_SPEC;
impl crate::RegisterSpec for UTMI_CKTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [utmi_cktrim::R](R) reader structure"]
impl crate::Readable for UTMI_CKTRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [utmi_cktrim::W](W) writer structure"]
impl crate::Writable for UTMI_CKTRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UTMI_CKTRIM to value 0"]
impl crate::Resettable for UTMI_CKTRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
