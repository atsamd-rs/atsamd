#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRE` reader - Data Register Empty Interrupt Disable"]
pub struct DRE_R(crate::FieldReader<bool, bool>);
impl DRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRE` writer - Data Register Empty Interrupt Disable"]
pub struct DRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `TXC` reader - Transmit Complete Interrupt Disable"]
pub struct TXC_R(crate::FieldReader<bool, bool>);
impl TXC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXC` writer - Transmit Complete Interrupt Disable"]
pub struct TXC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RXC` reader - Receive Complete Interrupt Disable"]
pub struct RXC_R(crate::FieldReader<bool, bool>);
impl RXC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXC` writer - Receive Complete Interrupt Disable"]
pub struct RXC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RXS` reader - Receive Start Interrupt Disable"]
pub struct RXS_R(crate::FieldReader<bool, bool>);
impl RXS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXS` writer - Receive Start Interrupt Disable"]
pub struct RXS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CTSIC` reader - Clear To Send Input Change Interrupt Disable"]
pub struct CTSIC_R(crate::FieldReader<bool, bool>);
impl CTSIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSIC` writer - Clear To Send Input Change Interrupt Disable"]
pub struct CTSIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RXBRK` reader - Break Received Interrupt Disable"]
pub struct RXBRK_R(crate::FieldReader<bool, bool>);
impl RXBRK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXBRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBRK` writer - Break Received Interrupt Disable"]
pub struct RXBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBRK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ERROR` reader - Combined Error Interrupt Disable"]
pub struct ERROR_R(crate::FieldReader<bool, bool>);
impl ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR` writer - Combined Error Interrupt Disable"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Data Register Empty Interrupt Disable"]
    #[inline(always)]
    pub fn dre(&self) -> DRE_R {
        DRE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt Disable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Complete Interrupt Disable"]
    #[inline(always)]
    pub fn rxc(&self) -> RXC_R {
        RXC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive Start Interrupt Disable"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear To Send Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn ctsic(&self) -> CTSIC_R {
        CTSIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Break Received Interrupt Disable"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Register Empty Interrupt Disable"]
    #[inline(always)]
    pub fn dre(&mut self) -> DRE_W {
        DRE_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Complete Interrupt Disable"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W { w: self }
    }
    #[doc = "Bit 2 - Receive Complete Interrupt Disable"]
    #[inline(always)]
    pub fn rxc(&mut self) -> RXC_W {
        RXC_W { w: self }
    }
    #[doc = "Bit 3 - Receive Start Interrupt Disable"]
    #[inline(always)]
    pub fn rxs(&mut self) -> RXS_W {
        RXS_W { w: self }
    }
    #[doc = "Bit 4 - Clear To Send Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn ctsic(&mut self) -> CTSIC_W {
        CTSIC_W { w: self }
    }
    #[doc = "Bit 5 - Break Received Interrupt Disable"]
    #[inline(always)]
    pub fn rxbrk(&mut self) -> RXBRK_W {
        RXBRK_W { w: self }
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
