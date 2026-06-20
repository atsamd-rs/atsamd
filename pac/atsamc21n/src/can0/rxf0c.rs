#[doc = "Register `RXF0C` reader"]
pub struct R(crate::R<RXF0C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXF0C` writer"]
pub struct W(crate::W<RXF0C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF0C_SPEC>;
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
impl From<crate::W<RXF0C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF0C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0SA` reader - Rx FIFO 0 Start Address"]
pub struct F0SA_R(crate::FieldReader<u16, u16>);
impl F0SA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        F0SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0SA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0SA` writer - Rx FIFO 0 Start Address"]
pub struct F0SA_W<'a> {
    w: &'a mut W,
}
impl<'a> F0SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `F0S` reader - Rx FIFO 0 Size"]
pub struct F0S_R(crate::FieldReader<u8, u8>);
impl F0S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F0S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0S` writer - Rx FIFO 0 Size"]
pub struct F0S_W<'a> {
    w: &'a mut W,
}
impl<'a> F0S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `F0WM` reader - Rx FIFO 0 Watermark"]
pub struct F0WM_R(crate::FieldReader<u8, u8>);
impl F0WM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F0WM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0WM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0WM` writer - Rx FIFO 0 Watermark"]
pub struct F0WM_W<'a> {
    w: &'a mut W,
}
impl<'a> F0WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
#[doc = "Field `F0OM` reader - FIFO 0 Operation Mode"]
pub struct F0OM_R(crate::FieldReader<bool, bool>);
impl F0OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F0OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0OM` writer - FIFO 0 Operation Mode"]
pub struct F0OM_W<'a> {
    w: &'a mut W,
}
impl<'a> F0OM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&mut self) -> F0SA_W {
        F0SA_W { w: self }
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&mut self) -> F0S_W {
        F0S_W { w: self }
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&mut self) -> F0WM_W {
        F0WM_W { w: self }
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode"]
    #[inline(always)]
    pub fn f0om(&mut self) -> F0OM_W {
        F0OM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx FIFO 0 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0c](index.html) module"]
pub struct RXF0C_SPEC;
impl crate::RegisterSpec for RXF0C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf0c::R](R) reader structure"]
impl crate::Readable for RXF0C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxf0c::W](W) writer structure"]
impl crate::Writable for RXF0C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXF0C to value 0"]
impl crate::Resettable for RXF0C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
