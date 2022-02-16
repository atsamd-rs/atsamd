#[doc = "Register `USBHS_DEVCTRL` reader"]
pub struct R(crate::R<USBHS_DEVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_DEVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_DEVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_DEVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_DEVCTRL` writer"]
pub struct W(crate::W<USBHS_DEVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_DEVCTRL_SPEC>;
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
impl From<crate::W<USBHS_DEVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_DEVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UADD` reader - USB Address"]
pub struct UADD_R(crate::FieldReader<u8, u8>);
impl UADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UADD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UADD` writer - USB Address"]
pub struct UADD_W<'a> {
    w: &'a mut W,
}
impl<'a> UADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `ADDEN` reader - Address Enable"]
pub struct ADDEN_R(crate::FieldReader<bool, bool>);
impl ADDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDEN` writer - Address Enable"]
pub struct ADDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `DETACH` reader - Detach"]
pub struct DETACH_R(crate::FieldReader<bool, bool>);
impl DETACH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DETACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DETACH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DETACH` writer - Detach"]
pub struct DETACH_W<'a> {
    w: &'a mut W,
}
impl<'a> DETACH_W<'a> {
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
#[doc = "Field `RMWKUP` reader - Remote Wake-Up"]
pub struct RMWKUP_R(crate::FieldReader<bool, bool>);
impl RMWKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMWKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMWKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMWKUP` writer - Remote Wake-Up"]
pub struct RMWKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RMWKUP_W<'a> {
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
#[doc = "Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPDCONF_A {
    #[doc = "0: The peripheral starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the host is high-speed-capable."]
    NORMAL = 0,
    #[doc = "1: For a better consumption, if high speed is not needed."]
    LOW_POWER = 1,
    #[doc = "2: Forced high speed."]
    HIGH_SPEED = 2,
    #[doc = "3: The peripheral remains in Full-speed mode whatever the host speed capability."]
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
    #[doc = "The peripheral starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the host is high-speed-capable."]
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
    #[doc = "The peripheral remains in Full-speed mode whatever the host speed capability."]
    #[inline(always)]
    pub fn forced_fs(self) -> &'a mut W {
        self.variant(SPDCONF_A::FORCED_FS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `LS` reader - Low-Speed Mode Force"]
pub struct LS_R(crate::FieldReader<bool, bool>);
impl LS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LS` writer - Low-Speed Mode Force"]
pub struct LS_W<'a> {
    w: &'a mut W,
}
impl<'a> LS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TSTJ` reader - Test mode J"]
pub struct TSTJ_R(crate::FieldReader<bool, bool>);
impl TSTJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTJ` writer - Test mode J"]
pub struct TSTJ_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTJ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TSTK` reader - Test mode K"]
pub struct TSTK_R(crate::FieldReader<bool, bool>);
impl TSTK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTK` writer - Test mode K"]
pub struct TSTK_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TSTPCKT` reader - Test packet mode"]
pub struct TSTPCKT_R(crate::FieldReader<bool, bool>);
impl TSTPCKT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTPCKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTPCKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTPCKT` writer - Test packet mode"]
pub struct TSTPCKT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTPCKT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `OPMODE2` reader - Specific Operational mode"]
pub struct OPMODE2_R(crate::FieldReader<bool, bool>);
impl OPMODE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OPMODE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPMODE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMODE2` writer - Specific Operational mode"]
pub struct OPMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - USB Address"]
    #[inline(always)]
    pub fn uadd(&self) -> UADD_R {
        UADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Address Enable"]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Detach"]
    #[inline(always)]
    pub fn detach(&self) -> DETACH_R {
        DETACH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Remote Wake-Up"]
    #[inline(always)]
    pub fn rmwkup(&self) -> RMWKUP_R {
        RMWKUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Low-Speed Mode Force"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&self) -> TSTPCKT_R {
        TSTPCKT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Specific Operational mode"]
    #[inline(always)]
    pub fn opmode2(&self) -> OPMODE2_R {
        OPMODE2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Address"]
    #[inline(always)]
    pub fn uadd(&mut self) -> UADD_W {
        UADD_W { w: self }
    }
    #[doc = "Bit 7 - Address Enable"]
    #[inline(always)]
    pub fn adden(&mut self) -> ADDEN_W {
        ADDEN_W { w: self }
    }
    #[doc = "Bit 8 - Detach"]
    #[inline(always)]
    pub fn detach(&mut self) -> DETACH_W {
        DETACH_W { w: self }
    }
    #[doc = "Bit 9 - Remote Wake-Up"]
    #[inline(always)]
    pub fn rmwkup(&mut self) -> RMWKUP_W {
        RMWKUP_W { w: self }
    }
    #[doc = "Bits 10:11 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&mut self) -> SPDCONF_W {
        SPDCONF_W { w: self }
    }
    #[doc = "Bit 12 - Low-Speed Mode Force"]
    #[inline(always)]
    pub fn ls(&mut self) -> LS_W {
        LS_W { w: self }
    }
    #[doc = "Bit 13 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&mut self) -> TSTJ_W {
        TSTJ_W { w: self }
    }
    #[doc = "Bit 14 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&mut self) -> TSTK_W {
        TSTK_W { w: self }
    }
    #[doc = "Bit 15 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&mut self) -> TSTPCKT_W {
        TSTPCKT_W { w: self }
    }
    #[doc = "Bit 16 - Specific Operational mode"]
    #[inline(always)]
    pub fn opmode2(&mut self) -> OPMODE2_W {
        OPMODE2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devctrl](index.html) module"]
pub struct USBHS_DEVCTRL_SPEC;
impl crate::RegisterSpec for USBHS_DEVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_devctrl::R](R) reader structure"]
impl crate::Readable for USBHS_DEVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_devctrl::W](W) writer structure"]
impl crate::Writable for USBHS_DEVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_DEVCTRL to value 0"]
impl crate::Resettable for USBHS_DEVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
