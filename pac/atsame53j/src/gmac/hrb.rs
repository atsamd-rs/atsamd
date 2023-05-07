#[doc = "Register `HRB` reader"]
pub struct R(crate::R<HRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRB` writer"]
pub struct W(crate::W<HRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRB_SPEC>;
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
impl From<crate::W<HRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Hash Address"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Hash Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HRB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Hash Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Register Bottom \\[31:0\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrb](index.html) module"]
pub struct HRB_SPEC;
impl crate::RegisterSpec for HRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrb::R](R) reader structure"]
impl crate::Readable for HRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrb::W](W) writer structure"]
impl crate::Writable for HRB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRB to value 0"]
impl crate::Resettable for HRB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
