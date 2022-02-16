#[doc = "Register `USBHS_HSTCTRL` reader"]
pub struct R(crate::R<USBHS_HSTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_HSTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_HSTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_HSTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_HSTCTRL` writer"]
pub struct W(crate::W<USBHS_HSTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_HSTCTRL_SPEC>;
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
impl From<crate::W<USBHS_HSTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_HSTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFE` reader - Start of Frame Generation Enable"]
pub struct SOFE_R(crate::FieldReader<bool, bool>);
impl SOFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFE` writer - Start of Frame Generation Enable"]
pub struct SOFE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RESET` reader - Send USB Reset"]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - Send USB Reset"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RESUME` reader - Send USB Resume"]
pub struct RESUME_R(crate::FieldReader<bool, bool>);
impl RESUME_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME` writer - Send USB Resume"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPDCONF_A {
    #[doc = "0: The host starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the downstream peripheral is high-speed capable."]
    NORMAL = 0,
    #[doc = "1: For a better consumption, if high speed is not needed."]
    LOW_POWER = 1,
    #[doc = "2: Forced high speed."]
    HIGH_SPEED = 2,
    #[doc = "3: The host remains in Full-speed mode whatever the peripheral speed capability."]
    FORCED_FS = 3,
}
impl From<SPDCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPDCONF` reader - Mode Configuration"]
pub struct SPDCONF_R(crate::FieldReader<u8, SPDCONF_A>);
impl SPDCONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPDCONF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDCONF_A {
        match self.bits {
            0 => SPDCONF_A::NORMAL,
            1 => SPDCONF_A::LOW_POWER,
            2 => SPDCONF_A::HIGH_SPEED,
            3 => SPDCONF_A::FORCED_FS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SPDCONF_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        **self == SPDCONF_A::LOW_POWER
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        **self == SPDCONF_A::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `FORCED_FS`"]
    #[inline(always)]
    pub fn is_forced_fs(&self) -> bool {
        **self == SPDCONF_A::FORCED_FS
    }
}
impl core::ops::Deref for SPDCONF_R {
    type Target = crate::FieldReader<u8, SPDCONF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDCONF` writer - Mode Configuration"]
pub struct SPDCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDCONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDCONF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The host starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SPDCONF_A::NORMAL)
    }
    #[doc = "For a better consumption, if high speed is not needed."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(SPDCONF_A::LOW_POWER)
    }
    #[doc = "Forced high speed."]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(SPDCONF_A::HIGH_SPEED)
    }
    #[doc = "The host remains in Full-speed mode whatever the peripheral speed capability."]
    #[inline(always)]
    pub fn forced_fs(self) -> &'a mut W {
        self.variant(SPDCONF_A::FORCED_FS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&mut self) -> SOFE_W {
        SOFE_W { w: self }
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&mut self) -> SPDCONF_W {
        SPDCONF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstctrl](index.html) module"]
pub struct USBHS_HSTCTRL_SPEC;
impl crate::RegisterSpec for USBHS_HSTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_hstctrl::R](R) reader structure"]
impl crate::Readable for USBHS_HSTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_hstctrl::W](W) writer structure"]
impl crate::Writable for USBHS_HSTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_HSTCTRL to value 0"]
impl crate::Resettable for USBHS_HSTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
