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
#[doc = "Possible values of the field `AESMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESMODER {
    #[doc = "Electronic code book mode"]
    ECB,
    #[doc = "Cipher block chaining mode"]
    CBC,
    #[doc = "Output feedback mode"]
    OFB,
    #[doc = "Cipher feedback mode"]
    CFB,
    #[doc = "Counter mode"]
    COUNTER,
    #[doc = "CCM mode"]
    CCM,
    #[doc = "Galois counter mode"]
    GCM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AESMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AESMODER::ECB => 0,
            AESMODER::CBC => 1,
            AESMODER::OFB => 2,
            AESMODER::CFB => 3,
            AESMODER::COUNTER => 4,
            AESMODER::CCM => 5,
            AESMODER::GCM => 6,
            AESMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AESMODER {
        match value {
            0 => AESMODER::ECB,
            1 => AESMODER::CBC,
            2 => AESMODER::OFB,
            3 => AESMODER::CFB,
            4 => AESMODER::COUNTER,
            5 => AESMODER::CCM,
            6 => AESMODER::GCM,
            i => AESMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline]
    pub fn is_ecb(&self) -> bool {
        *self == AESMODER::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline]
    pub fn is_cbc(&self) -> bool {
        *self == AESMODER::CBC
    }
    #[doc = "Checks if the value of the field is `OFB`"]
    #[inline]
    pub fn is_ofb(&self) -> bool {
        *self == AESMODER::OFB
    }
    #[doc = "Checks if the value of the field is `CFB`"]
    #[inline]
    pub fn is_cfb(&self) -> bool {
        *self == AESMODER::CFB
    }
    #[doc = "Checks if the value of the field is `COUNTER`"]
    #[inline]
    pub fn is_counter(&self) -> bool {
        *self == AESMODER::COUNTER
    }
    #[doc = "Checks if the value of the field is `CCM`"]
    #[inline]
    pub fn is_ccm(&self) -> bool {
        *self == AESMODER::CCM
    }
    #[doc = "Checks if the value of the field is `GCM`"]
    #[inline]
    pub fn is_gcm(&self) -> bool {
        *self == AESMODER::GCM
    }
}
#[doc = "Possible values of the field `CFBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFBSR {
    #[doc = "128-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _128BIT,
    #[doc = "64-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _64BIT,
    #[doc = "32-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _32BIT,
    #[doc = "16-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _16BIT,
    #[doc = "8-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _8BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CFBSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFBSR::_128BIT => 0,
            CFBSR::_64BIT => 1,
            CFBSR::_32BIT => 2,
            CFBSR::_16BIT => 3,
            CFBSR::_8BIT => 4,
            CFBSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFBSR {
        match value {
            0 => CFBSR::_128BIT,
            1 => CFBSR::_64BIT,
            2 => CFBSR::_32BIT,
            3 => CFBSR::_16BIT,
            4 => CFBSR::_8BIT,
            i => CFBSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_128BIT`"]
    #[inline]
    pub fn is_128bit(&self) -> bool {
        *self == CFBSR::_128BIT
    }
    #[doc = "Checks if the value of the field is `_64BIT`"]
    #[inline]
    pub fn is_64bit(&self) -> bool {
        *self == CFBSR::_64BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline]
    pub fn is_32bit(&self) -> bool {
        *self == CFBSR::_32BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline]
    pub fn is_16bit(&self) -> bool {
        *self == CFBSR::_16BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline]
    pub fn is_8bit(&self) -> bool {
        *self == CFBSR::_8BIT
    }
}
#[doc = "Possible values of the field `KEYSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSIZER {
    #[doc = "128-bit Key for Encryption / Decryption"]
    _128BIT,
    #[doc = "192-bit Key for Encryption / Decryption"]
    _192BIT,
    #[doc = "256-bit Key for Encryption / Decryption"]
    _256BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KEYSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEYSIZER::_128BIT => 0,
            KEYSIZER::_192BIT => 1,
            KEYSIZER::_256BIT => 2,
            KEYSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEYSIZER {
        match value {
            0 => KEYSIZER::_128BIT,
            1 => KEYSIZER::_192BIT,
            2 => KEYSIZER::_256BIT,
            i => KEYSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_128BIT`"]
    #[inline]
    pub fn is_128bit(&self) -> bool {
        *self == KEYSIZER::_128BIT
    }
    #[doc = "Checks if the value of the field is `_192BIT`"]
    #[inline]
    pub fn is_192bit(&self) -> bool {
        *self == KEYSIZER::_192BIT
    }
    #[doc = "Checks if the value of the field is `_256BIT`"]
    #[inline]
    pub fn is_256bit(&self) -> bool {
        *self == KEYSIZER::_256BIT
    }
}
#[doc = "Possible values of the field `CIPHER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIPHERR {
    #[doc = "Decryption"]
    DEC,
    #[doc = "Encryption"]
    ENC,
}
impl CIPHERR {
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
            CIPHERR::DEC => false,
            CIPHERR::ENC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIPHERR {
        match value {
            false => CIPHERR::DEC,
            true => CIPHERR::ENC,
        }
    }
    #[doc = "Checks if the value of the field is `DEC`"]
    #[inline]
    pub fn is_dec(&self) -> bool {
        *self == CIPHERR::DEC
    }
    #[doc = "Checks if the value of the field is `ENC`"]
    #[inline]
    pub fn is_enc(&self) -> bool {
        *self == CIPHERR::ENC
    }
}
#[doc = "Possible values of the field `STARTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTMODER {
    #[doc = "Start Encryption / Decryption in Manual mode"]
    MANUAL,
    #[doc = "Start Encryption / Decryption in Auto mode"]
    AUTO,
}
impl STARTMODER {
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
            STARTMODER::MANUAL => false,
            STARTMODER::AUTO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTMODER {
        match value {
            false => STARTMODER::MANUAL,
            true => STARTMODER::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline]
    pub fn is_manual(&self) -> bool {
        *self == STARTMODER::MANUAL
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == STARTMODER::AUTO
    }
}
#[doc = "Possible values of the field `LOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LODR {
    #[doc = "No effect"]
    NONE,
    #[doc = "Start encryption in Last Output Data mode"]
    LAST,
}
impl LODR {
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
            LODR::NONE => false,
            LODR::LAST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LODR {
        match value {
            false => LODR::NONE,
            true => LODR::LAST,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == LODR::NONE
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline]
    pub fn is_last(&self) -> bool {
        *self == LODR::LAST
    }
}
#[doc = "Possible values of the field `KEYGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYGENR {
    #[doc = "No effect"]
    NONE,
    #[doc = "Start Computation of the last NK words of the expanded key"]
    LAST,
}
impl KEYGENR {
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
            KEYGENR::NONE => false,
            KEYGENR::LAST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KEYGENR {
        match value {
            false => KEYGENR::NONE,
            true => KEYGENR::LAST,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == KEYGENR::NONE
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline]
    pub fn is_last(&self) -> bool {
        *self == KEYGENR::LAST
    }
}
#[doc = "Possible values of the field `XORKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XORKEYR {
    #[doc = "No effect"]
    NONE,
    #[doc = "The user keyword gets XORed with the previous keyword register content."]
    XOR,
}
impl XORKEYR {
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
            XORKEYR::NONE => false,
            XORKEYR::XOR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XORKEYR {
        match value {
            false => XORKEYR::NONE,
            true => XORKEYR::XOR,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == XORKEYR::NONE
    }
    #[doc = "Checks if the value of the field is `XOR`"]
    #[inline]
    pub fn is_xor(&self) -> bool {
        *self == XORKEYR::XOR
    }
}
#[doc = r" Value of the field"]
pub struct CTYPER {
    bits: u8,
}
impl CTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
#[doc = "Values that can be written to the field `AESMODE`"]
pub enum AESMODEW {
    #[doc = "Electronic code book mode"]
    ECB,
    #[doc = "Cipher block chaining mode"]
    CBC,
    #[doc = "Output feedback mode"]
    OFB,
    #[doc = "Cipher feedback mode"]
    CFB,
    #[doc = "Counter mode"]
    COUNTER,
    #[doc = "CCM mode"]
    CCM,
    #[doc = "Galois counter mode"]
    GCM,
}
impl AESMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AESMODEW::ECB => 0,
            AESMODEW::CBC => 1,
            AESMODEW::OFB => 2,
            AESMODEW::CFB => 3,
            AESMODEW::COUNTER => 4,
            AESMODEW::CCM => 5,
            AESMODEW::GCM => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AESMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _AESMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AESMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Electronic code book mode"]
    #[inline]
    pub fn ecb(self) -> &'a mut W {
        self.variant(AESMODEW::ECB)
    }
    #[doc = "Cipher block chaining mode"]
    #[inline]
    pub fn cbc(self) -> &'a mut W {
        self.variant(AESMODEW::CBC)
    }
    #[doc = "Output feedback mode"]
    #[inline]
    pub fn ofb(self) -> &'a mut W {
        self.variant(AESMODEW::OFB)
    }
    #[doc = "Cipher feedback mode"]
    #[inline]
    pub fn cfb(self) -> &'a mut W {
        self.variant(AESMODEW::CFB)
    }
    #[doc = "Counter mode"]
    #[inline]
    pub fn counter(self) -> &'a mut W {
        self.variant(AESMODEW::COUNTER)
    }
    #[doc = "CCM mode"]
    #[inline]
    pub fn ccm(self) -> &'a mut W {
        self.variant(AESMODEW::CCM)
    }
    #[doc = "Galois counter mode"]
    #[inline]
    pub fn gcm(self) -> &'a mut W {
        self.variant(AESMODEW::GCM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFBS`"]
pub enum CFBSW {
    #[doc = "128-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _128BIT,
    #[doc = "64-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _64BIT,
    #[doc = "32-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _32BIT,
    #[doc = "16-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _16BIT,
    #[doc = "8-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _8BIT,
}
impl CFBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFBSW::_128BIT => 0,
            CFBSW::_64BIT => 1,
            CFBSW::_32BIT => 2,
            CFBSW::_16BIT => 3,
            CFBSW::_8BIT => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFBSW<'a> {
    w: &'a mut W,
}
impl<'a> _CFBSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFBSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "128-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline]
    pub fn _128bit(self) -> &'a mut W {
        self.variant(CFBSW::_128BIT)
    }
    #[doc = "64-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline]
    pub fn _64bit(self) -> &'a mut W {
        self.variant(CFBSW::_64BIT)
    }
    #[doc = "32-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(CFBSW::_32BIT)
    }
    #[doc = "16-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(CFBSW::_16BIT)
    }
    #[doc = "8-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(CFBSW::_8BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KEYSIZE`"]
pub enum KEYSIZEW {
    #[doc = "128-bit Key for Encryption / Decryption"]
    _128BIT,
    #[doc = "192-bit Key for Encryption / Decryption"]
    _192BIT,
    #[doc = "256-bit Key for Encryption / Decryption"]
    _256BIT,
}
impl KEYSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEYSIZEW::_128BIT => 0,
            KEYSIZEW::_192BIT => 1,
            KEYSIZEW::_256BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "128-bit Key for Encryption / Decryption"]
    #[inline]
    pub fn _128bit(self) -> &'a mut W {
        self.variant(KEYSIZEW::_128BIT)
    }
    #[doc = "192-bit Key for Encryption / Decryption"]
    #[inline]
    pub fn _192bit(self) -> &'a mut W {
        self.variant(KEYSIZEW::_192BIT)
    }
    #[doc = "256-bit Key for Encryption / Decryption"]
    #[inline]
    pub fn _256bit(self) -> &'a mut W {
        self.variant(KEYSIZEW::_256BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CIPHER`"]
pub enum CIPHERW {
    #[doc = "Decryption"]
    DEC,
    #[doc = "Encryption"]
    ENC,
}
impl CIPHERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIPHERW::DEC => false,
            CIPHERW::ENC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIPHERW<'a> {
    w: &'a mut W,
}
impl<'a> _CIPHERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIPHERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Decryption"]
    #[inline]
    pub fn dec(self) -> &'a mut W {
        self.variant(CIPHERW::DEC)
    }
    #[doc = "Encryption"]
    #[inline]
    pub fn enc(self) -> &'a mut W {
        self.variant(CIPHERW::ENC)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STARTMODE`"]
pub enum STARTMODEW {
    #[doc = "Start Encryption / Decryption in Manual mode"]
    MANUAL,
    #[doc = "Start Encryption / Decryption in Auto mode"]
    AUTO,
}
impl STARTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTMODEW::MANUAL => false,
            STARTMODEW::AUTO => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start Encryption / Decryption in Manual mode"]
    #[inline]
    pub fn manual(self) -> &'a mut W {
        self.variant(STARTMODEW::MANUAL)
    }
    #[doc = "Start Encryption / Decryption in Auto mode"]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(STARTMODEW::AUTO)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOD`"]
pub enum LODW {
    #[doc = "No effect"]
    NONE,
    #[doc = "Start encryption in Last Output Data mode"]
    LAST,
}
impl LODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LODW::NONE => false,
            LODW::LAST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LODW<'a> {
    w: &'a mut W,
}
impl<'a> _LODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(LODW::NONE)
    }
    #[doc = "Start encryption in Last Output Data mode"]
    #[inline]
    pub fn last(self) -> &'a mut W {
        self.variant(LODW::LAST)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KEYGEN`"]
pub enum KEYGENW {
    #[doc = "No effect"]
    NONE,
    #[doc = "Start Computation of the last NK words of the expanded key"]
    LAST,
}
impl KEYGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KEYGENW::NONE => false,
            KEYGENW::LAST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYGENW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(KEYGENW::NONE)
    }
    #[doc = "Start Computation of the last NK words of the expanded key"]
    #[inline]
    pub fn last(self) -> &'a mut W {
        self.variant(KEYGENW::LAST)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XORKEY`"]
pub enum XORKEYW {
    #[doc = "No effect"]
    NONE,
    #[doc = "The user keyword gets XORed with the previous keyword register content."]
    XOR,
}
impl XORKEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XORKEYW::NONE => false,
            XORKEYW::XOR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XORKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _XORKEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XORKEYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(XORKEYW::NONE)
    }
    #[doc = "The user keyword gets XORed with the previous keyword register content."]
    #[inline]
    pub fn xor(self) -> &'a mut W {
        self.variant(XORKEYW::XOR)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTYPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWRSTR { bits }
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
    #[doc = "Bits 2:4 - AES Modes of operation"]
    #[inline]
    pub fn aesmode(&self) -> AESMODER {
        AESMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:7 - Cipher Feedback Block Size"]
    #[inline]
    pub fn cfbs(&self) -> CFBSR {
        CFBSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Encryption Key Size"]
    #[inline]
    pub fn keysize(&self) -> KEYSIZER {
        KEYSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Cipher Mode"]
    #[inline]
    pub fn cipher(&self) -> CIPHERR {
        CIPHERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Start Mode Select"]
    #[inline]
    pub fn startmode(&self) -> STARTMODER {
        STARTMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Last Output Data Mode"]
    #[inline]
    pub fn lod(&self) -> LODR {
        LODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Last Key Generation"]
    #[inline]
    pub fn keygen(&self) -> KEYGENR {
        KEYGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - XOR Key Operation"]
    #[inline]
    pub fn xorkey(&self) -> XORKEYR {
        XORKEYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Counter Measure Type"]
    #[inline]
    pub fn ctype(&self) -> CTYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTYPER { bits }
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
    #[doc = "Bits 2:4 - AES Modes of operation"]
    #[inline]
    pub fn aesmode(&mut self) -> _AESMODEW {
        _AESMODEW { w: self }
    }
    #[doc = "Bits 5:7 - Cipher Feedback Block Size"]
    #[inline]
    pub fn cfbs(&mut self) -> _CFBSW {
        _CFBSW { w: self }
    }
    #[doc = "Bits 8:9 - Encryption Key Size"]
    #[inline]
    pub fn keysize(&mut self) -> _KEYSIZEW {
        _KEYSIZEW { w: self }
    }
    #[doc = "Bit 10 - Cipher Mode"]
    #[inline]
    pub fn cipher(&mut self) -> _CIPHERW {
        _CIPHERW { w: self }
    }
    #[doc = "Bit 11 - Start Mode Select"]
    #[inline]
    pub fn startmode(&mut self) -> _STARTMODEW {
        _STARTMODEW { w: self }
    }
    #[doc = "Bit 12 - Last Output Data Mode"]
    #[inline]
    pub fn lod(&mut self) -> _LODW {
        _LODW { w: self }
    }
    #[doc = "Bit 13 - Last Key Generation"]
    #[inline]
    pub fn keygen(&mut self) -> _KEYGENW {
        _KEYGENW { w: self }
    }
    #[doc = "Bit 14 - XOR Key Operation"]
    #[inline]
    pub fn xorkey(&mut self) -> _XORKEYW {
        _XORKEYW { w: self }
    }
    #[doc = "Bits 16:19 - Counter Measure Type"]
    #[inline]
    pub fn ctype(&mut self) -> _CTYPEW {
        _CTYPEW { w: self }
    }
}
