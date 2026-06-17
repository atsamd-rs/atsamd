#[doc = "Register `BASEADDR` reader"]
pub struct R(crate::R<BASEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASEADDR` writer"]
pub struct W(crate::W<BASEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASEADDR_SPEC>;
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
impl From<crate::W<BASEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASEADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADDR` reader - Descriptor Memory Base Address"]
pub type BASEADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASEADDR` writer - Descriptor Memory Base Address"]
pub type BASEADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BASEADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn baseaddr(&mut self) -> BASEADDR_W<0> {
        BASEADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor Memory Section Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baseaddr](index.html) module"]
pub struct BASEADDR_SPEC;
impl crate::RegisterSpec for BASEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baseaddr::R](R) reader structure"]
impl crate::Readable for BASEADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baseaddr::W](W) writer structure"]
impl crate::Writable for BASEADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BASEADDR to value 0"]
impl crate::Resettable for BASEADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
