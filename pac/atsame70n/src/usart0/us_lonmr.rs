#[doc = "Register `US_LONMR` reader"]
pub struct R(crate::R<US_LONMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_LONMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_LONMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_LONMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_LONMR` writer"]
pub struct W(crate::W<US_LONMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_LONMR_SPEC>;
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
impl From<crate::W<US_LONMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_LONMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMT` reader - LON comm_type Parameter Value"]
pub struct COMMT_R(crate::FieldReader<bool, bool>);
impl COMMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMT` writer - LON comm_type Parameter Value"]
pub struct COMMT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `COLDET` reader - LON Collision Detection Feature"]
pub struct COLDET_R(crate::FieldReader<bool, bool>);
impl COLDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COLDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLDET` writer - LON Collision Detection Feature"]
pub struct COLDET_W<'a> {
    w: &'a mut W,
}
impl<'a> COLDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TCOL` reader - Terminate Frame upon Collision Notification"]
pub struct TCOL_R(crate::FieldReader<bool, bool>);
impl TCOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCOL` writer - Terminate Frame upon Collision Notification"]
pub struct TCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CDTAIL` reader - LON Collision Detection on Frame Tail"]
pub struct CDTAIL_R(crate::FieldReader<bool, bool>);
impl CDTAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CDTAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDTAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDTAIL` writer - LON Collision Detection on Frame Tail"]
pub struct CDTAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DMAM` reader - LON DMA Mode"]
pub struct DMAM_R(crate::FieldReader<bool, bool>);
impl DMAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAM` writer - LON DMA Mode"]
pub struct DMAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `LCDS` reader - LON Collision Detection Source"]
pub struct LCDS_R(crate::FieldReader<bool, bool>);
impl LCDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDS` writer - LON Collision Detection Source"]
pub struct LCDS_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `EOFS` reader - End of Frame Condition Size"]
pub struct EOFS_R(crate::FieldReader<u8, u8>);
impl EOFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EOFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOFS` writer - End of Frame Condition Size"]
pub struct EOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LON comm_type Parameter Value"]
    #[inline(always)]
    pub fn commt(&self) -> COMMT_R {
        COMMT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LON Collision Detection Feature"]
    #[inline(always)]
    pub fn coldet(&self) -> COLDET_R {
        COLDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Terminate Frame upon Collision Notification"]
    #[inline(always)]
    pub fn tcol(&self) -> TCOL_R {
        TCOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LON Collision Detection on Frame Tail"]
    #[inline(always)]
    pub fn cdtail(&self) -> CDTAIL_R {
        CDTAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LON DMA Mode"]
    #[inline(always)]
    pub fn dmam(&self) -> DMAM_R {
        DMAM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LON Collision Detection Source"]
    #[inline(always)]
    pub fn lcds(&self) -> LCDS_R {
        LCDS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - End of Frame Condition Size"]
    #[inline(always)]
    pub fn eofs(&self) -> EOFS_R {
        EOFS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LON comm_type Parameter Value"]
    #[inline(always)]
    pub fn commt(&mut self) -> COMMT_W {
        COMMT_W { w: self }
    }
    #[doc = "Bit 1 - LON Collision Detection Feature"]
    #[inline(always)]
    pub fn coldet(&mut self) -> COLDET_W {
        COLDET_W { w: self }
    }
    #[doc = "Bit 2 - Terminate Frame upon Collision Notification"]
    #[inline(always)]
    pub fn tcol(&mut self) -> TCOL_W {
        TCOL_W { w: self }
    }
    #[doc = "Bit 3 - LON Collision Detection on Frame Tail"]
    #[inline(always)]
    pub fn cdtail(&mut self) -> CDTAIL_W {
        CDTAIL_W { w: self }
    }
    #[doc = "Bit 4 - LON DMA Mode"]
    #[inline(always)]
    pub fn dmam(&mut self) -> DMAM_W {
        DMAM_W { w: self }
    }
    #[doc = "Bit 5 - LON Collision Detection Source"]
    #[inline(always)]
    pub fn lcds(&mut self) -> LCDS_W {
        LCDS_W { w: self }
    }
    #[doc = "Bits 16:23 - End of Frame Condition Size"]
    #[inline(always)]
    pub fn eofs(&mut self) -> EOFS_W {
        EOFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LON Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonmr](index.html) module"]
pub struct US_LONMR_SPEC;
impl crate::RegisterSpec for US_LONMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_lonmr::R](R) reader structure"]
impl crate::Readable for US_LONMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_lonmr::W](W) writer structure"]
impl crate::Writable for US_LONMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_LONMR to value 0"]
impl crate::Resettable for US_LONMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
