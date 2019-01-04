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
pub struct WBDISR {
    bits: bool,
}
impl WBDISR {
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
pub struct EOMDISR {
    bits: bool,
}
impl EOMDISR {
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
pub struct SLBDISR {
    bits: bool,
}
impl SLBDISR {
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
pub struct BBCR {
    bits: u8,
}
impl BBCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ASCDR {
    bits: bool,
}
impl ASCDR {
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
pub struct DUALBUFFR {
    bits: bool,
}
impl DUALBUFFR {
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
pub struct UIHASHR {
    bits: bool,
}
impl UIHASHR {
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
#[doc = "Possible values of the field `UALGO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UALGOR {
    #[doc = "SHA1 Algorithm"]
    SHA1,
    #[doc = "SHA256 Algorithm"]
    SHA256,
    #[doc = "SHA224 Algorithm"]
    SHA224,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UALGOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UALGOR::SHA1 => 0,
            UALGOR::SHA256 => 1,
            UALGOR::SHA224 => 4,
            UALGOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UALGOR {
        match value {
            0 => UALGOR::SHA1,
            1 => UALGOR::SHA256,
            4 => UALGOR::SHA224,
            i => UALGOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline]
    pub fn is_sha1(&self) -> bool {
        *self == UALGOR::SHA1
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline]
    pub fn is_sha256(&self) -> bool {
        *self == UALGOR::SHA256
    }
    #[doc = "Checks if the value of the field is `SHA224`"]
    #[inline]
    pub fn is_sha224(&self) -> bool {
        *self == UALGOR::SHA224
    }
}
#[doc = r" Value of the field"]
pub struct HAPROTR {
    bits: u8,
}
impl HAPROTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DAPROTR {
    bits: u8,
}
impl DAPROTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _WBDISW<'a> {
    w: &'a mut W,
}
impl<'a> _WBDISW<'a> {
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
pub struct _EOMDISW<'a> {
    w: &'a mut W,
}
impl<'a> _EOMDISW<'a> {
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
pub struct _SLBDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SLBDISW<'a> {
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
#[doc = r" Proxy"]
pub struct _BBCW<'a> {
    w: &'a mut W,
}
impl<'a> _BBCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASCDW<'a> {
    w: &'a mut W,
}
impl<'a> _ASCDW<'a> {
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
pub struct _DUALBUFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DUALBUFFW<'a> {
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
#[doc = r" Proxy"]
pub struct _UIHASHW<'a> {
    w: &'a mut W,
}
impl<'a> _UIHASHW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UALGO`"]
pub enum UALGOW {
    #[doc = "SHA1 Algorithm"]
    SHA1,
    #[doc = "SHA256 Algorithm"]
    SHA256,
    #[doc = "SHA224 Algorithm"]
    SHA224,
}
impl UALGOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UALGOW::SHA1 => 0,
            UALGOW::SHA256 => 1,
            UALGOW::SHA224 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UALGOW<'a> {
    w: &'a mut W,
}
impl<'a> _UALGOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UALGOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SHA1 Algorithm"]
    #[inline]
    pub fn sha1(self) -> &'a mut W {
        self.variant(UALGOW::SHA1)
    }
    #[doc = "SHA256 Algorithm"]
    #[inline]
    pub fn sha256(self) -> &'a mut W {
        self.variant(UALGOW::SHA256)
    }
    #[doc = "SHA224 Algorithm"]
    #[inline]
    pub fn sha224(self) -> &'a mut W {
        self.variant(UALGOW::SHA224)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HAPROTW<'a> {
    w: &'a mut W,
}
impl<'a> _HAPROTW<'a> {
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
#[doc = r" Proxy"]
pub struct _DAPROTW<'a> {
    w: &'a mut W,
}
impl<'a> _DAPROTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline]
    pub fn wbdis(&self) -> WBDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WBDISR { bits }
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline]
    pub fn eomdis(&self) -> EOMDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EOMDISR { bits }
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline]
    pub fn slbdis(&self) -> SLBDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLBDISR { bits }
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline]
    pub fn bbc(&self) -> BBCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BBCR { bits }
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline]
    pub fn ascd(&self) -> ASCDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASCDR { bits }
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline]
    pub fn dualbuff(&self) -> DUALBUFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DUALBUFFR { bits }
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline]
    pub fn uihash(&self) -> UIHASHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UIHASHR { bits }
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline]
    pub fn ualgo(&self) -> UALGOR {
        UALGOR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - Region Hash Area Protection"]
    #[inline]
    pub fn haprot(&self) -> HAPROTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HAPROTR { bits }
    }
    #[doc = "Bits 24:29 - Region Descriptor Area Protection"]
    #[inline]
    pub fn daprot(&self) -> DAPROTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DAPROTR { bits }
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
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline]
    pub fn wbdis(&mut self) -> _WBDISW {
        _WBDISW { w: self }
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline]
    pub fn eomdis(&mut self) -> _EOMDISW {
        _EOMDISW { w: self }
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline]
    pub fn slbdis(&mut self) -> _SLBDISW {
        _SLBDISW { w: self }
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline]
    pub fn bbc(&mut self) -> _BBCW {
        _BBCW { w: self }
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline]
    pub fn ascd(&mut self) -> _ASCDW {
        _ASCDW { w: self }
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline]
    pub fn dualbuff(&mut self) -> _DUALBUFFW {
        _DUALBUFFW { w: self }
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline]
    pub fn uihash(&mut self) -> _UIHASHW {
        _UIHASHW { w: self }
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline]
    pub fn ualgo(&mut self) -> _UALGOW {
        _UALGOW { w: self }
    }
    #[doc = "Bits 16:21 - Region Hash Area Protection"]
    #[inline]
    pub fn haprot(&mut self) -> _HAPROTW {
        _HAPROTW { w: self }
    }
    #[doc = "Bits 24:29 - Region Descriptor Area Protection"]
    #[inline]
    pub fn daprot(&mut self) -> _DAPROTW {
        _DAPROTW { w: self }
    }
}
