#[doc = "Register `BOD33` reader"]
pub struct R(crate::R<BOD33_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD33_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD33_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD33_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD33` writer"]
pub struct W(crate::W<BOD33_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD33_SPEC>;
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
impl From<crate::W<BOD33_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD33_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Action when Threshold Crossed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACTION_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: The BOD33 generates a reset"]
    RESET = 1,
    #[doc = "2: The BOD33 generates an interrupt"]
    INT = 2,
    #[doc = "3: The BOD33 puts the device in backup sleep mode"]
    BKUP = 3,
}
impl From<ACTION_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACTION` reader - Action when Threshold Crossed"]
pub struct ACTION_R(crate::FieldReader<u8, ACTION_A>);
impl ACTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTION_A {
        match self.bits {
            0 => ACTION_A::NONE,
            1 => ACTION_A::RESET,
            2 => ACTION_A::INT,
            3 => ACTION_A::BKUP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == ACTION_A::NONE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == ACTION_A::RESET
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        **self == ACTION_A::INT
    }
    #[doc = "Checks if the value of the field is `BKUP`"]
    #[inline(always)]
    pub fn is_bkup(&self) -> bool {
        **self == ACTION_A::BKUP
    }
}
impl core::ops::Deref for ACTION_R {
    type Target = crate::FieldReader<u8, ACTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTION` writer - Action when Threshold Crossed"]
pub struct ACTION_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTION_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTION_A::NONE)
    }
    #[doc = "The BOD33 generates a reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ACTION_A::RESET)
    }
    #[doc = "The BOD33 generates an interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(ACTION_A::INT)
    }
    #[doc = "The BOD33 puts the device in backup sleep mode"]
    #[inline(always)]
    pub fn bkup(self) -> &'a mut W {
        self.variant(ACTION_A::BKUP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `STDBYCFG` reader - Configuration in Standby mode"]
pub struct STDBYCFG_R(crate::FieldReader<bool, bool>);
impl STDBYCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STDBYCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STDBYCFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STDBYCFG` writer - Configuration in Standby mode"]
pub struct STDBYCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> STDBYCFG_W<'a> {
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
#[doc = "Field `RUNSTDBY` reader - Run in Standby mode"]
pub struct RUNSTDBY_R(crate::FieldReader<bool, bool>);
impl RUNSTDBY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNSTDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNSTDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNSTDBY` writer - Run in Standby mode"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
#[doc = "Field `RUNHIB` reader - Run in Hibernate mode"]
pub struct RUNHIB_R(crate::FieldReader<bool, bool>);
impl RUNHIB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNHIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNHIB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNHIB` writer - Run in Hibernate mode"]
pub struct RUNHIB_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNHIB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RUNBKUP` reader - Run in Backup mode"]
pub struct RUNBKUP_R(crate::FieldReader<bool, bool>);
impl RUNBKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNBKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNBKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNBKUP` writer - Run in Backup mode"]
pub struct RUNBKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNBKUP_W<'a> {
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
#[doc = "Field `HYST` reader - Hysteresis value"]
pub struct HYST_R(crate::FieldReader<u8, u8>);
impl HYST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HYST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST` writer - Hysteresis value"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: Not divided"]
    NODIV = 0,
    #[doc = "1: Divide clock by 4"]
    DIV4 = 1,
    #[doc = "2: Divide clock by 8"]
    DIV8 = 2,
    #[doc = "3: Divide clock by 16"]
    DIV16 = 3,
    #[doc = "4: Divide clock by 32"]
    DIV32 = 4,
    #[doc = "5: Divide clock by 64"]
    DIV64 = 5,
    #[doc = "6: Divide clock by 128"]
    DIV128 = 6,
    #[doc = "7: Divide clock by 256"]
    DIV256 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSEL` reader - Prescaler Select"]
pub struct PSEL_R(crate::FieldReader<u8, PSEL_A>);
impl PSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::NODIV,
            1 => PSEL_A::DIV4,
            2 => PSEL_A::DIV8,
            3 => PSEL_A::DIV16,
            4 => PSEL_A::DIV32,
            5 => PSEL_A::DIV64,
            6 => PSEL_A::DIV128,
            7 => PSEL_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NODIV`"]
    #[inline(always)]
    pub fn is_nodiv(&self) -> bool {
        **self == PSEL_A::NODIV
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PSEL_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PSEL_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PSEL_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == PSEL_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == PSEL_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == PSEL_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == PSEL_A::DIV256
    }
}
impl core::ops::Deref for PSEL_R {
    type Target = crate::FieldReader<u8, PSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Not divided"]
    #[inline(always)]
    pub fn nodiv(self) -> &'a mut W {
        self.variant(PSEL_A::NODIV)
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PSEL_A::DIV4)
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PSEL_A::DIV8)
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PSEL_A::DIV16)
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PSEL_A::DIV32)
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PSEL_A::DIV64)
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PSEL_A::DIV128)
    }
    #[doc = "Divide clock by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PSEL_A::DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `LEVEL` reader - Threshold Level for VDD"]
pub struct LEVEL_R(crate::FieldReader<u8, u8>);
impl LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEVEL` writer - Threshold Level for VDD"]
pub struct LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `VBATLEVEL` reader - Threshold Level in battery backup sleep mode for VBAT"]
pub struct VBATLEVEL_R(crate::FieldReader<u8, u8>);
impl VBATLEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VBATLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBATLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBATLEVEL` writer - Threshold Level in battery backup sleep mode for VBAT"]
pub struct VBATLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Action when Threshold Crossed"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Configuration in Standby mode"]
    #[inline(always)]
    pub fn stdbycfg(&self) -> STDBYCFG_R {
        STDBYCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Run in Standby mode"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run in Hibernate mode"]
    #[inline(always)]
    pub fn runhib(&self) -> RUNHIB_R {
        RUNHIB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&self) -> RUNBKUP_R {
        RUNBKUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Hysteresis value"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Threshold Level for VDD"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Threshold Level in battery backup sleep mode for VBAT"]
    #[inline(always)]
    pub fn vbatlevel(&self) -> VBATLEVEL_R {
        VBATLEVEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:3 - Action when Threshold Crossed"]
    #[inline(always)]
    pub fn action(&mut self) -> ACTION_W {
        ACTION_W { w: self }
    }
    #[doc = "Bit 4 - Configuration in Standby mode"]
    #[inline(always)]
    pub fn stdbycfg(&mut self) -> STDBYCFG_W {
        STDBYCFG_W { w: self }
    }
    #[doc = "Bit 5 - Run in Standby mode"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 6 - Run in Hibernate mode"]
    #[inline(always)]
    pub fn runhib(&mut self) -> RUNHIB_W {
        RUNHIB_W { w: self }
    }
    #[doc = "Bit 7 - Run in Backup mode"]
    #[inline(always)]
    pub fn runbkup(&mut self) -> RUNBKUP_W {
        RUNBKUP_W { w: self }
    }
    #[doc = "Bits 8:11 - Hysteresis value"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bits 12:14 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Bits 16:23 - Threshold Level for VDD"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W { w: self }
    }
    #[doc = "Bits 24:31 - Threshold Level in battery backup sleep mode for VBAT"]
    #[inline(always)]
    pub fn vbatlevel(&mut self) -> VBATLEVEL_W {
        VBATLEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD33 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33](index.html) module"]
pub struct BOD33_SPEC;
impl crate::RegisterSpec for BOD33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod33::R](R) reader structure"]
impl crate::Readable for BOD33_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod33::W](W) writer structure"]
impl crate::Writable for BOD33_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOD33 to value 0"]
impl crate::Resettable for BOD33_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
