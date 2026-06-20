#[doc = "Register `SAMPCTRL` reader"]
pub struct R(crate::R<SAMPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPCTRL` writer"]
pub struct W(crate::W<SAMPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPCTRL_SPEC>;
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
impl From<crate::W<SAMPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLEN` reader - Sampling Time Length"]
pub struct SAMPLEN_R(crate::FieldReader<u8, u8>);
impl SAMPLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SAMPLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMPLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPLEN` writer - Sampling Time Length"]
pub struct SAMPLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u8 & 0x3f);
        self.w
    }
}
#[doc = "Field `OFFCOMP` reader - Comparator Offset Compensation Enable"]
pub struct OFFCOMP_R(crate::FieldReader<bool, bool>);
impl OFFCOMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OFFCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFCOMP` writer - Comparator Offset Compensation Enable"]
pub struct OFFCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFCOMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Sampling Time Length"]
    #[inline(always)]
    pub fn samplen(&self) -> SAMPLEN_R {
        SAMPLEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Comparator Offset Compensation Enable"]
    #[inline(always)]
    pub fn offcomp(&self) -> OFFCOMP_R {
        OFFCOMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Sampling Time Length"]
    #[inline(always)]
    pub fn samplen(&mut self) -> SAMPLEN_W {
        SAMPLEN_W { w: self }
    }
    #[doc = "Bit 7 - Comparator Offset Compensation Enable"]
    #[inline(always)]
    pub fn offcomp(&mut self) -> OFFCOMP_W {
        OFFCOMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample Time Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampctrl](index.html) module"]
pub struct SAMPCTRL_SPEC;
impl crate::RegisterSpec for SAMPCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sampctrl::R](R) reader structure"]
impl crate::Readable for SAMPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sampctrl::W](W) writer structure"]
impl crate::Writable for SAMPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPCTRL to value 0"]
impl crate::Resettable for SAMPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
