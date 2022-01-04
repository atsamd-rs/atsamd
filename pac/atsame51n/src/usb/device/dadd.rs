#[doc = "Register `DADD` reader"]
pub struct R(crate::R<DADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADD` writer"]
pub struct W(crate::W<DADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADD_SPEC>;
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
impl From<crate::W<DADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADD` reader - Device Address"]
pub struct DADD_R(crate::FieldReader<u8, u8>);
impl DADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DADD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DADD` writer - Device Address"]
pub struct DADD_W<'a> {
    w: &'a mut W,
}
impl<'a> DADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u8 & 0x7f);
        self.w
    }
}
#[doc = "Field `ADDEN` reader - Device Address Enable"]
pub struct ADDEN_R(crate::FieldReader<bool, bool>);
impl ADDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDEN` writer - Device Address Enable"]
pub struct ADDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDEN_W<'a> {
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
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn dadd(&self) -> DADD_R {
        DADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Device Address Enable"]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Address"]
    #[inline(always)]
    pub fn dadd(&mut self) -> DADD_W {
        DADD_W { w: self }
    }
    #[doc = "Bit 7 - Device Address Enable"]
    #[inline(always)]
    pub fn adden(&mut self) -> ADDEN_W {
        ADDEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE Device Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dadd](index.html) module"]
pub struct DADD_SPEC;
impl crate::RegisterSpec for DADD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dadd::R](R) reader structure"]
impl crate::Readable for DADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dadd::W](W) writer structure"]
impl crate::Writable for DADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DADD to value 0"]
impl crate::Resettable for DADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
