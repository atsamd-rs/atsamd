#[doc = "Reader of register CHCTRLA"]
pub type R = crate::R<u32, super::CHCTRLA>;
#[doc = "Writer for register CHCTRLA"]
pub type W = crate::W<u32, super::CHCTRLA>;
#[doc = "Register CHCTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTRLA {
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
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSRC_A {
    #[doc = "0: Only software/event triggers"]
    DISABLE = 0,
}
impl From<TRIGSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGSRC`"]
pub type TRIGSRC_R = crate::R<u8, TRIGSRC_A>;
impl TRIGSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGSRC_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TRIGSRC_A::DISABLE
    }
}
#[doc = "Write proxy for field `TRIGSRC`"]
pub struct TRIGSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGACT_A {
    #[doc = "0: One trigger required for each block transfer"]
    BLOCK = 0,
    #[doc = "2: One trigger required for each burst transfer"]
    BURST = 2,
    #[doc = "3: One trigger required for each transaction"]
    TRANSACTION = 3,
}
impl From<TRIGACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGACT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGACT`"]
pub type TRIGACT_R = crate::R<u8, TRIGACT_A>;
impl TRIGACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGACT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGACT_A::BLOCK),
            2 => Val(TRIGACT_A::BURST),
            3 => Val(TRIGACT_A::TRANSACTION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == TRIGACT_A::BLOCK
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == TRIGACT_A::BURST
    }
    #[doc = "Checks if the value of the field is `TRANSACTION`"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == TRIGACT_A::TRANSACTION
    }
}
#[doc = "Write proxy for field `TRIGACT`"]
pub struct TRIGACT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGACT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn block(self) -> &'a mut W {
        self.variant(TRIGACT_A::BLOCK)
    }
    #[doc = "One trigger required for each burst transfer"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(TRIGACT_A::BURST)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut W {
        self.variant(TRIGACT_A::TRANSACTION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Burst Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURSTLEN_A {
    #[doc = "0: Single-beat burst length"]
    SINGLE = 0,
    #[doc = "1: 2-beats burst length"]
    _2BEAT = 1,
    #[doc = "2: 3-beats burst length"]
    _3BEAT = 2,
    #[doc = "3: 4-beats burst length"]
    _4BEAT = 3,
    #[doc = "4: 5-beats burst length"]
    _5BEAT = 4,
    #[doc = "5: 6-beats burst length"]
    _6BEAT = 5,
    #[doc = "6: 7-beats burst length"]
    _7BEAT = 6,
    #[doc = "7: 8-beats burst length"]
    _8BEAT = 7,
    #[doc = "8: 9-beats burst length"]
    _9BEAT = 8,
    #[doc = "9: 10-beats burst length"]
    _10BEAT = 9,
    #[doc = "10: 11-beats burst length"]
    _11BEAT = 10,
    #[doc = "11: 12-beats burst length"]
    _12BEAT = 11,
    #[doc = "12: 13-beats burst length"]
    _13BEAT = 12,
    #[doc = "13: 14-beats burst length"]
    _14BEAT = 13,
    #[doc = "14: 15-beats burst length"]
    _15BEAT = 14,
    #[doc = "15: 16-beats burst length"]
    _16BEAT = 15,
}
impl From<BURSTLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSTLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BURSTLEN`"]
pub type BURSTLEN_R = crate::R<u8, BURSTLEN_A>;
impl BURSTLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTLEN_A {
        match self.bits {
            0 => BURSTLEN_A::SINGLE,
            1 => BURSTLEN_A::_2BEAT,
            2 => BURSTLEN_A::_3BEAT,
            3 => BURSTLEN_A::_4BEAT,
            4 => BURSTLEN_A::_5BEAT,
            5 => BURSTLEN_A::_6BEAT,
            6 => BURSTLEN_A::_7BEAT,
            7 => BURSTLEN_A::_8BEAT,
            8 => BURSTLEN_A::_9BEAT,
            9 => BURSTLEN_A::_10BEAT,
            10 => BURSTLEN_A::_11BEAT,
            11 => BURSTLEN_A::_12BEAT,
            12 => BURSTLEN_A::_13BEAT,
            13 => BURSTLEN_A::_14BEAT,
            14 => BURSTLEN_A::_15BEAT,
            15 => BURSTLEN_A::_16BEAT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == BURSTLEN_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `_2BEAT`"]
    #[inline(always)]
    pub fn is_2beat(&self) -> bool {
        *self == BURSTLEN_A::_2BEAT
    }
    #[doc = "Checks if the value of the field is `_3BEAT`"]
    #[inline(always)]
    pub fn is_3beat(&self) -> bool {
        *self == BURSTLEN_A::_3BEAT
    }
    #[doc = "Checks if the value of the field is `_4BEAT`"]
    #[inline(always)]
    pub fn is_4beat(&self) -> bool {
        *self == BURSTLEN_A::_4BEAT
    }
    #[doc = "Checks if the value of the field is `_5BEAT`"]
    #[inline(always)]
    pub fn is_5beat(&self) -> bool {
        *self == BURSTLEN_A::_5BEAT
    }
    #[doc = "Checks if the value of the field is `_6BEAT`"]
    #[inline(always)]
    pub fn is_6beat(&self) -> bool {
        *self == BURSTLEN_A::_6BEAT
    }
    #[doc = "Checks if the value of the field is `_7BEAT`"]
    #[inline(always)]
    pub fn is_7beat(&self) -> bool {
        *self == BURSTLEN_A::_7BEAT
    }
    #[doc = "Checks if the value of the field is `_8BEAT`"]
    #[inline(always)]
    pub fn is_8beat(&self) -> bool {
        *self == BURSTLEN_A::_8BEAT
    }
    #[doc = "Checks if the value of the field is `_9BEAT`"]
    #[inline(always)]
    pub fn is_9beat(&self) -> bool {
        *self == BURSTLEN_A::_9BEAT
    }
    #[doc = "Checks if the value of the field is `_10BEAT`"]
    #[inline(always)]
    pub fn is_10beat(&self) -> bool {
        *self == BURSTLEN_A::_10BEAT
    }
    #[doc = "Checks if the value of the field is `_11BEAT`"]
    #[inline(always)]
    pub fn is_11beat(&self) -> bool {
        *self == BURSTLEN_A::_11BEAT
    }
    #[doc = "Checks if the value of the field is `_12BEAT`"]
    #[inline(always)]
    pub fn is_12beat(&self) -> bool {
        *self == BURSTLEN_A::_12BEAT
    }
    #[doc = "Checks if the value of the field is `_13BEAT`"]
    #[inline(always)]
    pub fn is_13beat(&self) -> bool {
        *self == BURSTLEN_A::_13BEAT
    }
    #[doc = "Checks if the value of the field is `_14BEAT`"]
    #[inline(always)]
    pub fn is_14beat(&self) -> bool {
        *self == BURSTLEN_A::_14BEAT
    }
    #[doc = "Checks if the value of the field is `_15BEAT`"]
    #[inline(always)]
    pub fn is_15beat(&self) -> bool {
        *self == BURSTLEN_A::_15BEAT
    }
    #[doc = "Checks if the value of the field is `_16BEAT`"]
    #[inline(always)]
    pub fn is_16beat(&self) -> bool {
        *self == BURSTLEN_A::_16BEAT
    }
}
#[doc = "Write proxy for field `BURSTLEN`"]
pub struct BURSTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTLEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-beat burst length"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(BURSTLEN_A::SINGLE)
    }
    #[doc = "2-beats burst length"]
    #[inline(always)]
    pub fn _2beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_2BEAT)
    }
    #[doc = "3-beats burst length"]
    #[inline(always)]
    pub fn _3beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_3BEAT)
    }
    #[doc = "4-beats burst length"]
    #[inline(always)]
    pub fn _4beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_4BEAT)
    }
    #[doc = "5-beats burst length"]
    #[inline(always)]
    pub fn _5beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_5BEAT)
    }
    #[doc = "6-beats burst length"]
    #[inline(always)]
    pub fn _6beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_6BEAT)
    }
    #[doc = "7-beats burst length"]
    #[inline(always)]
    pub fn _7beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_7BEAT)
    }
    #[doc = "8-beats burst length"]
    #[inline(always)]
    pub fn _8beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_8BEAT)
    }
    #[doc = "9-beats burst length"]
    #[inline(always)]
    pub fn _9beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_9BEAT)
    }
    #[doc = "10-beats burst length"]
    #[inline(always)]
    pub fn _10beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_10BEAT)
    }
    #[doc = "11-beats burst length"]
    #[inline(always)]
    pub fn _11beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_11BEAT)
    }
    #[doc = "12-beats burst length"]
    #[inline(always)]
    pub fn _12beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_12BEAT)
    }
    #[doc = "13-beats burst length"]
    #[inline(always)]
    pub fn _13beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_13BEAT)
    }
    #[doc = "14-beats burst length"]
    #[inline(always)]
    pub fn _14beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_14BEAT)
    }
    #[doc = "15-beats burst length"]
    #[inline(always)]
    pub fn _15beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_15BEAT)
    }
    #[doc = "16-beats burst length"]
    #[inline(always)]
    pub fn _16beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_16BEAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "FIFO Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THRESHOLD_A {
    #[doc = "0: Destination write starts after each beat source address read"]
    _1BEAT = 0,
    #[doc = "1: Destination write starts after 2-beats source address read"]
    _2BEATS = 1,
    #[doc = "2: Destination write starts after 4-beats source address read"]
    _4BEATS = 2,
    #[doc = "3: Destination write starts after 8-beats source address read"]
    _8BEATS = 3,
}
impl From<THRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: THRESHOLD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `THRESHOLD`"]
pub type THRESHOLD_R = crate::R<u8, THRESHOLD_A>;
impl THRESHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRESHOLD_A {
        match self.bits {
            0 => THRESHOLD_A::_1BEAT,
            1 => THRESHOLD_A::_2BEATS,
            2 => THRESHOLD_A::_4BEATS,
            3 => THRESHOLD_A::_8BEATS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1BEAT`"]
    #[inline(always)]
    pub fn is_1beat(&self) -> bool {
        *self == THRESHOLD_A::_1BEAT
    }
    #[doc = "Checks if the value of the field is `_2BEATS`"]
    #[inline(always)]
    pub fn is_2beats(&self) -> bool {
        *self == THRESHOLD_A::_2BEATS
    }
    #[doc = "Checks if the value of the field is `_4BEATS`"]
    #[inline(always)]
    pub fn is_4beats(&self) -> bool {
        *self == THRESHOLD_A::_4BEATS
    }
    #[doc = "Checks if the value of the field is `_8BEATS`"]
    #[inline(always)]
    pub fn is_8beats(&self) -> bool {
        *self == THRESHOLD_A::_8BEATS
    }
}
#[doc = "Write proxy for field `THRESHOLD`"]
pub struct THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THRESHOLD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Destination write starts after each beat source address read"]
    #[inline(always)]
    pub fn _1beat(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_1BEAT)
    }
    #[doc = "Destination write starts after 2-beats source address read"]
    #[inline(always)]
    pub fn _2beats(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_2BEATS)
    }
    #[doc = "Destination write starts after 4-beats source address read"]
    #[inline(always)]
    pub fn _4beats(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_4BEATS)
    }
    #[doc = "Destination write starts after 8-beats source address read"]
    #[inline(always)]
    pub fn _8beats(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_8BEATS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TRIGSRC_R {
        TRIGSRC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TRIGACT_R {
        TRIGACT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    pub fn burstlen(&self) -> BURSTLEN_R {
        BURSTLEN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&mut self) -> TRIGSRC_W {
        TRIGSRC_W { w: self }
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&mut self) -> TRIGACT_W {
        TRIGACT_W { w: self }
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    pub fn burstlen(&mut self) -> BURSTLEN_W {
        BURSTLEN_W { w: self }
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W {
        THRESHOLD_W { w: self }
    }
}
