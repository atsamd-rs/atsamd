#[doc = "Register `MCAN_RXF1A` reader"]
pub struct R(crate::R<MCAN_RXF1A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_RXF1A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_RXF1A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_RXF1A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_RXF1A` writer"]
pub struct W(crate::W<MCAN_RXF1A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_RXF1A_SPEC>;
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
impl From<crate::W<MCAN_RXF1A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_RXF1A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F1AI` reader - Receive FIFO 1 Acknowledge Index"]
pub struct F1AI_R(crate::FieldReader<u8, u8>);
impl F1AI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F1AI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1AI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1AI` writer - Receive FIFO 1 Acknowledge Index"]
pub struct F1AI_W<'a> {
    w: &'a mut W,
}
impl<'a> F1AI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Receive FIFO 1 Acknowledge Index"]
    #[inline(always)]
    pub fn f1ai(&self) -> F1AI_R {
        F1AI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Receive FIFO 1 Acknowledge Index"]
    #[inline(always)]
    pub fn f1ai(&mut self) -> F1AI_W {
        F1AI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO 1 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxf1a](index.html) module"]
pub struct MCAN_RXF1A_SPEC;
impl crate::RegisterSpec for MCAN_RXF1A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_rxf1a::R](R) reader structure"]
impl crate::Readable for MCAN_RXF1A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_rxf1a::W](W) writer structure"]
impl crate::Writable for MCAN_RXF1A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_RXF1A to value 0"]
impl crate::Resettable for MCAN_RXF1A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
