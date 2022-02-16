#[doc = "Register `USBHS_SR` reader"]
pub struct R(crate::R<USBHS_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDERRI` reader - Remote Device Connection Error Interrupt (Host mode only)"]
pub struct RDERRI_R(crate::FieldReader<bool, bool>);
impl RDERRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDERRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDERRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Speed Status (Device mode only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: Full-Speed mode"]
    FULL_SPEED = 0,
    #[doc = "1: High-Speed mode"]
    HIGH_SPEED = 1,
    #[doc = "2: Low-Speed mode"]
    LOW_SPEED = 2,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPEED` reader - Speed Status (Device mode only)"]
pub struct SPEED_R(crate::FieldReader<u8, SPEED_A>);
impl SPEED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPEED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEED_A> {
        match self.bits {
            0 => Some(SPEED_A::FULL_SPEED),
            1 => Some(SPEED_A::HIGH_SPEED),
            2 => Some(SPEED_A::LOW_SPEED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        **self == SPEED_A::FULL_SPEED
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        **self == SPEED_A::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `LOW_SPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        **self == SPEED_A::LOW_SPEED
    }
}
impl core::ops::Deref for SPEED_R {
    type Target = crate::FieldReader<u8, SPEED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKUSABLE` reader - UTMI Clock Usable"]
pub struct CLKUSABLE_R(crate::FieldReader<bool, bool>);
impl CLKUSABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKUSABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKUSABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt (Host mode only)"]
    #[inline(always)]
    pub fn rderri(&self) -> RDERRI_R {
        RDERRI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status (Device mode only)"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> CLKUSABLE_R {
        CLKUSABLE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
#[doc = "General Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_sr](index.html) module"]
pub struct USBHS_SR_SPEC;
impl crate::RegisterSpec for USBHS_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_sr::R](R) reader structure"]
impl crate::Readable for USBHS_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBHS_SR to value 0"]
impl crate::Resettable for USBHS_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
