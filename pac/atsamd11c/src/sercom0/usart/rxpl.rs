#[doc = "Register `RXPL` reader"]
pub struct R(crate::R<RXPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXPL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXPL` writer"]
pub struct W(crate::W<RXPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXPL_SPEC>;
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
impl From<crate::W<RXPL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPL` reader - Receive Pulse Length"]
pub struct RXPL_R(crate::FieldReader<u8, u8>);
impl RXPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPL` writer - Receive Pulse Length"]
pub struct RXPL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Pulse Length"]
    #[inline(always)]
    pub fn rxpl(&self) -> RXPL_R {
        RXPL_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Pulse Length"]
    #[inline(always)]
    pub fn rxpl(&mut self) -> RXPL_W {
        RXPL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Receive Pulse Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxpl](index.html) module"]
pub struct RXPL_SPEC;
impl crate::RegisterSpec for RXPL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxpl::R](R) reader structure"]
impl crate::Readable for RXPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxpl::W](W) writer structure"]
impl crate::Writable for RXPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXPL to value 0"]
impl crate::Resettable for RXPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
