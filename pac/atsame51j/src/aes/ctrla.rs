#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `AESMODE` reader - AES Modes of operation"]
pub type AESMODE_R = crate::FieldReader<u8, AESMODESELECT_A>;
#[doc = "AES Modes of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AESMODESELECT_A {
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
impl From<AESMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: AESMODESELECT_A) -> Self {
        variant as _
    }
}
impl AESMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AESMODESELECT_A> {
        match self.bits {
            0 => Some(AESMODESELECT_A::ECB),
            1 => Some(AESMODESELECT_A::CBC),
            2 => Some(AESMODESELECT_A::OFB),
            3 => Some(AESMODESELECT_A::CFB),
            4 => Some(AESMODESELECT_A::COUNTER),
            5 => Some(AESMODESELECT_A::CCM),
            6 => Some(AESMODESELECT_A::GCM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == AESMODESELECT_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == AESMODESELECT_A::CBC
    }
    #[doc = "Checks if the value of the field is `OFB`"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == AESMODESELECT_A::OFB
    }
    #[doc = "Checks if the value of the field is `CFB`"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        *self == AESMODESELECT_A::CFB
    }
    #[doc = "Checks if the value of the field is `COUNTER`"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == AESMODESELECT_A::COUNTER
    }
    #[doc = "Checks if the value of the field is `CCM`"]
    #[inline(always)]
    pub fn is_ccm(&self) -> bool {
        *self == AESMODESELECT_A::CCM
    }
    #[doc = "Checks if the value of the field is `GCM`"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == AESMODESELECT_A::GCM
    }
}
#[doc = "Field `AESMODE` writer - AES Modes of operation"]
pub type AESMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, AESMODESELECT_A, 3, O>;
impl<'a, const O: u8> AESMODE_W<'a, O> {
    #[doc = "Electronic code book mode"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(AESMODESELECT_A::ECB)
    }
    #[doc = "Cipher block chaining mode"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(AESMODESELECT_A::CBC)
    }
    #[doc = "Output feedback mode"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut W {
        self.variant(AESMODESELECT_A::OFB)
    }
    #[doc = "Cipher feedback mode"]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut W {
        self.variant(AESMODESELECT_A::CFB)
    }
    #[doc = "Counter mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut W {
        self.variant(AESMODESELECT_A::COUNTER)
    }
    #[doc = "CCM mode"]
    #[inline(always)]
    pub fn ccm(self) -> &'a mut W {
        self.variant(AESMODESELECT_A::CCM)
    }
    #[doc = "Galois counter mode"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut W {
        self.variant(AESMODESELECT_A::GCM)
    }
}
#[doc = "Field `CFBS` reader - Cipher Feedback Block Size"]
pub type CFBS_R = crate::FieldReader<u8, CFBSSELECT_A>;
#[doc = "Cipher Feedback Block Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFBSSELECT_A {
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
impl From<CFBSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CFBSSELECT_A) -> Self {
        variant as _
    }
}
impl CFBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFBSSELECT_A> {
        match self.bits {
            0 => Some(CFBSSELECT_A::_128BIT),
            1 => Some(CFBSSELECT_A::_64BIT),
            2 => Some(CFBSSELECT_A::_32BIT),
            3 => Some(CFBSSELECT_A::_16BIT),
            4 => Some(CFBSSELECT_A::_8BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_128BIT`"]
    #[inline(always)]
    pub fn is_128bit(&self) -> bool {
        *self == CFBSSELECT_A::_128BIT
    }
    #[doc = "Checks if the value of the field is `_64BIT`"]
    #[inline(always)]
    pub fn is_64bit(&self) -> bool {
        *self == CFBSSELECT_A::_64BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == CFBSSELECT_A::_32BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == CFBSSELECT_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == CFBSSELECT_A::_8BIT
    }
}
#[doc = "Field `CFBS` writer - Cipher Feedback Block Size"]
pub type CFBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, CFBSSELECT_A, 3, O>;
impl<'a, const O: u8> CFBS_W<'a, O> {
    #[doc = "128-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _128bit(self) -> &'a mut W {
        self.variant(CFBSSELECT_A::_128BIT)
    }
    #[doc = "64-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _64bit(self) -> &'a mut W {
        self.variant(CFBSSELECT_A::_64BIT)
    }
    #[doc = "32-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(CFBSSELECT_A::_32BIT)
    }
    #[doc = "16-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(CFBSSELECT_A::_16BIT)
    }
    #[doc = "8-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(CFBSSELECT_A::_8BIT)
    }
}
#[doc = "Field `KEYSIZE` reader - Encryption Key Size"]
pub type KEYSIZE_R = crate::FieldReader<u8, KEYSIZESELECT_A>;
#[doc = "Encryption Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSIZESELECT_A {
    #[doc = "0: 128-bit Key for Encryption / Decryption"]
    _128BIT = 0,
    #[doc = "1: 192-bit Key for Encryption / Decryption"]
    _192BIT = 1,
    #[doc = "2: 256-bit Key for Encryption / Decryption"]
    _256BIT = 2,
}
impl From<KEYSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYSIZESELECT_A) -> Self {
        variant as _
    }
}
impl KEYSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEYSIZESELECT_A> {
        match self.bits {
            0 => Some(KEYSIZESELECT_A::_128BIT),
            1 => Some(KEYSIZESELECT_A::_192BIT),
            2 => Some(KEYSIZESELECT_A::_256BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_128BIT`"]
    #[inline(always)]
    pub fn is_128bit(&self) -> bool {
        *self == KEYSIZESELECT_A::_128BIT
    }
    #[doc = "Checks if the value of the field is `_192BIT`"]
    #[inline(always)]
    pub fn is_192bit(&self) -> bool {
        *self == KEYSIZESELECT_A::_192BIT
    }
    #[doc = "Checks if the value of the field is `_256BIT`"]
    #[inline(always)]
    pub fn is_256bit(&self) -> bool {
        *self == KEYSIZESELECT_A::_256BIT
    }
}
#[doc = "Field `KEYSIZE` writer - Encryption Key Size"]
pub type KEYSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, KEYSIZESELECT_A, 2, O>;
impl<'a, const O: u8> KEYSIZE_W<'a, O> {
    #[doc = "128-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn _128bit(self) -> &'a mut W {
        self.variant(KEYSIZESELECT_A::_128BIT)
    }
    #[doc = "192-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn _192bit(self) -> &'a mut W {
        self.variant(KEYSIZESELECT_A::_192BIT)
    }
    #[doc = "256-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn _256bit(self) -> &'a mut W {
        self.variant(KEYSIZESELECT_A::_256BIT)
    }
}
#[doc = "Field `CIPHER` reader - Cipher Mode"]
pub type CIPHER_R = crate::BitReader<CIPHERSELECT_A>;
#[doc = "Cipher Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIPHERSELECT_A {
    #[doc = "0: Decryption"]
    DEC = 0,
    #[doc = "1: Encryption"]
    ENC = 1,
}
impl From<CIPHERSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CIPHERSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CIPHER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIPHERSELECT_A {
        match self.bits {
            false => CIPHERSELECT_A::DEC,
            true => CIPHERSELECT_A::ENC,
        }
    }
    #[doc = "Checks if the value of the field is `DEC`"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        *self == CIPHERSELECT_A::DEC
    }
    #[doc = "Checks if the value of the field is `ENC`"]
    #[inline(always)]
    pub fn is_enc(&self) -> bool {
        *self == CIPHERSELECT_A::ENC
    }
}
#[doc = "Field `CIPHER` writer - Cipher Mode"]
pub type CIPHER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, CIPHERSELECT_A, O>;
impl<'a, const O: u8> CIPHER_W<'a, O> {
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn dec(self) -> &'a mut W {
        self.variant(CIPHERSELECT_A::DEC)
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn enc(self) -> &'a mut W {
        self.variant(CIPHERSELECT_A::ENC)
    }
}
#[doc = "Field `STARTMODE` reader - Start Mode Select"]
pub type STARTMODE_R = crate::BitReader<STARTMODESELECT_A>;
#[doc = "Start Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTMODESELECT_A {
    #[doc = "0: Start Encryption / Decryption in Manual mode"]
    MANUAL = 0,
    #[doc = "1: Start Encryption / Decryption in Auto mode"]
    AUTO = 1,
}
impl From<STARTMODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: STARTMODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl STARTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTMODESELECT_A {
        match self.bits {
            false => STARTMODESELECT_A::MANUAL,
            true => STARTMODESELECT_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL`"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == STARTMODESELECT_A::MANUAL
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == STARTMODESELECT_A::AUTO
    }
}
#[doc = "Field `STARTMODE` writer - Start Mode Select"]
pub type STARTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, STARTMODESELECT_A, O>;
impl<'a, const O: u8> STARTMODE_W<'a, O> {
    #[doc = "Start Encryption / Decryption in Manual mode"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut W {
        self.variant(STARTMODESELECT_A::MANUAL)
    }
    #[doc = "Start Encryption / Decryption in Auto mode"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(STARTMODESELECT_A::AUTO)
    }
}
#[doc = "Field `LOD` reader - Last Output Data Mode"]
pub type LOD_R = crate::BitReader<LODSELECT_A>;
#[doc = "Last Output Data Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LODSELECT_A {
    #[doc = "0: No effect"]
    NONE = 0,
    #[doc = "1: Start encryption in Last Output Data mode"]
    LAST = 1,
}
impl From<LODSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LODSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LODSELECT_A {
        match self.bits {
            false => LODSELECT_A::NONE,
            true => LODSELECT_A::LAST,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LODSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == LODSELECT_A::LAST
    }
}
#[doc = "Field `LOD` writer - Last Output Data Mode"]
pub type LOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, LODSELECT_A, O>;
impl<'a, const O: u8> LOD_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LODSELECT_A::NONE)
    }
    #[doc = "Start encryption in Last Output Data mode"]
    #[inline(always)]
    pub fn last(self) -> &'a mut W {
        self.variant(LODSELECT_A::LAST)
    }
}
#[doc = "Field `KEYGEN` reader - Last Key Generation"]
pub type KEYGEN_R = crate::BitReader<KEYGENSELECT_A>;
#[doc = "Last Key Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYGENSELECT_A {
    #[doc = "0: No effect"]
    NONE = 0,
    #[doc = "1: Start Computation of the last NK words of the expanded key"]
    LAST = 1,
}
impl From<KEYGENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: KEYGENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl KEYGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYGENSELECT_A {
        match self.bits {
            false => KEYGENSELECT_A::NONE,
            true => KEYGENSELECT_A::LAST,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == KEYGENSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == KEYGENSELECT_A::LAST
    }
}
#[doc = "Field `KEYGEN` writer - Last Key Generation"]
pub type KEYGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, KEYGENSELECT_A, O>;
impl<'a, const O: u8> KEYGEN_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(KEYGENSELECT_A::NONE)
    }
    #[doc = "Start Computation of the last NK words of the expanded key"]
    #[inline(always)]
    pub fn last(self) -> &'a mut W {
        self.variant(KEYGENSELECT_A::LAST)
    }
}
#[doc = "Field `XORKEY` reader - XOR Key Operation"]
pub type XORKEY_R = crate::BitReader<XORKEYSELECT_A>;
#[doc = "XOR Key Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XORKEYSELECT_A {
    #[doc = "0: No effect"]
    NONE = 0,
    #[doc = "1: The user keyword gets XORed with the previous keyword register content."]
    XOR = 1,
}
impl From<XORKEYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: XORKEYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl XORKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XORKEYSELECT_A {
        match self.bits {
            false => XORKEYSELECT_A::NONE,
            true => XORKEYSELECT_A::XOR,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == XORKEYSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `XOR`"]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == XORKEYSELECT_A::XOR
    }
}
#[doc = "Field `XORKEY` writer - XOR Key Operation"]
pub type XORKEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, XORKEYSELECT_A, O>;
impl<'a, const O: u8> XORKEY_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(XORKEYSELECT_A::NONE)
    }
    #[doc = "The user keyword gets XORed with the previous keyword register content."]
    #[inline(always)]
    pub fn xor(self) -> &'a mut W {
        self.variant(XORKEYSELECT_A::XOR)
    }
}
#[doc = "Field `CTYPE` reader - Counter Measure Type"]
pub type CTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTYPE` writer - Counter Measure Type"]
pub type CTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - AES Modes of operation"]
    #[inline(always)]
    pub fn aesmode(&self) -> AESMODE_R {
        AESMODE_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Cipher Feedback Block Size"]
    #[inline(always)]
    pub fn cfbs(&self) -> CFBS_R {
        CFBS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Encryption Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Cipher Mode"]
    #[inline(always)]
    pub fn cipher(&self) -> CIPHER_R {
        CIPHER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Start Mode Select"]
    #[inline(always)]
    pub fn startmode(&self) -> STARTMODE_R {
        STARTMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&self) -> LOD_R {
        LOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Last Key Generation"]
    #[inline(always)]
    pub fn keygen(&self) -> KEYGEN_R {
        KEYGEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XOR Key Operation"]
    #[inline(always)]
    pub fn xorkey(&self) -> XORKEY_R {
        XORKEY_R::new(((self.bits >> 14) & 1) != 0)
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
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:4 - AES Modes of operation"]
    #[inline(always)]
    #[must_use]
    pub fn aesmode(&mut self) -> AESMODE_W<2> {
        AESMODE_W::new(self)
    }
    #[doc = "Bits 5:7 - Cipher Feedback Block Size"]
    #[inline(always)]
    #[must_use]
    pub fn cfbs(&mut self) -> CFBS_W<5> {
        CFBS_W::new(self)
    }
    #[doc = "Bits 8:9 - Encryption Key Size"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<8> {
        KEYSIZE_W::new(self)
    }
    #[doc = "Bit 10 - Cipher Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cipher(&mut self) -> CIPHER_W<10> {
        CIPHER_W::new(self)
    }
    #[doc = "Bit 11 - Start Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn startmode(&mut self) -> STARTMODE_W<11> {
        STARTMODE_W::new(self)
    }
    #[doc = "Bit 12 - Last Output Data Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lod(&mut self) -> LOD_W<12> {
        LOD_W::new(self)
    }
    #[doc = "Bit 13 - Last Key Generation"]
    #[inline(always)]
    #[must_use]
    pub fn keygen(&mut self) -> KEYGEN_W<13> {
        KEYGEN_W::new(self)
    }
    #[doc = "Bit 14 - XOR Key Operation"]
    #[inline(always)]
    #[must_use]
    pub fn xorkey(&mut self) -> XORKEY_W<14> {
        XORKEY_W::new(self)
    }
    #[doc = "Bits 16:19 - Counter Measure Type"]
    #[inline(always)]
    #[must_use]
    pub fn ctype(&mut self) -> CTYPE_W<16> {
        CTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
