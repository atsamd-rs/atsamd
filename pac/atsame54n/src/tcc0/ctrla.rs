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
#[doc = "Enhanced Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESOLUTION_A {
    #[doc = "0: Dithering is disabled"]
    NONE = 0,
    #[doc = "1: Dithering is done every 16 PWM frames"]
    DITH4 = 1,
    #[doc = "2: Dithering is done every 32 PWM frames"]
    DITH5 = 2,
    #[doc = "3: Dithering is done every 64 PWM frames"]
    DITH6 = 3,
}
impl From<RESOLUTION_A> for u8 {
    #[inline(always)]
    fn from(variant: RESOLUTION_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RESOLUTION`"]
pub type RESOLUTION_R = crate::R<u8, RESOLUTION_A>;
impl RESOLUTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESOLUTION_A {
        match self.bits {
            0 => RESOLUTION_A::NONE,
            1 => RESOLUTION_A::DITH4,
            2 => RESOLUTION_A::DITH5,
            3 => RESOLUTION_A::DITH6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RESOLUTION_A::NONE
    }
    #[doc = "Checks if the value of the field is `DITH4`"]
    #[inline(always)]
    pub fn is_dith4(&self) -> bool {
        *self == RESOLUTION_A::DITH4
    }
    #[doc = "Checks if the value of the field is `DITH5`"]
    #[inline(always)]
    pub fn is_dith5(&self) -> bool {
        *self == RESOLUTION_A::DITH5
    }
    #[doc = "Checks if the value of the field is `DITH6`"]
    #[inline(always)]
    pub fn is_dith6(&self) -> bool {
        *self == RESOLUTION_A::DITH6
    }
}
#[doc = "Write proxy for field `RESOLUTION`"]
pub struct RESOLUTION_W<'a> {
    w: &'a mut W,
}
impl<'a> RESOLUTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESOLUTION_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Dithering is disabled"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RESOLUTION_A::NONE)
    }
    #[doc = "Dithering is done every 16 PWM frames"]
    #[inline(always)]
    pub fn dith4(self) -> &'a mut W {
        self.variant(RESOLUTION_A::DITH4)
    }
    #[doc = "Dithering is done every 32 PWM frames"]
    #[inline(always)]
    pub fn dith5(self) -> &'a mut W {
        self.variant(RESOLUTION_A::DITH5)
    }
    #[doc = "Dithering is done every 64 PWM frames"]
    #[inline(always)]
    pub fn dith6(self) -> &'a mut W {
        self.variant(RESOLUTION_A::DITH6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: No division"]
    DIV1 = 0,
    #[doc = "1: Divide by 2"]
    DIV2 = 1,
    #[doc = "2: Divide by 4"]
    DIV4 = 2,
    #[doc = "3: Divide by 8"]
    DIV8 = 3,
    #[doc = "4: Divide by 16"]
    DIV16 = 4,
    #[doc = "5: Divide by 64"]
    DIV64 = 5,
    #[doc = "6: Divide by 256"]
    DIV256 = 6,
    #[doc = "7: Divide by 1024"]
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
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV16)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV64)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV256)
    }
    #[doc = "Divide by 1024"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Prescaler and Counter Synchronization Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCSYNC_A {
    #[doc = "0: Reload or reset counter on next GCLK"]
    GCLK = 0,
    #[doc = "1: Reload or reset counter on next prescaler clock"]
    PRESC = 1,
    #[doc = "2: Reload or reset counter on next GCLK and reset prescaler counter"]
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
    #[doc = "Reload or reset counter on next GCLK"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(PRESCSYNC_A::GCLK)
    }
    #[doc = "Reload or reset counter on next prescaler clock"]
    #[inline(always)]
    pub fn presc(self) -> &'a mut W {
        self.variant(PRESCSYNC_A::PRESC)
    }
    #[doc = "Reload or reset counter on next GCLK and reset prescaler counter"]
    #[inline(always)]
    pub fn resync(self) -> &'a mut W {
        self.variant(PRESCSYNC_A::RESYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `MSYNC`"]
pub type MSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSYNC`"]
pub struct MSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSYNC_W<'a> {
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
#[doc = "Reader of field `DMAOS`"]
pub type DMAOS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAOS`"]
pub struct DMAOS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAOS_W<'a> {
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
#[doc = "Reader of field `CPTEN0`"]
pub type CPTEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPTEN0`"]
pub struct CPTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTEN0_W<'a> {
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
#[doc = "Reader of field `CPTEN1`"]
pub type CPTEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPTEN1`"]
pub struct CPTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTEN1_W<'a> {
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
#[doc = "Reader of field `CPTEN2`"]
pub type CPTEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPTEN2`"]
pub struct CPTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTEN2_W<'a> {
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
#[doc = "Reader of field `CPTEN3`"]
pub type CPTEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPTEN3`"]
pub struct CPTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTEN3_W<'a> {
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
#[doc = "Reader of field `CPTEN4`"]
pub type CPTEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPTEN4`"]
pub struct CPTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTEN4_W<'a> {
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
#[doc = "Reader of field `CPTEN5`"]
pub type CPTEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPTEN5`"]
pub struct CPTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CPTEN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
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
    #[doc = "Bits 5:6 - Enhanced Resolution"]
    #[inline(always)]
    pub fn resolution(&self) -> RESOLUTION_R {
        RESOLUTION_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization Selection"]
    #[inline(always)]
    pub fn prescsync(&self) -> PRESCSYNC_R {
        PRESCSYNC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> ALOCK_R {
        ALOCK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Master Synchronization (only for TCC Slave Instance)"]
    #[inline(always)]
    pub fn msync(&self) -> MSYNC_R {
        MSYNC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DMA One-shot Trigger Mode"]
    #[inline(always)]
    pub fn dmaos(&self) -> DMAOS_R {
        DMAOS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&self) -> CPTEN0_R {
        CPTEN0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&self) -> CPTEN1_R {
        CPTEN1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Capture Channel 2 Enable"]
    #[inline(always)]
    pub fn cpten2(&self) -> CPTEN2_R {
        CPTEN2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Capture Channel 3 Enable"]
    #[inline(always)]
    pub fn cpten3(&self) -> CPTEN3_R {
        CPTEN3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Capture Channel 4 Enable"]
    #[inline(always)]
    pub fn cpten4(&self) -> CPTEN4_R {
        CPTEN4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Capture Channel 5 Enable"]
    #[inline(always)]
    pub fn cpten5(&self) -> CPTEN5_R {
        CPTEN5_R::new(((self.bits >> 29) & 0x01) != 0)
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
    #[doc = "Bits 5:6 - Enhanced Resolution"]
    #[inline(always)]
    pub fn resolution(&mut self) -> RESOLUTION_W {
        RESOLUTION_W { w: self }
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Bit 11 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization Selection"]
    #[inline(always)]
    pub fn prescsync(&mut self) -> PRESCSYNC_W {
        PRESCSYNC_W { w: self }
    }
    #[doc = "Bit 14 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&mut self) -> ALOCK_W {
        ALOCK_W { w: self }
    }
    #[doc = "Bit 15 - Master Synchronization (only for TCC Slave Instance)"]
    #[inline(always)]
    pub fn msync(&mut self) -> MSYNC_W {
        MSYNC_W { w: self }
    }
    #[doc = "Bit 23 - DMA One-shot Trigger Mode"]
    #[inline(always)]
    pub fn dmaos(&mut self) -> DMAOS_W {
        DMAOS_W { w: self }
    }
    #[doc = "Bit 24 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&mut self) -> CPTEN0_W {
        CPTEN0_W { w: self }
    }
    #[doc = "Bit 25 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&mut self) -> CPTEN1_W {
        CPTEN1_W { w: self }
    }
    #[doc = "Bit 26 - Capture Channel 2 Enable"]
    #[inline(always)]
    pub fn cpten2(&mut self) -> CPTEN2_W {
        CPTEN2_W { w: self }
    }
    #[doc = "Bit 27 - Capture Channel 3 Enable"]
    #[inline(always)]
    pub fn cpten3(&mut self) -> CPTEN3_W {
        CPTEN3_W { w: self }
    }
    #[doc = "Bit 28 - Capture Channel 4 Enable"]
    #[inline(always)]
    pub fn cpten4(&mut self) -> CPTEN4_W {
        CPTEN4_W { w: self }
    }
    #[doc = "Bit 29 - Capture Channel 5 Enable"]
    #[inline(always)]
    pub fn cpten5(&mut self) -> CPTEN5_W {
        CPTEN5_W { w: self }
    }
}
