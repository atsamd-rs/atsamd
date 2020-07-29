#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u32, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u32, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Timer Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Counter in 16-bit mode"]
    COUNT16 = 0,
    #[doc = "1: Counter in 8-bit mode"]
    COUNT8 = 1,
    #[doc = "2: Counter in 32-bit mode"]
    COUNT32 = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::COUNT16),
            1 => Val(MODE_A::COUNT8),
            2 => Val(MODE_A::COUNT32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `COUNT16`"]
    #[inline(always)]
    pub fn is_count16(&self) -> bool {
        *self == MODE_A::COUNT16
    }
    #[doc = "Checks if the value of the field is `COUNT8`"]
    #[inline(always)]
    pub fn is_count8(&self) -> bool {
        *self == MODE_A::COUNT8
    }
    #[doc = "Checks if the value of the field is `COUNT32`"]
    #[inline(always)]
    pub fn is_count32(&self) -> bool {
        *self == MODE_A::COUNT32
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Counter in 16-bit mode"]
    #[inline(always)]
    pub fn count16(self) -> &'a mut W {
        self.variant(MODE_A::COUNT16)
    }
    #[doc = "Counter in 8-bit mode"]
    #[inline(always)]
    pub fn count8(self) -> &'a mut W {
        self.variant(MODE_A::COUNT8)
    }
    #[doc = "Counter in 32-bit mode"]
    #[inline(always)]
    pub fn count32(self) -> &'a mut W {
        self.variant(MODE_A::COUNT32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Prescaler and Counter Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCSYNC_A {
    #[doc = "0: Reload or reset the counter on next generic clock"]
    GCLK = 0,
    #[doc = "1: Reload or reset the counter on next prescaler clock"]
    PRESC = 1,
    #[doc = "2: Reload or reset the counter on next generic clock and reset the prescaler counter"]
    RESYNC = 2,
}
impl From<PRESCSYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCSYNC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCSYNC`"]
pub type PRESCSYNC_R = crate::R<u8, PRESCSYNC_A>;
impl PRESCSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESCSYNC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESCSYNC_A::GCLK),
            1 => Val(PRESCSYNC_A::PRESC),
            2 => Val(PRESCSYNC_A::RESYNC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == PRESCSYNC_A::GCLK
    }
    #[doc = "Checks if the value of the field is `PRESC`"]
    #[inline(always)]
    pub fn is_presc(&self) -> bool {
        *self == PRESCSYNC_A::PRESC
    }
    #[doc = "Checks if the value of the field is `RESYNC`"]
    #[inline(always)]
    pub fn is_resync(&self) -> bool {
        *self == PRESCSYNC_A::RESYNC
    }
}
#[doc = "Write proxy for field `PRESCSYNC`"]
pub struct PRESCSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCSYNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reload or reset the counter on next generic clock"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(PRESCSYNC_A::GCLK)
    }
    #[doc = "Reload or reset the counter on next prescaler clock"]
    #[inline(always)]
    pub fn presc(self) -> &'a mut W {
        self.variant(PRESCSYNC_A::PRESC)
    }
    #[doc = "Reload or reset the counter on next generic clock and reset the prescaler counter"]
    #[inline(always)]
    pub fn resync(self) -> &'a mut W {
        self.variant(PRESCSYNC_A::RESYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `RUNSTDBY`"]
pub type RUNSTDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNSTDBY`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ONDEMAND`"]
pub type ONDEMAND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONDEMAND`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: Prescaler: GCLK_TC"]
    DIV1 = 0,
    #[doc = "1: Prescaler: GCLK_TC/2"]
    DIV2 = 1,
    #[doc = "2: Prescaler: GCLK_TC/4"]
    DIV4 = 2,
    #[doc = "3: Prescaler: GCLK_TC/8"]
    DIV8 = 3,
    #[doc = "4: Prescaler: GCLK_TC/16"]
    DIV16 = 4,
    #[doc = "5: Prescaler: GCLK_TC/64"]
    DIV64 = 5,
    #[doc = "6: Prescaler: GCLK_TC/256"]
    DIV256 = 6,
    #[doc = "7: Prescaler: GCLK_TC/1024"]
    DIV1024 = 7,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALER`"]
pub type PRESCALER_R = crate::R<u8, PRESCALER_A>;
impl PRESCALER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALER_A {
        match self.bits {
            0 => PRESCALER_A::DIV1,
            1 => PRESCALER_A::DIV2,
            2 => PRESCALER_A::DIV4,
            3 => PRESCALER_A::DIV8,
            4 => PRESCALER_A::DIV16,
            5 => PRESCALER_A::DIV64,
            6 => PRESCALER_A::DIV256,
            7 => PRESCALER_A::DIV1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALER_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALER_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALER_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALER_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALER_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALER_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALER_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALER_A::DIV1024
    }
}
#[doc = "Write proxy for field `PRESCALER`"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Prescaler: GCLK_TC"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV1)
    }
    #[doc = "Prescaler: GCLK_TC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV2)
    }
    #[doc = "Prescaler: GCLK_TC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV4)
    }
    #[doc = "Prescaler: GCLK_TC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV8)
    }
    #[doc = "Prescaler: GCLK_TC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV16)
    }
    #[doc = "Prescaler: GCLK_TC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV64)
    }
    #[doc = "Prescaler: GCLK_TC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV256)
    }
    #[doc = "Prescaler: GCLK_TC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `ALOCK`"]
pub type ALOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALOCK`"]
pub struct ALOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALOCK_W<'a> {
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
#[doc = "Reader of field `CAPTEN0`"]
pub type CAPTEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTEN0`"]
pub struct CAPTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTEN0_W<'a> {
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
#[doc = "Reader of field `CAPTEN1`"]
pub type CAPTEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPTEN1`"]
pub struct CAPTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTEN1_W<'a> {
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
#[doc = "Reader of field `COPEN0`"]
pub type COPEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COPEN0`"]
pub struct COPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> COPEN0_W<'a> {
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
#[doc = "Reader of field `COPEN1`"]
pub type COPEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COPEN1`"]
pub struct COPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> COPEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Capture Mode Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPTMODE0_A {
    #[doc = "0: Default capture"]
    DEFAULT = 0,
    #[doc = "1: Minimum capture"]
    CAPTMIN = 1,
    #[doc = "2: Maximum capture"]
    CAPTMAX = 2,
}
impl From<CAPTMODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTMODE0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CAPTMODE0`"]
pub type CAPTMODE0_R = crate::R<u8, CAPTMODE0_A>;
impl CAPTMODE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CAPTMODE0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CAPTMODE0_A::DEFAULT),
            1 => Val(CAPTMODE0_A::CAPTMIN),
            2 => Val(CAPTMODE0_A::CAPTMAX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CAPTMODE0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `CAPTMIN`"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTMODE0_A::CAPTMIN
    }
    #[doc = "Checks if the value of the field is `CAPTMAX`"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTMODE0_A::CAPTMAX
    }
}
#[doc = "Write proxy for field `CAPTMODE0`"]
pub struct CAPTMODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTMODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTMODE0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(CAPTMODE0_A::DEFAULT)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut W {
        self.variant(CAPTMODE0_A::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut W {
        self.variant(CAPTMODE0_A::CAPTMAX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Capture mode Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPTMODE1_A {
    #[doc = "0: Default capture"]
    DEFAULT = 0,
    #[doc = "1: Minimum capture"]
    CAPTMIN = 1,
    #[doc = "2: Maximum capture"]
    CAPTMAX = 2,
}
impl From<CAPTMODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTMODE1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CAPTMODE1`"]
pub type CAPTMODE1_R = crate::R<u8, CAPTMODE1_A>;
impl CAPTMODE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CAPTMODE1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CAPTMODE1_A::DEFAULT),
            1 => Val(CAPTMODE1_A::CAPTMIN),
            2 => Val(CAPTMODE1_A::CAPTMAX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CAPTMODE1_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `CAPTMIN`"]
    #[inline(always)]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTMODE1_A::CAPTMIN
    }
    #[doc = "Checks if the value of the field is `CAPTMAX`"]
    #[inline(always)]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTMODE1_A::CAPTMAX
    }
}
#[doc = "Write proxy for field `CAPTMODE1`"]
pub struct CAPTMODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTMODE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTMODE1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Default capture"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(CAPTMODE1_A::DEFAULT)
    }
    #[doc = "Minimum capture"]
    #[inline(always)]
    pub fn captmin(self) -> &'a mut W {
        self.variant(CAPTMODE1_A::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline(always)]
    pub fn captmax(self) -> &'a mut W {
        self.variant(CAPTMODE1_A::CAPTMAX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Timer Counter Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    pub fn prescsync(&self) -> PRESCSYNC_R {
        PRESCSYNC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> ALOCK_R {
        ALOCK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn capten0(&self) -> CAPTEN0_R {
        CAPTEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn capten1(&self) -> CAPTEN1_R {
        CAPTEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Capture On Pin 0 Enable"]
    #[inline(always)]
    pub fn copen0(&self) -> COPEN0_R {
        COPEN0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Capture On Pin 1 Enable"]
    #[inline(always)]
    pub fn copen1(&self) -> COPEN1_R {
        COPEN1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Capture Mode Channel 0"]
    #[inline(always)]
    pub fn captmode0(&self) -> CAPTMODE0_R {
        CAPTMODE0_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - Capture mode Channel 1"]
    #[inline(always)]
    pub fn captmode1(&self) -> CAPTMODE1_R {
        CAPTMODE1_R::new(((self.bits >> 27) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:3 - Timer Counter Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    pub fn prescsync(&mut self) -> PRESCSYNC_W {
        PRESCSYNC_W { w: self }
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - Clock On Demand"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W { w: self }
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&mut self) -> ALOCK_W {
        ALOCK_W { w: self }
    }
    #[doc = "Bit 16 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn capten0(&mut self) -> CAPTEN0_W {
        CAPTEN0_W { w: self }
    }
    #[doc = "Bit 17 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn capten1(&mut self) -> CAPTEN1_W {
        CAPTEN1_W { w: self }
    }
    #[doc = "Bit 20 - Capture On Pin 0 Enable"]
    #[inline(always)]
    pub fn copen0(&mut self) -> COPEN0_W {
        COPEN0_W { w: self }
    }
    #[doc = "Bit 21 - Capture On Pin 1 Enable"]
    #[inline(always)]
    pub fn copen1(&mut self) -> COPEN1_W {
        COPEN1_W { w: self }
    }
    #[doc = "Bits 24:25 - Capture Mode Channel 0"]
    #[inline(always)]
    pub fn captmode0(&mut self) -> CAPTMODE0_W {
        CAPTMODE0_W { w: self }
    }
    #[doc = "Bits 27:28 - Capture mode Channel 1"]
    #[inline(always)]
    pub fn captmode1(&mut self) -> CAPTMODE1_W {
        CAPTMODE1_W { w: self }
    }
}
