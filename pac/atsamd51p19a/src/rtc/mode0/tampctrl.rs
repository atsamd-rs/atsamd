#[doc = "Reader of register TAMPCTRL"]
pub type R = crate::R<u32, super::TAMPCTRL>;
#[doc = "Writer for register TAMPCTRL"]
pub type W = crate::W<u32, super::TAMPCTRL>;
#[doc = "Register TAMPCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMPCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Tamper Input 0 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IN0ACT_A {
    #[doc = "0: Off (Disabled)"]
    OFF = 0,
    #[doc = "1: Wake without timestamp"]
    WAKE = 1,
    #[doc = "2: Capture timestamp"]
    CAPTURE = 2,
    #[doc = "3: Compare IN0 to OUT"]
    ACTL = 3,
}
impl From<IN0ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: IN0ACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IN0ACT`"]
pub type IN0ACT_R = crate::R<u8, IN0ACT_A>;
impl IN0ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN0ACT_A {
        match self.bits {
            0 => IN0ACT_A::OFF,
            1 => IN0ACT_A::WAKE,
            2 => IN0ACT_A::CAPTURE,
            3 => IN0ACT_A::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IN0ACT_A::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == IN0ACT_A::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == IN0ACT_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == IN0ACT_A::ACTL
    }
}
#[doc = "Write proxy for field `IN0ACT`"]
pub struct IN0ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> IN0ACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN0ACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IN0ACT_A::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN0ACT_A::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN0ACT_A::CAPTURE)
    }
    #[doc = "Compare IN0 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN0ACT_A::ACTL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Tamper Input 1 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IN1ACT_A {
    #[doc = "0: Off (Disabled)"]
    OFF = 0,
    #[doc = "1: Wake without timestamp"]
    WAKE = 1,
    #[doc = "2: Capture timestamp"]
    CAPTURE = 2,
    #[doc = "3: Compare IN1 to OUT"]
    ACTL = 3,
}
impl From<IN1ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: IN1ACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IN1ACT`"]
pub type IN1ACT_R = crate::R<u8, IN1ACT_A>;
impl IN1ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN1ACT_A {
        match self.bits {
            0 => IN1ACT_A::OFF,
            1 => IN1ACT_A::WAKE,
            2 => IN1ACT_A::CAPTURE,
            3 => IN1ACT_A::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IN1ACT_A::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == IN1ACT_A::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == IN1ACT_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == IN1ACT_A::ACTL
    }
}
#[doc = "Write proxy for field `IN1ACT`"]
pub struct IN1ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1ACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN1ACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IN1ACT_A::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN1ACT_A::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN1ACT_A::CAPTURE)
    }
    #[doc = "Compare IN1 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN1ACT_A::ACTL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Tamper Input 2 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IN2ACT_A {
    #[doc = "0: Off (Disabled)"]
    OFF = 0,
    #[doc = "1: Wake without timestamp"]
    WAKE = 1,
    #[doc = "2: Capture timestamp"]
    CAPTURE = 2,
    #[doc = "3: Compare IN2 to OUT"]
    ACTL = 3,
}
impl From<IN2ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: IN2ACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IN2ACT`"]
pub type IN2ACT_R = crate::R<u8, IN2ACT_A>;
impl IN2ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN2ACT_A {
        match self.bits {
            0 => IN2ACT_A::OFF,
            1 => IN2ACT_A::WAKE,
            2 => IN2ACT_A::CAPTURE,
            3 => IN2ACT_A::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IN2ACT_A::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == IN2ACT_A::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == IN2ACT_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == IN2ACT_A::ACTL
    }
}
#[doc = "Write proxy for field `IN2ACT`"]
pub struct IN2ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2ACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN2ACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IN2ACT_A::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN2ACT_A::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN2ACT_A::CAPTURE)
    }
    #[doc = "Compare IN2 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN2ACT_A::ACTL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Tamper Input 3 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IN3ACT_A {
    #[doc = "0: Off (Disabled)"]
    OFF = 0,
    #[doc = "1: Wake without timestamp"]
    WAKE = 1,
    #[doc = "2: Capture timestamp"]
    CAPTURE = 2,
    #[doc = "3: Compare IN3 to OUT"]
    ACTL = 3,
}
impl From<IN3ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: IN3ACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IN3ACT`"]
pub type IN3ACT_R = crate::R<u8, IN3ACT_A>;
impl IN3ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN3ACT_A {
        match self.bits {
            0 => IN3ACT_A::OFF,
            1 => IN3ACT_A::WAKE,
            2 => IN3ACT_A::CAPTURE,
            3 => IN3ACT_A::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IN3ACT_A::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == IN3ACT_A::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == IN3ACT_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == IN3ACT_A::ACTL
    }
}
#[doc = "Write proxy for field `IN3ACT`"]
pub struct IN3ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> IN3ACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN3ACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IN3ACT_A::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN3ACT_A::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN3ACT_A::CAPTURE)
    }
    #[doc = "Compare IN3 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN3ACT_A::ACTL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Tamper Input 4 Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IN4ACT_A {
    #[doc = "0: Off (Disabled)"]
    OFF = 0,
    #[doc = "1: Wake without timestamp"]
    WAKE = 1,
    #[doc = "2: Capture timestamp"]
    CAPTURE = 2,
    #[doc = "3: Compare IN4 to OUT"]
    ACTL = 3,
}
impl From<IN4ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: IN4ACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IN4ACT`"]
pub type IN4ACT_R = crate::R<u8, IN4ACT_A>;
impl IN4ACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN4ACT_A {
        match self.bits {
            0 => IN4ACT_A::OFF,
            1 => IN4ACT_A::WAKE,
            2 => IN4ACT_A::CAPTURE,
            3 => IN4ACT_A::ACTL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == IN4ACT_A::OFF
    }
    #[doc = "Checks if the value of the field is `WAKE`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == IN4ACT_A::WAKE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == IN4ACT_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `ACTL`"]
    #[inline(always)]
    pub fn is_actl(&self) -> bool {
        *self == IN4ACT_A::ACTL
    }
}
#[doc = "Write proxy for field `IN4ACT`"]
pub struct IN4ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> IN4ACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN4ACT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Off (Disabled)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(IN4ACT_A::OFF)
    }
    #[doc = "Wake without timestamp"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut W {
        self.variant(IN4ACT_A::WAKE)
    }
    #[doc = "Capture timestamp"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(IN4ACT_A::CAPTURE)
    }
    #[doc = "Compare IN4 to OUT"]
    #[inline(always)]
    pub fn actl(self) -> &'a mut W {
        self.variant(IN4ACT_A::ACTL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `TAMLVL0`"]
pub type TAMLVL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMLVL0`"]
pub struct TAMLVL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMLVL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TAMLVL1`"]
pub type TAMLVL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMLVL1`"]
pub struct TAMLVL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMLVL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TAMLVL2`"]
pub type TAMLVL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMLVL2`"]
pub struct TAMLVL2_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMLVL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TAMLVL3`"]
pub type TAMLVL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMLVL3`"]
pub struct TAMLVL3_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMLVL3_W<'a> {
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
#[doc = "Reader of field `TAMLVL4`"]
pub type TAMLVL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMLVL4`"]
pub struct TAMLVL4_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMLVL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DEBNC0`"]
pub type DEBNC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBNC0`"]
pub struct DEBNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBNC0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DEBNC1`"]
pub type DEBNC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBNC1`"]
pub struct DEBNC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBNC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DEBNC2`"]
pub type DEBNC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBNC2`"]
pub struct DEBNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBNC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DEBNC3`"]
pub type DEBNC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBNC3`"]
pub struct DEBNC3_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBNC3_W<'a> {
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
#[doc = "Reader of field `DEBNC4`"]
pub type DEBNC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBNC4`"]
pub struct DEBNC4_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBNC4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Tamper Input 0 Action"]
    #[inline(always)]
    pub fn in0act(&self) -> IN0ACT_R {
        IN0ACT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Tamper Input 1 Action"]
    #[inline(always)]
    pub fn in1act(&self) -> IN1ACT_R {
        IN1ACT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Tamper Input 2 Action"]
    #[inline(always)]
    pub fn in2act(&self) -> IN2ACT_R {
        IN2ACT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Tamper Input 3 Action"]
    #[inline(always)]
    pub fn in3act(&self) -> IN3ACT_R {
        IN3ACT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Tamper Input 4 Action"]
    #[inline(always)]
    pub fn in4act(&self) -> IN4ACT_R {
        IN4ACT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Tamper Level Select 0"]
    #[inline(always)]
    pub fn tamlvl0(&self) -> TAMLVL0_R {
        TAMLVL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Tamper Level Select 1"]
    #[inline(always)]
    pub fn tamlvl1(&self) -> TAMLVL1_R {
        TAMLVL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Tamper Level Select 2"]
    #[inline(always)]
    pub fn tamlvl2(&self) -> TAMLVL2_R {
        TAMLVL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Tamper Level Select 3"]
    #[inline(always)]
    pub fn tamlvl3(&self) -> TAMLVL3_R {
        TAMLVL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Tamper Level Select 4"]
    #[inline(always)]
    pub fn tamlvl4(&self) -> TAMLVL4_R {
        TAMLVL4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Debouncer Enable 0"]
    #[inline(always)]
    pub fn debnc0(&self) -> DEBNC0_R {
        DEBNC0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Debouncer Enable 1"]
    #[inline(always)]
    pub fn debnc1(&self) -> DEBNC1_R {
        DEBNC1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Debouncer Enable 2"]
    #[inline(always)]
    pub fn debnc2(&self) -> DEBNC2_R {
        DEBNC2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Debouncer Enable 3"]
    #[inline(always)]
    pub fn debnc3(&self) -> DEBNC3_R {
        DEBNC3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Debouncer Enable 4"]
    #[inline(always)]
    pub fn debnc4(&self) -> DEBNC4_R {
        DEBNC4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Tamper Input 0 Action"]
    #[inline(always)]
    pub fn in0act(&mut self) -> IN0ACT_W {
        IN0ACT_W { w: self }
    }
    #[doc = "Bits 2:3 - Tamper Input 1 Action"]
    #[inline(always)]
    pub fn in1act(&mut self) -> IN1ACT_W {
        IN1ACT_W { w: self }
    }
    #[doc = "Bits 4:5 - Tamper Input 2 Action"]
    #[inline(always)]
    pub fn in2act(&mut self) -> IN2ACT_W {
        IN2ACT_W { w: self }
    }
    #[doc = "Bits 6:7 - Tamper Input 3 Action"]
    #[inline(always)]
    pub fn in3act(&mut self) -> IN3ACT_W {
        IN3ACT_W { w: self }
    }
    #[doc = "Bits 8:9 - Tamper Input 4 Action"]
    #[inline(always)]
    pub fn in4act(&mut self) -> IN4ACT_W {
        IN4ACT_W { w: self }
    }
    #[doc = "Bit 16 - Tamper Level Select 0"]
    #[inline(always)]
    pub fn tamlvl0(&mut self) -> TAMLVL0_W {
        TAMLVL0_W { w: self }
    }
    #[doc = "Bit 17 - Tamper Level Select 1"]
    #[inline(always)]
    pub fn tamlvl1(&mut self) -> TAMLVL1_W {
        TAMLVL1_W { w: self }
    }
    #[doc = "Bit 18 - Tamper Level Select 2"]
    #[inline(always)]
    pub fn tamlvl2(&mut self) -> TAMLVL2_W {
        TAMLVL2_W { w: self }
    }
    #[doc = "Bit 19 - Tamper Level Select 3"]
    #[inline(always)]
    pub fn tamlvl3(&mut self) -> TAMLVL3_W {
        TAMLVL3_W { w: self }
    }
    #[doc = "Bit 20 - Tamper Level Select 4"]
    #[inline(always)]
    pub fn tamlvl4(&mut self) -> TAMLVL4_W {
        TAMLVL4_W { w: self }
    }
    #[doc = "Bit 24 - Debouncer Enable 0"]
    #[inline(always)]
    pub fn debnc0(&mut self) -> DEBNC0_W {
        DEBNC0_W { w: self }
    }
    #[doc = "Bit 25 - Debouncer Enable 1"]
    #[inline(always)]
    pub fn debnc1(&mut self) -> DEBNC1_W {
        DEBNC1_W { w: self }
    }
    #[doc = "Bit 26 - Debouncer Enable 2"]
    #[inline(always)]
    pub fn debnc2(&mut self) -> DEBNC2_W {
        DEBNC2_W { w: self }
    }
    #[doc = "Bit 27 - Debouncer Enable 3"]
    #[inline(always)]
    pub fn debnc3(&mut self) -> DEBNC3_W {
        DEBNC3_W { w: self }
    }
    #[doc = "Bit 28 - Debouncer Enable 4"]
    #[inline(always)]
    pub fn debnc4(&mut self) -> DEBNC4_W {
        DEBNC4_W { w: self }
    }
}
