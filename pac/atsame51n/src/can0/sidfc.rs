#[doc = "Register `SIDFC` reader"]
pub struct R(crate::R<SIDFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIDFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIDFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIDFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIDFC` writer"]
pub struct W(crate::W<SIDFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIDFC_SPEC>;
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
impl From<crate::W<SIDFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIDFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLSSA` reader - Filter List Standard Start Address"]
pub type FLSSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLSSA` writer - Filter List Standard Start Address"]
pub type FLSSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIDFC_SPEC, u16, u16, 16, O>;
#[doc = "Field `LSS` reader - List Size Standard"]
pub type LSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSS` writer - List Size Standard"]
pub type LSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIDFC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flssa(&self) -> FLSSA_R {
        FLSSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - List Size Standard"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Filter List Standard Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flssa(&mut self) -> FLSSA_W<0> {
        FLSSA_W::new(self)
    }
    #[doc = "Bits 16:23 - List Size Standard"]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LSS_W<16> {
        LSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standard ID Filter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidfc](index.html) module"]
pub struct SIDFC_SPEC;
impl crate::RegisterSpec for SIDFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sidfc::R](R) reader structure"]
impl crate::Readable for SIDFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sidfc::W](W) writer structure"]
impl crate::Writable for SIDFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIDFC to value 0"]
impl crate::Resettable for SIDFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
