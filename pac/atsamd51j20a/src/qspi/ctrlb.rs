#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLB {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "SPI operating mode"]
    SPI,
    #[doc = "Serial Memory operating mode"]
    MEMORY,
}
impl MODER {
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
            MODER::SPI => false,
            MODER::MEMORY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::SPI,
            true => MODER::MEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline]
    pub fn is_spi(&self) -> bool {
        *self == MODER::SPI
    }
    #[doc = "Checks if the value of the field is `MEMORY`"]
    #[inline]
    pub fn is_memory(&self) -> bool {
        *self == MODER::MEMORY
    }
}
#[doc = r" Value of the field"]
pub struct LOOPENR {
    bits: bool,
}
impl LOOPENR {
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
pub struct WDRBTR {
    bits: bool,
}
impl WDRBTR {
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
pub struct SMEMREGR {
    bits: bool,
}
impl SMEMREGR {
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
#[doc = "Possible values of the field `CSMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSMODER {
    #[doc = "The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    NORELOAD,
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    LASTXFER,
    #[doc = "The chip select is deasserted systematically after each transfer."]
    SYSTEMATICALLY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSMODER::NORELOAD => 0,
            CSMODER::LASTXFER => 1,
            CSMODER::SYSTEMATICALLY => 2,
            CSMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSMODER {
        match value {
            0 => CSMODER::NORELOAD,
            1 => CSMODER::LASTXFER,
            2 => CSMODER::SYSTEMATICALLY,
            i => CSMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORELOAD`"]
    #[inline]
    pub fn is_noreload(&self) -> bool {
        *self == CSMODER::NORELOAD
    }
    #[doc = "Checks if the value of the field is `LASTXFER`"]
    #[inline]
    pub fn is_lastxfer(&self) -> bool {
        *self == CSMODER::LASTXFER
    }
    #[doc = "Checks if the value of the field is `SYSTEMATICALLY`"]
    #[inline]
    pub fn is_systematically(&self) -> bool {
        *self == CSMODER::SYSTEMATICALLY
    }
}
#[doc = "Possible values of the field `DATALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATALENR {
    #[doc = "8-bits transfer"]
    _8BITS,
    #[doc = "9 bits transfer"]
    _9BITS,
    #[doc = "10-bits transfer"]
    _10BITS,
    #[doc = "11-bits transfer"]
    _11BITS,
    #[doc = "12-bits transfer"]
    _12BITS,
    #[doc = "13-bits transfer"]
    _13BITS,
    #[doc = "14-bits transfer"]
    _14BITS,
    #[doc = "15-bits transfer"]
    _15BITS,
    #[doc = "16-bits transfer"]
    _16BITS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATALENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATALENR::_8BITS => 0,
            DATALENR::_9BITS => 1,
            DATALENR::_10BITS => 2,
            DATALENR::_11BITS => 3,
            DATALENR::_12BITS => 4,
            DATALENR::_13BITS => 5,
            DATALENR::_14BITS => 6,
            DATALENR::_15BITS => 7,
            DATALENR::_16BITS => 8,
            DATALENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATALENR {
        match value {
            0 => DATALENR::_8BITS,
            1 => DATALENR::_9BITS,
            2 => DATALENR::_10BITS,
            3 => DATALENR::_11BITS,
            4 => DATALENR::_12BITS,
            5 => DATALENR::_13BITS,
            6 => DATALENR::_14BITS,
            7 => DATALENR::_15BITS,
            8 => DATALENR::_16BITS,
            i => DATALENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline]
    pub fn is_8bits(&self) -> bool {
        *self == DATALENR::_8BITS
    }
    #[doc = "Checks if the value of the field is `_9BITS`"]
    #[inline]
    pub fn is_9bits(&self) -> bool {
        *self == DATALENR::_9BITS
    }
    #[doc = "Checks if the value of the field is `_10BITS`"]
    #[inline]
    pub fn is_10bits(&self) -> bool {
        *self == DATALENR::_10BITS
    }
    #[doc = "Checks if the value of the field is `_11BITS`"]
    #[inline]
    pub fn is_11bits(&self) -> bool {
        *self == DATALENR::_11BITS
    }
    #[doc = "Checks if the value of the field is `_12BITS`"]
    #[inline]
    pub fn is_12bits(&self) -> bool {
        *self == DATALENR::_12BITS
    }
    #[doc = "Checks if the value of the field is `_13BITS`"]
    #[inline]
    pub fn is_13bits(&self) -> bool {
        *self == DATALENR::_13BITS
    }
    #[doc = "Checks if the value of the field is `_14BITS`"]
    #[inline]
    pub fn is_14bits(&self) -> bool {
        *self == DATALENR::_14BITS
    }
    #[doc = "Checks if the value of the field is `_15BITS`"]
    #[inline]
    pub fn is_15bits(&self) -> bool {
        *self == DATALENR::_15BITS
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline]
    pub fn is_16bits(&self) -> bool {
        *self == DATALENR::_16BITS
    }
}
#[doc = r" Value of the field"]
pub struct DLYBCTR {
    bits: u8,
}
impl DLYBCTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYCSR {
    bits: u8,
}
impl DLYCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "SPI operating mode"]
    SPI,
    #[doc = "Serial Memory operating mode"]
    MEMORY,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::SPI => false,
            MODEW::MEMORY => true,
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
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI operating mode"]
    #[inline]
    pub fn spi(self) -> &'a mut W {
        self.variant(MODEW::SPI)
    }
    #[doc = "Serial Memory operating mode"]
    #[inline]
    pub fn memory(self) -> &'a mut W {
        self.variant(MODEW::MEMORY)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LOOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPENW<'a> {
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
pub struct _WDRBTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDRBTW<'a> {
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
pub struct _SMEMREGW<'a> {
    w: &'a mut W,
}
impl<'a> _SMEMREGW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSMODE`"]
pub enum CSMODEW {
    #[doc = "The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    NORELOAD,
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    LASTXFER,
    #[doc = "The chip select is deasserted systematically after each transfer."]
    SYSTEMATICALLY,
}
impl CSMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSMODEW::NORELOAD => 0,
            CSMODEW::LASTXFER => 1,
            CSMODEW::SYSTEMATICALLY => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The chip select is deasserted if TD has not been reloaded before the end of the current transfer."]
    #[inline]
    pub fn noreload(self) -> &'a mut W {
        self.variant(CSMODEW::NORELOAD)
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in TD has been transferred."]
    #[inline]
    pub fn lastxfer(self) -> &'a mut W {
        self.variant(CSMODEW::LASTXFER)
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline]
    pub fn systematically(self) -> &'a mut W {
        self.variant(CSMODEW::SYSTEMATICALLY)
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
#[doc = "Values that can be written to the field `DATALEN`"]
pub enum DATALENW {
    #[doc = "8-bits transfer"]
    _8BITS,
    #[doc = "9 bits transfer"]
    _9BITS,
    #[doc = "10-bits transfer"]
    _10BITS,
    #[doc = "11-bits transfer"]
    _11BITS,
    #[doc = "12-bits transfer"]
    _12BITS,
    #[doc = "13-bits transfer"]
    _13BITS,
    #[doc = "14-bits transfer"]
    _14BITS,
    #[doc = "15-bits transfer"]
    _15BITS,
    #[doc = "16-bits transfer"]
    _16BITS,
}
impl DATALENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATALENW::_8BITS => 0,
            DATALENW::_9BITS => 1,
            DATALENW::_10BITS => 2,
            DATALENW::_11BITS => 3,
            DATALENW::_12BITS => 4,
            DATALENW::_13BITS => 5,
            DATALENW::_14BITS => 6,
            DATALENW::_15BITS => 7,
            DATALENW::_16BITS => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATALENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATALENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATALENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bits transfer"]
    #[inline]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(DATALENW::_8BITS)
    }
    #[doc = "9 bits transfer"]
    #[inline]
    pub fn _9bits(self) -> &'a mut W {
        self.variant(DATALENW::_9BITS)
    }
    #[doc = "10-bits transfer"]
    #[inline]
    pub fn _10bits(self) -> &'a mut W {
        self.variant(DATALENW::_10BITS)
    }
    #[doc = "11-bits transfer"]
    #[inline]
    pub fn _11bits(self) -> &'a mut W {
        self.variant(DATALENW::_11BITS)
    }
    #[doc = "12-bits transfer"]
    #[inline]
    pub fn _12bits(self) -> &'a mut W {
        self.variant(DATALENW::_12BITS)
    }
    #[doc = "13-bits transfer"]
    #[inline]
    pub fn _13bits(self) -> &'a mut W {
        self.variant(DATALENW::_13BITS)
    }
    #[doc = "14-bits transfer"]
    #[inline]
    pub fn _14bits(self) -> &'a mut W {
        self.variant(DATALENW::_14BITS)
    }
    #[doc = "15-bits transfer"]
    #[inline]
    pub fn _15bits(self) -> &'a mut W {
        self.variant(DATALENW::_15BITS)
    }
    #[doc = "16-bits transfer"]
    #[inline]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(DATALENW::_16BITS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYBCTW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYBCTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYCSW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYCSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline]
    pub fn loopen(&self) -> LOOPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOOPENR { bits }
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline]
    pub fn wdrbt(&self) -> WDRBTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDRBTR { bits }
    }
    #[doc = "Bit 3 - Serial Memory reg"]
    #[inline]
    pub fn smemreg(&self) -> SMEMREGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SMEMREGR { bits }
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline]
    pub fn csmode(&self) -> CSMODER {
        CSMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Data Length"]
    #[inline]
    pub fn datalen(&self) -> DATALENR {
        DATALENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline]
    pub fn dlybct(&self) -> DLYBCTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYBCTR { bits }
    }
    #[doc = "Bits 24:31 - Minimum Inactive CS Delay"]
    #[inline]
    pub fn dlycs(&self) -> DLYCSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYCSR { bits }
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
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline]
    pub fn loopen(&mut self) -> _LOOPENW {
        _LOOPENW { w: self }
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline]
    pub fn wdrbt(&mut self) -> _WDRBTW {
        _WDRBTW { w: self }
    }
    #[doc = "Bit 3 - Serial Memory reg"]
    #[inline]
    pub fn smemreg(&mut self) -> _SMEMREGW {
        _SMEMREGW { w: self }
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline]
    pub fn csmode(&mut self) -> _CSMODEW {
        _CSMODEW { w: self }
    }
    #[doc = "Bits 8:11 - Data Length"]
    #[inline]
    pub fn datalen(&mut self) -> _DATALENW {
        _DATALENW { w: self }
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline]
    pub fn dlybct(&mut self) -> _DLYBCTW {
        _DLYBCTW { w: self }
    }
    #[doc = "Bits 24:31 - Minimum Inactive CS Delay"]
    #[inline]
    pub fn dlycs(&mut self) -> _DLYCSW {
        _DLYCSW { w: self }
    }
}
