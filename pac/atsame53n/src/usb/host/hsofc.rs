#[doc = "Register `HSOFC` reader"]
pub struct R(crate::R<HSOFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSOFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSOFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSOFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSOFC` writer"]
pub struct W(crate::W<HSOFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSOFC_SPEC>;
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
impl From<crate::W<HSOFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSOFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLENC` reader - Frame Length Control"]
pub struct FLENC_R(crate::FieldReader<u8, u8>);
impl FLENC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLENC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLENC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLENC` writer - Frame Length Control"]
pub struct FLENC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLENC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Field `FLENCE` reader - Frame Length Control Enable"]
pub struct FLENCE_R(crate::FieldReader<bool, bool>);
impl FLENCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLENCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLENCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLENCE` writer - Frame Length Control Enable"]
pub struct FLENCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLENCE_W<'a> {
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
    #[doc = "Bits 0:3 - Frame Length Control"]
    #[inline(always)]
    pub fn flenc(&self) -> FLENC_R {
        FLENC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Frame Length Control Enable"]
    #[inline(always)]
    pub fn flence(&self) -> FLENCE_R {
        FLENCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frame Length Control"]
    #[inline(always)]
    pub fn flenc(&mut self) -> FLENC_W {
        FLENC_W { w: self }
    }
    #[doc = "Bit 7 - Frame Length Control Enable"]
    #[inline(always)]
    pub fn flence(&mut self) -> FLENCE_W {
        FLENCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST Host Start Of Frame Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsofc](index.html) module"]
pub struct HSOFC_SPEC;
impl crate::RegisterSpec for HSOFC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [hsofc::R](R) reader structure"]
impl crate::Readable for HSOFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsofc::W](W) writer structure"]
impl crate::Writable for HSOFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSOFC to value 0"]
impl crate::Resettable for HSOFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
