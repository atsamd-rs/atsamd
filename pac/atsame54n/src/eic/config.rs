#[doc = "Register `CONFIG[%s]` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG[%s]` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `SENSE0` reader - Input Sense Configuration 0"]
pub type SENSE0_R = crate::FieldReader<SENSE0SELECT_A>;
#[doc = "Input Sense Configuration 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SENSE0SELECT_A {
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
impl From<SENSE0SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE0SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SENSE0SELECT_A {
    type Ux = u8;
}
impl SENSE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SENSE0SELECT_A> {
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
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE0SELECT_A::NONE
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE0SELECT_A::RISE
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE0SELECT_A::FALL
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE0SELECT_A::BOTH
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE0SELECT_A::HIGH
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE0SELECT_A::LOW
    }
}
#[doc = "Field `SENSE0` writer - Input Sense Configuration 0"]
pub type SENSE0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SENSE0SELECT_A>;
impl<'a, REG, const O: u8> SENSE0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE0SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE0SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE0SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE0SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE0SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE0SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN0` reader - Filter Enable 0"]
pub type FILTEN0_R = crate::BitReader;
#[doc = "Field `FILTEN0` writer - Filter Enable 0"]
pub type FILTEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SENSE1` reader - Input Sense Configuration 1"]
pub type SENSE1_R = crate::FieldReader<SENSE1SELECT_A>;
#[doc = "Input Sense Configuration 1\n\nValue on reset: 0"]
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
impl crate::FieldSpec for SENSE1SELECT_A {
    type Ux = u8;
}
impl SENSE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SENSE1SELECT_A> {
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
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE1SELECT_A::NONE
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE1SELECT_A::RISE
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE1SELECT_A::FALL
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE1SELECT_A::BOTH
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE1SELECT_A::HIGH
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE1SELECT_A::LOW
    }
}
#[doc = "Field `SENSE1` writer - Input Sense Configuration 1"]
pub type SENSE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SENSE1SELECT_A>;
impl<'a, REG, const O: u8> SENSE1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE1SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE1SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE1SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE1SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE1SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE1SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN1` reader - Filter Enable 1"]
pub type FILTEN1_R = crate::BitReader;
#[doc = "Field `FILTEN1` writer - Filter Enable 1"]
pub type FILTEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SENSE2` reader - Input Sense Configuration 2"]
pub type SENSE2_R = crate::FieldReader<SENSE2SELECT_A>;
#[doc = "Input Sense Configuration 2\n\nValue on reset: 0"]
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
impl crate::FieldSpec for SENSE2SELECT_A {
    type Ux = u8;
}
impl SENSE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SENSE2SELECT_A> {
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
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE2SELECT_A::NONE
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE2SELECT_A::RISE
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE2SELECT_A::FALL
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE2SELECT_A::BOTH
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE2SELECT_A::HIGH
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE2SELECT_A::LOW
    }
}
#[doc = "Field `SENSE2` writer - Input Sense Configuration 2"]
pub type SENSE2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SENSE2SELECT_A>;
impl<'a, REG, const O: u8> SENSE2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE2SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE2SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE2SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE2SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE2SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE2SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN2` reader - Filter Enable 2"]
pub type FILTEN2_R = crate::BitReader;
#[doc = "Field `FILTEN2` writer - Filter Enable 2"]
pub type FILTEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SENSE3` reader - Input Sense Configuration 3"]
pub type SENSE3_R = crate::FieldReader<SENSE3SELECT_A>;
#[doc = "Input Sense Configuration 3\n\nValue on reset: 0"]
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
impl crate::FieldSpec for SENSE3SELECT_A {
    type Ux = u8;
}
impl SENSE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SENSE3SELECT_A> {
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
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE3SELECT_A::NONE
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE3SELECT_A::RISE
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE3SELECT_A::FALL
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE3SELECT_A::BOTH
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE3SELECT_A::HIGH
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE3SELECT_A::LOW
    }
}
#[doc = "Field `SENSE3` writer - Input Sense Configuration 3"]
pub type SENSE3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SENSE3SELECT_A>;
impl<'a, REG, const O: u8> SENSE3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE3SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE3SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE3SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE3SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE3SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE3SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN3` reader - Filter Enable 3"]
pub type FILTEN3_R = crate::BitReader;
#[doc = "Field `FILTEN3` writer - Filter Enable 3"]
pub type FILTEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SENSE4` reader - Input Sense Configuration 4"]
pub type SENSE4_R = crate::FieldReader<SENSE4SELECT_A>;
#[doc = "Input Sense Configuration 4\n\nValue on reset: 0"]
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
impl crate::FieldSpec for SENSE4SELECT_A {
    type Ux = u8;
}
impl SENSE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SENSE4SELECT_A> {
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
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE4SELECT_A::NONE
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE4SELECT_A::RISE
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE4SELECT_A::FALL
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE4SELECT_A::BOTH
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE4SELECT_A::HIGH
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE4SELECT_A::LOW
    }
}
#[doc = "Field `SENSE4` writer - Input Sense Configuration 4"]
pub type SENSE4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SENSE4SELECT_A>;
impl<'a, REG, const O: u8> SENSE4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE4SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE4SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE4SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE4SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE4SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE4SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN4` reader - Filter Enable 4"]
pub type FILTEN4_R = crate::BitReader;
#[doc = "Field `FILTEN4` writer - Filter Enable 4"]
pub type FILTEN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SENSE5` reader - Input Sense Configuration 5"]
pub type SENSE5_R = crate::FieldReader<SENSE5SELECT_A>;
#[doc = "Input Sense Configuration 5\n\nValue on reset: 0"]
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
impl crate::FieldSpec for SENSE5SELECT_A {
    type Ux = u8;
}
impl SENSE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SENSE5SELECT_A> {
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
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE5SELECT_A::NONE
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE5SELECT_A::RISE
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE5SELECT_A::FALL
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE5SELECT_A::BOTH
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE5SELECT_A::HIGH
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE5SELECT_A::LOW
    }
}
#[doc = "Field `SENSE5` writer - Input Sense Configuration 5"]
pub type SENSE5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SENSE5SELECT_A>;
impl<'a, REG, const O: u8> SENSE5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE5SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE5SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE5SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE5SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE5SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE5SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN5` reader - Filter Enable 5"]
pub type FILTEN5_R = crate::BitReader;
#[doc = "Field `FILTEN5` writer - Filter Enable 5"]
pub type FILTEN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SENSE6` reader - Input Sense Configuration 6"]
pub type SENSE6_R = crate::FieldReader<SENSE6SELECT_A>;
#[doc = "Input Sense Configuration 6\n\nValue on reset: 0"]
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
impl crate::FieldSpec for SENSE6SELECT_A {
    type Ux = u8;
}
impl SENSE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SENSE6SELECT_A> {
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
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE6SELECT_A::NONE
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE6SELECT_A::RISE
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE6SELECT_A::FALL
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE6SELECT_A::BOTH
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE6SELECT_A::HIGH
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE6SELECT_A::LOW
    }
}
#[doc = "Field `SENSE6` writer - Input Sense Configuration 6"]
pub type SENSE6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SENSE6SELECT_A>;
impl<'a, REG, const O: u8> SENSE6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE6SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE6SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE6SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE6SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE6SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE6SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN6` reader - Filter Enable 6"]
pub type FILTEN6_R = crate::BitReader;
#[doc = "Field `FILTEN6` writer - Filter Enable 6"]
pub type FILTEN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SENSE7` reader - Input Sense Configuration 7"]
pub type SENSE7_R = crate::FieldReader<SENSE7SELECT_A>;
#[doc = "Input Sense Configuration 7\n\nValue on reset: 0"]
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
impl crate::FieldSpec for SENSE7SELECT_A {
    type Ux = u8;
}
impl SENSE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SENSE7SELECT_A> {
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
    #[doc = "No detection"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SENSE7SELECT_A::NONE
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == SENSE7SELECT_A::RISE
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == SENSE7SELECT_A::FALL
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SENSE7SELECT_A::BOTH
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE7SELECT_A::HIGH
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE7SELECT_A::LOW
    }
}
#[doc = "Field `SENSE7` writer - Input Sense Configuration 7"]
pub type SENSE7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SENSE7SELECT_A>;
impl<'a, REG, const O: u8> SENSE7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE7SELECT_A::NONE)
    }
    #[doc = "Rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE7SELECT_A::RISE)
    }
    #[doc = "Falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE7SELECT_A::FALL)
    }
    #[doc = "Both edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE7SELECT_A::BOTH)
    }
    #[doc = "High level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE7SELECT_A::HIGH)
    }
    #[doc = "Low level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SENSE7SELECT_A::LOW)
    }
}
#[doc = "Field `FILTEN7` reader - Filter Enable 7"]
pub type FILTEN7_R = crate::BitReader;
#[doc = "Field `FILTEN7` writer - Filter Enable 7"]
pub type FILTEN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Input Sense Configuration 0"]
    #[inline(always)]
    pub fn sense0(&self) -> SENSE0_R {
        SENSE0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Filter Enable 0"]
    #[inline(always)]
    pub fn filten0(&self) -> FILTEN0_R {
        FILTEN0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Input Sense Configuration 1"]
    #[inline(always)]
    pub fn sense1(&self) -> SENSE1_R {
        SENSE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Filter Enable 1"]
    #[inline(always)]
    pub fn filten1(&self) -> FILTEN1_R {
        FILTEN1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Input Sense Configuration 2"]
    #[inline(always)]
    pub fn sense2(&self) -> SENSE2_R {
        SENSE2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Filter Enable 2"]
    #[inline(always)]
    pub fn filten2(&self) -> FILTEN2_R {
        FILTEN2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Input Sense Configuration 3"]
    #[inline(always)]
    pub fn sense3(&self) -> SENSE3_R {
        SENSE3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Filter Enable 3"]
    #[inline(always)]
    pub fn filten3(&self) -> FILTEN3_R {
        FILTEN3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Input Sense Configuration 4"]
    #[inline(always)]
    pub fn sense4(&self) -> SENSE4_R {
        SENSE4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Filter Enable 4"]
    #[inline(always)]
    pub fn filten4(&self) -> FILTEN4_R {
        FILTEN4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Input Sense Configuration 5"]
    #[inline(always)]
    pub fn sense5(&self) -> SENSE5_R {
        SENSE5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Filter Enable 5"]
    #[inline(always)]
    pub fn filten5(&self) -> FILTEN5_R {
        FILTEN5_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Input Sense Configuration 6"]
    #[inline(always)]
    pub fn sense6(&self) -> SENSE6_R {
        SENSE6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Filter Enable 6"]
    #[inline(always)]
    pub fn filten6(&self) -> FILTEN6_R {
        FILTEN6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Input Sense Configuration 7"]
    #[inline(always)]
    pub fn sense7(&self) -> SENSE7_R {
        SENSE7_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Filter Enable 7"]
    #[inline(always)]
    pub fn filten7(&self) -> FILTEN7_R {
        FILTEN7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Sense Configuration 0"]
    #[inline(always)]
    #[must_use]
    pub fn sense0(&mut self) -> SENSE0_W<CONFIG_SPEC, 0> {
        SENSE0_W::new(self)
    }
    #[doc = "Bit 3 - Filter Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn filten0(&mut self) -> FILTEN0_W<CONFIG_SPEC, 3> {
        FILTEN0_W::new(self)
    }
    #[doc = "Bits 4:6 - Input Sense Configuration 1"]
    #[inline(always)]
    #[must_use]
    pub fn sense1(&mut self) -> SENSE1_W<CONFIG_SPEC, 4> {
        SENSE1_W::new(self)
    }
    #[doc = "Bit 7 - Filter Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn filten1(&mut self) -> FILTEN1_W<CONFIG_SPEC, 7> {
        FILTEN1_W::new(self)
    }
    #[doc = "Bits 8:10 - Input Sense Configuration 2"]
    #[inline(always)]
    #[must_use]
    pub fn sense2(&mut self) -> SENSE2_W<CONFIG_SPEC, 8> {
        SENSE2_W::new(self)
    }
    #[doc = "Bit 11 - Filter Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn filten2(&mut self) -> FILTEN2_W<CONFIG_SPEC, 11> {
        FILTEN2_W::new(self)
    }
    #[doc = "Bits 12:14 - Input Sense Configuration 3"]
    #[inline(always)]
    #[must_use]
    pub fn sense3(&mut self) -> SENSE3_W<CONFIG_SPEC, 12> {
        SENSE3_W::new(self)
    }
    #[doc = "Bit 15 - Filter Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn filten3(&mut self) -> FILTEN3_W<CONFIG_SPEC, 15> {
        FILTEN3_W::new(self)
    }
    #[doc = "Bits 16:18 - Input Sense Configuration 4"]
    #[inline(always)]
    #[must_use]
    pub fn sense4(&mut self) -> SENSE4_W<CONFIG_SPEC, 16> {
        SENSE4_W::new(self)
    }
    #[doc = "Bit 19 - Filter Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn filten4(&mut self) -> FILTEN4_W<CONFIG_SPEC, 19> {
        FILTEN4_W::new(self)
    }
    #[doc = "Bits 20:22 - Input Sense Configuration 5"]
    #[inline(always)]
    #[must_use]
    pub fn sense5(&mut self) -> SENSE5_W<CONFIG_SPEC, 20> {
        SENSE5_W::new(self)
    }
    #[doc = "Bit 23 - Filter Enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn filten5(&mut self) -> FILTEN5_W<CONFIG_SPEC, 23> {
        FILTEN5_W::new(self)
    }
    #[doc = "Bits 24:26 - Input Sense Configuration 6"]
    #[inline(always)]
    #[must_use]
    pub fn sense6(&mut self) -> SENSE6_W<CONFIG_SPEC, 24> {
        SENSE6_W::new(self)
    }
    #[doc = "Bit 27 - Filter Enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn filten6(&mut self) -> FILTEN6_W<CONFIG_SPEC, 27> {
        FILTEN6_W::new(self)
    }
    #[doc = "Bits 28:30 - Input Sense Configuration 7"]
    #[inline(always)]
    #[must_use]
    pub fn sense7(&mut self) -> SENSE7_W<CONFIG_SPEC, 28> {
        SENSE7_W::new(self)
    }
    #[doc = "Bit 31 - Filter Enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn filten7(&mut self) -> FILTEN7_W<CONFIG_SPEC, 31> {
        FILTEN7_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "External Interrupt Sense Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG[%s]
to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
