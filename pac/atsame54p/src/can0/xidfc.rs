#[doc = "Register `XIDFC` reader"]
pub struct R(crate::R<XIDFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XIDFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XIDFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XIDFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XIDFC` writer"]
pub struct W(crate::W<XIDFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XIDFC_SPEC>;
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
impl From<crate::W<XIDFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XIDFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLESA` reader - Filter List Extended Start Address"]
pub type FLESA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLESA` writer - Filter List Extended Start Address"]
pub type FLESA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XIDFC_SPEC, u16, u16, 16, O>;
#[doc = "Field `LSE` reader - List Size Extended"]
pub type LSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSE` writer - List Size Extended"]
pub type LSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XIDFC_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:15 - Filter List Extended Start Address"]
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - List Size Extended"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Filter List Extended Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flesa(&mut self) -> FLESA_W<0> {
        FLESA_W::new(self)
    }
    #[doc = "Bits 16:22 - List Size Extended"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LSE_W<16> {
        LSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended ID Filter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xidfc](index.html) module"]
pub struct XIDFC_SPEC;
impl crate::RegisterSpec for XIDFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xidfc::R](R) reader structure"]
impl crate::Readable for XIDFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xidfc::W](W) writer structure"]
impl crate::Writable for XIDFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XIDFC to value 0"]
impl crate::Resettable for XIDFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
