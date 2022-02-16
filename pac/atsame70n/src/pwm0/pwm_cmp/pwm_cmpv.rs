#[doc = "Register `PWM_CMPV` reader"]
pub struct R(crate::R<PWM_CMPV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CMPV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CMPV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CMPV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CMPV` writer"]
pub struct W(crate::W<PWM_CMPV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CMPV_SPEC>;
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
impl From<crate::W<PWM_CMPV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CMPV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CV` reader - Comparison x Value"]
pub struct CV_R(crate::FieldReader<u32, u32>);
impl CV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CV` writer - Comparison x Value"]
pub struct CV_W<'a> {
    w: &'a mut W,
}
impl<'a> CV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Comparison x Value Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CVM_A {
    #[doc = "0: Compare when counter is incrementing"]
    COMPARE_AT_INCREMENT = 0,
    #[doc = "1: Compare when counter is decrementing"]
    COMPARE_AT_DECREMENT = 1,
}
impl From<CVM_A> for bool {
    #[inline(always)]
    fn from(variant: CVM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CVM` reader - Comparison x Value Mode"]
pub struct CVM_R(crate::FieldReader<bool, CVM_A>);
impl CVM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CVM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CVM_A {
        match self.bits {
            false => CVM_A::COMPARE_AT_INCREMENT,
            true => CVM_A::COMPARE_AT_DECREMENT,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARE_AT_INCREMENT`"]
    #[inline(always)]
    pub fn is_compare_at_increment(&self) -> bool {
        **self == CVM_A::COMPARE_AT_INCREMENT
    }
    #[doc = "Checks if the value of the field is `COMPARE_AT_DECREMENT`"]
    #[inline(always)]
    pub fn is_compare_at_decrement(&self) -> bool {
        **self == CVM_A::COMPARE_AT_DECREMENT
    }
}
impl core::ops::Deref for CVM_R {
    type Target = crate::FieldReader<bool, CVM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVM` writer - Comparison x Value Mode"]
pub struct CVM_W<'a> {
    w: &'a mut W,
}
impl<'a> CVM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CVM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare when counter is incrementing"]
    #[inline(always)]
    pub fn compare_at_increment(self) -> &'a mut W {
        self.variant(CVM_A::COMPARE_AT_INCREMENT)
    }
    #[doc = "Compare when counter is decrementing"]
    #[inline(always)]
    pub fn compare_at_decrement(self) -> &'a mut W {
        self.variant(CVM_A::COMPARE_AT_DECREMENT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&self) -> CVM_R {
        CVM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&mut self) -> CV_W {
        CV_W { w: self }
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&mut self) -> CVM_W {
        CVM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 0 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmpv](index.html) module"]
pub struct PWM_CMPV_SPEC;
impl crate::RegisterSpec for PWM_CMPV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cmpv::R](R) reader structure"]
impl crate::Readable for PWM_CMPV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_cmpv::W](W) writer structure"]
impl crate::Writable for PWM_CMPV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_CMPV to value 0"]
impl crate::Resettable for PWM_CMPV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
