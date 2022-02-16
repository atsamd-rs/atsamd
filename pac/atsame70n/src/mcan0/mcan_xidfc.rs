#[doc = "Register `MCAN_XIDFC` reader"]
pub struct R(crate::R<MCAN_XIDFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_XIDFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_XIDFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_XIDFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_XIDFC` writer"]
pub struct W(crate::W<MCAN_XIDFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_XIDFC_SPEC>;
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
impl From<crate::W<MCAN_XIDFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_XIDFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLESA` reader - Filter List Extended Start Address"]
pub struct FLESA_R(crate::FieldReader<u16, u16>);
impl FLESA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FLESA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLESA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLESA` writer - Filter List Extended Start Address"]
pub struct FLESA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLESA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `LSE` reader - List Size Extended"]
pub struct LSE_R(crate::FieldReader<u8, u8>);
impl LSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSE` writer - List Size Extended"]
pub struct LSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Filter List Extended Start Address"]
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - List Size Extended"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Filter List Extended Start Address"]
    #[inline(always)]
    pub fn flesa(&mut self) -> FLESA_W {
        FLESA_W { w: self }
    }
    #[doc = "Bits 16:22 - List Size Extended"]
    #[inline(always)]
    pub fn lse(&mut self) -> LSE_W {
        LSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended ID Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_xidfc](index.html) module"]
pub struct MCAN_XIDFC_SPEC;
impl crate::RegisterSpec for MCAN_XIDFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_xidfc::R](R) reader structure"]
impl crate::Readable for MCAN_XIDFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_xidfc::W](W) writer structure"]
impl crate::Writable for MCAN_XIDFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_XIDFC to value 0"]
impl crate::Resettable for MCAN_XIDFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
