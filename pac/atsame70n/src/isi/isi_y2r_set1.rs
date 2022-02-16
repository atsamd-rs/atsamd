#[doc = "Register `ISI_Y2R_SET1` reader"]
pub struct R(crate::R<ISI_Y2R_SET1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_Y2R_SET1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_Y2R_SET1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_Y2R_SET1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISI_Y2R_SET1` writer"]
pub struct W(crate::W<ISI_Y2R_SET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_Y2R_SET1_SPEC>;
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
impl From<crate::W<ISI_Y2R_SET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_Y2R_SET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C4` reader - Color Space Conversion Matrix Coefficient C4"]
pub struct C4_R(crate::FieldReader<u16, u16>);
impl C4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        C4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C4` writer - Color Space Conversion Matrix Coefficient C4"]
pub struct C4_W<'a> {
    w: &'a mut W,
}
impl<'a> C4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `Yoff` reader - Color Space Conversion Luminance Default Offset"]
pub struct YOFF_R(crate::FieldReader<bool, bool>);
impl YOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        YOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Yoff` writer - Color Space Conversion Luminance Default Offset"]
pub struct YOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> YOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `Croff` reader - Color Space Conversion Red Chrominance Default Offset"]
pub struct CROFF_R(crate::FieldReader<bool, bool>);
impl CROFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CROFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CROFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Croff` writer - Color Space Conversion Red Chrominance Default Offset"]
pub struct CROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CROFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `Cboff` reader - Color Space Conversion Blue Chrominance Default Offset"]
pub struct CBOFF_R(crate::FieldReader<bool, bool>);
impl CBOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cboff` writer - Color Space Conversion Blue Chrominance Default Offset"]
pub struct CBOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CBOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&self) -> C4_R {
        C4_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - Color Space Conversion Luminance Default Offset"]
    #[inline(always)]
    pub fn yoff(&self) -> YOFF_R {
        YOFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Color Space Conversion Red Chrominance Default Offset"]
    #[inline(always)]
    pub fn croff(&self) -> CROFF_R {
        CROFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Color Space Conversion Blue Chrominance Default Offset"]
    #[inline(always)]
    pub fn cboff(&self) -> CBOFF_R {
        CBOFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&mut self) -> C4_W {
        C4_W { w: self }
    }
    #[doc = "Bit 12 - Color Space Conversion Luminance Default Offset"]
    #[inline(always)]
    pub fn yoff(&mut self) -> YOFF_W {
        YOFF_W { w: self }
    }
    #[doc = "Bit 13 - Color Space Conversion Red Chrominance Default Offset"]
    #[inline(always)]
    pub fn croff(&mut self) -> CROFF_W {
        CROFF_W { w: self }
    }
    #[doc = "Bit 14 - Color Space Conversion Blue Chrominance Default Offset"]
    #[inline(always)]
    pub fn cboff(&mut self) -> CBOFF_W {
        CBOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_y2r_set1](index.html) module"]
pub struct ISI_Y2R_SET1_SPEC;
impl crate::RegisterSpec for ISI_Y2R_SET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_y2r_set1::R](R) reader structure"]
impl crate::Readable for ISI_Y2R_SET1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isi_y2r_set1::W](W) writer structure"]
impl crate::Writable for ISI_Y2R_SET1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_Y2R_SET1 to value 0"]
impl crate::Resettable for ISI_Y2R_SET1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
