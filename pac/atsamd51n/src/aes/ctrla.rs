#[doc = "Reader of register CTRLA"]
pub type R = crate::R<u32, super::CTRLA>;
#[doc = "Writer for register CTRLA"]
pub type W = crate::W<u32, super::CTRLA>;
#[doc = "Register CTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA {
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
#[doc = "AES Modes of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AESMODE_A {
    #[doc = "0: Electronic code book mode"]
    ECB = 0,
    #[doc = "1: Cipher block chaining mode"]
    CBC = 1,
    #[doc = "2: Output feedback mode"]
    OFB = 2,
    #[doc = "3: Cipher feedback mode"]
    CFB = 3,
    #[doc = "4: Counter mode"]
    COUNTER = 4,
    #[doc = "5: CCM mode"]
    CCM = 5,
    #[doc = "6: Galois counter mode"]
    GCM = 6,
}
impl From<AESMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: AESMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AESMODE`"]
pub type AESMODE_R = crate::R<u8, AESMODE_A>;
impl AESMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AESMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AESMODE_A::ECB),
            1 => Val(AESMODE_A::CBC),
            2 => Val(AESMODE_A::OFB),
            3 => Val(AESMODE_A::CFB),
            4 => Val(AESMODE_A::COUNTER),
            5 => Val(AESMODE_A::CCM),
            6 => Val(AESMODE_A::GCM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == AESMODE_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == AESMODE_A::CBC
    }
    #[doc = "Checks if the value of the field is `OFB`"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == AESMODE_A::OFB
    }
    #[doc = "Checks if the value of the field is `CFB`"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        *self == AESMODE_A::CFB
    }
    #[doc = "Checks if the value of the field is `COUNTER`"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == AESMODE_A::COUNTER
    }
    #[doc = "Checks if the value of the field is `CCM`"]
    #[inline(always)]
    pub fn is_ccm(&self) -> bool {
        *self == AESMODE_A::CCM
    }
    #[doc = "Checks if the value of the field is `GCM`"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == AESMODE_A::GCM
    }
}
#[doc = "Write proxy for field `AESMODE`"]
pub struct AESMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AESMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Electronic code book mode"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(AESMODE_A::ECB)
    }
    #[doc = "Cipher block chaining mode"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(AESMODE_A::CBC)
    }
    #[doc = "Output feedback mode"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut W {
        self.variant(AESMODE_A::OFB)
    }
    #[doc = "Cipher feedback mode"]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut W {
        self.variant(AESMODE_A::CFB)
    }
    #[doc = "Counter mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut W {
        self.variant(AESMODE_A::COUNTER)
    }
    #[doc = "CCM mode"]
    #[inline(always)]
    pub fn ccm(self) -> &'a mut W {
        self.variant(AESMODE_A::CCM)
    }
    #[doc = "Galois counter mode"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut W {
        self.variant(AESMODE_A::GCM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Cipher Feedback Block Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFBS_A {
    #[doc = "0: 128-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _128BIT = 0,
    #[doc = "1: 64-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _64BIT = 1,
    #[doc = "2: 32-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _32BIT = 2,
    #[doc = "3: 16-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _16BIT = 3,
    #[doc = "4: 8-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _8BIT = 4,
}
impl From<CFBS_A> for u8 {
    #[inline(always)]
    fn from(variant: CFBS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFBS`"]
pub type CFBS_R = crate::R<u8, CFBS_A>;
impl CFBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CFBS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CFBS_A::_128BIT),
            1 => Val(CFBS_A::_64BIT),
            2 => Val(CFBS_A::_32BIT),
            3 => Val(CFBS_A::_16BIT),
            4 => Val(CFBS_A::_8BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_128BIT`"]
    #[inline(always)]
    pub fn is_128bit(&self) -> bool {
        *self == CFBS_A::_128BIT
    }
    #[doc = "Checks if the value of the field is `_64BIT`"]
    #[inline(always)]
    pub fn is_64bit(&self) -> bool {
        *self == CFBS_A::_64BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == CFBS_A::_32BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == CFBS_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == CFBS_A::_8BIT
    }
}
#[doc = "Write proxy for field `CFBS`"]
pub struct CFBS_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFBS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "128-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _128bit(self) -> &'a mut W {
        self.variant(CFBS_A::_128BIT)
    }
    #[doc = "64-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _64bit(self) -> &'a mut W {
        self.variant(CFBS_A::_64BIT)
    }
    #[doc = "32-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(CFBS_A::_32BIT)
    }
    #[doc = "16-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(CFBS_A::_16BIT)
    }
    #[doc = "8-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(CFBS_A::_8BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Encryption Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEYSIZE_A {
    #[doc = "0: 128-bit Key for Encryption / Decryption"]
    _128BIT = 0,
    #[doc = "1: 192-bit Key for Encryption / Decryption"]
    _192BIT = 1,
    #[doc = "2: 256-bit Key for Encryption / Decryption"]
    _256BIT = 2,
}
impl From<KEYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KEYSIZE`"]
pub type KEYSIZE_R = crate::R<u8, KEYSIZE_A>;
impl KEYSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KEYSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KEYSIZE_A::_128BIT),
            1 => Val(KEYSIZE_A::_192BIT),
            2 => Val(KEYSIZE_A::_256BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_128BIT`"]
    #[inline(always)]
    pub fn is_128bit(&self) -> bool {
        *self == KEYSIZE_A::_128BIT
    }
    #[doc = "Checks if the value of the field is `_192BIT`"]
    #[inline(always)]
    pub fn is_192bit(&self) -> bool {
        *self == KEYSIZE_A::_192BIT
    }
    #[doc = "Checks if the value of the field is `_256BIT`"]
    #[inline(always)]
    pub fn is_256bit(&self) -> bool {
        *self == KEYSIZE_A::_256BIT
    }
}
#[doc = "Write proxy for field `KEYSIZE`"]
pub struct KEYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "128-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn _128bit(self) -> &'a mut W {
        self.variant(KEYSIZE_A::_128BIT)
    }
    #[doc = "192-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn _192bit(self) -> &'a mut W {
        self.variant(KEYSIZE_A::_192BIT)
    }
    #[doc = "256-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn _256bit(self) -> &'a mut W {
        self.variant(KEYSIZE_A::_256BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Cipher Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIPHER_A {
    #[doc = "0: Decryption"]
    DEC = 0,
    #[doc = "1: Encryption"]
    ENC = 1,
}
impl From<CIPHER_A> for bool {
    #[inline(always)]
    fn from(variant: CIPHER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CIPHER`"]
pub type CIPHER_R = crate::R<bool, CIPHER_A>;
impl CIPHER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIPHER_A {
        match self.bits {
            false => CIPHER_A::DEC,
            true => CIPHER_A::ENC,
        }
    }
    #[doc = "Checks if the value of the field is `DEC`"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        *self == CIPHER_A::DEC
    }
    #[doc = "Checks if the value of the field is `ENC`"]
    #[inline(always)]
    pub fn is_enc(&self) -> bool {
        *self == CIPHER_A::ENC
    }
}
#[doc = "Write proxy for field `CIPHER`"]
pub struct CIPHER_W<'a> {
    w: &'a mut W,
}
impl<'a> CIPHER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIPHER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn dec(self) -> &'a mut W {
        self.variant(CIPHER_A::DEC)
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn enc(self) -> &'a mut W {
        self.variant(CIPHER_A::ENC)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Start Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTMODE_A {
    #[doc = "0: Start Encryption / Decryption in Manual mode"]
    MANUAL = 0,
    #[doc = "1: Start Encryption / Decryption in Auto mode"]
    AUTO = 1,
}
impl From<STARTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: STARTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STARTMODE`"]
pub type STARTMODE_R = crate::R<bool, STARTMODE_A>;
impl STARTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTMODE_A {
        match self.bits {
            false => STARTMODE_A::MANUAL,
            true => STARTMODE_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == STARTMODE_A::MANUAL
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == STARTMODE_A::AUTO
    }
}
#[doc = "Write proxy for field `STARTMODE`"]
pub struct STARTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start Encryption / Decryption in Manual mode"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(STARTMODE_A::MANUAL)
    }
    #[doc = "Start Encryption / Decryption in Auto mode"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(STARTMODE_A::AUTO)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Last Output Data Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOD_A {
    #[doc = "0: No effect"]
    NONE = 0,
    #[doc = "1: Start encryption in Last Output Data mode"]
    LAST = 1,
}
impl From<LOD_A> for bool {
    #[inline(always)]
    fn from(variant: LOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOD`"]
pub type LOD_R = crate::R<bool, LOD_A>;
impl LOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOD_A {
        match self.bits {
            false => LOD_A::NONE,
            true => LOD_A::LAST,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LOD_A::NONE
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == LOD_A::LAST
    }
}
#[doc = "Write proxy for field `LOD`"]
pub struct LOD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LOD_A::NONE)
    }
    #[doc = "Start encryption in Last Output Data mode"]
    #[inline(always)]
    pub fn last(self) -> &'a mut W {
        self.variant(LOD_A::LAST)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Last Key Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYGEN_A {
    #[doc = "0: No effect"]
    NONE = 0,
    #[doc = "1: Start Computation of the last NK words of the expanded key"]
    LAST = 1,
}
impl From<KEYGEN_A> for bool {
    #[inline(always)]
    fn from(variant: KEYGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KEYGEN`"]
pub type KEYGEN_R = crate::R<bool, KEYGEN_A>;
impl KEYGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYGEN_A {
        match self.bits {
            false => KEYGEN_A::NONE,
            true => KEYGEN_A::LAST,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == KEYGEN_A::NONE
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == KEYGEN_A::LAST
    }
}
#[doc = "Write proxy for field `KEYGEN`"]
pub struct KEYGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(KEYGEN_A::NONE)
    }
    #[doc = "Start Computation of the last NK words of the expanded key"]
    #[inline(always)]
    pub fn last(self) -> &'a mut W {
        self.variant(KEYGEN_A::LAST)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "XOR Key Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XORKEY_A {
    #[doc = "0: No effect"]
    NONE = 0,
    #[doc = "1: The user keyword gets XORed with the previous keyword register content."]
    XOR = 1,
}
impl From<XORKEY_A> for bool {
    #[inline(always)]
    fn from(variant: XORKEY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XORKEY`"]
pub type XORKEY_R = crate::R<bool, XORKEY_A>;
impl XORKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XORKEY_A {
        match self.bits {
            false => XORKEY_A::NONE,
            true => XORKEY_A::XOR,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == XORKEY_A::NONE
    }
    #[doc = "Checks if the value of the field is `XOR`"]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == XORKEY_A::XOR
    }
}
#[doc = "Write proxy for field `XORKEY`"]
pub struct XORKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> XORKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XORKEY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(XORKEY_A::NONE)
    }
    #[doc = "The user keyword gets XORed with the previous keyword register content."]
    #[inline(always)]
    pub fn xor(self) -> &'a mut W {
        self.variant(XORKEY_A::XOR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CTYPE`"]
pub type CTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTYPE`"]
pub struct CTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - AES Modes of operation"]
    #[inline(always)]
    pub fn aesmode(&self) -> AESMODE_R {
        AESMODE_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - Cipher Feedback Block Size"]
    #[inline(always)]
    pub fn cfbs(&self) -> CFBS_R {
        CFBS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Encryption Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Cipher Mode"]
    #[inline(always)]
    pub fn cipher(&self) -> CIPHER_R {
        CIPHER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Start Mode Select"]
    #[inline(always)]
    pub fn startmode(&self) -> STARTMODE_R {
        STARTMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&self) -> LOD_R {
        LOD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Last Key Generation"]
    #[inline(always)]
    pub fn keygen(&self) -> KEYGEN_R {
        KEYGEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XOR Key Operation"]
    #[inline(always)]
    pub fn xorkey(&self) -> XORKEY_R {
        XORKEY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Counter Measure Type"]
    #[inline(always)]
    pub fn ctype(&self) -> CTYPE_R {
        CTYPE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:4 - AES Modes of operation"]
    #[inline(always)]
    pub fn aesmode(&mut self) -> AESMODE_W {
        AESMODE_W { w: self }
    }
    #[doc = "Bits 5:7 - Cipher Feedback Block Size"]
    #[inline(always)]
    pub fn cfbs(&mut self) -> CFBS_W {
        CFBS_W { w: self }
    }
    #[doc = "Bits 8:9 - Encryption Key Size"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W {
        KEYSIZE_W { w: self }
    }
    #[doc = "Bit 10 - Cipher Mode"]
    #[inline(always)]
    pub fn cipher(&mut self) -> CIPHER_W {
        CIPHER_W { w: self }
    }
    #[doc = "Bit 11 - Start Mode Select"]
    #[inline(always)]
    pub fn startmode(&mut self) -> STARTMODE_W {
        STARTMODE_W { w: self }
    }
    #[doc = "Bit 12 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&mut self) -> LOD_W {
        LOD_W { w: self }
    }
    #[doc = "Bit 13 - Last Key Generation"]
    #[inline(always)]
    pub fn keygen(&mut self) -> KEYGEN_W {
        KEYGEN_W { w: self }
    }
    #[doc = "Bit 14 - XOR Key Operation"]
    #[inline(always)]
    pub fn xorkey(&mut self) -> XORKEY_W {
        XORKEY_W { w: self }
    }
    #[doc = "Bits 16:19 - Counter Measure Type"]
    #[inline(always)]
    pub fn ctype(&mut self) -> CTYPE_W {
        CTYPE_W { w: self }
    }
}
