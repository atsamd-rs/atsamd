#[doc = "Register `DSCR` reader"]
pub struct R(crate::R<DSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSCR` writer"]
pub struct W(crate::W<DSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCR_SPEC>;
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
impl From<crate::W<DSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DASA` reader - Descriptor Area Start Address"]
pub struct DASA_R(crate::FieldReader<u32, u32>);
impl DASA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DASA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DASA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DASA` writer - Descriptor Area Start Address"]
pub struct DASA_W<'a> {
    w: &'a mut W,
}
impl<'a> DASA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | ((value as u32 & 0x03ff_ffff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - Descriptor Area Start Address"]
    #[inline(always)]
    pub fn dasa(&self) -> DASA_R {
        DASA_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 6:31 - Descriptor Area Start Address"]
    #[inline(always)]
    pub fn dasa(&mut self) -> DASA_W {
        DASA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region Descriptor Area Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscr](index.html) module"]
pub struct DSCR_SPEC;
impl crate::RegisterSpec for DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dscr::R](R) reader structure"]
impl crate::Readable for DSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dscr::W](W) writer structure"]
impl crate::Writable for DSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSCR to value 0"]
impl crate::Resettable for DSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
