#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BOD12 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
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
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
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
    #[doc = "The BOD12 generates a reset"]
    RESET,
    #[doc = "The BOD12 generates an interrupt"]
    INT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACTIONR::NONE => 0,
            ACTIONR::RESET => 0x01,
            ACTIONR::INT => 0x02,
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
            2 => ACTIONR::INT,
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
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline]
    pub fn is_int(&self) -> bool {
        *self == ACTIONR::INT
    }
}
#[doc = r" Value of the field"]
pub struct STDBYCFGR {
    bits: bool,
}
impl STDBYCFGR {
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
pub struct ACTCFGR {
    bits: bool,
}
impl ACTCFGR {
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
    DIV1024,
    #[doc = "Divide clock by 2048"]
    DIV2048,
    #[doc = "Divide clock by 4096"]
    DIV4096,
    #[doc = "Divide clock by 8192"]
    DIV8192,
    #[doc = "Divide clock by 16384"]
    DIV16384,
    #[doc = "Divide clock by 32768"]
    DIV32768,
    #[doc = "Divide clock by 65536"]
    DIV65536,
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::DIV2 => 0,
            PSELR::DIV4 => 0x01,
            PSELR::DIV8 => 0x02,
            PSELR::DIV16 => 0x03,
            PSELR::DIV32 => 0x04,
            PSELR::DIV64 => 0x05,
            PSELR::DIV128 => 0x06,
            PSELR::DIV256 => 0x07,
            PSELR::DIV512 => 0x08,
            PSELR::DIV1024 => 0x09,
            PSELR::DIV2048 => 0x0a,
            PSELR::DIV4096 => 0x0b,
            PSELR::DIV8192 => 0x0c,
            PSELR::DIV16384 => 0x0d,
            PSELR::DIV32768 => 0x0e,
            PSELR::DIV65536 => 0x0f,
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
            9 => PSELR::DIV1024,
            10 => PSELR::DIV2048,
            11 => PSELR::DIV4096,
            12 => PSELR::DIV8192,
            13 => PSELR::DIV16384,
            14 => PSELR::DIV32768,
            15 => PSELR::DIV65536,
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
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == PSELR::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline]
    pub fn is_div2048(&self) -> bool {
        *self == PSELR::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline]
    pub fn is_div4096(&self) -> bool {
        *self == PSELR::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline]
    pub fn is_div8192(&self) -> bool {
        *self == PSELR::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline]
    pub fn is_div16384(&self) -> bool {
        *self == PSELR::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline]
    pub fn is_div32768(&self) -> bool {
        *self == PSELR::DIV32768
    }
    #[doc = "Checks if the value of the field is `DIV65536`"]
    #[inline]
    pub fn is_div65536(&self) -> bool {
        *self == PSELR::DIV65536
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
        self.w.bits &= !(0x01 << 1);
        self.w.bits |= ((value as u32) & 0x01) << 1;
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
        self.w.bits &= !(0x01 << 2);
        self.w.bits |= ((value as u32) & 0x01) << 2;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIONW {
    #[doc = "No action"]
    NONE,
    #[doc = "The BOD12 generates a reset"]
    RESET,
    #[doc = "The BOD12 generates an interrupt"]
    INT,
}
impl ACTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACTIONW::NONE => 0,
            ACTIONW::RESET => 1,
            ACTIONW::INT => 2,
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
    #[doc = "The BOD12 generates a reset"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(ACTIONW::RESET)
    }
    #[doc = "The BOD12 generates an interrupt"]
    #[inline]
    pub fn int(self) -> &'a mut W {
        self.variant(ACTIONW::INT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 3);
        self.w.bits |= ((value as u32) & 0x03) << 3;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STDBYCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _STDBYCFGW<'a> {
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
        self.w.bits &= !(0x01 << 5);
        self.w.bits |= ((value as u32) & 0x01) << 5;
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
        self.w.bits &= !(0x01 << 6);
        self.w.bits |= ((value as u32) & 0x01) << 6;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTCFGW<'a> {
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
        self.w.bits &= !(0x01 << 8);
        self.w.bits |= ((value as u32) & 0x01) << 8;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    DIV1024,
    #[doc = "Divide clock by 2048"]
    DIV2048,
    #[doc = "Divide clock by 4096"]
    DIV4096,
    #[doc = "Divide clock by 8192"]
    DIV8192,
    #[doc = "Divide clock by 16384"]
    DIV16384,
    #[doc = "Divide clock by 32768"]
    DIV32768,
    #[doc = "Divide clock by 65536"]
    DIV65536,
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
            PSELW::DIV1024 => 9,
            PSELW::DIV2048 => 10,
            PSELW::DIV4096 => 11,
            PSELW::DIV8192 => 12,
            PSELW::DIV16384 => 13,
            PSELW::DIV32768 => 14,
            PSELW::DIV65536 => 15,
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
    pub fn div1024(self) -> &'a mut W {
        self.variant(PSELW::DIV1024)
    }
    #[doc = "Divide clock by 2048"]
    #[inline]
    pub fn div2048(self) -> &'a mut W {
        self.variant(PSELW::DIV2048)
    }
    #[doc = "Divide clock by 4096"]
    #[inline]
    pub fn div4096(self) -> &'a mut W {
        self.variant(PSELW::DIV4096)
    }
    #[doc = "Divide clock by 8192"]
    #[inline]
    pub fn div8192(self) -> &'a mut W {
        self.variant(PSELW::DIV8192)
    }
    #[doc = "Divide clock by 16384"]
    #[inline]
    pub fn div16384(self) -> &'a mut W {
        self.variant(PSELW::DIV16384)
    }
    #[doc = "Divide clock by 32768"]
    #[inline]
    pub fn div32768(self) -> &'a mut W {
        self.variant(PSELW::DIV32768)
    }
    #[doc = "Divide clock by 65536"]
    #[inline]
    pub fn div65536(self) -> &'a mut W {
        self.variant(PSELW::DIV65536)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 12);
        self.w.bits |= ((value as u32) & 0x0f) << 12;
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
        self.w.bits &= !(0x3f << 16);
        self.w.bits |= ((value as u32) & 0x3f) << 16;
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
        let bits = ((self.bits >> 1) & 0x01) != 0;
        ENABLER { bits }
    }
    #[doc = "Bit 2 - Hysteresis Enable"]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        HYSTR { bits }
    }
    #[doc = "Bits 3:4 - Action when Threshold Crossed"]
    #[inline]
    pub fn action(&self) -> ACTIONR {
        ACTIONR::_from(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Configuration in Standby mode"]
    #[inline]
    pub fn stdbycfg(&self) -> STDBYCFGR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        STDBYCFGR { bits }
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 8 - Configuration in Active mode"]
    #[inline]
    pub fn actcfg(&self) -> ACTCFGR {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        ACTCFGR { bits }
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Threshold Level"]
    #[inline]
    pub fn level(&self) -> LEVELR {
        let bits = ((self.bits >> 16) & 0x3f) as u8;
        LEVELR { bits }
    }
}
impl W {
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
    #[doc = "Bit 2 - Hysteresis Enable"]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
    #[doc = "Bits 3:4 - Action when Threshold Crossed"]
    #[inline]
    pub fn action(&mut self) -> _ACTIONW {
        _ACTIONW { w: self }
    }
    #[doc = "Bit 5 - Configuration in Standby mode"]
    #[inline]
    pub fn stdbycfg(&mut self) -> _STDBYCFGW {
        _STDBYCFGW { w: self }
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 8 - Configuration in Active mode"]
    #[inline]
    pub fn actcfg(&mut self) -> _ACTCFGW {
        _ACTCFGW { w: self }
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
    #[doc = "Bits 16:21 - Threshold Level"]
    #[inline]
    pub fn level(&mut self) -> _LEVELW {
        _LEVELW { w: self }
    }
}
