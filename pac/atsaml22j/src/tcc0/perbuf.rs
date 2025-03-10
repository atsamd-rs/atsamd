#[doc = "Register `PERBUF` reader"]
pub struct R(crate::R<PERBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERBUF` writer"]
pub struct W(crate::W<PERBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERBUF_SPEC>;
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
impl From<crate::W<PERBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERBUF` reader - Period Buffer Value"]
pub type PERBUF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERBUF` writer - Period Buffer Value"]
pub type PERBUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERBUF_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&self) -> PERBUF_R {
        PERBUF_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn perbuf(&mut self) -> PERBUF_W<0> {
        PERBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Period Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbuf](index.html) module"]
pub struct PERBUF_SPEC;
impl crate::RegisterSpec for PERBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perbuf::R](R) reader structure"]
impl crate::Readable for PERBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perbuf::W](W) writer structure"]
impl crate::Writable for PERBUF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERBUF to value 0xffff_ffff"]
impl crate::Resettable for PERBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
