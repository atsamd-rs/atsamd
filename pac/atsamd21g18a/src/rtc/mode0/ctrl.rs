#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRL {
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
    pub const fn reset_value() -> u16 {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Mode 0: 32-bit Counter"]
    COUNT32,
    #[doc = "Mode 1: 16-bit Counter"]
    COUNT16,
    #[doc = "Mode 2: Clock/Calendar"]
    CLOCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::COUNT32 => 0,
            MODER::COUNT16 => 0x01,
            MODER::CLOCK => 0x02,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::COUNT32,
            1 => MODER::COUNT16,
            2 => MODER::CLOCK,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COUNT32`"]
    #[inline]
    pub fn is_count32(&self) -> bool {
        *self == MODER::COUNT32
    }
    #[doc = "Checks if the value of the field is `COUNT16`"]
    #[inline]
    pub fn is_count16(&self) -> bool {
        *self == MODER::COUNT16
    }
    #[doc = "Checks if the value of the field is `CLOCK`"]
    #[inline]
    pub fn is_clock(&self) -> bool {
        *self == MODER::CLOCK
    }
}
#[doc = r" Value of the field"]
pub struct MATCHCLRR {
    bits: bool,
}
impl MATCHCLRR {
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
#[doc = "Possible values of the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERR {
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    DIV1,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/2"]
    DIV2,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/4"]
    DIV4,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/8"]
    DIV8,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/16"]
    DIV16,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/32"]
    DIV32,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/64"]
    DIV64,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/128"]
    DIV128,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/256"]
    DIV256,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/512"]
    DIV512,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1024"]
    DIV1024,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALERR::DIV1 => 0,
            PRESCALERR::DIV2 => 0x01,
            PRESCALERR::DIV4 => 0x02,
            PRESCALERR::DIV8 => 0x03,
            PRESCALERR::DIV16 => 0x04,
            PRESCALERR::DIV32 => 0x05,
            PRESCALERR::DIV64 => 0x06,
            PRESCALERR::DIV128 => 0x07,
            PRESCALERR::DIV256 => 0x08,
            PRESCALERR::DIV512 => 0x09,
            PRESCALERR::DIV1024 => 0x0a,
            PRESCALERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALERR {
        match value {
            0 => PRESCALERR::DIV1,
            1 => PRESCALERR::DIV2,
            2 => PRESCALERR::DIV4,
            3 => PRESCALERR::DIV8,
            4 => PRESCALERR::DIV16,
            5 => PRESCALERR::DIV32,
            6 => PRESCALERR::DIV64,
            7 => PRESCALERR::DIV128,
            8 => PRESCALERR::DIV256,
            9 => PRESCALERR::DIV512,
            10 => PRESCALERR::DIV1024,
            i => PRESCALERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALERR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALERR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALERR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALERR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALERR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == PRESCALERR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == PRESCALERR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == PRESCALERR::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALERR::DIV1024
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTW<'a> {
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
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u16) & 0x01) << 0;
        self.w
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
        self.w.bits |= ((value as u16) & 0x01) << 1;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEW {
    #[doc = "Mode 0: 32-bit Counter"]
    COUNT32,
    #[doc = "Mode 1: 16-bit Counter"]
    COUNT16,
    #[doc = "Mode 2: Clock/Calendar"]
    CLOCK,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::COUNT32 => 0,
            MODEW::COUNT16 => 1,
            MODEW::CLOCK => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Mode 0: 32-bit Counter"]
    #[inline]
    pub fn count32(self) -> &'a mut W {
        self.variant(MODEW::COUNT32)
    }
    #[doc = "Mode 1: 16-bit Counter"]
    #[inline]
    pub fn count16(self) -> &'a mut W {
        self.variant(MODEW::COUNT16)
    }
    #[doc = "Mode 2: Clock/Calendar"]
    #[inline]
    pub fn clock(self) -> &'a mut W {
        self.variant(MODEW::CLOCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 2);
        self.w.bits |= ((value as u16) & 0x03) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MATCHCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCHCLRW<'a> {
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
        self.w.bits &= !(0x01 << 7);
        self.w.bits |= ((value as u16) & 0x01) << 7;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERW {
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    DIV1,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/2"]
    DIV2,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/4"]
    DIV4,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/8"]
    DIV8,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/16"]
    DIV16,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/32"]
    DIV32,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/64"]
    DIV64,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/128"]
    DIV128,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/256"]
    DIV256,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/512"]
    DIV512,
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1024"]
    DIV1024,
}
impl PRESCALERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALERW::DIV1 => 0,
            PRESCALERW::DIV2 => 1,
            PRESCALERW::DIV4 => 2,
            PRESCALERW::DIV8 => 3,
            PRESCALERW::DIV16 => 4,
            PRESCALERW::DIV32 => 5,
            PRESCALERW::DIV64 => 6,
            PRESCALERW::DIV128 => 7,
            PRESCALERW::DIV256 => 8,
            PRESCALERW::DIV512 => 9,
            PRESCALERW::DIV1024 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV1)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV2)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV4)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV8)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV16)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV32)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV64)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV128)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV256)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV512)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x0f << 8);
        self.w.bits |= ((value as u16) & 0x0f) << 8;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        ENABLER { bits }
    }
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Clear on Match"]
    #[inline]
    pub fn matchclr(&self) -> MATCHCLRR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        MATCHCLRR { bits }
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        PRESCALERR::_from(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 7 - Clear on Match"]
    #[inline]
    pub fn matchclr(&mut self) -> _MATCHCLRW {
        _MATCHCLRW { w: self }
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
}
