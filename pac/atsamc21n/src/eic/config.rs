#[doc = "Register `CONFIG[%s]` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG[%s]` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input Sense Configuration 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SENSE0_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising edge detection"]
    RISE = 1,
    #[doc = "2: Falling edge detection"]
    FALL = 2,
    #[doc = "3: Both edges detection"]
    BOTH = 3,
    #[doc = "4: High level detection"]
    HIGH = 4,
    #[doc = "5: Low level detection"]
    LOW = 5,
}
impl From<SENSE0_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SENSE0` reader - Input Sense Configuration 0"]
pub struct SENSE0_R(crate::FieldReader<u8, SENSE0_A>);
impl SENSE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE0_A> {
        match self.bits {
            0 => Some(SENSE0_A::NONE),
            1 => Some(SENSE0_A::RISE),
            2 => Some(SENSE0_A::FALL),
            3 => Some(SENSE0_A::BOTH),
            4 => Some(SENSE0_A::HIGH),
            5 => Some(SENSE0_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SENSE0_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == SENSE0_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == SENSE0_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == SENSE0_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SENSE0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SENSE0_A::LOW
    }
}
impl core::ops::Deref for SENSE0_R {
    type Target = crate::FieldReader<u8, SENSE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE0` writer - Input Sense Configuration 0"]
pub struct SENSE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSE0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE0_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE0_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE0_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE0_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE0_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE0_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `FILTEN0` reader - Filter Enable 0"]
pub struct FILTEN0_R(crate::FieldReader<bool, bool>);
impl FILTEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTEN0` writer - Filter Enable 0"]
pub struct FILTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN0_W<'a> {
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
#[doc = "Input Sense Configuration 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SENSE1_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising edge detection"]
    RISE = 1,
    #[doc = "2: Falling edge detection"]
    FALL = 2,
    #[doc = "3: Both edges detection"]
    BOTH = 3,
    #[doc = "4: High level detection"]
    HIGH = 4,
    #[doc = "5: Low level detection"]
    LOW = 5,
}
impl From<SENSE1_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SENSE1` reader - Input Sense Configuration 1"]
pub struct SENSE1_R(crate::FieldReader<u8, SENSE1_A>);
impl SENSE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE1_A> {
        match self.bits {
            0 => Some(SENSE1_A::NONE),
            1 => Some(SENSE1_A::RISE),
            2 => Some(SENSE1_A::FALL),
            3 => Some(SENSE1_A::BOTH),
            4 => Some(SENSE1_A::HIGH),
            5 => Some(SENSE1_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SENSE1_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == SENSE1_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == SENSE1_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == SENSE1_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SENSE1_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SENSE1_A::LOW
    }
}
impl core::ops::Deref for SENSE1_R {
    type Target = crate::FieldReader<u8, SENSE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE1` writer - Input Sense Configuration 1"]
pub struct SENSE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSE1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE1_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE1_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE1_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE1_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE1_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE1_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `FILTEN1` reader - Filter Enable 1"]
pub struct FILTEN1_R(crate::FieldReader<bool, bool>);
impl FILTEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTEN1` writer - Filter Enable 1"]
pub struct FILTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN1_W<'a> {
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
#[doc = "Input Sense Configuration 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SENSE2_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising edge detection"]
    RISE = 1,
    #[doc = "2: Falling edge detection"]
    FALL = 2,
    #[doc = "3: Both edges detection"]
    BOTH = 3,
    #[doc = "4: High level detection"]
    HIGH = 4,
    #[doc = "5: Low level detection"]
    LOW = 5,
}
impl From<SENSE2_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SENSE2` reader - Input Sense Configuration 2"]
pub struct SENSE2_R(crate::FieldReader<u8, SENSE2_A>);
impl SENSE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE2_A> {
        match self.bits {
            0 => Some(SENSE2_A::NONE),
            1 => Some(SENSE2_A::RISE),
            2 => Some(SENSE2_A::FALL),
            3 => Some(SENSE2_A::BOTH),
            4 => Some(SENSE2_A::HIGH),
            5 => Some(SENSE2_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SENSE2_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == SENSE2_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == SENSE2_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == SENSE2_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SENSE2_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SENSE2_A::LOW
    }
}
impl core::ops::Deref for SENSE2_R {
    type Target = crate::FieldReader<u8, SENSE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE2` writer - Input Sense Configuration 2"]
pub struct SENSE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSE2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE2_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE2_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE2_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE2_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE2_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE2_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `FILTEN2` reader - Filter Enable 2"]
pub struct FILTEN2_R(crate::FieldReader<bool, bool>);
impl FILTEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTEN2` writer - Filter Enable 2"]
pub struct FILTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Input Sense Configuration 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SENSE3_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising edge detection"]
    RISE = 1,
    #[doc = "2: Falling edge detection"]
    FALL = 2,
    #[doc = "3: Both edges detection"]
    BOTH = 3,
    #[doc = "4: High level detection"]
    HIGH = 4,
    #[doc = "5: Low level detection"]
    LOW = 5,
}
impl From<SENSE3_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SENSE3` reader - Input Sense Configuration 3"]
pub struct SENSE3_R(crate::FieldReader<u8, SENSE3_A>);
impl SENSE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE3_A> {
        match self.bits {
            0 => Some(SENSE3_A::NONE),
            1 => Some(SENSE3_A::RISE),
            2 => Some(SENSE3_A::FALL),
            3 => Some(SENSE3_A::BOTH),
            4 => Some(SENSE3_A::HIGH),
            5 => Some(SENSE3_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SENSE3_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == SENSE3_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == SENSE3_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == SENSE3_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SENSE3_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SENSE3_A::LOW
    }
}
impl core::ops::Deref for SENSE3_R {
    type Target = crate::FieldReader<u8, SENSE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE3` writer - Input Sense Configuration 3"]
pub struct SENSE3_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSE3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE3_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE3_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE3_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE3_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE3_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE3_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `FILTEN3` reader - Filter Enable 3"]
pub struct FILTEN3_R(crate::FieldReader<bool, bool>);
impl FILTEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTEN3` writer - Filter Enable 3"]
pub struct FILTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN3_W<'a> {
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
#[doc = "Input Sense Configuration 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SENSE4_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising edge detection"]
    RISE = 1,
    #[doc = "2: Falling edge detection"]
    FALL = 2,
    #[doc = "3: Both edges detection"]
    BOTH = 3,
    #[doc = "4: High level detection"]
    HIGH = 4,
    #[doc = "5: Low level detection"]
    LOW = 5,
}
impl From<SENSE4_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SENSE4` reader - Input Sense Configuration 4"]
pub struct SENSE4_R(crate::FieldReader<u8, SENSE4_A>);
impl SENSE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE4_A> {
        match self.bits {
            0 => Some(SENSE4_A::NONE),
            1 => Some(SENSE4_A::RISE),
            2 => Some(SENSE4_A::FALL),
            3 => Some(SENSE4_A::BOTH),
            4 => Some(SENSE4_A::HIGH),
            5 => Some(SENSE4_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SENSE4_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == SENSE4_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == SENSE4_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == SENSE4_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SENSE4_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SENSE4_A::LOW
    }
}
impl core::ops::Deref for SENSE4_R {
    type Target = crate::FieldReader<u8, SENSE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE4` writer - Input Sense Configuration 4"]
pub struct SENSE4_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSE4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE4_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE4_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE4_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE4_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE4_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE4_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `FILTEN4` reader - Filter Enable 4"]
pub struct FILTEN4_R(crate::FieldReader<bool, bool>);
impl FILTEN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTEN4` writer - Filter Enable 4"]
pub struct FILTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Input Sense Configuration 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SENSE5_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising edge detection"]
    RISE = 1,
    #[doc = "2: Falling edge detection"]
    FALL = 2,
    #[doc = "3: Both edges detection"]
    BOTH = 3,
    #[doc = "4: High level detection"]
    HIGH = 4,
    #[doc = "5: Low level detection"]
    LOW = 5,
}
impl From<SENSE5_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SENSE5` reader - Input Sense Configuration 5"]
pub struct SENSE5_R(crate::FieldReader<u8, SENSE5_A>);
impl SENSE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE5_A> {
        match self.bits {
            0 => Some(SENSE5_A::NONE),
            1 => Some(SENSE5_A::RISE),
            2 => Some(SENSE5_A::FALL),
            3 => Some(SENSE5_A::BOTH),
            4 => Some(SENSE5_A::HIGH),
            5 => Some(SENSE5_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SENSE5_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == SENSE5_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == SENSE5_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == SENSE5_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SENSE5_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SENSE5_A::LOW
    }
}
impl core::ops::Deref for SENSE5_R {
    type Target = crate::FieldReader<u8, SENSE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE5` writer - Input Sense Configuration 5"]
pub struct SENSE5_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSE5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE5_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE5_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE5_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE5_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE5_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE5_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `FILTEN5` reader - Filter Enable 5"]
pub struct FILTEN5_R(crate::FieldReader<bool, bool>);
impl FILTEN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTEN5` writer - Filter Enable 5"]
pub struct FILTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Input Sense Configuration 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SENSE6_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising edge detection"]
    RISE = 1,
    #[doc = "2: Falling edge detection"]
    FALL = 2,
    #[doc = "3: Both edges detection"]
    BOTH = 3,
    #[doc = "4: High level detection"]
    HIGH = 4,
    #[doc = "5: Low level detection"]
    LOW = 5,
}
impl From<SENSE6_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SENSE6` reader - Input Sense Configuration 6"]
pub struct SENSE6_R(crate::FieldReader<u8, SENSE6_A>);
impl SENSE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE6_A> {
        match self.bits {
            0 => Some(SENSE6_A::NONE),
            1 => Some(SENSE6_A::RISE),
            2 => Some(SENSE6_A::FALL),
            3 => Some(SENSE6_A::BOTH),
            4 => Some(SENSE6_A::HIGH),
            5 => Some(SENSE6_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SENSE6_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == SENSE6_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == SENSE6_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == SENSE6_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SENSE6_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SENSE6_A::LOW
    }
}
impl core::ops::Deref for SENSE6_R {
    type Target = crate::FieldReader<u8, SENSE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE6` writer - Input Sense Configuration 6"]
pub struct SENSE6_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSE6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE6_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE6_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE6_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE6_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE6_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE6_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `FILTEN6` reader - Filter Enable 6"]
pub struct FILTEN6_R(crate::FieldReader<bool, bool>);
impl FILTEN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTEN6` writer - Filter Enable 6"]
pub struct FILTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Input Sense Configuration 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SENSE7_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising edge detection"]
    RISE = 1,
    #[doc = "2: Falling edge detection"]
    FALL = 2,
    #[doc = "3: Both edges detection"]
    BOTH = 3,
    #[doc = "4: High level detection"]
    HIGH = 4,
    #[doc = "5: Low level detection"]
    LOW = 5,
}
impl From<SENSE7_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SENSE7` reader - Input Sense Configuration 7"]
pub struct SENSE7_R(crate::FieldReader<u8, SENSE7_A>);
impl SENSE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SENSE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE7_A> {
        match self.bits {
            0 => Some(SENSE7_A::NONE),
            1 => Some(SENSE7_A::RISE),
            2 => Some(SENSE7_A::FALL),
            3 => Some(SENSE7_A::BOTH),
            4 => Some(SENSE7_A::HIGH),
            5 => Some(SENSE7_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SENSE7_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == SENSE7_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == SENSE7_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == SENSE7_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SENSE7_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SENSE7_A::LOW
    }
}
impl core::ops::Deref for SENSE7_R {
    type Target = crate::FieldReader<u8, SENSE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENSE7` writer - Input Sense Configuration 7"]
pub struct SENSE7_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSE7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE7_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE7_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE7_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE7_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE7_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE7_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `FILTEN7` reader - Filter Enable 7"]
pub struct FILTEN7_R(crate::FieldReader<bool, bool>);
impl FILTEN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTEN7` writer - Filter Enable 7"]
pub struct FILTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Input Sense Configuration 0"]
    #[inline(always)]
    pub fn sense0(&self) -> SENSE0_R {
        SENSE0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Filter Enable 0"]
    #[inline(always)]
    pub fn filten0(&self) -> FILTEN0_R {
        FILTEN0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Input Sense Configuration 1"]
    #[inline(always)]
    pub fn sense1(&self) -> SENSE1_R {
        SENSE1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Filter Enable 1"]
    #[inline(always)]
    pub fn filten1(&self) -> FILTEN1_R {
        FILTEN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Input Sense Configuration 2"]
    #[inline(always)]
    pub fn sense2(&self) -> SENSE2_R {
        SENSE2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Filter Enable 2"]
    #[inline(always)]
    pub fn filten2(&self) -> FILTEN2_R {
        FILTEN2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Input Sense Configuration 3"]
    #[inline(always)]
    pub fn sense3(&self) -> SENSE3_R {
        SENSE3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Filter Enable 3"]
    #[inline(always)]
    pub fn filten3(&self) -> FILTEN3_R {
        FILTEN3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Input Sense Configuration 4"]
    #[inline(always)]
    pub fn sense4(&self) -> SENSE4_R {
        SENSE4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - Filter Enable 4"]
    #[inline(always)]
    pub fn filten4(&self) -> FILTEN4_R {
        FILTEN4_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Input Sense Configuration 5"]
    #[inline(always)]
    pub fn sense5(&self) -> SENSE5_R {
        SENSE5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Filter Enable 5"]
    #[inline(always)]
    pub fn filten5(&self) -> FILTEN5_R {
        FILTEN5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Input Sense Configuration 6"]
    #[inline(always)]
    pub fn sense6(&self) -> SENSE6_R {
        SENSE6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Filter Enable 6"]
    #[inline(always)]
    pub fn filten6(&self) -> FILTEN6_R {
        FILTEN6_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Input Sense Configuration 7"]
    #[inline(always)]
    pub fn sense7(&self) -> SENSE7_R {
        SENSE7_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Filter Enable 7"]
    #[inline(always)]
    pub fn filten7(&self) -> FILTEN7_R {
        FILTEN7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Sense Configuration 0"]
    #[inline(always)]
    pub fn sense0(&mut self) -> SENSE0_W {
        SENSE0_W { w: self }
    }
    #[doc = "Bit 3 - Filter Enable 0"]
    #[inline(always)]
    pub fn filten0(&mut self) -> FILTEN0_W {
        FILTEN0_W { w: self }
    }
    #[doc = "Bits 4:6 - Input Sense Configuration 1"]
    #[inline(always)]
    pub fn sense1(&mut self) -> SENSE1_W {
        SENSE1_W { w: self }
    }
    #[doc = "Bit 7 - Filter Enable 1"]
    #[inline(always)]
    pub fn filten1(&mut self) -> FILTEN1_W {
        FILTEN1_W { w: self }
    }
    #[doc = "Bits 8:10 - Input Sense Configuration 2"]
    #[inline(always)]
    pub fn sense2(&mut self) -> SENSE2_W {
        SENSE2_W { w: self }
    }
    #[doc = "Bit 11 - Filter Enable 2"]
    #[inline(always)]
    pub fn filten2(&mut self) -> FILTEN2_W {
        FILTEN2_W { w: self }
    }
    #[doc = "Bits 12:14 - Input Sense Configuration 3"]
    #[inline(always)]
    pub fn sense3(&mut self) -> SENSE3_W {
        SENSE3_W { w: self }
    }
    #[doc = "Bit 15 - Filter Enable 3"]
    #[inline(always)]
    pub fn filten3(&mut self) -> FILTEN3_W {
        FILTEN3_W { w: self }
    }
    #[doc = "Bits 16:18 - Input Sense Configuration 4"]
    #[inline(always)]
    pub fn sense4(&mut self) -> SENSE4_W {
        SENSE4_W { w: self }
    }
    #[doc = "Bit 19 - Filter Enable 4"]
    #[inline(always)]
    pub fn filten4(&mut self) -> FILTEN4_W {
        FILTEN4_W { w: self }
    }
    #[doc = "Bits 20:22 - Input Sense Configuration 5"]
    #[inline(always)]
    pub fn sense5(&mut self) -> SENSE5_W {
        SENSE5_W { w: self }
    }
    #[doc = "Bit 23 - Filter Enable 5"]
    #[inline(always)]
    pub fn filten5(&mut self) -> FILTEN5_W {
        FILTEN5_W { w: self }
    }
    #[doc = "Bits 24:26 - Input Sense Configuration 6"]
    #[inline(always)]
    pub fn sense6(&mut self) -> SENSE6_W {
        SENSE6_W { w: self }
    }
    #[doc = "Bit 27 - Filter Enable 6"]
    #[inline(always)]
    pub fn filten6(&mut self) -> FILTEN6_W {
        FILTEN6_W { w: self }
    }
    #[doc = "Bits 28:30 - Input Sense Configuration 7"]
    #[inline(always)]
    pub fn sense7(&mut self) -> SENSE7_W {
        SENSE7_W { w: self }
    }
    #[doc = "Bit 31 - Filter Enable 7"]
    #[inline(always)]
    pub fn filten7(&mut self) -> FILTEN7_W {
        FILTEN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Sense Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG[%s]
to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
