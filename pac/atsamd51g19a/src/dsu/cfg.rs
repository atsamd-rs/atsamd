#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
        0x02
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct LQOSR {
    bits: u8,
}
impl LQOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DCCDMALEVEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCCDMALEVELR {
    #[doc = "Trigger rises when DCC is empty"]
    EMPTY,
    #[doc = "Trigger rises when DCC is full"]
    FULL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCCDMALEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCCDMALEVELR::EMPTY => 0,
            DCCDMALEVELR::FULL => 0x01,
            DCCDMALEVELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCCDMALEVELR {
        match value {
            0 => DCCDMALEVELR::EMPTY,
            1 => DCCDMALEVELR::FULL,
            i => DCCDMALEVELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == DCCDMALEVELR::EMPTY
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == DCCDMALEVELR::FULL
    }
}
#[doc = r" Value of the field"]
pub struct ETBRAMENR {
    bits: bool,
}
impl ETBRAMENR {
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
#[doc = r" Proxy"]
pub struct _LQOSW<'a> {
    w: &'a mut W,
}
impl<'a> _LQOSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 0);
        self.w.bits |= ((value as u32) & 0x03) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCCDMALEVEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCCDMALEVELW {
    #[doc = "Trigger rises when DCC is empty"]
    EMPTY,
    #[doc = "Trigger rises when DCC is full"]
    FULL,
}
impl DCCDMALEVELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCCDMALEVELW::EMPTY => 0,
            DCCDMALEVELW::FULL => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCCDMALEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _DCCDMALEVELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCCDMALEVELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Trigger rises when DCC is empty"]
    #[inline]
    pub fn empty(self) -> &'a mut W {
        self.variant(DCCDMALEVELW::EMPTY)
    }
    #[doc = "Trigger rises when DCC is full"]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(DCCDMALEVELW::FULL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 2);
        self.w.bits |= ((value as u32) & 0x03) << 2;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ETBRAMENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETBRAMENW<'a> {
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
        self.w.bits &= !(0x01 << 4);
        self.w.bits |= ((value as u32) & 0x01) << 4;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline]
    pub fn lqos(&self) -> LQOSR {
        let bits = ((self.bits >> 0) & 0x03) as u8;
        LQOSR { bits }
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline]
    pub fn dccdmalevel(&self) -> DCCDMALEVELR {
        DCCDMALEVELR::_from(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline]
    pub fn etbramen(&self) -> ETBRAMENR {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        ETBRAMENR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Latency Quality Of Service"]
    #[inline]
    pub fn lqos(&mut self) -> _LQOSW {
        _LQOSW { w: self }
    }
    #[doc = "Bits 2:3 - DMA Trigger Level"]
    #[inline]
    pub fn dccdmalevel(&mut self) -> _DCCDMALEVELW {
        _DCCDMALEVELW { w: self }
    }
    #[doc = "Bit 4 - Trace Control"]
    #[inline]
    pub fn etbramen(&mut self) -> _ETBRAMENW {
        _ETBRAMENW { w: self }
    }
}
