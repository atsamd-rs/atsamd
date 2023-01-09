#[doc = "Register `CMDMASK` reader"]
pub struct R(crate::R<CMDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDMASK` writer"]
pub struct W(crate::W<CMDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDMASK_SPEC>;
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
impl From<crate::W<CMDMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDMASK` reader - Segments Data Mask"]
pub type SDMASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SDMASK` writer - Segments Data Mask"]
pub type SDMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDMASK_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Segments Data Mask"]
    #[inline(always)]
    pub fn sdmask(&self) -> SDMASK_R {
        SDMASK_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Segments Data Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sdmask(&mut self) -> SDMASK_W<0> {
        SDMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Character Mapping Segments Data Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdmask](index.html) module"]
pub struct CMDMASK_SPEC;
impl crate::RegisterSpec for CMDMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdmask::R](R) reader structure"]
impl crate::Readable for CMDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdmask::W](W) writer structure"]
impl crate::Writable for CMDMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDMASK to value 0"]
impl crate::Resettable for CMDMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
