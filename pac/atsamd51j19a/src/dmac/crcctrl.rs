#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CRCCTRL {
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
#[doc = "Possible values of the field `CRCBEATSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCBEATSIZER {
    #[doc = "8-bit bus transfer"]
    BYTE,
    #[doc = "16-bit bus transfer"]
    HWORD,
    #[doc = "32-bit bus transfer"]
    WORD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRCBEATSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRCBEATSIZER::BYTE => 0,
            CRCBEATSIZER::HWORD => 1,
            CRCBEATSIZER::WORD => 2,
            CRCBEATSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRCBEATSIZER {
        match value {
            0 => CRCBEATSIZER::BYTE,
            1 => CRCBEATSIZER::HWORD,
            2 => CRCBEATSIZER::WORD,
            i => CRCBEATSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline]
    pub fn is_byte(&self) -> bool {
        *self == CRCBEATSIZER::BYTE
    }
    #[doc = "Checks if the value of the field is `HWORD`"]
    #[inline]
    pub fn is_hword(&self) -> bool {
        *self == CRCBEATSIZER::HWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline]
    pub fn is_word(&self) -> bool {
        *self == CRCBEATSIZER::WORD
    }
}
#[doc = "Possible values of the field `CRCPOLY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCPOLYR {
    #[doc = "CRC-16 (CRC-CCITT)"]
    CRC16,
    #[doc = "CRC32 (IEEE 802.3)"]
    CRC32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRCPOLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRCPOLYR::CRC16 => 0,
            CRCPOLYR::CRC32 => 1,
            CRCPOLYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRCPOLYR {
        match value {
            0 => CRCPOLYR::CRC16,
            1 => CRCPOLYR::CRC32,
            i => CRCPOLYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CRC16`"]
    #[inline]
    pub fn is_crc16(&self) -> bool {
        *self == CRCPOLYR::CRC16
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline]
    pub fn is_crc32(&self) -> bool {
        *self == CRCPOLYR::CRC32
    }
}
#[doc = "Possible values of the field `CRCSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCSRCR {
    #[doc = "CRC Disabled"]
    DISABLE,
    #[doc = "I/O interface"]
    IO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRCSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRCSRCR::DISABLE => 0,
            CRCSRCR::IO => 1,
            CRCSRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRCSRCR {
        match value {
            0 => CRCSRCR::DISABLE,
            1 => CRCSRCR::IO,
            i => CRCSRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CRCSRCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline]
    pub fn is_io(&self) -> bool {
        *self == CRCSRCR::IO
    }
}
#[doc = "Possible values of the field `CRCMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCMODER {
    #[doc = "Default operating mode"]
    DEFAULT,
    #[doc = "Memory CRC monitor operating mode"]
    CRCMON,
    #[doc = "Memory CRC generation operating mode"]
    CRCGEN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRCMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRCMODER::DEFAULT => 0,
            CRCMODER::CRCMON => 2,
            CRCMODER::CRCGEN => 3,
            CRCMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRCMODER {
        match value {
            0 => CRCMODER::DEFAULT,
            2 => CRCMODER::CRCMON,
            3 => CRCMODER::CRCGEN,
            i => CRCMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == CRCMODER::DEFAULT
    }
    #[doc = "Checks if the value of the field is `CRCMON`"]
    #[inline]
    pub fn is_crcmon(&self) -> bool {
        *self == CRCMODER::CRCMON
    }
    #[doc = "Checks if the value of the field is `CRCGEN`"]
    #[inline]
    pub fn is_crcgen(&self) -> bool {
        *self == CRCMODER::CRCGEN
    }
}
#[doc = "Values that can be written to the field `CRCBEATSIZE`"]
pub enum CRCBEATSIZEW {
    #[doc = "8-bit bus transfer"]
    BYTE,
    #[doc = "16-bit bus transfer"]
    HWORD,
    #[doc = "32-bit bus transfer"]
    WORD,
}
impl CRCBEATSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRCBEATSIZEW::BYTE => 0,
            CRCBEATSIZEW::HWORD => 1,
            CRCBEATSIZEW::WORD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCBEATSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCBEATSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCBEATSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit bus transfer"]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(CRCBEATSIZEW::BYTE)
    }
    #[doc = "16-bit bus transfer"]
    #[inline]
    pub fn hword(self) -> &'a mut W {
        self.variant(CRCBEATSIZEW::HWORD)
    }
    #[doc = "32-bit bus transfer"]
    #[inline]
    pub fn word(self) -> &'a mut W {
        self.variant(CRCBEATSIZEW::WORD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRCPOLY`"]
pub enum CRCPOLYW {
    #[doc = "CRC-16 (CRC-CCITT)"]
    CRC16,
    #[doc = "CRC32 (IEEE 802.3)"]
    CRC32,
}
impl CRCPOLYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRCPOLYW::CRC16 => 0,
            CRCPOLYW::CRC32 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCPOLYW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCPOLYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCPOLYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CRC-16 (CRC-CCITT)"]
    #[inline]
    pub fn crc16(self) -> &'a mut W {
        self.variant(CRCPOLYW::CRC16)
    }
    #[doc = "CRC32 (IEEE 802.3)"]
    #[inline]
    pub fn crc32(self) -> &'a mut W {
        self.variant(CRCPOLYW::CRC32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRCSRC`"]
pub enum CRCSRCW {
    #[doc = "CRC Disabled"]
    DISABLE,
    #[doc = "I/O interface"]
    IO,
}
impl CRCSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRCSRCW::DISABLE => 0,
            CRCSRCW::IO => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CRC Disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCSRCW::DISABLE)
    }
    #[doc = "I/O interface"]
    #[inline]
    pub fn io(self) -> &'a mut W {
        self.variant(CRCSRCW::IO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRCMODE`"]
pub enum CRCMODEW {
    #[doc = "Default operating mode"]
    DEFAULT,
    #[doc = "Memory CRC monitor operating mode"]
    CRCMON,
    #[doc = "Memory CRC generation operating mode"]
    CRCGEN,
}
impl CRCMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRCMODEW::DEFAULT => 0,
            CRCMODEW::CRCMON => 2,
            CRCMODEW::CRCGEN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Default operating mode"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(CRCMODEW::DEFAULT)
    }
    #[doc = "Memory CRC monitor operating mode"]
    #[inline]
    pub fn crcmon(self) -> &'a mut W {
        self.variant(CRCMODEW::CRCMON)
    }
    #[doc = "Memory CRC generation operating mode"]
    #[inline]
    pub fn crcgen(self) -> &'a mut W {
        self.variant(CRCMODEW::CRCGEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline]
    pub fn crcbeatsize(&self) -> CRCBEATSIZER {
        CRCBEATSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline]
    pub fn crcpoly(&self) -> CRCPOLYR {
        CRCPOLYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline]
    pub fn crcsrc(&self) -> CRCSRCR {
        CRCSRCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 14:15 - CRC Operating Mode"]
    #[inline]
    pub fn crcmode(&self) -> CRCMODER {
        CRCMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - CRC Beat Size"]
    #[inline]
    pub fn crcbeatsize(&mut self) -> _CRCBEATSIZEW {
        _CRCBEATSIZEW { w: self }
    }
    #[doc = "Bits 2:3 - CRC Polynomial Type"]
    #[inline]
    pub fn crcpoly(&mut self) -> _CRCPOLYW {
        _CRCPOLYW { w: self }
    }
    #[doc = "Bits 8:13 - CRC Input Source"]
    #[inline]
    pub fn crcsrc(&mut self) -> _CRCSRCW {
        _CRCSRCW { w: self }
    }
    #[doc = "Bits 14:15 - CRC Operating Mode"]
    #[inline]
    pub fn crcmode(&mut self) -> _CRCMODEW {
        _CRCMODEW { w: self }
    }
}
