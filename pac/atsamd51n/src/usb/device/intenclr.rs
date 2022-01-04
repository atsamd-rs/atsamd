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
#[doc = "Field `SUSPEND` reader - Suspend Interrupt Enable"]
pub struct SUSPEND_R(crate::FieldReader<bool, bool>);
impl SUSPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPEND` writer - Suspend Interrupt Enable"]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
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
#[doc = "Field `MSOF` reader - Micro Start of Frame Interrupt Enable in High Speed Mode"]
pub struct MSOF_R(crate::FieldReader<bool, bool>);
impl MSOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSOF` writer - Micro Start of Frame Interrupt Enable in High Speed Mode"]
pub struct MSOF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOF_W<'a> {
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
#[doc = "Field `SOF` reader - Start Of Frame Interrupt Enable"]
pub struct SOF_R(crate::FieldReader<bool, bool>);
impl SOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOF` writer - Start Of Frame Interrupt Enable"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `EORST` reader - End of Reset Interrupt Enable"]
pub struct EORST_R(crate::FieldReader<bool, bool>);
impl EORST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EORST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EORST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EORST` writer - End of Reset Interrupt Enable"]
pub struct EORST_W<'a> {
    w: &'a mut W,
}
impl<'a> EORST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `WAKEUP` reader - Wake Up Interrupt Enable"]
pub struct WAKEUP_R(crate::FieldReader<bool, bool>);
impl WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` writer - Wake Up Interrupt Enable"]
pub struct WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
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
#[doc = "Field `EORSM` reader - End Of Resume Interrupt Enable"]
pub struct EORSM_R(crate::FieldReader<bool, bool>);
impl EORSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EORSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EORSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EORSM` writer - End Of Resume Interrupt Enable"]
pub struct EORSM_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSM_W<'a> {
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
#[doc = "Field `UPRSM` reader - Upstream Resume Interrupt Enable"]
pub struct UPRSM_R(crate::FieldReader<bool, bool>);
impl UPRSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPRSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPRSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPRSM` writer - Upstream Resume Interrupt Enable"]
pub struct UPRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RAMACER` reader - Ram Access Interrupt Enable"]
pub struct RAMACER_R(crate::FieldReader<bool, bool>);
impl RAMACER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMACER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMACER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMACER` writer - Ram Access Interrupt Enable"]
pub struct RAMACER_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `LPMNYET` reader - Link Power Management Not Yet Interrupt Enable"]
pub struct LPMNYET_R(crate::FieldReader<bool, bool>);
impl LPMNYET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPMNYET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMNYET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMNYET` writer - Link Power Management Not Yet Interrupt Enable"]
pub struct LPMNYET_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMNYET_W<'a> {
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
#[doc = "Field `LPMSUSP` reader - Link Power Management Suspend Interrupt Enable"]
pub struct LPMSUSP_R(crate::FieldReader<bool, bool>);
impl LPMSUSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPMSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMSUSP` writer - Link Power Management Suspend Interrupt Enable"]
pub struct LPMSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMSUSP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Enable in High Speed Mode"]
    #[inline(always)]
    pub fn msof(&self) -> MSOF_R {
        MSOF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn eorst(&self) -> EORST_R {
        EORST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake Up Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn eorsm(&self) -> EORSM_R {
        EORSM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Ram Access Interrupt Enable"]
    #[inline(always)]
    pub fn ramacer(&self) -> RAMACER_R {
        RAMACER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Link Power Management Not Yet Interrupt Enable"]
    #[inline(always)]
    pub fn lpmnyet(&self) -> LPMNYET_R {
        LPMNYET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Link Power Management Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn lpmsusp(&self) -> LPMSUSP_R {
        LPMSUSP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Enable in High Speed Mode"]
    #[inline(always)]
    pub fn msof(&mut self) -> MSOF_W {
        MSOF_W { w: self }
    }
    #[doc = "Bit 2 - Start Of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 3 - End of Reset Interrupt Enable"]
    #[inline(always)]
    pub fn eorst(&mut self) -> EORST_W {
        EORST_W { w: self }
    }
    #[doc = "Bit 4 - Wake Up Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
    #[doc = "Bit 5 - End Of Resume Interrupt Enable"]
    #[inline(always)]
    pub fn eorsm(&mut self) -> EORSM_W {
        EORSM_W { w: self }
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Enable"]
    #[inline(always)]
    pub fn uprsm(&mut self) -> UPRSM_W {
        UPRSM_W { w: self }
    }
    #[doc = "Bit 7 - Ram Access Interrupt Enable"]
    #[inline(always)]
    pub fn ramacer(&mut self) -> RAMACER_W {
        RAMACER_W { w: self }
    }
    #[doc = "Bit 8 - Link Power Management Not Yet Interrupt Enable"]
    #[inline(always)]
    pub fn lpmnyet(&mut self) -> LPMNYET_W {
        LPMNYET_W { w: self }
    }
    #[doc = "Bit 9 - Link Power Management Suspend Interrupt Enable"]
    #[inline(always)]
    pub fn lpmsusp(&mut self) -> LPMSUSP_W {
        LPMSUSP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DEVICE Device Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
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
