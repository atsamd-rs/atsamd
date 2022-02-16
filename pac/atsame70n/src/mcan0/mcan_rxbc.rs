#[doc = "Register `MCAN_RXBC` reader"]
pub struct R(crate::R<MCAN_RXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_RXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_RXBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_RXBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_RXBC` writer"]
pub struct W(crate::W<MCAN_RXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_RXBC_SPEC>;
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
impl From<crate::W<MCAN_RXBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_RXBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBSA` reader - Receive Buffer Start Address"]
pub struct RBSA_R(crate::FieldReader<u16, u16>);
impl RBSA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RBSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBSA` writer - Receive Buffer Start Address"]
pub struct RBSA_W<'a> {
    w: &'a mut W,
}
impl<'a> RBSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Receive Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&self) -> RBSA_R {
        RBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - Receive Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&mut self) -> RBSA_W {
        RBSA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Rx Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxbc](index.html) module"]
pub struct MCAN_RXBC_SPEC;
impl crate::RegisterSpec for MCAN_RXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_rxbc::R](R) reader structure"]
impl crate::Readable for MCAN_RXBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_rxbc::W](W) writer structure"]
impl crate::Writable for MCAN_RXBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_RXBC to value 0"]
impl crate::Resettable for MCAN_RXBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
