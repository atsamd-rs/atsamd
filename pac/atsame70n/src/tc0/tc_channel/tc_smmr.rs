#[doc = "Register `TC_SMMR` reader"]
pub struct R(crate::R<TC_SMMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_SMMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_SMMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_SMMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TC_SMMR` writer"]
pub struct W(crate::W<TC_SMMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TC_SMMR_SPEC>;
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
impl From<crate::W<TC_SMMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TC_SMMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCEN` reader - Gray Count Enable"]
pub struct GCEN_R(crate::FieldReader<bool, bool>);
impl GCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCEN` writer - Gray Count Enable"]
pub struct GCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DOWN` reader - Down Count"]
pub struct DOWN_R(crate::FieldReader<bool, bool>);
impl DOWN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOWN` writer - Down Count"]
pub struct DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Gray Count Enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Down Count"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gray Count Enable"]
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W {
        GCEN_W { w: self }
    }
    #[doc = "Bit 1 - Down Count"]
    #[inline(always)]
    pub fn down(&mut self) -> DOWN_W {
        DOWN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stepper Motor Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_smmr](index.html) module"]
pub struct TC_SMMR_SPEC;
impl crate::RegisterSpec for TC_SMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc_smmr::R](R) reader structure"]
impl crate::Readable for TC_SMMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tc_smmr::W](W) writer structure"]
impl crate::Writable for TC_SMMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TC_SMMR to value 0"]
impl crate::Resettable for TC_SMMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
