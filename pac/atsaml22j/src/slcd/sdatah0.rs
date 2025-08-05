#[doc = "Register `SDATAH0` reader"]
pub struct R(crate::R<SDATAH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDATAH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDATAH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDATAH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDATAH0` writer"]
pub struct W(crate::W<SDATAH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDATAH0_SPEC>;
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
impl From<crate::W<SDATAH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDATAH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDATA` reader - Segments Data"]
pub type SDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDATA` writer - Segments Data"]
pub type SDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDATAH0_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Segments Data"]
    #[inline(always)]
    pub fn sdata(&self) -> SDATA_R {
        SDATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Segments Data"]
    #[inline(always)]
    #[must_use]
    pub fn sdata(&mut self) -> SDATA_W<0> {
        SDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segments Data High for COM0 Line\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdatah0](index.html) module"]
pub struct SDATAH0_SPEC;
impl crate::RegisterSpec for SDATAH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdatah0::R](R) reader structure"]
impl crate::Readable for SDATAH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdatah0::W](W) writer structure"]
impl crate::Writable for SDATAH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDATAH0 to value 0"]
impl crate::Resettable for SDATAH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
