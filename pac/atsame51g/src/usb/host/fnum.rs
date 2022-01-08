#[doc = "Register `FNUM` reader"]
pub struct R(crate::R<FNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FNUM` writer"]
pub struct W(crate::W<FNUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FNUM_SPEC>;
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
impl From<crate::W<FNUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FNUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFNUM` reader - Micro Frame Number"]
pub struct MFNUM_R(crate::FieldReader<u8, u8>);
impl MFNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MFNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFNUM` writer - Micro Frame Number"]
pub struct MFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> MFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
#[doc = "Field `FNUM` reader - Frame Number"]
pub struct FNUM_R(crate::FieldReader<u16, u16>);
impl FNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNUM` writer - Frame Number"]
pub struct FNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> FNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 3)) | ((value as u16 & 0x07ff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&mut self) -> MFNUM_W {
        MFNUM_W { w: self }
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&mut self) -> FNUM_W {
        FNUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST Host Frame Number\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnum](index.html) module"]
pub struct FNUM_SPEC;
impl crate::RegisterSpec for FNUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fnum::R](R) reader structure"]
impl crate::Readable for FNUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fnum::W](W) writer structure"]
impl crate::Writable for FNUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FNUM to value 0"]
impl crate::Resettable for FNUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
