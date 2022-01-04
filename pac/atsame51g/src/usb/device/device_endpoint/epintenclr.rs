#[doc = "Register `EPINTENCLR` reader"]
pub struct R(crate::R<EPINTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPINTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPINTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPINTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPINTENCLR` writer"]
pub struct W(crate::W<EPINTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPINTENCLR_SPEC>;
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
impl From<crate::W<EPINTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPINTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRCPT0` reader - Transfer Complete 0 Interrupt Disable"]
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
#[doc = "Field `TRCPT0` writer - Transfer Complete 0 Interrupt Disable"]
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
#[doc = "Field `TRCPT1` reader - Transfer Complete 1 Interrupt Disable"]
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
#[doc = "Field `TRCPT1` writer - Transfer Complete 1 Interrupt Disable"]
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
#[doc = "Field `TRFAIL0` reader - Error Flow 0 Interrupt Disable"]
pub struct TRFAIL0_R(crate::FieldReader<bool, bool>);
impl TRFAIL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRFAIL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRFAIL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRFAIL0` writer - Error Flow 0 Interrupt Disable"]
pub struct TRFAIL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRFAIL0_W<'a> {
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
#[doc = "Field `TRFAIL1` reader - Error Flow 1 Interrupt Disable"]
pub struct TRFAIL1_R(crate::FieldReader<bool, bool>);
impl TRFAIL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRFAIL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRFAIL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRFAIL1` writer - Error Flow 1 Interrupt Disable"]
pub struct TRFAIL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRFAIL1_W<'a> {
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
#[doc = "Field `RXSTP` reader - Received Setup Interrupt Disable"]
pub struct RXSTP_R(crate::FieldReader<bool, bool>);
impl RXSTP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXSTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTP` writer - Received Setup Interrupt Disable"]
pub struct RXSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTP_W<'a> {
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
#[doc = "Field `STALL0` reader - Stall 0 In/Out Interrupt Disable"]
pub struct STALL0_R(crate::FieldReader<bool, bool>);
impl STALL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL0` writer - Stall 0 In/Out Interrupt Disable"]
pub struct STALL0_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL0_W<'a> {
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
#[doc = "Field `STALL1` reader - Stall 1 In/Out Interrupt Disable"]
pub struct STALL1_R(crate::FieldReader<bool, bool>);
impl STALL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL1` writer - Stall 1 In/Out Interrupt Disable"]
pub struct STALL1_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt0(&self) -> TRCPT0_R {
        TRCPT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt1(&self) -> TRCPT1_R {
        TRCPT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail0(&self) -> TRFAIL0_R {
        TRFAIL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail1(&self) -> TRFAIL1_R {
        TRFAIL1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Received Setup Interrupt Disable"]
    #[inline(always)]
    pub fn rxstp(&self) -> RXSTP_R {
        RXSTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stall 0 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall0(&self) -> STALL0_R {
        STALL0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stall 1 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall1(&self) -> STALL1_R {
        STALL1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt0(&mut self) -> TRCPT0_W {
        TRCPT0_W { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trcpt1(&mut self) -> TRCPT1_W {
        TRCPT1_W { w: self }
    }
    #[doc = "Bit 2 - Error Flow 0 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail0(&mut self) -> TRFAIL0_W {
        TRFAIL0_W { w: self }
    }
    #[doc = "Bit 3 - Error Flow 1 Interrupt Disable"]
    #[inline(always)]
    pub fn trfail1(&mut self) -> TRFAIL1_W {
        TRFAIL1_W { w: self }
    }
    #[doc = "Bit 4 - Received Setup Interrupt Disable"]
    #[inline(always)]
    pub fn rxstp(&mut self) -> RXSTP_W {
        RXSTP_W { w: self }
    }
    #[doc = "Bit 5 - Stall 0 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall0(&mut self) -> STALL0_W {
        STALL0_W { w: self }
    }
    #[doc = "Bit 6 - Stall 1 In/Out Interrupt Disable"]
    #[inline(always)]
    pub fn stall1(&mut self) -> STALL1_W {
        STALL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE_ENDPOINT End Point Interrupt Clear Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintenclr](index.html) module"]
pub struct EPINTENCLR_SPEC;
impl crate::RegisterSpec for EPINTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [epintenclr::R](R) reader structure"]
impl crate::Readable for EPINTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epintenclr::W](W) writer structure"]
impl crate::Writable for EPINTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPINTENCLR to value 0"]
impl crate::Resettable for EPINTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
