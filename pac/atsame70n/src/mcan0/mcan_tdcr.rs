#[doc = "Register `MCAN_TDCR` reader"]
pub struct R(crate::R<MCAN_TDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_TDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_TDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_TDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_TDCR` writer"]
pub struct W(crate::W<MCAN_TDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_TDCR_SPEC>;
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
impl From<crate::W<MCAN_TDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_TDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDCF` reader - Transmitter Delay Compensation Filter"]
pub struct TDCF_R(crate::FieldReader<u8, u8>);
impl TDCF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCF` writer - Transmitter Delay Compensation Filter"]
pub struct TDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `TDCO` reader - Transmitter Delay Compensation Offset"]
pub struct TDCO_R(crate::FieldReader<u8, u8>);
impl TDCO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCO` writer - Transmitter Delay Compensation Offset"]
pub struct TDCO_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter"]
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter"]
    #[inline(always)]
    pub fn tdcf(&mut self) -> TDCF_W {
        TDCF_W { w: self }
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&mut self) -> TDCO_W {
        TDCO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Delay Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_tdcr](index.html) module"]
pub struct MCAN_TDCR_SPEC;
impl crate::RegisterSpec for MCAN_TDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_tdcr::R](R) reader structure"]
impl crate::Readable for MCAN_TDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_tdcr::W](W) writer structure"]
impl crate::Writable for MCAN_TDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_TDCR to value 0"]
impl crate::Resettable for MCAN_TDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
