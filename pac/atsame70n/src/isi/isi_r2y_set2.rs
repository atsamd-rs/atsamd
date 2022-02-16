#[doc = "Register `ISI_R2Y_SET2` reader"]
pub struct R(crate::R<ISI_R2Y_SET2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_R2Y_SET2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_R2Y_SET2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_R2Y_SET2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISI_R2Y_SET2` writer"]
pub struct W(crate::W<ISI_R2Y_SET2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_R2Y_SET2_SPEC>;
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
impl From<crate::W<ISI_R2Y_SET2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_R2Y_SET2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C6` reader - Color Space Conversion Matrix Coefficient C6"]
pub struct C6_R(crate::FieldReader<u8, u8>);
impl C6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        C6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C6` writer - Color Space Conversion Matrix Coefficient C6"]
pub struct C6_W<'a> {
    w: &'a mut W,
}
impl<'a> C6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `C7` reader - Color Space Conversion Matrix Coefficient C7"]
pub struct C7_R(crate::FieldReader<u8, u8>);
impl C7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        C7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C7` writer - Color Space Conversion Matrix Coefficient C7"]
pub struct C7_W<'a> {
    w: &'a mut W,
}
impl<'a> C7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `C8` reader - Color Space Conversion Matrix Coefficient C8"]
pub struct C8_R(crate::FieldReader<u8, u8>);
impl C8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        C8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C8` writer - Color Space Conversion Matrix Coefficient C8"]
pub struct C8_W<'a> {
    w: &'a mut W,
}
impl<'a> C8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `Boff` reader - Color Space Conversion Blue Component Offset"]
pub struct BOFF_R(crate::FieldReader<bool, bool>);
impl BOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Boff` writer - Color Space Conversion Blue Component Offset"]
pub struct BOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C6"]
    #[inline(always)]
    pub fn c6(&self) -> C6_R {
        C6_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C7"]
    #[inline(always)]
    pub fn c7(&self) -> C7_R {
        C7_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C8"]
    #[inline(always)]
    pub fn c8(&self) -> C8_R {
        C8_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Blue Component Offset"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C6"]
    #[inline(always)]
    pub fn c6(&mut self) -> C6_W {
        C6_W { w: self }
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C7"]
    #[inline(always)]
    pub fn c7(&mut self) -> C7_W {
        C7_W { w: self }
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C8"]
    #[inline(always)]
    pub fn c8(&mut self) -> C8_W {
        C8_W { w: self }
    }
    #[doc = "Bit 24 - Color Space Conversion Blue Component Offset"]
    #[inline(always)]
    pub fn boff(&mut self) -> BOFF_W {
        BOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_r2y_set2](index.html) module"]
pub struct ISI_R2Y_SET2_SPEC;
impl crate::RegisterSpec for ISI_R2Y_SET2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_r2y_set2::R](R) reader structure"]
impl crate::Readable for ISI_R2Y_SET2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isi_r2y_set2::W](W) writer structure"]
impl crate::Writable for ISI_R2Y_SET2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_R2Y_SET2 to value 0"]
impl crate::Resettable for ISI_R2Y_SET2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
