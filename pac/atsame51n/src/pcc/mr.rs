#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCEN` reader - Parallel Capture Enable"]
pub struct PCEN_R(crate::FieldReader<bool, bool>);
impl PCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCEN` writer - Parallel Capture Enable"]
pub struct PCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCEN_W<'a> {
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
#[doc = "Field `DSIZE` reader - Data size"]
pub struct DSIZE_R(crate::FieldReader<u8, u8>);
impl DSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSIZE` writer - Data size"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `SCALE` reader - Scale data"]
pub struct SCALE_R(crate::FieldReader<bool, bool>);
impl SCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCALE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCALE` writer - Scale data"]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `ALWYS` reader - Always Sampling"]
pub struct ALWYS_R(crate::FieldReader<bool, bool>);
impl ALWYS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALWYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALWYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALWYS` writer - Always Sampling"]
pub struct ALWYS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWYS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `HALFS` reader - Half Sampling"]
pub struct HALFS_R(crate::FieldReader<bool, bool>);
impl HALFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALFS` writer - Half Sampling"]
pub struct HALFS_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FRSTS` reader - First sample"]
pub struct FRSTS_R(crate::FieldReader<bool, bool>);
impl FRSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRSTS` writer - First sample"]
pub struct FRSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `ISIZE` reader - Input Data Size"]
pub struct ISIZE_R(crate::FieldReader<u8, u8>);
impl ISIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ISIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISIZE` writer - Input Data Size"]
pub struct ISIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `CID` reader - Clear If Disabled"]
pub struct CID_R(crate::FieldReader<u8, u8>);
impl CID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CID` writer - Clear If Disabled"]
pub struct CID_W<'a> {
    w: &'a mut W,
}
impl<'a> CID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Parallel Capture Enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Data size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Scale data"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Always Sampling"]
    #[inline(always)]
    pub fn alwys(&self) -> ALWYS_R {
        ALWYS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Half Sampling"]
    #[inline(always)]
    pub fn halfs(&self) -> HALFS_R {
        HALFS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - First sample"]
    #[inline(always)]
    pub fn frsts(&self) -> FRSTS_R {
        FRSTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Input Data Size"]
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 30:31 - Clear If Disabled"]
    #[inline(always)]
    pub fn cid(&self) -> CID_R {
        CID_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Parallel Capture Enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Data size"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bit 8 - Scale data"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Bit 9 - Always Sampling"]
    #[inline(always)]
    pub fn alwys(&mut self) -> ALWYS_W {
        ALWYS_W { w: self }
    }
    #[doc = "Bit 10 - Half Sampling"]
    #[inline(always)]
    pub fn halfs(&mut self) -> HALFS_W {
        HALFS_W { w: self }
    }
    #[doc = "Bit 11 - First sample"]
    #[inline(always)]
    pub fn frsts(&mut self) -> FRSTS_W {
        FRSTS_W { w: self }
    }
    #[doc = "Bits 16:18 - Input Data Size"]
    #[inline(always)]
    pub fn isize(&mut self) -> ISIZE_W {
        ISIZE_W { w: self }
    }
    #[doc = "Bits 30:31 - Clear If Disabled"]
    #[inline(always)]
    pub fn cid(&mut self) -> CID_W {
        CID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
