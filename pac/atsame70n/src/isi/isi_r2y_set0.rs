#[doc = "Register `ISI_R2Y_SET0` reader"]
pub struct R(crate::R<ISI_R2Y_SET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISI_R2Y_SET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISI_R2Y_SET0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISI_R2Y_SET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISI_R2Y_SET0` writer"]
pub struct W(crate::W<ISI_R2Y_SET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISI_R2Y_SET0_SPEC>;
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
impl From<crate::W<ISI_R2Y_SET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISI_R2Y_SET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C0` reader - Color Space Conversion Matrix Coefficient C0"]
pub struct C0_R(crate::FieldReader<u8, u8>);
impl C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C0` writer - Color Space Conversion Matrix Coefficient C0"]
pub struct C0_W<'a> {
    w: &'a mut W,
}
impl<'a> C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `C1` reader - Color Space Conversion Matrix Coefficient C1"]
pub struct C1_R(crate::FieldReader<u8, u8>);
impl C1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C1` writer - Color Space Conversion Matrix Coefficient C1"]
pub struct C1_W<'a> {
    w: &'a mut W,
}
impl<'a> C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `C2` reader - Color Space Conversion Matrix Coefficient C2"]
pub struct C2_R(crate::FieldReader<u8, u8>);
impl C2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        C2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2` writer - Color Space Conversion Matrix Coefficient C2"]
pub struct C2_W<'a> {
    w: &'a mut W,
}
impl<'a> C2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `Roff` reader - Color Space Conversion Red Component Offset"]
pub struct ROFF_R(crate::FieldReader<bool, bool>);
impl ROFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Roff` writer - Color Space Conversion Red Component Offset"]
pub struct ROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ROFF_W<'a> {
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
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C0"]
    #[inline(always)]
    pub fn c0(&self) -> C0_R {
        C0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C1"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C2"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Red Component Offset"]
    #[inline(always)]
    pub fn roff(&self) -> ROFF_R {
        ROFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C0"]
    #[inline(always)]
    pub fn c0(&mut self) -> C0_W {
        C0_W { w: self }
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C1"]
    #[inline(always)]
    pub fn c1(&mut self) -> C1_W {
        C1_W { w: self }
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C2"]
    #[inline(always)]
    pub fn c2(&mut self) -> C2_W {
        C2_W { w: self }
    }
    #[doc = "Bit 24 - Color Space Conversion Red Component Offset"]
    #[inline(always)]
    pub fn roff(&mut self) -> ROFF_W {
        ROFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_r2y_set0](index.html) module"]
pub struct ISI_R2Y_SET0_SPEC;
impl crate::RegisterSpec for ISI_R2Y_SET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isi_r2y_set0::R](R) reader structure"]
impl crate::Readable for ISI_R2Y_SET0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isi_r2y_set0::W](W) writer structure"]
impl crate::Writable for ISI_R2Y_SET0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISI_R2Y_SET0 to value 0"]
impl crate::Resettable for ISI_R2Y_SET0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
