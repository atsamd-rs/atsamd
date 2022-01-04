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
#[doc = "Field `HYST` reader - Hysteresis"]
pub struct HYST_R(crate::FieldReader<bool, bool>);
impl HYST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HYST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HYST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST` writer - Hysteresis"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
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
#[doc = "BOD33 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACTION_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: The BOD33 generates a reset"]
    RESET = 1,
    #[doc = "2: The BOD33 generates an interrupt"]
    INTERRUPT = 2,
}
impl From<ACTION_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACTION` reader - BOD33 Action"]
pub struct ACTION_R(crate::FieldReader<u8, ACTION_A>);
impl ACTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACTION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACTION_A> {
        match self.bits {
            0 => Some(ACTION_A::NONE),
            1 => Some(ACTION_A::RESET),
            2 => Some(ACTION_A::INTERRUPT),
            _ => None,
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
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        **self == ACTION_A::INTERRUPT
    }
}
impl core::ops::Deref for ACTION_R {
    type Target = crate::FieldReader<u8, ACTION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTION` writer - BOD33 Action"]
pub struct ACTION_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACTION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
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
    pub fn interrupt(self) -> &'a mut W {
        self.variant(ACTION_A::INTERRUPT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
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
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `MODE` reader - Operation Mode"]
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Operation Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
#[doc = "Field `CEN` reader - Clock Enable"]
pub struct CEN_R(crate::FieldReader<bool, bool>);
impl CEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEN` writer - Clock Enable"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
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
#[doc = "Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: Divide clock by 2"]
    DIV2 = 0,
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
    #[doc = "8: Divide clock by 512"]
    DIV512 = 8,
    #[doc = "9: Divide clock by 1024"]
    DIV1K = 9,
    #[doc = "10: Divide clock by 2048"]
    DIV2K = 10,
    #[doc = "11: Divide clock by 4096"]
    DIV4K = 11,
    #[doc = "12: Divide clock by 8192"]
    DIV8K = 12,
    #[doc = "13: Divide clock by 16384"]
    DIV16K = 13,
    #[doc = "14: Divide clock by 32768"]
    DIV32K = 14,
    #[doc = "15: Divide clock by 65536"]
    DIV64K = 15,
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
            0 => PSEL_A::DIV2,
            1 => PSEL_A::DIV4,
            2 => PSEL_A::DIV8,
            3 => PSEL_A::DIV16,
            4 => PSEL_A::DIV32,
            5 => PSEL_A::DIV64,
            6 => PSEL_A::DIV128,
            7 => PSEL_A::DIV256,
            8 => PSEL_A::DIV512,
            9 => PSEL_A::DIV1K,
            10 => PSEL_A::DIV2K,
            11 => PSEL_A::DIV4K,
            12 => PSEL_A::DIV8K,
            13 => PSEL_A::DIV16K,
            14 => PSEL_A::DIV32K,
            15 => PSEL_A::DIV64K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PSEL_A::DIV2
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
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        **self == PSEL_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1K`"]
    #[inline(always)]
    pub fn is_div1k(&self) -> bool {
        **self == PSEL_A::DIV1K
    }
    #[doc = "Checks if the value of the field is `DIV2K`"]
    #[inline(always)]
    pub fn is_div2k(&self) -> bool {
        **self == PSEL_A::DIV2K
    }
    #[doc = "Checks if the value of the field is `DIV4K`"]
    #[inline(always)]
    pub fn is_div4k(&self) -> bool {
        **self == PSEL_A::DIV4K
    }
    #[doc = "Checks if the value of the field is `DIV8K`"]
    #[inline(always)]
    pub fn is_div8k(&self) -> bool {
        **self == PSEL_A::DIV8K
    }
    #[doc = "Checks if the value of the field is `DIV16K`"]
    #[inline(always)]
    pub fn is_div16k(&self) -> bool {
        **self == PSEL_A::DIV16K
    }
    #[doc = "Checks if the value of the field is `DIV32K`"]
    #[inline(always)]
    pub fn is_div32k(&self) -> bool {
        **self == PSEL_A::DIV32K
    }
    #[doc = "Checks if the value of the field is `DIV64K`"]
    #[inline(always)]
    pub fn is_div64k(&self) -> bool {
        **self == PSEL_A::DIV64K
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
    #[doc = "Divide clock by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PSEL_A::DIV2)
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
    #[doc = "Divide clock by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PSEL_A::DIV512)
    }
    #[doc = "Divide clock by 1024"]
    #[inline(always)]
    pub fn div1k(self) -> &'a mut W {
        self.variant(PSEL_A::DIV1K)
    }
    #[doc = "Divide clock by 2048"]
    #[inline(always)]
    pub fn div2k(self) -> &'a mut W {
        self.variant(PSEL_A::DIV2K)
    }
    #[doc = "Divide clock by 4096"]
    #[inline(always)]
    pub fn div4k(self) -> &'a mut W {
        self.variant(PSEL_A::DIV4K)
    }
    #[doc = "Divide clock by 8192"]
    #[inline(always)]
    pub fn div8k(self) -> &'a mut W {
        self.variant(PSEL_A::DIV8K)
    }
    #[doc = "Divide clock by 16384"]
    #[inline(always)]
    pub fn div16k(self) -> &'a mut W {
        self.variant(PSEL_A::DIV16K)
    }
    #[doc = "Divide clock by 32768"]
    #[inline(always)]
    pub fn div32k(self) -> &'a mut W {
        self.variant(PSEL_A::DIV32K)
    }
    #[doc = "Divide clock by 65536"]
    #[inline(always)]
    pub fn div64k(self) -> &'a mut W {
        self.variant(PSEL_A::DIV64K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `LEVEL` reader - BOD33 Threshold Level"]
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
#[doc = "Field `LEVEL` writer - BOD33 Threshold Level"]
pub struct LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hysteresis"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - BOD33 Action"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - BOD33 Threshold Level"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Hysteresis"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bits 3:4 - BOD33 Action"]
    #[inline(always)]
    pub fn action(&mut self) -> ACTION_W {
        ACTION_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 8 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 9 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Bits 16:21 - BOD33 Threshold Level"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "3.3V Brown-Out Detector (BOD33) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33](index.html) module"]
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
