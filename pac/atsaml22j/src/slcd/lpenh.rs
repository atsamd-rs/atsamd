#[doc = "Register `LPENH` reader"]
pub struct R(crate::R<LPENH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPENH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPENH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPENH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPENH` writer"]
pub struct W(crate::W<LPENH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPENH_SPEC>;
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
impl From<crate::W<LPENH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPENH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPEN` reader - LCD Pin Enable"]
pub type LPEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LPEN` writer - LCD Pin Enable"]
pub type LPEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPENH_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - LCD Pin Enable"]
    #[inline(always)]
    pub fn lpen(&self) -> LPEN_R {
        LPEN_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - LCD Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpen(&mut self) -> LPEN_W<0> {
        LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Pin Enable High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpenh](index.html) module"]
pub struct LPENH_SPEC;
impl crate::RegisterSpec for LPENH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpenh::R](R) reader structure"]
impl crate::Readable for LPENH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpenh::W](W) writer structure"]
impl crate::Writable for LPENH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPENH to value 0"]
impl crate::Resettable for LPENH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
