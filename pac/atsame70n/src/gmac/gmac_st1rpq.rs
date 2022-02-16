#[doc = "Register `GMAC_ST1RPQ[%s]` reader"]
pub struct R(crate::R<GMAC_ST1RPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_ST1RPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_ST1RPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_ST1RPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMAC_ST1RPQ[%s]` writer"]
pub struct W(crate::W<GMAC_ST1RPQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMAC_ST1RPQ_SPEC>;
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
impl From<crate::W<GMAC_ST1RPQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMAC_ST1RPQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QNB` reader - Queue Number (0-5)"]
pub struct QNB_R(crate::FieldReader<u8, u8>);
impl QNB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QNB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QNB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QNB` writer - Queue Number (0-5)"]
pub struct QNB_W<'a> {
    w: &'a mut W,
}
impl<'a> QNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `DSTCM` reader - Differentiated Services or Traffic Class Match"]
pub struct DSTCM_R(crate::FieldReader<u8, u8>);
impl DSTCM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DSTCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSTCM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSTCM` writer - Differentiated Services or Traffic Class Match"]
pub struct DSTCM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | ((value as u32 & 0xff) << 4);
        self.w
    }
}
#[doc = "Field `UDPM` reader - UDP Port Match"]
pub struct UDPM_R(crate::FieldReader<u16, u16>);
impl UDPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UDPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDPM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDPM` writer - UDP Port Match"]
pub struct UDPM_W<'a> {
    w: &'a mut W,
}
impl<'a> UDPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 12)) | ((value as u32 & 0xffff) << 12);
        self.w
    }
}
#[doc = "Field `DSTCE` reader - Differentiated Services or Traffic Class Match Enable"]
pub struct DSTCE_R(crate::FieldReader<bool, bool>);
impl DSTCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSTCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSTCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSTCE` writer - Differentiated Services or Traffic Class Match Enable"]
pub struct DSTCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `UDPE` reader - UDP Port Match Enable"]
pub struct UDPE_R(crate::FieldReader<bool, bool>);
impl UDPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UDPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDPE` writer - UDP Port Match Enable"]
pub struct UDPE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&self) -> QNB_R {
        QNB_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:11 - Differentiated Services or Traffic Class Match"]
    #[inline(always)]
    pub fn dstcm(&self) -> DSTCM_R {
        DSTCM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:27 - UDP Port Match"]
    #[inline(always)]
    pub fn udpm(&self) -> UDPM_R {
        UDPM_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28 - Differentiated Services or Traffic Class Match Enable"]
    #[inline(always)]
    pub fn dstce(&self) -> DSTCE_R {
        DSTCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - UDP Port Match Enable"]
    #[inline(always)]
    pub fn udpe(&self) -> UDPE_R {
        UDPE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&mut self) -> QNB_W {
        QNB_W { w: self }
    }
    #[doc = "Bits 4:11 - Differentiated Services or Traffic Class Match"]
    #[inline(always)]
    pub fn dstcm(&mut self) -> DSTCM_W {
        DSTCM_W { w: self }
    }
    #[doc = "Bits 12:27 - UDP Port Match"]
    #[inline(always)]
    pub fn udpm(&mut self) -> UDPM_W {
        UDPM_W { w: self }
    }
    #[doc = "Bit 28 - Differentiated Services or Traffic Class Match Enable"]
    #[inline(always)]
    pub fn dstce(&mut self) -> DSTCE_W {
        DSTCE_W { w: self }
    }
    #[doc = "Bit 29 - UDP Port Match Enable"]
    #[inline(always)]
    pub fn udpe(&mut self) -> UDPE_W {
        UDPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Screening Type 1 Register Priority Queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_st1rpq](index.html) module"]
pub struct GMAC_ST1RPQ_SPEC;
impl crate::RegisterSpec for GMAC_ST1RPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_st1rpq::R](R) reader structure"]
impl crate::Readable for GMAC_ST1RPQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmac_st1rpq::W](W) writer structure"]
impl crate::Writable for GMAC_ST1RPQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMAC_ST1RPQ[%s]
to value 0"]
impl crate::Resettable for GMAC_ST1RPQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
