#[doc = "Register `MCAN_TXEFA` reader"]
pub struct R(crate::R<MCAN_TXEFA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_TXEFA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_TXEFA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_TXEFA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_TXEFA` writer"]
pub struct W(crate::W<MCAN_TXEFA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_TXEFA_SPEC>;
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
impl From<crate::W<MCAN_TXEFA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_TXEFA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFAI` reader - Event FIFO Acknowledge Index"]
pub struct EFAI_R(crate::FieldReader<u8, u8>);
impl EFAI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EFAI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFAI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFAI` writer - Event FIFO Acknowledge Index"]
pub struct EFAI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFAI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
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
    pub fn efai(&mut self) -> EFAI_W {
        EFAI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Event FIFO Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txefa](index.html) module"]
pub struct MCAN_TXEFA_SPEC;
impl crate::RegisterSpec for MCAN_TXEFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_txefa::R](R) reader structure"]
impl crate::Readable for MCAN_TXEFA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_txefa::W](W) writer structure"]
impl crate::Writable for MCAN_TXEFA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_TXEFA to value 0"]
impl crate::Resettable for MCAN_TXEFA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
