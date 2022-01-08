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
#[doc = "Field `RXRDY0` reader - Receive Ready 0 Interrupt Enable"]
pub struct RXRDY0_R(crate::FieldReader<bool, bool>);
impl RXRDY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRDY0` writer - Receive Ready 0 Interrupt Enable"]
pub struct RXRDY0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRDY0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `RXRDY1` reader - Receive Ready 1 Interrupt Enable"]
pub struct RXRDY1_R(crate::FieldReader<bool, bool>);
impl RXRDY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRDY1` writer - Receive Ready 1 Interrupt Enable"]
pub struct RXRDY1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRDY1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RXOR0` reader - Receive Overrun 0 Interrupt Enable"]
pub struct RXOR0_R(crate::FieldReader<bool, bool>);
impl RXOR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOR0` writer - Receive Overrun 0 Interrupt Enable"]
pub struct RXOR0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RXOR1` reader - Receive Overrun 1 Interrupt Enable"]
pub struct RXOR1_R(crate::FieldReader<bool, bool>);
impl RXOR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOR1` writer - Receive Overrun 1 Interrupt Enable"]
pub struct RXOR1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TXRDY0` reader - Transmit Ready 0 Interrupt Enable"]
pub struct TXRDY0_R(crate::FieldReader<bool, bool>);
impl TXRDY0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY0` writer - Transmit Ready 0 Interrupt Enable"]
pub struct TXRDY0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TXRDY1` reader - Transmit Ready 1 Interrupt Enable"]
pub struct TXRDY1_R(crate::FieldReader<bool, bool>);
impl TXRDY1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY1` writer - Transmit Ready 1 Interrupt Enable"]
pub struct TXRDY1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TXUR0` reader - Transmit Underrun 0 Interrupt Enable"]
pub struct TXUR0_R(crate::FieldReader<bool, bool>);
impl TXUR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXUR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUR0` writer - Transmit Underrun 0 Interrupt Enable"]
pub struct TXUR0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TXUR1` reader - Transmit Underrun 1 Interrupt Enable"]
pub struct TXUR1_R(crate::FieldReader<bool, bool>);
impl TXUR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXUR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUR1` writer - Transmit Underrun 1 Interrupt Enable"]
pub struct TXUR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy0(&self) -> RXRDY0_R {
        RXRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy1(&self) -> RXRDY1_R {
        RXRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor0(&self) -> RXOR0_R {
        RXOR0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor1(&self) -> RXOR1_R {
        RXOR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy0(&self) -> TXRDY0_R {
        TXRDY0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy1(&self) -> TXRDY1_R {
        TXRDY1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txur0(&self) -> TXUR0_R {
        TXUR0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txur1(&self) -> TXUR1_R {
        TXUR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy0(&mut self) -> RXRDY0_W {
        RXRDY0_W { w: self }
    }
    #[doc = "Bit 1 - Receive Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy1(&mut self) -> RXRDY1_W {
        RXRDY1_W { w: self }
    }
    #[doc = "Bit 4 - Receive Overrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor0(&mut self) -> RXOR0_W {
        RXOR0_W { w: self }
    }
    #[doc = "Bit 5 - Receive Overrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn rxor1(&mut self) -> RXOR1_W {
        RXOR1_W { w: self }
    }
    #[doc = "Bit 8 - Transmit Ready 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy0(&mut self) -> TXRDY0_W {
        TXRDY0_W { w: self }
    }
    #[doc = "Bit 9 - Transmit Ready 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy1(&mut self) -> TXRDY1_W {
        TXRDY1_W { w: self }
    }
    #[doc = "Bit 12 - Transmit Underrun 0 Interrupt Enable"]
    #[inline(always)]
    pub fn txur0(&mut self) -> TXUR0_W {
        TXUR0_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Underrun 1 Interrupt Enable"]
    #[inline(always)]
    pub fn txur1(&mut self) -> TXUR1_W {
        TXUR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u16;
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
