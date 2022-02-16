#[doc = "Register `TC_FMR` reader"]
pub struct R(crate::R<TC_FMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_FMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_FMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_FMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TC_FMR` writer"]
pub struct W(crate::W<TC_FMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TC_FMR_SPEC>;
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
impl From<crate::W<TC_FMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TC_FMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCF0` reader - Enable Compare Fault Channel 0"]
pub struct ENCF0_R(crate::FieldReader<bool, bool>);
impl ENCF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENCF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCF0` writer - Enable Compare Fault Channel 0"]
pub struct ENCF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCF0_W<'a> {
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
#[doc = "Field `ENCF1` reader - Enable Compare Fault Channel 1"]
pub struct ENCF1_R(crate::FieldReader<bool, bool>);
impl ENCF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENCF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCF1` writer - Enable Compare Fault Channel 1"]
pub struct ENCF1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCF1_W<'a> {
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
    #[doc = "Bit 0 - Enable Compare Fault Channel 0"]
    #[inline(always)]
    pub fn encf0(&self) -> ENCF0_R {
        ENCF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Compare Fault Channel 1"]
    #[inline(always)]
    pub fn encf1(&self) -> ENCF1_R {
        ENCF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Compare Fault Channel 0"]
    #[inline(always)]
    pub fn encf0(&mut self) -> ENCF0_W {
        ENCF0_W { w: self }
    }
    #[doc = "Bit 1 - Enable Compare Fault Channel 1"]
    #[inline(always)]
    pub fn encf1(&mut self) -> ENCF1_W {
        ENCF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_fmr](index.html) module"]
pub struct TC_FMR_SPEC;
impl crate::RegisterSpec for TC_FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc_fmr::R](R) reader structure"]
impl crate::Readable for TC_FMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tc_fmr::W](W) writer structure"]
impl crate::Writable for TC_FMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TC_FMR to value 0"]
impl crate::Resettable for TC_FMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
