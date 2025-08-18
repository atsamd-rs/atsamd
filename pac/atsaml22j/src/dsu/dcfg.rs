#[doc = "Register `DCFG%s` reader"]
pub struct R(crate::R<DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFG%s` writer"]
pub struct W(crate::W<DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFG_SPEC>;
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
impl From<crate::W<DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCFG` reader - Device Configuration"]
pub type DCFG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DCFG` writer - Device Configuration"]
pub type DCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Device Configuration"]
    #[inline(always)]
    pub fn dcfg(&self) -> DCFG_R {
        DCFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dcfg(&mut self) -> DCFG_W<0> {
        DCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](index.html) module"]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfg::R](R) reader structure"]
impl crate::Readable for DCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfg::W](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCFG%s to value 0"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
