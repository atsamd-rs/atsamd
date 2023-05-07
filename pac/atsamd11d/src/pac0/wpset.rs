#[doc = "Register `WPSET` reader"]
pub struct R(crate::R<WPSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPSET` writer"]
pub struct W(crate::W<WPSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPSET_SPEC>;
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
impl From<crate::W<WPSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WP` reader - Write Protection Set"]
pub type WP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WP` writer - Write Protection Set"]
pub type WP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPSET_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 1:31 - Write Protection Set"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:31 - Write Protection Set"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<1> {
        WP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protection Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpset](index.html) module"]
pub struct WPSET_SPEC;
impl crate::RegisterSpec for WPSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpset::R](R) reader structure"]
impl crate::Readable for WPSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpset::W](W) writer structure"]
impl crate::Writable for WPSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPSET to value 0"]
impl crate::Resettable for WPSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
