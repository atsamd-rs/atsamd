#[doc = "Reader of register CONFIG[%s]"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG[%s]"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `SENSE0`"]
pub type SENSE0_R = crate::R<u8, SENSE0_A>;
impl SENSE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SENSE0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SENSE0_A::NONE),
            1 => Val(SENSE0_A::RISE),
            2 => Val(SENSE0_A::FALL),
            3 => Val(SENSE0_A::BOTH),
            4 => Val(SENSE0_A::HIGH),
            5 => Val(SENSE0_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE0_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE0_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE0_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE0_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE0_A::LOW
    }
}
#[doc = "Write proxy for field `SENSE0`"]
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `FILTEN0`"]
pub type FILTEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTEN0`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
#[doc = "Reader of field `SENSE1`"]
pub type SENSE1_R = crate::R<u8, SENSE1_A>;
impl SENSE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SENSE1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SENSE1_A::NONE),
            1 => Val(SENSE1_A::RISE),
            2 => Val(SENSE1_A::FALL),
            3 => Val(SENSE1_A::BOTH),
            4 => Val(SENSE1_A::HIGH),
            5 => Val(SENSE1_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE1_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE1_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE1_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE1_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE1_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE1_A::LOW
    }
}
#[doc = "Write proxy for field `SENSE1`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `FILTEN1`"]
pub type FILTEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTEN1`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
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
#[doc = "Reader of field `SENSE2`"]
pub type SENSE2_R = crate::R<u8, SENSE2_A>;
impl SENSE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SENSE2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SENSE2_A::NONE),
            1 => Val(SENSE2_A::RISE),
            2 => Val(SENSE2_A::FALL),
            3 => Val(SENSE2_A::BOTH),
            4 => Val(SENSE2_A::HIGH),
            5 => Val(SENSE2_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE2_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE2_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE2_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE2_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE2_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE2_A::LOW
    }
}
#[doc = "Write proxy for field `SENSE2`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `FILTEN2`"]
pub type FILTEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTEN2`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
#[doc = "Reader of field `SENSE3`"]
pub type SENSE3_R = crate::R<u8, SENSE3_A>;
impl SENSE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SENSE3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SENSE3_A::NONE),
            1 => Val(SENSE3_A::RISE),
            2 => Val(SENSE3_A::FALL),
            3 => Val(SENSE3_A::BOTH),
            4 => Val(SENSE3_A::HIGH),
            5 => Val(SENSE3_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE3_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE3_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE3_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE3_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE3_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE3_A::LOW
    }
}
#[doc = "Write proxy for field `SENSE3`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `FILTEN3`"]
pub type FILTEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTEN3`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
#[doc = "Reader of field `SENSE4`"]
pub type SENSE4_R = crate::R<u8, SENSE4_A>;
impl SENSE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SENSE4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SENSE4_A::NONE),
            1 => Val(SENSE4_A::RISE),
            2 => Val(SENSE4_A::FALL),
            3 => Val(SENSE4_A::BOTH),
            4 => Val(SENSE4_A::HIGH),
            5 => Val(SENSE4_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE4_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE4_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE4_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE4_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE4_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE4_A::LOW
    }
}
#[doc = "Write proxy for field `SENSE4`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `FILTEN4`"]
pub type FILTEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTEN4`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
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
#[doc = "Reader of field `SENSE5`"]
pub type SENSE5_R = crate::R<u8, SENSE5_A>;
impl SENSE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SENSE5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SENSE5_A::NONE),
            1 => Val(SENSE5_A::RISE),
            2 => Val(SENSE5_A::FALL),
            3 => Val(SENSE5_A::BOTH),
            4 => Val(SENSE5_A::HIGH),
            5 => Val(SENSE5_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE5_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE5_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE5_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE5_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE5_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE5_A::LOW
    }
}
#[doc = "Write proxy for field `SENSE5`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `FILTEN5`"]
pub type FILTEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTEN5`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
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
#[doc = "Reader of field `SENSE6`"]
pub type SENSE6_R = crate::R<u8, SENSE6_A>;
impl SENSE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SENSE6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SENSE6_A::NONE),
            1 => Val(SENSE6_A::RISE),
            2 => Val(SENSE6_A::FALL),
            3 => Val(SENSE6_A::BOTH),
            4 => Val(SENSE6_A::HIGH),
            5 => Val(SENSE6_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE6_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE6_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE6_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE6_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE6_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE6_A::LOW
    }
}
#[doc = "Write proxy for field `SENSE6`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `FILTEN6`"]
pub type FILTEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTEN6`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
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
#[doc = "Reader of field `SENSE7`"]
pub type SENSE7_R = crate::R<u8, SENSE7_A>;
impl SENSE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SENSE7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SENSE7_A::NONE),
            1 => Val(SENSE7_A::RISE),
            2 => Val(SENSE7_A::FALL),
            3 => Val(SENSE7_A::BOTH),
            4 => Val(SENSE7_A::HIGH),
            5 => Val(SENSE7_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE7_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE7_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE7_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE7_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE7_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE7_A::LOW
    }
}
#[doc = "Write proxy for field `SENSE7`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `FILTEN7`"]
pub type FILTEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTEN7`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
