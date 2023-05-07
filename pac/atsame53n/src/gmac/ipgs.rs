#[doc = "Register `IPGS` reader"]
pub struct R(crate::R<IPGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPGS` writer"]
pub struct W(crate::W<IPGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPGS_SPEC>;
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
impl From<crate::W<IPGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FL` reader - Frame Length"]
pub type FL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FL` writer - Frame Length"]
pub type FL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPGS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Frame Length"]
    #[inline(always)]
    pub fn fl(&self) -> FL_R {
        FL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn fl(&mut self) -> FL_W<0> {
        FL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPG Stretch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipgs](index.html) module"]
pub struct IPGS_SPEC;
impl crate::RegisterSpec for IPGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipgs::R](R) reader structure"]
impl crate::Readable for IPGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipgs::W](W) writer structure"]
impl crate::Writable for IPGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPGS to value 0"]
impl crate::Resettable for IPGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
