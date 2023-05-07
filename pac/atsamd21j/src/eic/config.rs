#[doc = "Register `CONFIG%s` reader"]
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
#[doc = "Register `CONFIG%s` writer"]
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
#[doc = "Field `SENSE0` reader - Input Sense 0 Configuration"]
pub type SENSE0_R = crate::FieldReader<u8, SENSE0SELECT_A>;
#[doc = "Input Sense 0 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SENSE0SELECT_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising-edge detection"]
    RISE = 1,
    #[doc = "2: Falling-edge detection"]
    FALL = 2,
    #[doc = "3: Both-edges detection"]
    BOTH = 3,
    #[doc = "4: High-level detection"]
    HIGH = 4,
    #[doc = "5: Low-level detection"]
    LOW = 5,
}
impl From<SENSE0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE0SELECT_A) -> Self {
        variant as _
    }
}
impl SENSE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE0SELECT_A> {
        match self.bits {
            0 => Some(SENSE0SELECT_A::NONE),
            1 => Some(SENSE0SELECT_A::RISE),
            2 => Some(SENSE0SELECT_A::FALL),
            3 => Some(SENSE0SELECT_A::BOTH),
            4 => Some(SENSE0SELECT_A::HIGH),
            5 => Some(SENSE0SELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE0SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE0SELECT_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE0SELECT_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE0SELECT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE0SELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE0SELECT_A::LOW
    }
}
#[doc = "Field `SENSE0` writer - Input Sense 0 Configuration"]
pub type SENSE0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, SENSE0SELECT_A, 3, O>;
impl<'a, const O: u8> SENSE0_W<'a, O> {
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE0SELECT_A::NONE)
    }
    #[doc = "Rising-edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE0SELECT_A::RISE)
    }
    #[doc = "Falling-edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE0SELECT_A::FALL)
    }
    #[doc = "Both-edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE0SELECT_A::BOTH)
    }
    #[doc = "High-level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE0SELECT_A::HIGH)
    }
    #[doc = "Low-level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE0SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN0` reader - Filter 0 Enable"]
pub type FILTEN0_R = crate::BitReader<bool>;
#[doc = "Field `FILTEN0` writer - Filter 0 Enable"]
pub type FILTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `SENSE1` reader - Input Sense 1 Configuration"]
pub type SENSE1_R = crate::FieldReader<u8, SENSE1SELECT_A>;
#[doc = "Input Sense 1 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SENSE1SELECT_A {
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
impl From<SENSE1SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE1SELECT_A) -> Self {
        variant as _
    }
}
impl SENSE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE1SELECT_A> {
        match self.bits {
            0 => Some(SENSE1SELECT_A::NONE),
            1 => Some(SENSE1SELECT_A::RISE),
            2 => Some(SENSE1SELECT_A::FALL),
            3 => Some(SENSE1SELECT_A::BOTH),
            4 => Some(SENSE1SELECT_A::HIGH),
            5 => Some(SENSE1SELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE1SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE1SELECT_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE1SELECT_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE1SELECT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE1SELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE1SELECT_A::LOW
    }
}
#[doc = "Field `SENSE1` writer - Input Sense 1 Configuration"]
pub type SENSE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, SENSE1SELECT_A, 3, O>;
impl<'a, const O: u8> SENSE1_W<'a, O> {
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE1SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE1SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE1SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE1SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE1SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE1SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN1` reader - Filter 1 Enable"]
pub type FILTEN1_R = crate::BitReader<bool>;
#[doc = "Field `FILTEN1` writer - Filter 1 Enable"]
pub type FILTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `SENSE2` reader - Input Sense 2 Configuration"]
pub type SENSE2_R = crate::FieldReader<u8, SENSE2SELECT_A>;
#[doc = "Input Sense 2 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SENSE2SELECT_A {
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
impl From<SENSE2SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE2SELECT_A) -> Self {
        variant as _
    }
}
impl SENSE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE2SELECT_A> {
        match self.bits {
            0 => Some(SENSE2SELECT_A::NONE),
            1 => Some(SENSE2SELECT_A::RISE),
            2 => Some(SENSE2SELECT_A::FALL),
            3 => Some(SENSE2SELECT_A::BOTH),
            4 => Some(SENSE2SELECT_A::HIGH),
            5 => Some(SENSE2SELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE2SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE2SELECT_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE2SELECT_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE2SELECT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE2SELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE2SELECT_A::LOW
    }
}
#[doc = "Field `SENSE2` writer - Input Sense 2 Configuration"]
pub type SENSE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, SENSE2SELECT_A, 3, O>;
impl<'a, const O: u8> SENSE2_W<'a, O> {
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE2SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE2SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE2SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE2SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE2SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE2SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN2` reader - Filter 2 Enable"]
pub type FILTEN2_R = crate::BitReader<bool>;
#[doc = "Field `FILTEN2` writer - Filter 2 Enable"]
pub type FILTEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `SENSE3` reader - Input Sense 3 Configuration"]
pub type SENSE3_R = crate::FieldReader<u8, SENSE3SELECT_A>;
#[doc = "Input Sense 3 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SENSE3SELECT_A {
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
impl From<SENSE3SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE3SELECT_A) -> Self {
        variant as _
    }
}
impl SENSE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE3SELECT_A> {
        match self.bits {
            0 => Some(SENSE3SELECT_A::NONE),
            1 => Some(SENSE3SELECT_A::RISE),
            2 => Some(SENSE3SELECT_A::FALL),
            3 => Some(SENSE3SELECT_A::BOTH),
            4 => Some(SENSE3SELECT_A::HIGH),
            5 => Some(SENSE3SELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE3SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE3SELECT_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE3SELECT_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE3SELECT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE3SELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE3SELECT_A::LOW
    }
}
#[doc = "Field `SENSE3` writer - Input Sense 3 Configuration"]
pub type SENSE3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, SENSE3SELECT_A, 3, O>;
impl<'a, const O: u8> SENSE3_W<'a, O> {
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE3SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE3SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE3SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE3SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE3SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE3SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN3` reader - Filter 3 Enable"]
pub type FILTEN3_R = crate::BitReader<bool>;
#[doc = "Field `FILTEN3` writer - Filter 3 Enable"]
pub type FILTEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `SENSE4` reader - Input Sense 4 Configuration"]
pub type SENSE4_R = crate::FieldReader<u8, SENSE4SELECT_A>;
#[doc = "Input Sense 4 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SENSE4SELECT_A {
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
impl From<SENSE4SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE4SELECT_A) -> Self {
        variant as _
    }
}
impl SENSE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE4SELECT_A> {
        match self.bits {
            0 => Some(SENSE4SELECT_A::NONE),
            1 => Some(SENSE4SELECT_A::RISE),
            2 => Some(SENSE4SELECT_A::FALL),
            3 => Some(SENSE4SELECT_A::BOTH),
            4 => Some(SENSE4SELECT_A::HIGH),
            5 => Some(SENSE4SELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE4SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE4SELECT_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE4SELECT_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE4SELECT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE4SELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE4SELECT_A::LOW
    }
}
#[doc = "Field `SENSE4` writer - Input Sense 4 Configuration"]
pub type SENSE4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, SENSE4SELECT_A, 3, O>;
impl<'a, const O: u8> SENSE4_W<'a, O> {
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE4SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE4SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE4SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE4SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE4SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE4SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN4` reader - Filter 4 Enable"]
pub type FILTEN4_R = crate::BitReader<bool>;
#[doc = "Field `FILTEN4` writer - Filter 4 Enable"]
pub type FILTEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `SENSE5` reader - Input Sense 5 Configuration"]
pub type SENSE5_R = crate::FieldReader<u8, SENSE5SELECT_A>;
#[doc = "Input Sense 5 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SENSE5SELECT_A {
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
impl From<SENSE5SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE5SELECT_A) -> Self {
        variant as _
    }
}
impl SENSE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE5SELECT_A> {
        match self.bits {
            0 => Some(SENSE5SELECT_A::NONE),
            1 => Some(SENSE5SELECT_A::RISE),
            2 => Some(SENSE5SELECT_A::FALL),
            3 => Some(SENSE5SELECT_A::BOTH),
            4 => Some(SENSE5SELECT_A::HIGH),
            5 => Some(SENSE5SELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE5SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE5SELECT_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE5SELECT_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE5SELECT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE5SELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE5SELECT_A::LOW
    }
}
#[doc = "Field `SENSE5` writer - Input Sense 5 Configuration"]
pub type SENSE5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, SENSE5SELECT_A, 3, O>;
impl<'a, const O: u8> SENSE5_W<'a, O> {
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE5SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE5SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE5SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE5SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE5SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE5SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN5` reader - Filter 5 Enable"]
pub type FILTEN5_R = crate::BitReader<bool>;
#[doc = "Field `FILTEN5` writer - Filter 5 Enable"]
pub type FILTEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `SENSE6` reader - Input Sense 6 Configuration"]
pub type SENSE6_R = crate::FieldReader<u8, SENSE6SELECT_A>;
#[doc = "Input Sense 6 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SENSE6SELECT_A {
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
impl From<SENSE6SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE6SELECT_A) -> Self {
        variant as _
    }
}
impl SENSE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE6SELECT_A> {
        match self.bits {
            0 => Some(SENSE6SELECT_A::NONE),
            1 => Some(SENSE6SELECT_A::RISE),
            2 => Some(SENSE6SELECT_A::FALL),
            3 => Some(SENSE6SELECT_A::BOTH),
            4 => Some(SENSE6SELECT_A::HIGH),
            5 => Some(SENSE6SELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE6SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE6SELECT_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE6SELECT_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE6SELECT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE6SELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE6SELECT_A::LOW
    }
}
#[doc = "Field `SENSE6` writer - Input Sense 6 Configuration"]
pub type SENSE6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, SENSE6SELECT_A, 3, O>;
impl<'a, const O: u8> SENSE6_W<'a, O> {
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE6SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE6SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE6SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE6SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE6SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE6SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN6` reader - Filter 6 Enable"]
pub type FILTEN6_R = crate::BitReader<bool>;
#[doc = "Field `FILTEN6` writer - Filter 6 Enable"]
pub type FILTEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `SENSE7` reader - Input Sense 7 Configuration"]
pub type SENSE7_R = crate::FieldReader<u8, SENSE7SELECT_A>;
#[doc = "Input Sense 7 Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SENSE7SELECT_A {
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
impl From<SENSE7SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE7SELECT_A) -> Self {
        variant as _
    }
}
impl SENSE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE7SELECT_A> {
        match self.bits {
            0 => Some(SENSE7SELECT_A::NONE),
            1 => Some(SENSE7SELECT_A::RISE),
            2 => Some(SENSE7SELECT_A::FALL),
            3 => Some(SENSE7SELECT_A::BOTH),
            4 => Some(SENSE7SELECT_A::HIGH),
            5 => Some(SENSE7SELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE7SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE7SELECT_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE7SELECT_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE7SELECT_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE7SELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE7SELECT_A::LOW
    }
}
#[doc = "Field `SENSE7` writer - Input Sense 7 Configuration"]
pub type SENSE7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, SENSE7SELECT_A, 3, O>;
impl<'a, const O: u8> SENSE7_W<'a, O> {
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SENSE7SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(SENSE7SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(SENSE7SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SENSE7SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE7SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE7SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN7` reader - Filter 7 Enable"]
pub type FILTEN7_R = crate::BitReader<bool>;
#[doc = "Field `FILTEN7` writer - Filter 7 Enable"]
pub type FILTEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Input Sense 0 Configuration"]
    #[inline(always)]
    pub fn sense0(&self) -> SENSE0_R {
        SENSE0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Filter 0 Enable"]
    #[inline(always)]
    pub fn filten0(&self) -> FILTEN0_R {
        FILTEN0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Input Sense 1 Configuration"]
    #[inline(always)]
    pub fn sense1(&self) -> SENSE1_R {
        SENSE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Filter 1 Enable"]
    #[inline(always)]
    pub fn filten1(&self) -> FILTEN1_R {
        FILTEN1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Input Sense 2 Configuration"]
    #[inline(always)]
    pub fn sense2(&self) -> SENSE2_R {
        SENSE2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Filter 2 Enable"]
    #[inline(always)]
    pub fn filten2(&self) -> FILTEN2_R {
        FILTEN2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Input Sense 3 Configuration"]
    #[inline(always)]
    pub fn sense3(&self) -> SENSE3_R {
        SENSE3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Filter 3 Enable"]
    #[inline(always)]
    pub fn filten3(&self) -> FILTEN3_R {
        FILTEN3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Input Sense 4 Configuration"]
    #[inline(always)]
    pub fn sense4(&self) -> SENSE4_R {
        SENSE4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Filter 4 Enable"]
    #[inline(always)]
    pub fn filten4(&self) -> FILTEN4_R {
        FILTEN4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Input Sense 5 Configuration"]
    #[inline(always)]
    pub fn sense5(&self) -> SENSE5_R {
        SENSE5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Filter 5 Enable"]
    #[inline(always)]
    pub fn filten5(&self) -> FILTEN5_R {
        FILTEN5_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Input Sense 6 Configuration"]
    #[inline(always)]
    pub fn sense6(&self) -> SENSE6_R {
        SENSE6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Filter 6 Enable"]
    #[inline(always)]
    pub fn filten6(&self) -> FILTEN6_R {
        FILTEN6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Input Sense 7 Configuration"]
    #[inline(always)]
    pub fn sense7(&self) -> SENSE7_R {
        SENSE7_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Filter 7 Enable"]
    #[inline(always)]
    pub fn filten7(&self) -> FILTEN7_R {
        FILTEN7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Sense 0 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sense0(&mut self) -> SENSE0_W<0> {
        SENSE0_W::new(self)
    }
    #[doc = "Bit 3 - Filter 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filten0(&mut self) -> FILTEN0_W<3> {
        FILTEN0_W::new(self)
    }
    #[doc = "Bits 4:6 - Input Sense 1 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sense1(&mut self) -> SENSE1_W<4> {
        SENSE1_W::new(self)
    }
    #[doc = "Bit 7 - Filter 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filten1(&mut self) -> FILTEN1_W<7> {
        FILTEN1_W::new(self)
    }
    #[doc = "Bits 8:10 - Input Sense 2 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sense2(&mut self) -> SENSE2_W<8> {
        SENSE2_W::new(self)
    }
    #[doc = "Bit 11 - Filter 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filten2(&mut self) -> FILTEN2_W<11> {
        FILTEN2_W::new(self)
    }
    #[doc = "Bits 12:14 - Input Sense 3 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sense3(&mut self) -> SENSE3_W<12> {
        SENSE3_W::new(self)
    }
    #[doc = "Bit 15 - Filter 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filten3(&mut self) -> FILTEN3_W<15> {
        FILTEN3_W::new(self)
    }
    #[doc = "Bits 16:18 - Input Sense 4 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sense4(&mut self) -> SENSE4_W<16> {
        SENSE4_W::new(self)
    }
    #[doc = "Bit 19 - Filter 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filten4(&mut self) -> FILTEN4_W<19> {
        FILTEN4_W::new(self)
    }
    #[doc = "Bits 20:22 - Input Sense 5 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sense5(&mut self) -> SENSE5_W<20> {
        SENSE5_W::new(self)
    }
    #[doc = "Bit 23 - Filter 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filten5(&mut self) -> FILTEN5_W<23> {
        FILTEN5_W::new(self)
    }
    #[doc = "Bits 24:26 - Input Sense 6 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sense6(&mut self) -> SENSE6_W<24> {
        SENSE6_W::new(self)
    }
    #[doc = "Bit 27 - Filter 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filten6(&mut self) -> FILTEN6_W<27> {
        FILTEN6_W::new(self)
    }
    #[doc = "Bits 28:30 - Input Sense 7 Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sense7(&mut self) -> SENSE7_W<28> {
        SENSE7_W::new(self)
    }
    #[doc = "Bit 31 - Filter 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filten7(&mut self) -> FILTEN7_W<31> {
        FILTEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG%s to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
