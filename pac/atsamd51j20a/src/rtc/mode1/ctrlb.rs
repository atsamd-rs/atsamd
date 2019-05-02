#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRLB {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct GP0ENR {
    bits: bool,
}
impl GP0ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct GP2ENR {
    bits: bool,
}
impl GP2ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DEBMAJR {
    bits: bool,
}
impl DEBMAJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DEBASYNCR {
    bits: bool,
}
impl DEBASYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RTCOUTR {
    bits: bool,
}
impl RTCOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMAENR {
    bits: bool,
}
impl DMAENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `DEBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBFR {
    #[doc = "CLK_RTC_DEB = CLK_RTC/2"]
    DIV2,
    #[doc = "CLK_RTC_DEB = CLK_RTC/4"]
    DIV4,
    #[doc = "CLK_RTC_DEB = CLK_RTC/8"]
    DIV8,
    #[doc = "CLK_RTC_DEB = CLK_RTC/16"]
    DIV16,
    #[doc = "CLK_RTC_DEB = CLK_RTC/32"]
    DIV32,
    #[doc = "CLK_RTC_DEB = CLK_RTC/64"]
    DIV64,
    #[doc = "CLK_RTC_DEB = CLK_RTC/128"]
    DIV128,
    #[doc = "CLK_RTC_DEB = CLK_RTC/256"]
    DIV256,
}
impl DEBFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEBFR::DIV2 => 0,
            DEBFR::DIV4 => 1,
            DEBFR::DIV8 => 2,
            DEBFR::DIV16 => 3,
            DEBFR::DIV32 => 4,
            DEBFR::DIV64 => 5,
            DEBFR::DIV128 => 6,
            DEBFR::DIV256 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEBFR {
        match value {
            0 => DEBFR::DIV2,
            1 => DEBFR::DIV4,
            2 => DEBFR::DIV8,
            3 => DEBFR::DIV16,
            4 => DEBFR::DIV32,
            5 => DEBFR::DIV64,
            6 => DEBFR::DIV128,
            7 => DEBFR::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == DEBFR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == DEBFR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == DEBFR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == DEBFR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == DEBFR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == DEBFR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == DEBFR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == DEBFR::DIV256
    }
}
#[doc = "Possible values of the field `ACTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTFR {
    #[doc = "CLK_RTC_OUT = CLK_RTC/2"]
    DIV2,
    #[doc = "CLK_RTC_OUT = CLK_RTC/4"]
    DIV4,
    #[doc = "CLK_RTC_OUT = CLK_RTC/8"]
    DIV8,
    #[doc = "CLK_RTC_OUT = CLK_RTC/16"]
    DIV16,
    #[doc = "CLK_RTC_OUT = CLK_RTC/32"]
    DIV32,
    #[doc = "CLK_RTC_OUT = CLK_RTC/64"]
    DIV64,
    #[doc = "CLK_RTC_OUT = CLK_RTC/128"]
    DIV128,
    #[doc = "CLK_RTC_OUT = CLK_RTC/256"]
    DIV256,
}
impl ACTFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTFR::DIV2 => 0,
            ACTFR::DIV4 => 1,
            ACTFR::DIV8 => 2,
            ACTFR::DIV16 => 3,
            ACTFR::DIV32 => 4,
            ACTFR::DIV64 => 5,
            ACTFR::DIV128 => 6,
            ACTFR::DIV256 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACTFR {
        match value {
            0 => ACTFR::DIV2,
            1 => ACTFR::DIV4,
            2 => ACTFR::DIV8,
            3 => ACTFR::DIV16,
            4 => ACTFR::DIV32,
            5 => ACTFR::DIV64,
            6 => ACTFR::DIV128,
            7 => ACTFR::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == ACTFR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == ACTFR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == ACTFR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == ACTFR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == ACTFR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == ACTFR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == ACTFR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == ACTFR::DIV256
    }
}
#[doc = r" Proxy"]
pub struct _GP0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _GP0ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GP2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _GP2ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBMAJW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBMAJW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEBASYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBASYNCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTCOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCOUTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DEBF`"]
pub enum DEBFW {
    #[doc = "CLK_RTC_DEB = CLK_RTC/2"]
    DIV2,
    #[doc = "CLK_RTC_DEB = CLK_RTC/4"]
    DIV4,
    #[doc = "CLK_RTC_DEB = CLK_RTC/8"]
    DIV8,
    #[doc = "CLK_RTC_DEB = CLK_RTC/16"]
    DIV16,
    #[doc = "CLK_RTC_DEB = CLK_RTC/32"]
    DIV32,
    #[doc = "CLK_RTC_DEB = CLK_RTC/64"]
    DIV64,
    #[doc = "CLK_RTC_DEB = CLK_RTC/128"]
    DIV128,
    #[doc = "CLK_RTC_DEB = CLK_RTC/256"]
    DIV256,
}
impl DEBFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEBFW::DIV2 => 0,
            DEBFW::DIV4 => 1,
            DEBFW::DIV8 => 2,
            DEBFW::DIV16 => 3,
            DEBFW::DIV32 => 4,
            DEBFW::DIV64 => 5,
            DEBFW::DIV128 => 6,
            DEBFW::DIV256 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBFW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(DEBFW::DIV2)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(DEBFW::DIV4)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(DEBFW::DIV8)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(DEBFW::DIV16)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(DEBFW::DIV32)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(DEBFW::DIV64)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(DEBFW::DIV128)
    }
    #[doc = "CLK_RTC_DEB = CLK_RTC/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(DEBFW::DIV256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACTF`"]
pub enum ACTFW {
    #[doc = "CLK_RTC_OUT = CLK_RTC/2"]
    DIV2,
    #[doc = "CLK_RTC_OUT = CLK_RTC/4"]
    DIV4,
    #[doc = "CLK_RTC_OUT = CLK_RTC/8"]
    DIV8,
    #[doc = "CLK_RTC_OUT = CLK_RTC/16"]
    DIV16,
    #[doc = "CLK_RTC_OUT = CLK_RTC/32"]
    DIV32,
    #[doc = "CLK_RTC_OUT = CLK_RTC/64"]
    DIV64,
    #[doc = "CLK_RTC_OUT = CLK_RTC/128"]
    DIV128,
    #[doc = "CLK_RTC_OUT = CLK_RTC/256"]
    DIV256,
}
impl ACTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTFW::DIV2 => 0,
            ACTFW::DIV4 => 1,
            ACTFW::DIV8 => 2,
            ACTFW::DIV16 => 3,
            ACTFW::DIV32 => 4,
            ACTFW::DIV64 => 5,
            ACTFW::DIV128 => 6,
            ACTFW::DIV256 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTFW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(ACTFW::DIV2)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(ACTFW::DIV4)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(ACTFW::DIV8)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(ACTFW::DIV16)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(ACTFW::DIV32)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(ACTFW::DIV64)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(ACTFW::DIV128)
    }
    #[doc = "CLK_RTC_OUT = CLK_RTC/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(ACTFW::DIV256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - General Purpose 0 Enable"]
    #[inline]
    pub fn gp0en(&self) -> GP0ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        GP0ENR { bits }
    }
    #[doc = "Bit 1 - General Purpose 2 Enable"]
    #[inline]
    pub fn gp2en(&self) -> GP2ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        GP2ENR { bits }
    }
    #[doc = "Bit 4 - Debouncer Majority Enable"]
    #[inline]
    pub fn debmaj(&self) -> DEBMAJR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DEBMAJR { bits }
    }
    #[doc = "Bit 5 - Debouncer Asynchronous Enable"]
    #[inline]
    pub fn debasync(&self) -> DEBASYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DEBASYNCR { bits }
    }
    #[doc = "Bit 6 - RTC Output Enable"]
    #[inline]
    pub fn rtcout(&self) -> RTCOUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        RTCOUTR { bits }
    }
    #[doc = "Bit 7 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DMAENR { bits }
    }
    #[doc = "Bits 8:10 - Debounce Freqnuency"]
    #[inline]
    pub fn debf(&self) -> DEBFR {
        DEBFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 12:14 - Active Layer Freqnuency"]
    #[inline]
    pub fn actf(&self) -> ACTFR {
        ACTFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - General Purpose 0 Enable"]
    #[inline]
    pub fn gp0en(&mut self) -> _GP0ENW {
        _GP0ENW { w: self }
    }
    #[doc = "Bit 1 - General Purpose 2 Enable"]
    #[inline]
    pub fn gp2en(&mut self) -> _GP2ENW {
        _GP2ENW { w: self }
    }
    #[doc = "Bit 4 - Debouncer Majority Enable"]
    #[inline]
    pub fn debmaj(&mut self) -> _DEBMAJW {
        _DEBMAJW { w: self }
    }
    #[doc = "Bit 5 - Debouncer Asynchronous Enable"]
    #[inline]
    pub fn debasync(&mut self) -> _DEBASYNCW {
        _DEBASYNCW { w: self }
    }
    #[doc = "Bit 6 - RTC Output Enable"]
    #[inline]
    pub fn rtcout(&mut self) -> _RTCOUTW {
        _RTCOUTW { w: self }
    }
    #[doc = "Bit 7 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bits 8:10 - Debounce Freqnuency"]
    #[inline]
    pub fn debf(&mut self) -> _DEBFW {
        _DEBFW { w: self }
    }
    #[doc = "Bits 12:14 - Active Layer Freqnuency"]
    #[inline]
    pub fn actf(&mut self) -> _ACTFW {
        _ACTFW { w: self }
    }
}
