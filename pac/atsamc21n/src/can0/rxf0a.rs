#[doc = "Register `RXF0A` reader"]
pub struct R(crate::R<RXF0A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXF0A` writer"]
pub struct W(crate::W<RXF0A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF0A_SPEC>;
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
impl From<crate::W<RXF0A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF0A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0AI` reader - Rx FIFO 0 Acknowledge Index"]
pub struct F0AI_R(crate::FieldReader<u8, u8>);
impl F0AI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F0AI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0AI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0AI` writer - Rx FIFO 0 Acknowledge Index"]
pub struct F0AI_W<'a> {
    w: &'a mut W,
}
impl<'a> F0AI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub fn f0ai(&self) -> F0AI_R {
        F0AI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub fn f0ai(&mut self) -> F0AI_W {
        F0AI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx FIFO 0 Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0a](index.html) module"]
pub struct RXF0A_SPEC;
impl crate::RegisterSpec for RXF0A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf0a::R](R) reader structure"]
impl crate::Readable for RXF0A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxf0a::W](W) writer structure"]
impl crate::Writable for RXF0A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXF0A to value 0"]
impl crate::Resettable for RXF0A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
