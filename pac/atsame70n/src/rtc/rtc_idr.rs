#[doc = "Register `RTC_IDR` writer"]
pub struct W(crate::W<RTC_IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_IDR_SPEC>;
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
impl From<crate::W<RTC_IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACKDIS` writer - Acknowledge Update Interrupt Disable"]
pub struct ACKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKDIS_W<'a> {
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
#[doc = "Field `ALRDIS` writer - Alarm Interrupt Disable"]
pub struct ALRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRDIS_W<'a> {
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
#[doc = "Field `SECDIS` writer - Second Event Interrupt Disable"]
pub struct SECDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SECDIS_W<'a> {
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
#[doc = "Field `TIMDIS` writer - Time Event Interrupt Disable"]
pub struct TIMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMDIS_W<'a> {
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
#[doc = "Field `CALDIS` writer - Calendar Event Interrupt Disable"]
pub struct CALDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CALDIS_W<'a> {
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
#[doc = "Field `TDERRDIS` writer - Time and/or Date Error Interrupt Disable"]
pub struct TDERRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TDERRDIS_W<'a> {
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
    #[doc = "Bit 0 - Acknowledge Update Interrupt Disable"]
    #[inline(always)]
    pub fn ackdis(&mut self) -> ACKDIS_W {
        ACKDIS_W { w: self }
    }
    #[doc = "Bit 1 - Alarm Interrupt Disable"]
    #[inline(always)]
    pub fn alrdis(&mut self) -> ALRDIS_W {
        ALRDIS_W { w: self }
    }
    #[doc = "Bit 2 - Second Event Interrupt Disable"]
    #[inline(always)]
    pub fn secdis(&mut self) -> SECDIS_W {
        SECDIS_W { w: self }
    }
    #[doc = "Bit 3 - Time Event Interrupt Disable"]
    #[inline(always)]
    pub fn timdis(&mut self) -> TIMDIS_W {
        TIMDIS_W { w: self }
    }
    #[doc = "Bit 4 - Calendar Event Interrupt Disable"]
    #[inline(always)]
    pub fn caldis(&mut self) -> CALDIS_W {
        CALDIS_W { w: self }
    }
    #[doc = "Bit 5 - Time and/or Date Error Interrupt Disable"]
    #[inline(always)]
    pub fn tderrdis(&mut self) -> TDERRDIS_W {
        TDERRDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_idr](index.html) module"]
pub struct RTC_IDR_SPEC;
impl crate::RegisterSpec for RTC_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rtc_idr::W](W) writer structure"]
impl crate::Writable for RTC_IDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_IDR to value 0"]
impl crate::Resettable for RTC_IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
