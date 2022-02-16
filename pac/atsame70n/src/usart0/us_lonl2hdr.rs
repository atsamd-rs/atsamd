#[doc = "Register `US_LONL2HDR` reader"]
pub struct R(crate::R<US_LONL2HDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_LONL2HDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_LONL2HDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_LONL2HDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_LONL2HDR` writer"]
pub struct W(crate::W<US_LONL2HDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_LONL2HDR_SPEC>;
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
impl From<crate::W<US_LONL2HDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_LONL2HDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLI` reader - LON Backlog Increment"]
pub struct BLI_R(crate::FieldReader<u8, u8>);
impl BLI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLI` writer - LON Backlog Increment"]
pub struct BLI_W<'a> {
    w: &'a mut W,
}
impl<'a> BLI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `ALTP` reader - LON Alternate Path Bit"]
pub struct ALTP_R(crate::FieldReader<bool, bool>);
impl ALTP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALTP` writer - LON Alternate Path Bit"]
pub struct ALTP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PB` reader - LON Priority Bit"]
pub struct PB_R(crate::FieldReader<bool, bool>);
impl PB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB` writer - LON Priority Bit"]
pub struct PB_W<'a> {
    w: &'a mut W,
}
impl<'a> PB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - LON Backlog Increment"]
    #[inline(always)]
    pub fn bli(&self) -> BLI_R {
        BLI_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - LON Alternate Path Bit"]
    #[inline(always)]
    pub fn altp(&self) -> ALTP_R {
        ALTP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LON Priority Bit"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - LON Backlog Increment"]
    #[inline(always)]
    pub fn bli(&mut self) -> BLI_W {
        BLI_W { w: self }
    }
    #[doc = "Bit 6 - LON Alternate Path Bit"]
    #[inline(always)]
    pub fn altp(&mut self) -> ALTP_W {
        ALTP_W { w: self }
    }
    #[doc = "Bit 7 - LON Priority Bit"]
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W {
        PB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LON L2HDR Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonl2hdr](index.html) module"]
pub struct US_LONL2HDR_SPEC;
impl crate::RegisterSpec for US_LONL2HDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_lonl2hdr::R](R) reader structure"]
impl crate::Readable for US_LONL2HDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_lonl2hdr::W](W) writer structure"]
impl crate::Writable for US_LONL2HDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_LONL2HDR to value 0"]
impl crate::Resettable for US_LONL2HDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
