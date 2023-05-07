#[doc = "Register `RXF1A` reader"]
pub struct R(crate::R<RXF1A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF1A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF1A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF1A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXF1A` writer"]
pub struct W(crate::W<RXF1A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF1A_SPEC>;
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
impl From<crate::W<RXF1A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF1A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F1AI` reader - Rx FIFO 1 Acknowledge Index"]
pub type F1AI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1AI` writer - Rx FIFO 1 Acknowledge Index"]
pub type F1AI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXF1A_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 1 Acknowledge Index"]
    #[inline(always)]
    pub fn f1ai(&self) -> F1AI_R {
        F1AI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 1 Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn f1ai(&mut self) -> F1AI_W<0> {
        F1AI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx FIFO 1 Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf1a](index.html) module"]
pub struct RXF1A_SPEC;
impl crate::RegisterSpec for RXF1A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf1a::R](R) reader structure"]
impl crate::Readable for RXF1A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxf1a::W](W) writer structure"]
impl crate::Writable for RXF1A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXF1A to value 0"]
impl crate::Resettable for RXF1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
