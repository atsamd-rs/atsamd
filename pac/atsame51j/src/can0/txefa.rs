#[doc = "Register `TXEFA` reader"]
pub struct R(crate::R<TXEFA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEFA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEFA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEFA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXEFA` writer"]
pub struct W(crate::W<TXEFA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXEFA_SPEC>;
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
impl From<crate::W<TXEFA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXEFA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFAI` reader - Event FIFO Acknowledge Index"]
pub type EFAI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFAI` writer - Event FIFO Acknowledge Index"]
pub type EFAI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXEFA_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub fn efai(&self) -> EFAI_R {
        EFAI_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Event FIFO Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn efai(&mut self) -> EFAI_W<0> {
        EFAI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Event FIFO Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefa](index.html) module"]
pub struct TXEFA_SPEC;
impl crate::RegisterSpec for TXEFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txefa::R](R) reader structure"]
impl crate::Readable for TXEFA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txefa::W](W) writer structure"]
impl crate::Writable for TXEFA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXEFA to value 0"]
impl crate::Resettable for TXEFA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
