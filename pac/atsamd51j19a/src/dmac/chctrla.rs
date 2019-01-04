#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHCTRLA {
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
pub struct SWRSTR {
    bits: bool,
}
impl SWRSTR {
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
#[doc = "Possible values of the field `TRIGSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGSRCR {
    #[doc = "Only software/event triggers"]
    DISABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIGSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGSRCR::DISABLE => 0,
            TRIGSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGSRCR {
        match value {
            0 => TRIGSRCR::DISABLE,
            i => TRIGSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == TRIGSRCR::DISABLE
    }
}
#[doc = "Possible values of the field `TRIGACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGACTR {
    #[doc = "One trigger required for each block transfer"]
    BLOCK,
    #[doc = "One trigger required for each burst transfer"]
    BURST,
    #[doc = "One trigger required for each transaction"]
    TRANSACTION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIGACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGACTR::BLOCK => 0,
            TRIGACTR::BURST => 2,
            TRIGACTR::TRANSACTION => 3,
            TRIGACTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGACTR {
        match value {
            0 => TRIGACTR::BLOCK,
            2 => TRIGACTR::BURST,
            3 => TRIGACTR::TRANSACTION,
            i => TRIGACTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline]
    pub fn is_block(&self) -> bool {
        *self == TRIGACTR::BLOCK
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline]
    pub fn is_burst(&self) -> bool {
        *self == TRIGACTR::BURST
    }
    #[doc = "Checks if the value of the field is `TRANSACTION`"]
    #[inline]
    pub fn is_transaction(&self) -> bool {
        *self == TRIGACTR::TRANSACTION
    }
}
#[doc = "Possible values of the field `BURSTLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTLENR {
    #[doc = "Single-beat burst length"]
    SINGLE,
    #[doc = "2-beats burst length"]
    _2BEAT,
    #[doc = "3-beats burst length"]
    _3BEAT,
    #[doc = "4-beats burst length"]
    _4BEAT,
    #[doc = "5-beats burst length"]
    _5BEAT,
    #[doc = "6-beats burst length"]
    _6BEAT,
    #[doc = "7-beats burst length"]
    _7BEAT,
    #[doc = "8-beats burst length"]
    _8BEAT,
    #[doc = "9-beats burst length"]
    _9BEAT,
    #[doc = "10-beats burst length"]
    _10BEAT,
    #[doc = "11-beats burst length"]
    _11BEAT,
    #[doc = "12-beats burst length"]
    _12BEAT,
    #[doc = "13-beats burst length"]
    _13BEAT,
    #[doc = "14-beats burst length"]
    _14BEAT,
    #[doc = "15-beats burst length"]
    _15BEAT,
    #[doc = "16-beats burst length"]
    _16BEAT,
}
impl BURSTLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BURSTLENR::SINGLE => 0,
            BURSTLENR::_2BEAT => 1,
            BURSTLENR::_3BEAT => 2,
            BURSTLENR::_4BEAT => 3,
            BURSTLENR::_5BEAT => 4,
            BURSTLENR::_6BEAT => 5,
            BURSTLENR::_7BEAT => 6,
            BURSTLENR::_8BEAT => 7,
            BURSTLENR::_9BEAT => 8,
            BURSTLENR::_10BEAT => 9,
            BURSTLENR::_11BEAT => 10,
            BURSTLENR::_12BEAT => 11,
            BURSTLENR::_13BEAT => 12,
            BURSTLENR::_14BEAT => 13,
            BURSTLENR::_15BEAT => 14,
            BURSTLENR::_16BEAT => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BURSTLENR {
        match value {
            0 => BURSTLENR::SINGLE,
            1 => BURSTLENR::_2BEAT,
            2 => BURSTLENR::_3BEAT,
            3 => BURSTLENR::_4BEAT,
            4 => BURSTLENR::_5BEAT,
            5 => BURSTLENR::_6BEAT,
            6 => BURSTLENR::_7BEAT,
            7 => BURSTLENR::_8BEAT,
            8 => BURSTLENR::_9BEAT,
            9 => BURSTLENR::_10BEAT,
            10 => BURSTLENR::_11BEAT,
            11 => BURSTLENR::_12BEAT,
            12 => BURSTLENR::_13BEAT,
            13 => BURSTLENR::_14BEAT,
            14 => BURSTLENR::_15BEAT,
            15 => BURSTLENR::_16BEAT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == BURSTLENR::SINGLE
    }
    #[doc = "Checks if the value of the field is `_2BEAT`"]
    #[inline]
    pub fn is_2beat(&self) -> bool {
        *self == BURSTLENR::_2BEAT
    }
    #[doc = "Checks if the value of the field is `_3BEAT`"]
    #[inline]
    pub fn is_3beat(&self) -> bool {
        *self == BURSTLENR::_3BEAT
    }
    #[doc = "Checks if the value of the field is `_4BEAT`"]
    #[inline]
    pub fn is_4beat(&self) -> bool {
        *self == BURSTLENR::_4BEAT
    }
    #[doc = "Checks if the value of the field is `_5BEAT`"]
    #[inline]
    pub fn is_5beat(&self) -> bool {
        *self == BURSTLENR::_5BEAT
    }
    #[doc = "Checks if the value of the field is `_6BEAT`"]
    #[inline]
    pub fn is_6beat(&self) -> bool {
        *self == BURSTLENR::_6BEAT
    }
    #[doc = "Checks if the value of the field is `_7BEAT`"]
    #[inline]
    pub fn is_7beat(&self) -> bool {
        *self == BURSTLENR::_7BEAT
    }
    #[doc = "Checks if the value of the field is `_8BEAT`"]
    #[inline]
    pub fn is_8beat(&self) -> bool {
        *self == BURSTLENR::_8BEAT
    }
    #[doc = "Checks if the value of the field is `_9BEAT`"]
    #[inline]
    pub fn is_9beat(&self) -> bool {
        *self == BURSTLENR::_9BEAT
    }
    #[doc = "Checks if the value of the field is `_10BEAT`"]
    #[inline]
    pub fn is_10beat(&self) -> bool {
        *self == BURSTLENR::_10BEAT
    }
    #[doc = "Checks if the value of the field is `_11BEAT`"]
    #[inline]
    pub fn is_11beat(&self) -> bool {
        *self == BURSTLENR::_11BEAT
    }
    #[doc = "Checks if the value of the field is `_12BEAT`"]
    #[inline]
    pub fn is_12beat(&self) -> bool {
        *self == BURSTLENR::_12BEAT
    }
    #[doc = "Checks if the value of the field is `_13BEAT`"]
    #[inline]
    pub fn is_13beat(&self) -> bool {
        *self == BURSTLENR::_13BEAT
    }
    #[doc = "Checks if the value of the field is `_14BEAT`"]
    #[inline]
    pub fn is_14beat(&self) -> bool {
        *self == BURSTLENR::_14BEAT
    }
    #[doc = "Checks if the value of the field is `_15BEAT`"]
    #[inline]
    pub fn is_15beat(&self) -> bool {
        *self == BURSTLENR::_15BEAT
    }
    #[doc = "Checks if the value of the field is `_16BEAT`"]
    #[inline]
    pub fn is_16beat(&self) -> bool {
        *self == BURSTLENR::_16BEAT
    }
}
#[doc = "Possible values of the field `THRESHOLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRESHOLDR {
    #[doc = "Destination write starts after each beat source address read"]
    _1BEAT,
    #[doc = "Destination write starts after 2-beats source address read"]
    _2BEATS,
    #[doc = "Destination write starts after 4-beats source address read"]
    _4BEATS,
    #[doc = "Destination write starts after 8-beats source address read"]
    _8BEATS,
}
impl THRESHOLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            THRESHOLDR::_1BEAT => 0,
            THRESHOLDR::_2BEATS => 1,
            THRESHOLDR::_4BEATS => 2,
            THRESHOLDR::_8BEATS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> THRESHOLDR {
        match value {
            0 => THRESHOLDR::_1BEAT,
            1 => THRESHOLDR::_2BEATS,
            2 => THRESHOLDR::_4BEATS,
            3 => THRESHOLDR::_8BEATS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1BEAT`"]
    #[inline]
    pub fn is_1beat(&self) -> bool {
        *self == THRESHOLDR::_1BEAT
    }
    #[doc = "Checks if the value of the field is `_2BEATS`"]
    #[inline]
    pub fn is_2beats(&self) -> bool {
        *self == THRESHOLDR::_2BEATS
    }
    #[doc = "Checks if the value of the field is `_4BEATS`"]
    #[inline]
    pub fn is_4beats(&self) -> bool {
        *self == THRESHOLDR::_4BEATS
    }
    #[doc = "Checks if the value of the field is `_8BEATS`"]
    #[inline]
    pub fn is_8beats(&self) -> bool {
        *self == THRESHOLDR::_8BEATS
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
#[doc = "Values that can be written to the field `TRIGSRC`"]
pub enum TRIGSRCW {
    #[doc = "Only software/event triggers"]
    DISABLE,
}
impl TRIGSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGSRCW::DISABLE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Only software/event triggers"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(TRIGSRCW::DISABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIGACT`"]
pub enum TRIGACTW {
    #[doc = "One trigger required for each block transfer"]
    BLOCK,
    #[doc = "One trigger required for each burst transfer"]
    BURST,
    #[doc = "One trigger required for each transaction"]
    TRANSACTION,
}
impl TRIGACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGACTW::BLOCK => 0,
            TRIGACTW::BURST => 2,
            TRIGACTW::TRANSACTION => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGACTW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGACTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline]
    pub fn block(self) -> &'a mut W {
        self.variant(TRIGACTW::BLOCK)
    }
    #[doc = "One trigger required for each burst transfer"]
    #[inline]
    pub fn burst(self) -> &'a mut W {
        self.variant(TRIGACTW::BURST)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline]
    pub fn transaction(self) -> &'a mut W {
        self.variant(TRIGACTW::TRANSACTION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BURSTLEN`"]
pub enum BURSTLENW {
    #[doc = "Single-beat burst length"]
    SINGLE,
    #[doc = "2-beats burst length"]
    _2BEAT,
    #[doc = "3-beats burst length"]
    _3BEAT,
    #[doc = "4-beats burst length"]
    _4BEAT,
    #[doc = "5-beats burst length"]
    _5BEAT,
    #[doc = "6-beats burst length"]
    _6BEAT,
    #[doc = "7-beats burst length"]
    _7BEAT,
    #[doc = "8-beats burst length"]
    _8BEAT,
    #[doc = "9-beats burst length"]
    _9BEAT,
    #[doc = "10-beats burst length"]
    _10BEAT,
    #[doc = "11-beats burst length"]
    _11BEAT,
    #[doc = "12-beats burst length"]
    _12BEAT,
    #[doc = "13-beats burst length"]
    _13BEAT,
    #[doc = "14-beats burst length"]
    _14BEAT,
    #[doc = "15-beats burst length"]
    _15BEAT,
    #[doc = "16-beats burst length"]
    _16BEAT,
}
impl BURSTLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BURSTLENW::SINGLE => 0,
            BURSTLENW::_2BEAT => 1,
            BURSTLENW::_3BEAT => 2,
            BURSTLENW::_4BEAT => 3,
            BURSTLENW::_5BEAT => 4,
            BURSTLENW::_6BEAT => 5,
            BURSTLENW::_7BEAT => 6,
            BURSTLENW::_8BEAT => 7,
            BURSTLENW::_9BEAT => 8,
            BURSTLENW::_10BEAT => 9,
            BURSTLENW::_11BEAT => 10,
            BURSTLENW::_12BEAT => 11,
            BURSTLENW::_13BEAT => 12,
            BURSTLENW::_14BEAT => 13,
            BURSTLENW::_15BEAT => 14,
            BURSTLENW::_16BEAT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTLENW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTLENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single-beat burst length"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(BURSTLENW::SINGLE)
    }
    #[doc = "2-beats burst length"]
    #[inline]
    pub fn _2beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_2BEAT)
    }
    #[doc = "3-beats burst length"]
    #[inline]
    pub fn _3beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_3BEAT)
    }
    #[doc = "4-beats burst length"]
    #[inline]
    pub fn _4beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_4BEAT)
    }
    #[doc = "5-beats burst length"]
    #[inline]
    pub fn _5beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_5BEAT)
    }
    #[doc = "6-beats burst length"]
    #[inline]
    pub fn _6beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_6BEAT)
    }
    #[doc = "7-beats burst length"]
    #[inline]
    pub fn _7beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_7BEAT)
    }
    #[doc = "8-beats burst length"]
    #[inline]
    pub fn _8beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_8BEAT)
    }
    #[doc = "9-beats burst length"]
    #[inline]
    pub fn _9beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_9BEAT)
    }
    #[doc = "10-beats burst length"]
    #[inline]
    pub fn _10beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_10BEAT)
    }
    #[doc = "11-beats burst length"]
    #[inline]
    pub fn _11beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_11BEAT)
    }
    #[doc = "12-beats burst length"]
    #[inline]
    pub fn _12beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_12BEAT)
    }
    #[doc = "13-beats burst length"]
    #[inline]
    pub fn _13beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_13BEAT)
    }
    #[doc = "14-beats burst length"]
    #[inline]
    pub fn _14beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_14BEAT)
    }
    #[doc = "15-beats burst length"]
    #[inline]
    pub fn _15beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_15BEAT)
    }
    #[doc = "16-beats burst length"]
    #[inline]
    pub fn _16beat(self) -> &'a mut W {
        self.variant(BURSTLENW::_16BEAT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `THRESHOLD`"]
pub enum THRESHOLDW {
    #[doc = "Destination write starts after each beat source address read"]
    _1BEAT,
    #[doc = "Destination write starts after 2-beats source address read"]
    _2BEATS,
    #[doc = "Destination write starts after 4-beats source address read"]
    _4BEATS,
    #[doc = "Destination write starts after 8-beats source address read"]
    _8BEATS,
}
impl THRESHOLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            THRESHOLDW::_1BEAT => 0,
            THRESHOLDW::_2BEATS => 1,
            THRESHOLDW::_4BEATS => 2,
            THRESHOLDW::_8BEATS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _THRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _THRESHOLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: THRESHOLDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Destination write starts after each beat source address read"]
    #[inline]
    pub fn _1beat(self) -> &'a mut W {
        self.variant(THRESHOLDW::_1BEAT)
    }
    #[doc = "Destination write starts after 2-beats source address read"]
    #[inline]
    pub fn _2beats(self) -> &'a mut W {
        self.variant(THRESHOLDW::_2BEATS)
    }
    #[doc = "Destination write starts after 4-beats source address read"]
    #[inline]
    pub fn _4beats(self) -> &'a mut W {
        self.variant(THRESHOLDW::_4BEATS)
    }
    #[doc = "Destination write starts after 8-beats source address read"]
    #[inline]
    pub fn _8beats(self) -> &'a mut W {
        self.variant(THRESHOLDW::_8BEATS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWRSTR { bits }
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENABLER { bits }
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RUNSTDBYR { bits }
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline]
    pub fn trigsrc(&self) -> TRIGSRCR {
        TRIGSRCR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline]
    pub fn trigact(&self) -> TRIGACTR {
        TRIGACTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline]
    pub fn burstlen(&self) -> BURSTLENR {
        BURSTLENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline]
    pub fn threshold(&self) -> THRESHOLDR {
        THRESHOLDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline]
    pub fn trigsrc(&mut self) -> _TRIGSRCW {
        _TRIGSRCW { w: self }
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline]
    pub fn trigact(&mut self) -> _TRIGACTW {
        _TRIGACTW { w: self }
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline]
    pub fn burstlen(&mut self) -> _BURSTLENW {
        _BURSTLENW { w: self }
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline]
    pub fn threshold(&mut self) -> _THRESHOLDW {
        _THRESHOLDW { w: self }
    }
}
