#[doc = "Register `TAMPCTRL` reader"]
pub struct R(crate::R<TAMPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMPCTRL` writer"]
pub struct W(crate::W<TAMPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMPCTRL_SPEC>;
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
impl From<crate::W<TAMPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN0ACT` reader - Tamper Input 0 Action"]
pub type IN0ACT_R = crate::FieldReader<u8, IN0ACTSELECT_A>;
#[doc = "Tamper Input 0 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IN0ACTSELECT_A {
    #[doc = "0: Off (Disabled)"]
    OFF = 0,
    #[doc = "1: Wake without timestamp"]
    WAKE = 1,
    #[doc = "2: Capture timestamp"]
    CAPTURE = 2,
    #[doc = "3: Compare IN0 to OUT"]
    ACTL = 3,
}
impl From<IN0ACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IN0ACTSELECT_A) -> Self {
        variant as _
    }
}
impl IN0ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN0ACTSELECT_A {
        match self.bits {
            0 => IN0ACTSELECT_A::OFF,
            1 => IN0ACTSELECT_A::WAKE,
            2 => IN0ACTSELECT_A::CAPTURE,
            3 => IN0ACTSELECT_A::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IN0ACTSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == IN0ACTSELECT_A::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == IN0ACTSELECT_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == IN0ACTSELECT_A::ACTL
    }
}
#[doc = "Field `IN0ACT` writer - Tamper Input 0 Action"]
pub type IN0ACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAMPCTRL_SPEC, u8, IN0ACTSELECT_A, 2, O>;
impl<'a, const O: u8> IN0ACT_W<'a, O> {
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IN0ACTSELECT_A::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN0ACTSELECT_A::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN0ACTSELECT_A::CAPTURE)
    }
    #[doc = "Compare IN0 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN0ACTSELECT_A::ACTL)
    }
}
#[doc = "Field `IN1ACT` reader - Tamper Input 1 Action"]
pub type IN1ACT_R = crate::FieldReader<u8, IN1ACTSELECT_A>;
#[doc = "Tamper Input 1 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IN1ACTSELECT_A {
    #[doc = "0: Off (Disabled)"]
    OFF = 0,
    #[doc = "1: Wake without timestamp"]
    WAKE = 1,
    #[doc = "2: Capture timestamp"]
    CAPTURE = 2,
    #[doc = "3: Compare IN1 to OUT"]
    ACTL = 3,
}
impl From<IN1ACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IN1ACTSELECT_A) -> Self {
        variant as _
    }
}
impl IN1ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN1ACTSELECT_A {
        match self.bits {
            0 => IN1ACTSELECT_A::OFF,
            1 => IN1ACTSELECT_A::WAKE,
            2 => IN1ACTSELECT_A::CAPTURE,
            3 => IN1ACTSELECT_A::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IN1ACTSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == IN1ACTSELECT_A::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == IN1ACTSELECT_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == IN1ACTSELECT_A::ACTL
    }
}
#[doc = "Field `IN1ACT` writer - Tamper Input 1 Action"]
pub type IN1ACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAMPCTRL_SPEC, u8, IN1ACTSELECT_A, 2, O>;
impl<'a, const O: u8> IN1ACT_W<'a, O> {
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IN1ACTSELECT_A::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN1ACTSELECT_A::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN1ACTSELECT_A::CAPTURE)
    }
    #[doc = "Compare IN1 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN1ACTSELECT_A::ACTL)
    }
}
#[doc = "Field `IN2ACT` reader - Tamper Input 2 Action"]
pub type IN2ACT_R = crate::FieldReader<u8, IN2ACTSELECT_A>;
#[doc = "Tamper Input 2 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IN2ACTSELECT_A {
    #[doc = "0: Off (Disabled)"]
    OFF = 0,
    #[doc = "1: Wake without timestamp"]
    WAKE = 1,
    #[doc = "2: Capture timestamp"]
    CAPTURE = 2,
    #[doc = "3: Compare IN2 to OUT"]
    ACTL = 3,
}
impl From<IN2ACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IN2ACTSELECT_A) -> Self {
        variant as _
    }
}
impl IN2ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN2ACTSELECT_A {
        match self.bits {
            0 => IN2ACTSELECT_A::OFF,
            1 => IN2ACTSELECT_A::WAKE,
            2 => IN2ACTSELECT_A::CAPTURE,
            3 => IN2ACTSELECT_A::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IN2ACTSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == IN2ACTSELECT_A::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == IN2ACTSELECT_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == IN2ACTSELECT_A::ACTL
    }
}
#[doc = "Field `IN2ACT` writer - Tamper Input 2 Action"]
pub type IN2ACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAMPCTRL_SPEC, u8, IN2ACTSELECT_A, 2, O>;
impl<'a, const O: u8> IN2ACT_W<'a, O> {
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IN2ACTSELECT_A::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN2ACTSELECT_A::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN2ACTSELECT_A::CAPTURE)
    }
    #[doc = "Compare IN2 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN2ACTSELECT_A::ACTL)
    }
}
#[doc = "Field `IN3ACT` reader - Tamper Input 3 Action"]
pub type IN3ACT_R = crate::FieldReader<u8, IN3ACTSELECT_A>;
#[doc = "Tamper Input 3 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IN3ACTSELECT_A {
    #[doc = "0: Off (Disabled)"]
    OFF = 0,
    #[doc = "1: Wake without timestamp"]
    WAKE = 1,
    #[doc = "2: Capture timestamp"]
    CAPTURE = 2,
    #[doc = "3: Compare IN3 to OUT"]
    ACTL = 3,
}
impl From<IN3ACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IN3ACTSELECT_A) -> Self {
        variant as _
    }
}
impl IN3ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN3ACTSELECT_A {
        match self.bits {
            0 => IN3ACTSELECT_A::OFF,
            1 => IN3ACTSELECT_A::WAKE,
            2 => IN3ACTSELECT_A::CAPTURE,
            3 => IN3ACTSELECT_A::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IN3ACTSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == IN3ACTSELECT_A::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == IN3ACTSELECT_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == IN3ACTSELECT_A::ACTL
    }
}
#[doc = "Field `IN3ACT` writer - Tamper Input 3 Action"]
pub type IN3ACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAMPCTRL_SPEC, u8, IN3ACTSELECT_A, 2, O>;
impl<'a, const O: u8> IN3ACT_W<'a, O> {
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IN3ACTSELECT_A::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN3ACTSELECT_A::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN3ACTSELECT_A::CAPTURE)
    }
    #[doc = "Compare IN3 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN3ACTSELECT_A::ACTL)
    }
}
#[doc = "Field `IN4ACT` reader - Tamper Input 4 Action"]
pub type IN4ACT_R = crate::FieldReader<u8, IN4ACTSELECT_A>;
#[doc = "Tamper Input 4 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IN4ACTSELECT_A {
    #[doc = "0: Off (Disabled)"]
    OFF = 0,
    #[doc = "1: Wake without timestamp"]
    WAKE = 1,
    #[doc = "2: Capture timestamp"]
    CAPTURE = 2,
    #[doc = "3: Compare IN4 to OUT"]
    ACTL = 3,
}
impl From<IN4ACTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IN4ACTSELECT_A) -> Self {
        variant as _
    }
}
impl IN4ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN4ACTSELECT_A {
        match self.bits {
            0 => IN4ACTSELECT_A::OFF,
            1 => IN4ACTSELECT_A::WAKE,
            2 => IN4ACTSELECT_A::CAPTURE,
            3 => IN4ACTSELECT_A::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IN4ACTSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == IN4ACTSELECT_A::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == IN4ACTSELECT_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == IN4ACTSELECT_A::ACTL
    }
}
#[doc = "Field `IN4ACT` writer - Tamper Input 4 Action"]
pub type IN4ACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAMPCTRL_SPEC, u8, IN4ACTSELECT_A, 2, O>;
impl<'a, const O: u8> IN4ACT_W<'a, O> {
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IN4ACTSELECT_A::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN4ACTSELECT_A::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN4ACTSELECT_A::CAPTURE)
    }
    #[doc = "Compare IN4 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN4ACTSELECT_A::ACTL)
    }
}
#[doc = "Field `TAMLVL0` reader - Tamper Level Select 0"]
pub type TAMLVL0_R = crate::BitReader<bool>;
#[doc = "Field `TAMLVL0` writer - Tamper Level Select 0"]
pub type TAMLVL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCTRL_SPEC, bool, O>;
#[doc = "Field `TAMLVL1` reader - Tamper Level Select 1"]
pub type TAMLVL1_R = crate::BitReader<bool>;
#[doc = "Field `TAMLVL1` writer - Tamper Level Select 1"]
pub type TAMLVL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCTRL_SPEC, bool, O>;
#[doc = "Field `TAMLVL2` reader - Tamper Level Select 2"]
pub type TAMLVL2_R = crate::BitReader<bool>;
#[doc = "Field `TAMLVL2` writer - Tamper Level Select 2"]
pub type TAMLVL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCTRL_SPEC, bool, O>;
#[doc = "Field `TAMLVL3` reader - Tamper Level Select 3"]
pub type TAMLVL3_R = crate::BitReader<bool>;
#[doc = "Field `TAMLVL3` writer - Tamper Level Select 3"]
pub type TAMLVL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCTRL_SPEC, bool, O>;
#[doc = "Field `TAMLVL4` reader - Tamper Level Select 4"]
pub type TAMLVL4_R = crate::BitReader<bool>;
#[doc = "Field `TAMLVL4` writer - Tamper Level Select 4"]
pub type TAMLVL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCTRL_SPEC, bool, O>;
#[doc = "Field `DEBNC0` reader - Debouncer Enable 0"]
pub type DEBNC0_R = crate::BitReader<bool>;
#[doc = "Field `DEBNC0` writer - Debouncer Enable 0"]
pub type DEBNC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCTRL_SPEC, bool, O>;
#[doc = "Field `DEBNC1` reader - Debouncer Enable 1"]
pub type DEBNC1_R = crate::BitReader<bool>;
#[doc = "Field `DEBNC1` writer - Debouncer Enable 1"]
pub type DEBNC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCTRL_SPEC, bool, O>;
#[doc = "Field `DEBNC2` reader - Debouncer Enable 2"]
pub type DEBNC2_R = crate::BitReader<bool>;
#[doc = "Field `DEBNC2` writer - Debouncer Enable 2"]
pub type DEBNC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCTRL_SPEC, bool, O>;
#[doc = "Field `DEBNC3` reader - Debouncer Enable 3"]
pub type DEBNC3_R = crate::BitReader<bool>;
#[doc = "Field `DEBNC3` writer - Debouncer Enable 3"]
pub type DEBNC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCTRL_SPEC, bool, O>;
#[doc = "Field `DEBNC4` reader - Debouncer Enable 4"]
pub type DEBNC4_R = crate::BitReader<bool>;
#[doc = "Field `DEBNC4` writer - Debouncer Enable 4"]
pub type DEBNC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Tamper Input 0 Action"]
    #[inline(always)]
    pub fn in0act(&self) -> IN0ACT_R {
        IN0ACT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Tamper Input 1 Action"]
    #[inline(always)]
    pub fn in1act(&self) -> IN1ACT_R {
        IN1ACT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Tamper Input 2 Action"]
    #[inline(always)]
    pub fn in2act(&self) -> IN2ACT_R {
        IN2ACT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Tamper Input 3 Action"]
    #[inline(always)]
    pub fn in3act(&self) -> IN3ACT_R {
        IN3ACT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Tamper Input 4 Action"]
    #[inline(always)]
    pub fn in4act(&self) -> IN4ACT_R {
        IN4ACT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Tamper Level Select 0"]
    #[inline(always)]
    pub fn tamlvl0(&self) -> TAMLVL0_R {
        TAMLVL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper Level Select 1"]
    #[inline(always)]
    pub fn tamlvl1(&self) -> TAMLVL1_R {
        TAMLVL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tamper Level Select 2"]
    #[inline(always)]
    pub fn tamlvl2(&self) -> TAMLVL2_R {
        TAMLVL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Tamper Level Select 3"]
    #[inline(always)]
    pub fn tamlvl3(&self) -> TAMLVL3_R {
        TAMLVL3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tamper Level Select 4"]
    #[inline(always)]
    pub fn tamlvl4(&self) -> TAMLVL4_R {
        TAMLVL4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Debouncer Enable 0"]
    #[inline(always)]
    pub fn debnc0(&self) -> DEBNC0_R {
        DEBNC0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Debouncer Enable 1"]
    #[inline(always)]
    pub fn debnc1(&self) -> DEBNC1_R {
        DEBNC1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Debouncer Enable 2"]
    #[inline(always)]
    pub fn debnc2(&self) -> DEBNC2_R {
        DEBNC2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Debouncer Enable 3"]
    #[inline(always)]
    pub fn debnc3(&self) -> DEBNC3_R {
        DEBNC3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Debouncer Enable 4"]
    #[inline(always)]
    pub fn debnc4(&self) -> DEBNC4_R {
        DEBNC4_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Tamper Input 0 Action"]
    #[inline(always)]
    #[must_use]
    pub fn in0act(&mut self) -> IN0ACT_W<0> {
        IN0ACT_W::new(self)
    }
    #[doc = "Bits 2:3 - Tamper Input 1 Action"]
    #[inline(always)]
    #[must_use]
    pub fn in1act(&mut self) -> IN1ACT_W<2> {
        IN1ACT_W::new(self)
    }
    #[doc = "Bits 4:5 - Tamper Input 2 Action"]
    #[inline(always)]
    #[must_use]
    pub fn in2act(&mut self) -> IN2ACT_W<4> {
        IN2ACT_W::new(self)
    }
    #[doc = "Bits 6:7 - Tamper Input 3 Action"]
    #[inline(always)]
    #[must_use]
    pub fn in3act(&mut self) -> IN3ACT_W<6> {
        IN3ACT_W::new(self)
    }
    #[doc = "Bits 8:9 - Tamper Input 4 Action"]
    #[inline(always)]
    #[must_use]
    pub fn in4act(&mut self) -> IN4ACT_W<8> {
        IN4ACT_W::new(self)
    }
    #[doc = "Bit 16 - Tamper Level Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn tamlvl0(&mut self) -> TAMLVL0_W<16> {
        TAMLVL0_W::new(self)
    }
    #[doc = "Bit 17 - Tamper Level Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn tamlvl1(&mut self) -> TAMLVL1_W<17> {
        TAMLVL1_W::new(self)
    }
    #[doc = "Bit 18 - Tamper Level Select 2"]
    #[inline(always)]
    #[must_use]
    pub fn tamlvl2(&mut self) -> TAMLVL2_W<18> {
        TAMLVL2_W::new(self)
    }
    #[doc = "Bit 19 - Tamper Level Select 3"]
    #[inline(always)]
    #[must_use]
    pub fn tamlvl3(&mut self) -> TAMLVL3_W<19> {
        TAMLVL3_W::new(self)
    }
    #[doc = "Bit 20 - Tamper Level Select 4"]
    #[inline(always)]
    #[must_use]
    pub fn tamlvl4(&mut self) -> TAMLVL4_W<20> {
        TAMLVL4_W::new(self)
    }
    #[doc = "Bit 24 - Debouncer Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn debnc0(&mut self) -> DEBNC0_W<24> {
        DEBNC0_W::new(self)
    }
    #[doc = "Bit 25 - Debouncer Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn debnc1(&mut self) -> DEBNC1_W<25> {
        DEBNC1_W::new(self)
    }
    #[doc = "Bit 26 - Debouncer Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn debnc2(&mut self) -> DEBNC2_W<26> {
        DEBNC2_W::new(self)
    }
    #[doc = "Bit 27 - Debouncer Enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn debnc3(&mut self) -> DEBNC3_W<27> {
        DEBNC3_W::new(self)
    }
    #[doc = "Bit 28 - Debouncer Enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn debnc4(&mut self) -> DEBNC4_W<28> {
        DEBNC4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampctrl](index.html) module"]
pub struct TAMPCTRL_SPEC;
impl crate::RegisterSpec for TAMPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tampctrl::R](R) reader structure"]
impl crate::Readable for TAMPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tampctrl::W](W) writer structure"]
impl crate::Writable for TAMPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMPCTRL to value 0"]
impl crate::Resettable for TAMPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
