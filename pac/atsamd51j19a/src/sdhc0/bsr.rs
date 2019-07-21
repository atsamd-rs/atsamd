#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::BSR {
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
pub struct BLOCKSIZER {
    bits: u16,
}
impl BLOCKSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `BOUNDARY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOUNDARYR {
    #[doc = "4k bytes"]
    _4K,
    #[doc = "8k bytes"]
    _8K,
    #[doc = "16k bytes"]
    _16K,
    #[doc = "32k bytes"]
    _32K,
    #[doc = "64k bytes"]
    _64K,
    #[doc = "128k bytes"]
    _128K,
    #[doc = "256k bytes"]
    _256K,
    #[doc = "512k bytes"]
    _512K,
}
impl BOUNDARYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BOUNDARYR::_4K => 0,
            BOUNDARYR::_8K => 0x01,
            BOUNDARYR::_16K => 0x02,
            BOUNDARYR::_32K => 0x03,
            BOUNDARYR::_64K => 0x04,
            BOUNDARYR::_128K => 0x05,
            BOUNDARYR::_256K => 0x06,
            BOUNDARYR::_512K => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BOUNDARYR {
        match value {
            0 => BOUNDARYR::_4K,
            1 => BOUNDARYR::_8K,
            2 => BOUNDARYR::_16K,
            3 => BOUNDARYR::_32K,
            4 => BOUNDARYR::_64K,
            5 => BOUNDARYR::_128K,
            6 => BOUNDARYR::_256K,
            7 => BOUNDARYR::_512K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline]
    pub fn is_4k(&self) -> bool {
        *self == BOUNDARYR::_4K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline]
    pub fn is_8k(&self) -> bool {
        *self == BOUNDARYR::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline]
    pub fn is_16k(&self) -> bool {
        *self == BOUNDARYR::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline]
    pub fn is_32k(&self) -> bool {
        *self == BOUNDARYR::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline]
    pub fn is_64k(&self) -> bool {
        *self == BOUNDARYR::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline]
    pub fn is_128k(&self) -> bool {
        *self == BOUNDARYR::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline]
    pub fn is_256k(&self) -> bool {
        *self == BOUNDARYR::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline]
    pub fn is_512k(&self) -> bool {
        *self == BOUNDARYR::_512K
    }
}
#[doc = r" Proxy"]
pub struct _BLOCKSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLOCKSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(0x03ff << 0);
        self.w.bits |= ((value as u16) & 0x03ff) << 0;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOUNDARY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOUNDARYW {
    #[doc = "4k bytes"]
    _4K,
    #[doc = "8k bytes"]
    _8K,
    #[doc = "16k bytes"]
    _16K,
    #[doc = "32k bytes"]
    _32K,
    #[doc = "64k bytes"]
    _64K,
    #[doc = "128k bytes"]
    _128K,
    #[doc = "256k bytes"]
    _256K,
    #[doc = "512k bytes"]
    _512K,
}
impl BOUNDARYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BOUNDARYW::_4K => 0,
            BOUNDARYW::_8K => 1,
            BOUNDARYW::_16K => 2,
            BOUNDARYW::_32K => 3,
            BOUNDARYW::_64K => 4,
            BOUNDARYW::_128K => 5,
            BOUNDARYW::_256K => 6,
            BOUNDARYW::_512K => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOUNDARYW<'a> {
    w: &'a mut W,
}
impl<'a> _BOUNDARYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOUNDARYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4k bytes"]
    #[inline]
    pub fn _4k(self) -> &'a mut W {
        self.variant(BOUNDARYW::_4K)
    }
    #[doc = "8k bytes"]
    #[inline]
    pub fn _8k(self) -> &'a mut W {
        self.variant(BOUNDARYW::_8K)
    }
    #[doc = "16k bytes"]
    #[inline]
    pub fn _16k(self) -> &'a mut W {
        self.variant(BOUNDARYW::_16K)
    }
    #[doc = "32k bytes"]
    #[inline]
    pub fn _32k(self) -> &'a mut W {
        self.variant(BOUNDARYW::_32K)
    }
    #[doc = "64k bytes"]
    #[inline]
    pub fn _64k(self) -> &'a mut W {
        self.variant(BOUNDARYW::_64K)
    }
    #[doc = "128k bytes"]
    #[inline]
    pub fn _128k(self) -> &'a mut W {
        self.variant(BOUNDARYW::_128K)
    }
    #[doc = "256k bytes"]
    #[inline]
    pub fn _256k(self) -> &'a mut W {
        self.variant(BOUNDARYW::_256K)
    }
    #[doc = "512k bytes"]
    #[inline]
    pub fn _512k(self) -> &'a mut W {
        self.variant(BOUNDARYW::_512K)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 12);
        self.w.bits |= ((value as u16) & 0x07) << 12;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:9 - Transfer Block Size"]
    #[inline]
    pub fn blocksize(&self) -> BLOCKSIZER {
        let bits = ((self.bits >> 0) & 0x03ff) as u16;
        BLOCKSIZER { bits }
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary"]
    #[inline]
    pub fn boundary(&self) -> BOUNDARYR {
        BOUNDARYR::_from(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Transfer Block Size"]
    #[inline]
    pub fn blocksize(&mut self) -> _BLOCKSIZEW {
        _BLOCKSIZEW { w: self }
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary"]
    #[inline]
    pub fn boundary(&mut self) -> _BOUNDARYW {
        _BOUNDARYW { w: self }
    }
}
