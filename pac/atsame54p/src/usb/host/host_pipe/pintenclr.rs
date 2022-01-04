#[doc = "Register `PINTENCLR` reader"]
pub struct R(crate::R<PINTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINTENCLR` writer"]
pub struct W(crate::W<PINTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINTENCLR_SPEC>;
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
impl From<crate::W<PINTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Disable"]
pub struct TRCPT0_R(crate::FieldReader<bool, bool>);
impl TRCPT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRCPT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCPT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Disable"]
pub struct TRCPT0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCPT0_W<'a> {
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
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Disable"]
pub struct TRCPT1_R(crate::FieldReader<bool, bool>);
impl TRCPT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRCPT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCPT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Disable"]
pub struct TRCPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCPT1_W<'a> {
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
#[doc = "Field `TRFAIL` reader - Error Flow Interrupt Disable"]
pub struct TRFAIL_R(crate::FieldReader<bool, bool>);
impl TRFAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRFAIL` writer - Error Flow Interrupt Disable"]
pub struct TRFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRFAIL_W<'a> {
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
#[doc = "Field `PERR` reader - Pipe Error Interrupt Disable"]
pub struct PERR_R(crate::FieldReader<bool, bool>);
impl PERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERR` writer - Pipe Error Interrupt Disable"]
pub struct PERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERR_W<'a> {
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
#[doc = "Field `TXSTP` reader - Transmit Setup Interrupt Disable"]
pub struct TXSTP_R(crate::FieldReader<bool, bool>);
impl TXSTP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXSTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTP` writer - Transmit Setup Interrupt Disable"]
pub struct TXSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTP_W<'a> {
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
#[doc = "Field `STALL` reader - Stall Inetrrupt Disable"]
pub struct STALL_R(crate::FieldReader<bool, bool>);
impl STALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL` writer - Stall Inetrrupt Disable"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer Complete 0 Disable"]
    #[inline(always)]
    pub fn trcpt0(&self) -> TRCPT0_R {
        TRCPT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Disable"]
    #[inline(always)]
    pub fn trcpt1(&self) -> TRCPT1_R {
        TRCPT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error Flow Interrupt Disable"]
    #[inline(always)]
    pub fn trfail(&self) -> TRFAIL_R {
        TRFAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Disable"]
    #[inline(always)]
    pub fn txstp(&self) -> TXSTP_R {
        TXSTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stall Inetrrupt Disable"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Disable"]
    #[inline(always)]
    pub fn trcpt0(&mut self) -> TRCPT0_W {
        TRCPT0_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete 1 Disable"]
    #[inline(always)]
    pub fn trcpt1(&mut self) -> TRCPT1_W {
        TRCPT1_W { w: self }
    }
    #[doc = "Bit 2 - Error Flow Interrupt Disable"]
    #[inline(always)]
    pub fn trfail(&mut self) -> TRFAIL_W {
        TRFAIL_W { w: self }
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Disable"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Setup Interrupt Disable"]
    #[inline(always)]
    pub fn txstp(&mut self) -> TXSTP_W {
        TXSTP_W { w: self }
    }
    #[doc = "Bit 5 - Stall Inetrrupt Disable"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HOST_PIPE Pipe Interrupt Flag Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintenclr](index.html) module"]
pub struct PINTENCLR_SPEC;
impl crate::RegisterSpec for PINTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pintenclr::R](R) reader structure"]
impl crate::Readable for PINTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pintenclr::W](W) writer structure"]
impl crate::Writable for PINTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINTENCLR to value 0"]
impl crate::Resettable for PINTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
