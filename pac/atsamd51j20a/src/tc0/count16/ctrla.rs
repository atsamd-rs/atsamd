#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLA {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Counter in 16-bit mode"]
    COUNT16,
    #[doc = "Counter in 8-bit mode"]
    COUNT8,
    #[doc = "Counter in 32-bit mode"]
    COUNT32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::COUNT16 => 0,
            MODER::COUNT8 => 1,
            MODER::COUNT32 => 2,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::COUNT16,
            1 => MODER::COUNT8,
            2 => MODER::COUNT32,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COUNT16`"]
    #[inline]
    pub fn is_count16(&self) -> bool {
        *self == MODER::COUNT16
    }
    #[doc = "Checks if the value of the field is `COUNT8`"]
    #[inline]
    pub fn is_count8(&self) -> bool {
        *self == MODER::COUNT8
    }
    #[doc = "Checks if the value of the field is `COUNT32`"]
    #[inline]
    pub fn is_count32(&self) -> bool {
        *self == MODER::COUNT32
    }
}
#[doc = "Possible values of the field `PRESCSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCSYNCR {
    #[doc = "Reload or reset the counter on next generic clock"]
    GCLK,
    #[doc = "Reload or reset the counter on next prescaler clock"]
    PRESC,
    #[doc = "Reload or reset the counter on next generic clock and reset the prescaler counter"]
    RESYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCSYNCR::GCLK => 0,
            PRESCSYNCR::PRESC => 1,
            PRESCSYNCR::RESYNC => 2,
            PRESCSYNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCSYNCR {
        match value {
            0 => PRESCSYNCR::GCLK,
            1 => PRESCSYNCR::PRESC,
            2 => PRESCSYNCR::RESYNC,
            i => PRESCSYNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline]
    pub fn is_gclk(&self) -> bool {
        *self == PRESCSYNCR::GCLK
    }
    #[doc = "Checks if the value of the field is `PRESC`"]
    #[inline]
    pub fn is_presc(&self) -> bool {
        *self == PRESCSYNCR::PRESC
    }
    #[doc = "Checks if the value of the field is `RESYNC`"]
    #[inline]
    pub fn is_resync(&self) -> bool {
        *self == PRESCSYNCR::RESYNC
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
pub struct ONDEMANDR {
    bits: bool,
}
impl ONDEMANDR {
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
    #[doc = "Prescaler: GCLK_TC"]
    DIV1,
    #[doc = "Prescaler: GCLK_TC/2"]
    DIV2,
    #[doc = "Prescaler: GCLK_TC/4"]
    DIV4,
    #[doc = "Prescaler: GCLK_TC/8"]
    DIV8,
    #[doc = "Prescaler: GCLK_TC/16"]
    DIV16,
    #[doc = "Prescaler: GCLK_TC/64"]
    DIV64,
    #[doc = "Prescaler: GCLK_TC/256"]
    DIV256,
    #[doc = "Prescaler: GCLK_TC/1024"]
    DIV1024,
}
impl PRESCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALERR::DIV1 => 0,
            PRESCALERR::DIV2 => 1,
            PRESCALERR::DIV4 => 2,
            PRESCALERR::DIV8 => 3,
            PRESCALERR::DIV16 => 4,
            PRESCALERR::DIV64 => 5,
            PRESCALERR::DIV256 => 6,
            PRESCALERR::DIV1024 => 7,
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
            5 => PRESCALERR::DIV64,
            6 => PRESCALERR::DIV256,
            7 => PRESCALERR::DIV1024,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALERR::DIV1024
    }
}
#[doc = r" Value of the field"]
pub struct ALOCKR {
    bits: bool,
}
impl ALOCKR {
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
pub struct CAPTEN0R {
    bits: bool,
}
impl CAPTEN0R {
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
pub struct CAPTEN1R {
    bits: bool,
}
impl CAPTEN1R {
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
pub struct COPEN0R {
    bits: bool,
}
impl COPEN0R {
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
pub struct COPEN1R {
    bits: bool,
}
impl COPEN1R {
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
#[doc = "Possible values of the field `CAPTMODE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTMODE0R {
    #[doc = "Default capture"]
    DEFAULT,
    #[doc = "Minimum capture"]
    CAPTMIN,
    #[doc = "Maximum capture"]
    CAPTMAX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CAPTMODE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAPTMODE0R::DEFAULT => 0,
            CAPTMODE0R::CAPTMIN => 1,
            CAPTMODE0R::CAPTMAX => 2,
            CAPTMODE0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAPTMODE0R {
        match value {
            0 => CAPTMODE0R::DEFAULT,
            1 => CAPTMODE0R::CAPTMIN,
            2 => CAPTMODE0R::CAPTMAX,
            i => CAPTMODE0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == CAPTMODE0R::DEFAULT
    }
    #[doc = "Checks if the value of the field is `CAPTMIN`"]
    #[inline]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTMODE0R::CAPTMIN
    }
    #[doc = "Checks if the value of the field is `CAPTMAX`"]
    #[inline]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTMODE0R::CAPTMAX
    }
}
#[doc = "Possible values of the field `CAPTMODE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTMODE1R {
    #[doc = "Default capture"]
    DEFAULT,
    #[doc = "Minimum capture"]
    CAPTMIN,
    #[doc = "Maximum capture"]
    CAPTMAX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CAPTMODE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CAPTMODE1R::DEFAULT => 0,
            CAPTMODE1R::CAPTMIN => 1,
            CAPTMODE1R::CAPTMAX => 2,
            CAPTMODE1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CAPTMODE1R {
        match value {
            0 => CAPTMODE1R::DEFAULT,
            1 => CAPTMODE1R::CAPTMIN,
            2 => CAPTMODE1R::CAPTMAX,
            i => CAPTMODE1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == CAPTMODE1R::DEFAULT
    }
    #[doc = "Checks if the value of the field is `CAPTMIN`"]
    #[inline]
    pub fn is_captmin(&self) -> bool {
        *self == CAPTMODE1R::CAPTMIN
    }
    #[doc = "Checks if the value of the field is `CAPTMAX`"]
    #[inline]
    pub fn is_captmax(&self) -> bool {
        *self == CAPTMODE1R::CAPTMAX
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
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Counter in 16-bit mode"]
    COUNT16,
    #[doc = "Counter in 8-bit mode"]
    COUNT8,
    #[doc = "Counter in 32-bit mode"]
    COUNT32,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::COUNT16 => 0,
            MODEW::COUNT8 => 1,
            MODEW::COUNT32 => 2,
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
    #[doc = "Counter in 16-bit mode"]
    #[inline]
    pub fn count16(self) -> &'a mut W {
        self.variant(MODEW::COUNT16)
    }
    #[doc = "Counter in 8-bit mode"]
    #[inline]
    pub fn count8(self) -> &'a mut W {
        self.variant(MODEW::COUNT8)
    }
    #[doc = "Counter in 32-bit mode"]
    #[inline]
    pub fn count32(self) -> &'a mut W {
        self.variant(MODEW::COUNT32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCSYNC`"]
pub enum PRESCSYNCW {
    #[doc = "Reload or reset the counter on next generic clock"]
    GCLK,
    #[doc = "Reload or reset the counter on next prescaler clock"]
    PRESC,
    #[doc = "Reload or reset the counter on next generic clock and reset the prescaler counter"]
    RESYNC,
}
impl PRESCSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCSYNCW::GCLK => 0,
            PRESCSYNCW::PRESC => 1,
            PRESCSYNCW::RESYNC => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCSYNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reload or reset the counter on next generic clock"]
    #[inline]
    pub fn gclk(self) -> &'a mut W {
        self.variant(PRESCSYNCW::GCLK)
    }
    #[doc = "Reload or reset the counter on next prescaler clock"]
    #[inline]
    pub fn presc(self) -> &'a mut W {
        self.variant(PRESCSYNCW::PRESC)
    }
    #[doc = "Reload or reset the counter on next generic clock and reset the prescaler counter"]
    #[inline]
    pub fn resync(self) -> &'a mut W {
        self.variant(PRESCSYNCW::RESYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
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
pub struct _ONDEMANDW<'a> {
    w: &'a mut W,
}
impl<'a> _ONDEMANDW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALER`"]
pub enum PRESCALERW {
    #[doc = "Prescaler: GCLK_TC"]
    DIV1,
    #[doc = "Prescaler: GCLK_TC/2"]
    DIV2,
    #[doc = "Prescaler: GCLK_TC/4"]
    DIV4,
    #[doc = "Prescaler: GCLK_TC/8"]
    DIV8,
    #[doc = "Prescaler: GCLK_TC/16"]
    DIV16,
    #[doc = "Prescaler: GCLK_TC/64"]
    DIV64,
    #[doc = "Prescaler: GCLK_TC/256"]
    DIV256,
    #[doc = "Prescaler: GCLK_TC/1024"]
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
            PRESCALERW::DIV64 => 5,
            PRESCALERW::DIV256 => 6,
            PRESCALERW::DIV1024 => 7,
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
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Prescaler: GCLK_TC"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV1)
    }
    #[doc = "Prescaler: GCLK_TC/2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV2)
    }
    #[doc = "Prescaler: GCLK_TC/4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV4)
    }
    #[doc = "Prescaler: GCLK_TC/8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV8)
    }
    #[doc = "Prescaler: GCLK_TC/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV16)
    }
    #[doc = "Prescaler: GCLK_TC/64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV64)
    }
    #[doc = "Prescaler: GCLK_TC/256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV256)
    }
    #[doc = "Prescaler: GCLK_TC/1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _ALOCKW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAPTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTEN0W<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CAPTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTEN1W<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COPEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _COPEN0W<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COPEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _COPEN1W<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTMODE0`"]
pub enum CAPTMODE0W {
    #[doc = "Default capture"]
    DEFAULT,
    #[doc = "Minimum capture"]
    CAPTMIN,
    #[doc = "Maximum capture"]
    CAPTMAX,
}
impl CAPTMODE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAPTMODE0W::DEFAULT => 0,
            CAPTMODE0W::CAPTMIN => 1,
            CAPTMODE0W::CAPTMAX => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTMODE0W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTMODE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTMODE0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Default capture"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(CAPTMODE0W::DEFAULT)
    }
    #[doc = "Minimum capture"]
    #[inline]
    pub fn captmin(self) -> &'a mut W {
        self.variant(CAPTMODE0W::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline]
    pub fn captmax(self) -> &'a mut W {
        self.variant(CAPTMODE0W::CAPTMAX)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAPTMODE1`"]
pub enum CAPTMODE1W {
    #[doc = "Default capture"]
    DEFAULT,
    #[doc = "Minimum capture"]
    CAPTMIN,
    #[doc = "Maximum capture"]
    CAPTMAX,
}
impl CAPTMODE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAPTMODE1W::DEFAULT => 0,
            CAPTMODE1W::CAPTMIN => 1,
            CAPTMODE1W::CAPTMAX => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTMODE1W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTMODE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTMODE1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Default capture"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(CAPTMODE1W::DEFAULT)
    }
    #[doc = "Minimum capture"]
    #[inline]
    pub fn captmin(self) -> &'a mut W {
        self.variant(CAPTMODE1W::CAPTMIN)
    }
    #[doc = "Maximum capture"]
    #[inline]
    pub fn captmax(self) -> &'a mut W {
        self.variant(CAPTMODE1W::CAPTMAX)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 2:3 - Timer Counter Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Prescaler and Counter Synchronization"]
    #[inline]
    pub fn prescsync(&self) -> PRESCSYNCR {
        PRESCSYNCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bit 7 - Clock On Demand"]
    #[inline]
    pub fn ondemand(&self) -> ONDEMANDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ONDEMANDR { bits }
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        PRESCALERR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline]
    pub fn alock(&self) -> ALOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALOCKR { bits }
    }
    #[doc = "Bit 16 - Capture Channel 0 Enable"]
    #[inline]
    pub fn capten0(&self) -> CAPTEN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAPTEN0R { bits }
    }
    #[doc = "Bit 17 - Capture Channel 1 Enable"]
    #[inline]
    pub fn capten1(&self) -> CAPTEN1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAPTEN1R { bits }
    }
    #[doc = "Bit 20 - Capture On Pin 0 Enable"]
    #[inline]
    pub fn copen0(&self) -> COPEN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COPEN0R { bits }
    }
    #[doc = "Bit 21 - Capture On Pin 1 Enable"]
    #[inline]
    pub fn copen1(&self) -> COPEN1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COPEN1R { bits }
    }
    #[doc = "Bits 24:25 - Capture Mode Channel 0"]
    #[inline]
    pub fn captmode0(&self) -> CAPTMODE0R {
        CAPTMODE0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:28 - Capture mode Channel 1"]
    #[inline]
    pub fn captmode1(&self) -> CAPTMODE1R {
        CAPTMODE1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
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
    #[doc = "Bits 2:3 - Timer Counter Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Prescaler and Counter Synchronization"]
    #[inline]
    pub fn prescsync(&mut self) -> _PRESCSYNCW {
        _PRESCSYNCW { w: self }
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bit 7 - Clock On Demand"]
    #[inline]
    pub fn ondemand(&mut self) -> _ONDEMANDW {
        _ONDEMANDW { w: self }
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
    #[doc = "Bit 11 - Auto Lock"]
    #[inline]
    pub fn alock(&mut self) -> _ALOCKW {
        _ALOCKW { w: self }
    }
    #[doc = "Bit 16 - Capture Channel 0 Enable"]
    #[inline]
    pub fn capten0(&mut self) -> _CAPTEN0W {
        _CAPTEN0W { w: self }
    }
    #[doc = "Bit 17 - Capture Channel 1 Enable"]
    #[inline]
    pub fn capten1(&mut self) -> _CAPTEN1W {
        _CAPTEN1W { w: self }
    }
    #[doc = "Bit 20 - Capture On Pin 0 Enable"]
    #[inline]
    pub fn copen0(&mut self) -> _COPEN0W {
        _COPEN0W { w: self }
    }
    #[doc = "Bit 21 - Capture On Pin 1 Enable"]
    #[inline]
    pub fn copen1(&mut self) -> _COPEN1W {
        _COPEN1W { w: self }
    }
    #[doc = "Bits 24:25 - Capture Mode Channel 0"]
    #[inline]
    pub fn captmode0(&mut self) -> _CAPTMODE0W {
        _CAPTMODE0W { w: self }
    }
    #[doc = "Bits 27:28 - Capture mode Channel 1"]
    #[inline]
    pub fn captmode1(&mut self) -> _CAPTMODE1W {
        _CAPTMODE1W { w: self }
    }
}
