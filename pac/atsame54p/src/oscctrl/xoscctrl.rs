#[doc = "Register `XOSCCTRL[%s]` reader"]
pub struct R(crate::R<XOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSCCTRL[%s]` writer"]
pub struct W(crate::W<XOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSCCTRL_SPEC>;
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
impl From<crate::W<XOSCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
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
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
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
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub struct XTALEN_R(crate::FieldReader<bool, bool>);
impl XTALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub struct XTALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALEN_W<'a> {
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
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub struct ONDEMAND_R(crate::FieldReader<bool, bool>);
impl ONDEMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONDEMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONDEMAND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub struct ONDEMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEMAND_W<'a> {
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
#[doc = "Field `LOWBUFGAIN` reader - Low Buffer Gain Enable"]
pub struct LOWBUFGAIN_R(crate::FieldReader<bool, bool>);
impl LOWBUFGAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOWBUFGAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOWBUFGAIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWBUFGAIN` writer - Low Buffer Gain Enable"]
pub struct LOWBUFGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWBUFGAIN_W<'a> {
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
#[doc = "Field `IPTAT` reader - Oscillator Current Reference"]
pub struct IPTAT_R(crate::FieldReader<u8, u8>);
impl IPTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IPTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPTAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPTAT` writer - Oscillator Current Reference"]
pub struct IPTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> IPTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `IMULT` reader - Oscillator Current Multiplier"]
pub struct IMULT_R(crate::FieldReader<u8, u8>);
impl IMULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IMULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMULT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMULT` writer - Oscillator Current Multiplier"]
pub struct IMULT_W<'a> {
    w: &'a mut W,
}
impl<'a> IMULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `ENALC` reader - Automatic Loop Control Enable"]
pub struct ENALC_R(crate::FieldReader<bool, bool>);
impl ENALC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENALC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENALC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENALC` writer - Automatic Loop Control Enable"]
pub struct ENALC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENALC_W<'a> {
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
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub struct CFDEN_R(crate::FieldReader<bool, bool>);
impl CFDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub struct CFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEN_W<'a> {
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
#[doc = "Field `SWBEN` reader - Xosc Clock Switch Enable"]
pub struct SWBEN_R(crate::FieldReader<bool, bool>);
impl SWBEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWBEN` writer - Xosc Clock Switch Enable"]
pub struct SWBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 31 us"]
    CYCLE1 = 0,
    #[doc = "1: 61 us"]
    CYCLE2 = 1,
    #[doc = "2: 122 us"]
    CYCLE4 = 2,
    #[doc = "3: 244 us"]
    CYCLE8 = 3,
    #[doc = "4: 488 us"]
    CYCLE16 = 4,
    #[doc = "5: 977 us"]
    CYCLE32 = 5,
    #[doc = "6: 1953 us"]
    CYCLE64 = 6,
    #[doc = "7: 3906 us"]
    CYCLE128 = 7,
    #[doc = "8: 7813 us"]
    CYCLE256 = 8,
    #[doc = "9: 15625 us"]
    CYCLE512 = 9,
    #[doc = "10: 31250 us"]
    CYCLE1024 = 10,
    #[doc = "11: 62500 us"]
    CYCLE2048 = 11,
    #[doc = "12: 125000 us"]
    CYCLE4096 = 12,
    #[doc = "13: 250000 us"]
    CYCLE8192 = 13,
    #[doc = "14: 500000 us"]
    CYCLE16384 = 14,
    #[doc = "15: 1000000 us"]
    CYCLE32768 = 15,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STARTUP` reader - Start-Up Time"]
pub struct STARTUP_R(crate::FieldReader<u8, STARTUP_A>);
impl STARTUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STARTUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTUP_A {
        match self.bits {
            0 => STARTUP_A::CYCLE1,
            1 => STARTUP_A::CYCLE2,
            2 => STARTUP_A::CYCLE4,
            3 => STARTUP_A::CYCLE8,
            4 => STARTUP_A::CYCLE16,
            5 => STARTUP_A::CYCLE32,
            6 => STARTUP_A::CYCLE64,
            7 => STARTUP_A::CYCLE128,
            8 => STARTUP_A::CYCLE256,
            9 => STARTUP_A::CYCLE512,
            10 => STARTUP_A::CYCLE1024,
            11 => STARTUP_A::CYCLE2048,
            12 => STARTUP_A::CYCLE4096,
            13 => STARTUP_A::CYCLE8192,
            14 => STARTUP_A::CYCLE16384,
            15 => STARTUP_A::CYCLE32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE1`"]
    #[inline(always)]
    pub fn is_cycle1(&self) -> bool {
        **self == STARTUP_A::CYCLE1
    }
    #[doc = "Checks if the value of the field is `CYCLE2`"]
    #[inline(always)]
    pub fn is_cycle2(&self) -> bool {
        **self == STARTUP_A::CYCLE2
    }
    #[doc = "Checks if the value of the field is `CYCLE4`"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        **self == STARTUP_A::CYCLE4
    }
    #[doc = "Checks if the value of the field is `CYCLE8`"]
    #[inline(always)]
    pub fn is_cycle8(&self) -> bool {
        **self == STARTUP_A::CYCLE8
    }
    #[doc = "Checks if the value of the field is `CYCLE16`"]
    #[inline(always)]
    pub fn is_cycle16(&self) -> bool {
        **self == STARTUP_A::CYCLE16
    }
    #[doc = "Checks if the value of the field is `CYCLE32`"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        **self == STARTUP_A::CYCLE32
    }
    #[doc = "Checks if the value of the field is `CYCLE64`"]
    #[inline(always)]
    pub fn is_cycle64(&self) -> bool {
        **self == STARTUP_A::CYCLE64
    }
    #[doc = "Checks if the value of the field is `CYCLE128`"]
    #[inline(always)]
    pub fn is_cycle128(&self) -> bool {
        **self == STARTUP_A::CYCLE128
    }
    #[doc = "Checks if the value of the field is `CYCLE256`"]
    #[inline(always)]
    pub fn is_cycle256(&self) -> bool {
        **self == STARTUP_A::CYCLE256
    }
    #[doc = "Checks if the value of the field is `CYCLE512`"]
    #[inline(always)]
    pub fn is_cycle512(&self) -> bool {
        **self == STARTUP_A::CYCLE512
    }
    #[doc = "Checks if the value of the field is `CYCLE1024`"]
    #[inline(always)]
    pub fn is_cycle1024(&self) -> bool {
        **self == STARTUP_A::CYCLE1024
    }
    #[doc = "Checks if the value of the field is `CYCLE2048`"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        **self == STARTUP_A::CYCLE2048
    }
    #[doc = "Checks if the value of the field is `CYCLE4096`"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        **self == STARTUP_A::CYCLE4096
    }
    #[doc = "Checks if the value of the field is `CYCLE8192`"]
    #[inline(always)]
    pub fn is_cycle8192(&self) -> bool {
        **self == STARTUP_A::CYCLE8192
    }
    #[doc = "Checks if the value of the field is `CYCLE16384`"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        **self == STARTUP_A::CYCLE16384
    }
    #[doc = "Checks if the value of the field is `CYCLE32768`"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        **self == STARTUP_A::CYCLE32768
    }
}
impl core::ops::Deref for STARTUP_R {
    type Target = crate::FieldReader<u8, STARTUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTUP` writer - Start-Up Time"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTUP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "31 us"]
    #[inline(always)]
    pub fn cycle1(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE1)
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn cycle2(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE2)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE4)
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn cycle8(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE8)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn cycle16(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE16)
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE32)
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn cycle64(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE64)
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn cycle128(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE128)
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn cycle256(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE256)
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn cycle512(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE512)
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn cycle1024(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE1024)
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE2048)
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE4096)
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn cycle8192(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE8192)
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE16384)
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE32768)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Clock Failure Detector Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFDPRESC_A {
    #[doc = "0: 48 MHz"]
    DIV1 = 0,
    #[doc = "1: 24 MHz"]
    DIV2 = 1,
    #[doc = "2: 12 MHz"]
    DIV4 = 2,
    #[doc = "3: 6 MHz"]
    DIV8 = 3,
    #[doc = "4: 3 MHz"]
    DIV16 = 4,
    #[doc = "5: 1.5 MHz"]
    DIV32 = 5,
    #[doc = "6: 0.75 MHz"]
    DIV64 = 6,
    #[doc = "7: 0.3125 MHz"]
    DIV128 = 7,
}
impl From<CFDPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFDPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFDPRESC` reader - Clock Failure Detector Prescaler"]
pub struct CFDPRESC_R(crate::FieldReader<u8, CFDPRESC_A>);
impl CFDPRESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFDPRESC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFDPRESC_A> {
        match self.bits {
            0 => Some(CFDPRESC_A::DIV1),
            1 => Some(CFDPRESC_A::DIV2),
            2 => Some(CFDPRESC_A::DIV4),
            3 => Some(CFDPRESC_A::DIV8),
            4 => Some(CFDPRESC_A::DIV16),
            5 => Some(CFDPRESC_A::DIV32),
            6 => Some(CFDPRESC_A::DIV64),
            7 => Some(CFDPRESC_A::DIV128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == CFDPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == CFDPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == CFDPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == CFDPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == CFDPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == CFDPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == CFDPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == CFDPRESC_A::DIV128
    }
}
impl core::ops::Deref for CFDPRESC_R {
    type Target = crate::FieldReader<u8, CFDPRESC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFDPRESC` writer - Clock Failure Detector Prescaler"]
pub struct CFDPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFDPRESC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV1)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV2)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV4)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV8)
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV16)
    }
    #[doc = "1.5 MHz"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV32)
    }
    #[doc = "0.75 MHz"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV64)
    }
    #[doc = "0.3125 MHz"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CFDPRESC_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XTALEN_R {
        XTALEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Low Buffer Gain Enable"]
    #[inline(always)]
    pub fn lowbufgain(&self) -> LOWBUFGAIN_R {
        LOWBUFGAIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Oscillator Current Reference"]
    #[inline(always)]
    pub fn iptat(&self) -> IPTAT_R {
        IPTAT_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:14 - Oscillator Current Multiplier"]
    #[inline(always)]
    pub fn imult(&self) -> IMULT_R {
        IMULT_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Automatic Loop Control Enable"]
    #[inline(always)]
    pub fn enalc(&self) -> ENALC_R {
        ENALC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&self) -> SWBEN_R {
        SWBEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CFDPRESC_R {
        CFDPRESC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&mut self) -> XTALEN_W {
        XTALEN_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W { w: self }
    }
    #[doc = "Bit 8 - Low Buffer Gain Enable"]
    #[inline(always)]
    pub fn lowbufgain(&mut self) -> LOWBUFGAIN_W {
        LOWBUFGAIN_W { w: self }
    }
    #[doc = "Bits 9:10 - Oscillator Current Reference"]
    #[inline(always)]
    pub fn iptat(&mut self) -> IPTAT_W {
        IPTAT_W { w: self }
    }
    #[doc = "Bits 11:14 - Oscillator Current Multiplier"]
    #[inline(always)]
    pub fn imult(&mut self) -> IMULT_W {
        IMULT_W { w: self }
    }
    #[doc = "Bit 15 - Automatic Loop Control Enable"]
    #[inline(always)]
    pub fn enalc(&mut self) -> ENALC_W {
        ENALC_W { w: self }
    }
    #[doc = "Bit 16 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W {
        CFDEN_W { w: self }
    }
    #[doc = "Bit 17 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&mut self) -> SWBEN_W {
        SWBEN_W { w: self }
    }
    #[doc = "Bits 20:23 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
    #[doc = "Bits 24:27 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&mut self) -> CFDPRESC_W {
        CFDPRESC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Multipurpose Crystal Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xoscctrl](index.html) module"]
pub struct XOSCCTRL_SPEC;
impl crate::RegisterSpec for XOSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xoscctrl::R](R) reader structure"]
impl crate::Readable for XOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xoscctrl::W](W) writer structure"]
impl crate::Writable for XOSCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XOSCCTRL[%s]
to value 0x80"]
impl crate::Resettable for XOSCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
