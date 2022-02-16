#[doc = "Register `RTC_SCCR` writer"]
pub struct W(crate::W<RTC_SCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SCCR_SPEC>;
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
impl From<crate::W<RTC_SCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACKCLR` writer - Acknowledge Clear"]
pub struct ACKCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ALRCLR` writer - Alarm Clear"]
pub struct ALRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SECCLR` writer - Second Clear"]
pub struct SECCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TIMCLR` writer - Time Clear"]
pub struct TIMCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CALCLR` writer - Calendar Clear"]
pub struct CALCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CALCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TDERRCLR` writer - Time and/or Date Free Running Error Clear"]
pub struct TDERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TDERRCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Acknowledge Clear"]
    #[inline(always)]
    pub fn ackclr(&mut self) -> ACKCLR_W {
        ACKCLR_W { w: self }
    }
    #[doc = "Bit 1 - Alarm Clear"]
    #[inline(always)]
    pub fn alrclr(&mut self) -> ALRCLR_W {
        ALRCLR_W { w: self }
    }
    #[doc = "Bit 2 - Second Clear"]
    #[inline(always)]
    pub fn secclr(&mut self) -> SECCLR_W {
        SECCLR_W { w: self }
    }
    #[doc = "Bit 3 - Time Clear"]
    #[inline(always)]
    pub fn timclr(&mut self) -> TIMCLR_W {
        TIMCLR_W { w: self }
    }
    #[doc = "Bit 4 - Calendar Clear"]
    #[inline(always)]
    pub fn calclr(&mut self) -> CALCLR_W {
        CALCLR_W { w: self }
    }
    #[doc = "Bit 5 - Time and/or Date Free Running Error Clear"]
    #[inline(always)]
    pub fn tderrclr(&mut self) -> TDERRCLR_W {
        TDERRCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Clear Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sccr](index.html) module"]
pub struct RTC_SCCR_SPEC;
impl crate::RegisterSpec for RTC_SCCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rtc_sccr::W](W) writer structure"]
impl crate::Writable for RTC_SCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SCCR to value 0"]
impl crate::Resettable for RTC_SCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
