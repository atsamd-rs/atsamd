#[doc = "Register `MCAN_RXF1C` reader"]
pub struct R(crate::R<MCAN_RXF1C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_RXF1C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_RXF1C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_RXF1C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_RXF1C` writer"]
pub struct W(crate::W<MCAN_RXF1C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_RXF1C_SPEC>;
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
impl From<crate::W<MCAN_RXF1C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_RXF1C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F1SA` reader - Receive FIFO 1 Start Address"]
pub struct F1SA_R(crate::FieldReader<u16, u16>);
impl F1SA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        F1SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1SA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1SA` writer - Receive FIFO 1 Start Address"]
pub struct F1SA_W<'a> {
    w: &'a mut W,
}
impl<'a> F1SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `F1S` reader - Receive FIFO 1 Start Address"]
pub struct F1S_R(crate::FieldReader<u8, u8>);
impl F1S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F1S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1S` writer - Receive FIFO 1 Start Address"]
pub struct F1S_W<'a> {
    w: &'a mut W,
}
impl<'a> F1S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `F1WM` reader - Receive FIFO 1 Watermark"]
pub struct F1WM_R(crate::FieldReader<u8, u8>);
impl F1WM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        F1WM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1WM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1WM` writer - Receive FIFO 1 Watermark"]
pub struct F1WM_W<'a> {
    w: &'a mut W,
}
impl<'a> F1WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
#[doc = "Field `F1OM` reader - FIFO 1 Operation Mode"]
pub struct F1OM_R(crate::FieldReader<bool, bool>);
impl F1OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        F1OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F1OM` writer - FIFO 1 Operation Mode"]
pub struct F1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> F1OM_W<'a> {
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
    #[doc = "Bits 2:15 - Receive FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1sa(&self) -> F1SA_R {
        F1SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Receive FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1s(&self) -> F1S_R {
        F1S_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Receive FIFO 1 Watermark"]
    #[inline(always)]
    pub fn f1wm(&self) -> F1WM_R {
        F1WM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 1 Operation Mode"]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Receive FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1sa(&mut self) -> F1SA_W {
        F1SA_W { w: self }
    }
    #[doc = "Bits 16:22 - Receive FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1s(&mut self) -> F1S_W {
        F1S_W { w: self }
    }
    #[doc = "Bits 24:30 - Receive FIFO 1 Watermark"]
    #[inline(always)]
    pub fn f1wm(&mut self) -> F1WM_W {
        F1WM_W { w: self }
    }
    #[doc = "Bit 31 - FIFO 1 Operation Mode"]
    #[inline(always)]
    pub fn f1om(&mut self) -> F1OM_W {
        F1OM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO 1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxf1c](index.html) module"]
pub struct MCAN_RXF1C_SPEC;
impl crate::RegisterSpec for MCAN_RXF1C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_rxf1c::R](R) reader structure"]
impl crate::Readable for MCAN_RXF1C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_rxf1c::W](W) writer structure"]
impl crate::Writable for MCAN_RXF1C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_RXF1C to value 0"]
impl crate::Resettable for MCAN_RXF1C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
