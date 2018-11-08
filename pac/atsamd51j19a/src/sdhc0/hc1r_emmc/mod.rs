#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::HC1R_EMMC {
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
#[doc = "Possible values of the field `DW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWR {
    #[doc = "1-bit mode"]
    _1BIT,
    #[doc = "4-bit mode"]
    _4BIT,
}
impl DWR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DWR::_1BIT => false,
            DWR::_4BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DWR {
        match value {
            false => DWR::_1BIT,
            true => DWR::_4BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_1BIT`"]
    #[inline]
    pub fn is_1bit(&self) -> bool {
        *self == DWR::_1BIT
    }
    #[doc = "Checks if the value of the field is `_4BIT`"]
    #[inline]
    pub fn is_4bit(&self) -> bool {
        *self == DWR::_4BIT
    }
}
#[doc = "Possible values of the field `HSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSENR {
    #[doc = "Normal Speed mode"]
    NORMAL,
    #[doc = "High Speed mode"]
    HIGH,
}
impl HSENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HSENR::NORMAL => false,
            HSENR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSENR {
        match value {
            false => HSENR::NORMAL,
            true => HSENR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == HSENR::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == HSENR::HIGH
    }
}
#[doc = "Possible values of the field `DMASEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASELR {
    #[doc = "SDMA is selected"]
    SDMA,
    #[doc = "32-bit Address ADMA2 is selected"]
    _32BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMASELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMASELR::SDMA => 0,
            DMASELR::_32BIT => 2,
            DMASELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMASELR {
        match value {
            0 => DMASELR::SDMA,
            2 => DMASELR::_32BIT,
            i => DMASELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDMA`"]
    #[inline]
    pub fn is_sdma(&self) -> bool {
        *self == DMASELR::SDMA
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline]
    pub fn is_32bit(&self) -> bool {
        *self == DMASELR::_32BIT
    }
}
#[doc = "Values that can be written to the field `DW`"]
pub enum DWW {
    #[doc = "1-bit mode"]
    _1BIT,
    #[doc = "4-bit mode"]
    _4BIT,
}
impl DWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DWW::_1BIT => false,
            DWW::_4BIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DWW<'a> {
    w: &'a mut W,
}
impl<'a> _DWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "1-bit mode"]
    #[inline]
    pub fn _1bit(self) -> &'a mut W {
        self.variant(DWW::_1BIT)
    }
    #[doc = "4-bit mode"]
    #[inline]
    pub fn _4bit(self) -> &'a mut W {
        self.variant(DWW::_4BIT)
    }
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HSEN`"]
pub enum HSENW {
    #[doc = "Normal Speed mode"]
    NORMAL,
    #[doc = "High Speed mode"]
    HIGH,
}
impl HSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSENW::NORMAL => false,
            HSENW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSENW<'a> {
    w: &'a mut W,
}
impl<'a> _HSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Speed mode"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(HSENW::NORMAL)
    }
    #[doc = "High Speed mode"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(HSENW::HIGH)
    }
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMASEL`"]
pub enum DMASELW {
    #[doc = "SDMA is selected"]
    SDMA,
    #[doc = "32-bit Address ADMA2 is selected"]
    _32BIT,
}
impl DMASELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DMASELW::SDMA => 0,
            DMASELW::_32BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMASELW<'a> {
    w: &'a mut W,
}
impl<'a> _DMASELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMASELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SDMA is selected"]
    #[inline]
    pub fn sdma(self) -> &'a mut W {
        self.variant(DMASELW::SDMA)
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(DMASELW::_32BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 1 - Data Width"]
    #[inline]
    pub fn dw(&self) -> DWR {
        DWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline]
    pub fn hsen(&self) -> HSENR {
        HSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline]
    pub fn dmasel(&self) -> DMASELR {
        DMASELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3584 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Data Width"]
    #[inline]
    pub fn dw(&mut self) -> _DWW {
        _DWW { w: self }
    }
    #[doc = "Bit 2 - High Speed Enable"]
    #[inline]
    pub fn hsen(&mut self) -> _HSENW {
        _HSENW { w: self }
    }
    #[doc = "Bits 3:4 - DMA Select"]
    #[inline]
    pub fn dmasel(&mut self) -> _DMASELW {
        _DMASELW { w: self }
    }
}
