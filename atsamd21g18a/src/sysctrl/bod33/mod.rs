#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BOD33 {
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
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
pub struct HYSTR {
    bits: bool,
}
impl HYSTR {
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
#[doc = "Possible values of the field `ACTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIONR {
    #[doc = "No action"]
    NONE,
    #[doc = "The BOD33 generates a reset"]
    RESET,
    #[doc = "The BOD33 generates an interrupt"]
    INTERRUPT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTIONR::NONE => 0,
            ACTIONR::RESET => 1,
            ACTIONR::INTERRUPT => 2,
            ACTIONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACTIONR {
        match value {
            0 => ACTIONR::NONE,
            1 => ACTIONR::RESET,
            2 => ACTIONR::INTERRUPT,
            i => ACTIONR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACTIONR::NONE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == ACTIONR::RESET
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == ACTIONR::INTERRUPT
    }
}
#[doc = r" Value of the field"]
pub struct RUNSTDBYR {
    bits: bool,
}
impl RUNSTDBYR {
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
pub struct MODER {
    bits: bool,
}
impl MODER {
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
pub struct CENR {
    bits: bool,
}
impl CENR {
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
#[doc = "Possible values of the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELR {
    #[doc = "Divide clock by 2"]
    DIV2,
    #[doc = "Divide clock by 4"]
    DIV4,
    #[doc = "Divide clock by 8"]
    DIV8,
    #[doc = "Divide clock by 16"]
    DIV16,
    #[doc = "Divide clock by 32"]
    DIV32,
    #[doc = "Divide clock by 64"]
    DIV64,
    #[doc = "Divide clock by 128"]
    DIV128,
    #[doc = "Divide clock by 256"]
    DIV256,
    #[doc = "Divide clock by 512"]
    DIV512,
    #[doc = "Divide clock by 1024"]
    DIV1K,
    #[doc = "Divide clock by 2048"]
    DIV2K,
    #[doc = "Divide clock by 4096"]
    DIV4K,
    #[doc = "Divide clock by 8192"]
    DIV8K,
    #[doc = "Divide clock by 16384"]
    DIV16K,
    #[doc = "Divide clock by 32768"]
    DIV32K,
    #[doc = "Divide clock by 65536"]
    DIV64K,
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::DIV2 => 0,
            PSELR::DIV4 => 1,
            PSELR::DIV8 => 2,
            PSELR::DIV16 => 3,
            PSELR::DIV32 => 4,
            PSELR::DIV64 => 5,
            PSELR::DIV128 => 6,
            PSELR::DIV256 => 7,
            PSELR::DIV512 => 8,
            PSELR::DIV1K => 9,
            PSELR::DIV2K => 10,
            PSELR::DIV4K => 11,
            PSELR::DIV8K => 12,
            PSELR::DIV16K => 13,
            PSELR::DIV32K => 14,
            PSELR::DIV64K => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELR {
        match value {
            0 => PSELR::DIV2,
            1 => PSELR::DIV4,
            2 => PSELR::DIV8,
            3 => PSELR::DIV16,
            4 => PSELR::DIV32,
            5 => PSELR::DIV64,
            6 => PSELR::DIV128,
            7 => PSELR::DIV256,
            8 => PSELR::DIV512,
            9 => PSELR::DIV1K,
            10 => PSELR::DIV2K,
            11 => PSELR::DIV4K,
            12 => PSELR::DIV8K,
            13 => PSELR::DIV16K,
            14 => PSELR::DIV32K,
            15 => PSELR::DIV64K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PSELR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PSELR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PSELR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PSELR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PSELR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PSELR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PSELR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PSELR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == PSELR::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1K`"]
    #[inline]
    pub fn is_div1k(&self) -> bool {
        *self == PSELR::DIV1K
    }
    #[doc = "Checks if the value of the field is `DIV2K`"]
    #[inline]
    pub fn is_div2k(&self) -> bool {
        *self == PSELR::DIV2K
    }
    #[doc = "Checks if the value of the field is `DIV4K`"]
    #[inline]
    pub fn is_div4k(&self) -> bool {
        *self == PSELR::DIV4K
    }
    #[doc = "Checks if the value of the field is `DIV8K`"]
    #[inline]
    pub fn is_div8k(&self) -> bool {
        *self == PSELR::DIV8K
    }
    #[doc = "Checks if the value of the field is `DIV16K`"]
    #[inline]
    pub fn is_div16k(&self) -> bool {
        *self == PSELR::DIV16K
    }
    #[doc = "Checks if the value of the field is `DIV32K`"]
    #[inline]
    pub fn is_div32k(&self) -> bool {
        *self == PSELR::DIV32K
    }
    #[doc = "Checks if the value of the field is `DIV64K`"]
    #[inline]
    pub fn is_div64k(&self) -> bool {
        *self == PSELR::DIV64K
    }
}
#[doc = r" Value of the field"]
pub struct LEVELR {
    bits: u8,
}
impl LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACTION`"]
pub enum ACTIONW {
    #[doc = "No action"]
    NONE,
    #[doc = "The BOD33 generates a reset"]
    RESET,
    #[doc = "The BOD33 generates an interrupt"]
    INTERRUPT,
}
impl ACTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTIONW::NONE => 0,
            ACTIONW::RESET => 1,
            ACTIONW::INTERRUPT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACTIONW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No action"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTIONW::NONE)
    }
    #[doc = "The BOD33 generates a reset"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ACTIONW::RESET)
    }
    #[doc = "The BOD33 generates an interrupt"]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(ACTIONW::INTERRUPT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RUNSTDBYW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNSTDBYW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CENW<'a> {
    w: &'a mut W,
}
impl<'a> _CENW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
pub enum PSELW {
    #[doc = "Divide clock by 2"]
    DIV2,
    #[doc = "Divide clock by 4"]
    DIV4,
    #[doc = "Divide clock by 8"]
    DIV8,
    #[doc = "Divide clock by 16"]
    DIV16,
    #[doc = "Divide clock by 32"]
    DIV32,
    #[doc = "Divide clock by 64"]
    DIV64,
    #[doc = "Divide clock by 128"]
    DIV128,
    #[doc = "Divide clock by 256"]
    DIV256,
    #[doc = "Divide clock by 512"]
    DIV512,
    #[doc = "Divide clock by 1024"]
    DIV1K,
    #[doc = "Divide clock by 2048"]
    DIV2K,
    #[doc = "Divide clock by 4096"]
    DIV4K,
    #[doc = "Divide clock by 8192"]
    DIV8K,
    #[doc = "Divide clock by 16384"]
    DIV16K,
    #[doc = "Divide clock by 32768"]
    DIV32K,
    #[doc = "Divide clock by 65536"]
    DIV64K,
}
impl PSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELW::DIV2 => 0,
            PSELW::DIV4 => 1,
            PSELW::DIV8 => 2,
            PSELW::DIV16 => 3,
            PSELW::DIV32 => 4,
            PSELW::DIV64 => 5,
            PSELW::DIV128 => 6,
            PSELW::DIV256 => 7,
            PSELW::DIV512 => 8,
            PSELW::DIV1K => 9,
            PSELW::DIV2K => 10,
            PSELW::DIV4K => 11,
            PSELW::DIV8K => 12,
            PSELW::DIV16K => 13,
            PSELW::DIV32K => 14,
            PSELW::DIV64K => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide clock by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PSELW::DIV2)
    }
    #[doc = "Divide clock by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PSELW::DIV4)
    }
    #[doc = "Divide clock by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PSELW::DIV8)
    }
    #[doc = "Divide clock by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PSELW::DIV16)
    }
    #[doc = "Divide clock by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PSELW::DIV32)
    }
    #[doc = "Divide clock by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PSELW::DIV64)
    }
    #[doc = "Divide clock by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PSELW::DIV128)
    }
    #[doc = "Divide clock by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PSELW::DIV256)
    }
    #[doc = "Divide clock by 512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(PSELW::DIV512)
    }
    #[doc = "Divide clock by 1024"]
    #[inline]
    pub fn div1k(self) -> &'a mut W {
        self.variant(PSELW::DIV1K)
    }
    #[doc = "Divide clock by 2048"]
    #[inline]
    pub fn div2k(self) -> &'a mut W {
        self.variant(PSELW::DIV2K)
    }
    #[doc = "Divide clock by 4096"]
    #[inline]
    pub fn div4k(self) -> &'a mut W {
        self.variant(PSELW::DIV4K)
    }
    #[doc = "Divide clock by 8192"]
    #[inline]
    pub fn div8k(self) -> &'a mut W {
        self.variant(PSELW::DIV8K)
    }
    #[doc = "Divide clock by 16384"]
    #[inline]
    pub fn div16k(self) -> &'a mut W {
        self.variant(PSELW::DIV16K)
    }
    #[doc = "Divide clock by 32768"]
    #[inline]
    pub fn div32k(self) -> &'a mut W {
        self.variant(PSELW::DIV32K)
    }
    #[doc = "Divide clock by 65536"]
    #[inline]
    pub fn div64k(self) -> &'a mut W {
        self.variant(PSELW::DIV64K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _LEVELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 2 - Hysteresis"]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYSTR { bits }
    }
    #[doc = "Bits 3:4 - BOD33 Action"]
    #[inline]
    pub fn action(&self) -> ACTIONR {
        ACTIONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 8 - Operation Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MODER { bits }
    }
    #[doc = "Bit 9 - Clock Enable"]
    #[inline]
    pub fn cen(&self) -> CENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CENR { bits }
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - BOD33 Threshold Level"]
    #[inline]
    pub fn level(&self) -> LEVELR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LEVELR { bits }
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 2 - Hysteresis"]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
    #[doc = "Bits 3:4 - BOD33 Action"]
    #[inline]
    pub fn action(&mut self) -> _ACTIONW {
        _ACTIONW { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 8 - Operation Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 9 - Clock Enable"]
    #[inline]
    pub fn cen(&mut self) -> _CENW {
        _CENW { w: self }
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
    #[doc = "Bits 16:21 - BOD33 Threshold Level"]
    #[inline]
    pub fn level(&mut self) -> _LEVELW {
        _LEVELW { w: self }
    }
}
