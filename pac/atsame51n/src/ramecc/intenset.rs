#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINGLEE` reader - Single Bit ECC Error Interrupt Enable Set"]
pub struct SINGLEE_R(crate::FieldReader<bool, bool>);
impl SINGLEE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SINGLEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINGLEE` writer - Single Bit ECC Error Interrupt Enable Set"]
pub struct SINGLEE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLEE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `DUALE` reader - Dual Bit ECC Error Interrupt Enable Set"]
pub struct DUALE_R(crate::FieldReader<bool, bool>);
impl DUALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUALE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUALE` writer - Dual Bit ECC Error Interrupt Enable Set"]
pub struct DUALE_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Single Bit ECC Error Interrupt Enable Set"]
    #[inline(always)]
    pub fn singlee(&self) -> SINGLEE_R {
        SINGLEE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Dual Bit ECC Error Interrupt Enable Set"]
    #[inline(always)]
    pub fn duale(&self) -> DUALE_R {
        DUALE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Bit ECC Error Interrupt Enable Set"]
    #[inline(always)]
    pub fn singlee(&mut self) -> SINGLEE_W {
        SINGLEE_W { w: self }
    }
    #[doc = "Bit 1 - Dual Bit ECC Error Interrupt Enable Set"]
    #[inline(always)]
    pub fn duale(&mut self) -> DUALE_W {
        DUALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
