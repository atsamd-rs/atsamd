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
#[doc = "Field `UNDERRUN` reader - Underrun Interrupt Enable"]
pub struct UNDERRUN_R(crate::FieldReader<bool, bool>);
impl UNDERRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNDERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNDERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNDERRUN` writer - Underrun Interrupt Enable"]
pub struct UNDERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERRUN_W<'a> {
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
#[doc = "Field `EMPTY` reader - Data Buffer Empty Interrupt Enable"]
pub struct EMPTY_R(crate::FieldReader<bool, bool>);
impl EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMPTY` writer - Data Buffer Empty Interrupt Enable"]
pub struct EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTY_W<'a> {
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
#[doc = "Field `SYNCRDY` reader - Synchronization Ready Interrupt Enable"]
pub struct SYNCRDY_R(crate::FieldReader<bool, bool>);
impl SYNCRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCRDY` writer - Synchronization Ready Interrupt Enable"]
pub struct SYNCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SYNCRDY_R {
        SYNCRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UNDERRUN_W {
        UNDERRUN_W { w: self }
    }
    #[doc = "Bit 1 - Data Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty(&mut self) -> EMPTY_W {
        EMPTY_W { w: self }
    }
    #[doc = "Bit 2 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&mut self) -> SYNCRDY_W {
        SYNCRDY_W { w: self }
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
