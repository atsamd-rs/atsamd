#[doc = "Register `CVR` reader"]
pub struct R(crate::R<CVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CVR` writer"]
pub struct W(crate::W<CVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CVR_SPEC>;
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
impl From<crate::W<CVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRENT` reader - Current value at the time the register is accessed"]
pub type CURRENT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CURRENT` writer - Current value at the time the register is accessed"]
pub type CURRENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CVR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Current value at the time the register is accessed"]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Current value at the time the register is accessed"]
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CURRENT_W<0> {
        CURRENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvr](index.html) module"]
pub struct CVR_SPEC;
impl crate::RegisterSpec for CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cvr::R](R) reader structure"]
impl crate::Readable for CVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cvr::W](W) writer structure"]
impl crate::Writable for CVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CVR to value 0"]
impl crate::Resettable for CVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
