#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "AES Modes of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aesmodeselect {
    #[doc = "0: Electronic code book mode"]
    Ecb = 0,
    #[doc = "1: Cipher block chaining mode"]
    Cbc = 1,
    #[doc = "2: Output feedback mode"]
    Ofb = 2,
    #[doc = "3: Cipher feedback mode"]
    Cfb = 3,
    #[doc = "4: Counter mode"]
    Counter = 4,
    #[doc = "5: CCM mode"]
    Ccm = 5,
    #[doc = "6: Galois counter mode"]
    Gcm = 6,
}
impl From<Aesmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Aesmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aesmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Aesmodeselect {}
#[doc = "Field `AESMODE` reader - AES Modes of operation"]
pub type AesmodeR = crate::FieldReader<Aesmodeselect>;
impl AesmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aesmodeselect> {
        match self.bits {
            0 => Some(Aesmodeselect::Ecb),
            1 => Some(Aesmodeselect::Cbc),
            2 => Some(Aesmodeselect::Ofb),
            3 => Some(Aesmodeselect::Cfb),
            4 => Some(Aesmodeselect::Counter),
            5 => Some(Aesmodeselect::Ccm),
            6 => Some(Aesmodeselect::Gcm),
            _ => None,
        }
    }
    #[doc = "Electronic code book mode"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == Aesmodeselect::Ecb
    }
    #[doc = "Cipher block chaining mode"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == Aesmodeselect::Cbc
    }
    #[doc = "Output feedback mode"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == Aesmodeselect::Ofb
    }
    #[doc = "Cipher feedback mode"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        *self == Aesmodeselect::Cfb
    }
    #[doc = "Counter mode"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == Aesmodeselect::Counter
    }
    #[doc = "CCM mode"]
    #[inline(always)]
    pub fn is_ccm(&self) -> bool {
        *self == Aesmodeselect::Ccm
    }
    #[doc = "Galois counter mode"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == Aesmodeselect::Gcm
    }
}
#[doc = "Field `AESMODE` writer - AES Modes of operation"]
pub type AesmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Aesmodeselect>;
impl<'a, REG> AesmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Electronic code book mode"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(Aesmodeselect::Ecb)
    }
    #[doc = "Cipher block chaining mode"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(Aesmodeselect::Cbc)
    }
    #[doc = "Output feedback mode"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut crate::W<REG> {
        self.variant(Aesmodeselect::Ofb)
    }
    #[doc = "Cipher feedback mode"]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut crate::W<REG> {
        self.variant(Aesmodeselect::Cfb)
    }
    #[doc = "Counter mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(Aesmodeselect::Counter)
    }
    #[doc = "CCM mode"]
    #[inline(always)]
    pub fn ccm(self) -> &'a mut crate::W<REG> {
        self.variant(Aesmodeselect::Ccm)
    }
    #[doc = "Galois counter mode"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut crate::W<REG> {
        self.variant(Aesmodeselect::Gcm)
    }
}
#[doc = "Cipher Feedback Block Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfbsselect {
    #[doc = "0: 128-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _128bit = 0,
    #[doc = "1: 64-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _64bit = 1,
    #[doc = "2: 32-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _32bit = 2,
    #[doc = "3: 16-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _16bit = 3,
    #[doc = "4: 8-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    _8bit = 4,
}
impl From<Cfbsselect> for u8 {
    #[inline(always)]
    fn from(variant: Cfbsselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfbsselect {
    type Ux = u8;
}
impl crate::IsEnum for Cfbsselect {}
#[doc = "Field `CFBS` reader - Cipher Feedback Block Size"]
pub type CfbsR = crate::FieldReader<Cfbsselect>;
impl CfbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfbsselect> {
        match self.bits {
            0 => Some(Cfbsselect::_128bit),
            1 => Some(Cfbsselect::_64bit),
            2 => Some(Cfbsselect::_32bit),
            3 => Some(Cfbsselect::_16bit),
            4 => Some(Cfbsselect::_8bit),
            _ => None,
        }
    }
    #[doc = "128-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn is_128bit(&self) -> bool {
        *self == Cfbsselect::_128bit
    }
    #[doc = "64-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn is_64bit(&self) -> bool {
        *self == Cfbsselect::_64bit
    }
    #[doc = "32-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == Cfbsselect::_32bit
    }
    #[doc = "16-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Cfbsselect::_16bit
    }
    #[doc = "8-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Cfbsselect::_8bit
    }
}
#[doc = "Field `CFBS` writer - Cipher Feedback Block Size"]
pub type CfbsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfbsselect>;
impl<'a, REG> CfbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _128bit(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbsselect::_128bit)
    }
    #[doc = "64-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _64bit(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbsselect::_64bit)
    }
    #[doc = "32-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbsselect::_32bit)
    }
    #[doc = "16-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbsselect::_16bit)
    }
    #[doc = "8-bit Input data block for Encryption/Decryption in Cipher Feedback mode"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Cfbsselect::_8bit)
    }
}
#[doc = "Encryption Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keysizeselect {
    #[doc = "0: 128-bit Key for Encryption / Decryption"]
    _128bit = 0,
    #[doc = "1: 192-bit Key for Encryption / Decryption"]
    _192bit = 1,
    #[doc = "2: 256-bit Key for Encryption / Decryption"]
    _256bit = 2,
}
impl From<Keysizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Keysizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keysizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Keysizeselect {}
#[doc = "Field `KEYSIZE` reader - Encryption Key Size"]
pub type KeysizeR = crate::FieldReader<Keysizeselect>;
impl KeysizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keysizeselect> {
        match self.bits {
            0 => Some(Keysizeselect::_128bit),
            1 => Some(Keysizeselect::_192bit),
            2 => Some(Keysizeselect::_256bit),
            _ => None,
        }
    }
    #[doc = "128-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn is_128bit(&self) -> bool {
        *self == Keysizeselect::_128bit
    }
    #[doc = "192-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn is_192bit(&self) -> bool {
        *self == Keysizeselect::_192bit
    }
    #[doc = "256-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn is_256bit(&self) -> bool {
        *self == Keysizeselect::_256bit
    }
}
#[doc = "Field `KEYSIZE` writer - Encryption Key Size"]
pub type KeysizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Keysizeselect>;
impl<'a, REG> KeysizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn _128bit(self) -> &'a mut crate::W<REG> {
        self.variant(Keysizeselect::_128bit)
    }
    #[doc = "192-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn _192bit(self) -> &'a mut crate::W<REG> {
        self.variant(Keysizeselect::_192bit)
    }
    #[doc = "256-bit Key for Encryption / Decryption"]
    #[inline(always)]
    pub fn _256bit(self) -> &'a mut crate::W<REG> {
        self.variant(Keysizeselect::_256bit)
    }
}
#[doc = "Cipher Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cipherselect {
    #[doc = "0: Decryption"]
    Dec = 0,
    #[doc = "1: Encryption"]
    Enc = 1,
}
impl From<Cipherselect> for bool {
    #[inline(always)]
    fn from(variant: Cipherselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIPHER` reader - Cipher Mode"]
pub type CipherR = crate::BitReader<Cipherselect>;
impl CipherR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cipherselect {
        match self.bits {
            false => Cipherselect::Dec,
            true => Cipherselect::Enc,
        }
    }
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        *self == Cipherselect::Dec
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn is_enc(&self) -> bool {
        *self == Cipherselect::Enc
    }
}
#[doc = "Field `CIPHER` writer - Cipher Mode"]
pub type CipherW<'a, REG> = crate::BitWriter<'a, REG, Cipherselect>;
impl<'a, REG> CipherW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Decryption"]
    #[inline(always)]
    pub fn dec(self) -> &'a mut crate::W<REG> {
        self.variant(Cipherselect::Dec)
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn enc(self) -> &'a mut crate::W<REG> {
        self.variant(Cipherselect::Enc)
    }
}
#[doc = "Start Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Startmodeselect {
    #[doc = "0: Start Encryption / Decryption in Manual mode"]
    Manual = 0,
    #[doc = "1: Start Encryption / Decryption in Auto mode"]
    Auto = 1,
}
impl From<Startmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Startmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTMODE` reader - Start Mode Select"]
pub type StartmodeR = crate::BitReader<Startmodeselect>;
impl StartmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startmodeselect {
        match self.bits {
            false => Startmodeselect::Manual,
            true => Startmodeselect::Auto,
        }
    }
    #[doc = "Start Encryption / Decryption in Manual mode"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Startmodeselect::Manual
    }
    #[doc = "Start Encryption / Decryption in Auto mode"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Startmodeselect::Auto
    }
}
#[doc = "Field `STARTMODE` writer - Start Mode Select"]
pub type StartmodeW<'a, REG> = crate::BitWriter<'a, REG, Startmodeselect>;
impl<'a, REG> StartmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start Encryption / Decryption in Manual mode"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Startmodeselect::Manual)
    }
    #[doc = "Start Encryption / Decryption in Auto mode"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Startmodeselect::Auto)
    }
}
#[doc = "Last Output Data Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lodselect {
    #[doc = "0: No effect"]
    None = 0,
    #[doc = "1: Start encryption in Last Output Data mode"]
    Last = 1,
}
impl From<Lodselect> for bool {
    #[inline(always)]
    fn from(variant: Lodselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOD` reader - Last Output Data Mode"]
pub type LodR = crate::BitReader<Lodselect>;
impl LodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lodselect {
        match self.bits {
            false => Lodselect::None,
            true => Lodselect::Last,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Lodselect::None
    }
    #[doc = "Start encryption in Last Output Data mode"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == Lodselect::Last
    }
}
#[doc = "Field `LOD` writer - Last Output Data Mode"]
pub type LodW<'a, REG> = crate::BitWriter<'a, REG, Lodselect>;
impl<'a, REG> LodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Lodselect::None)
    }
    #[doc = "Start encryption in Last Output Data mode"]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(Lodselect::Last)
    }
}
#[doc = "Last Key Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keygenselect {
    #[doc = "0: No effect"]
    None = 0,
    #[doc = "1: Start Computation of the last NK words of the expanded key"]
    Last = 1,
}
impl From<Keygenselect> for bool {
    #[inline(always)]
    fn from(variant: Keygenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEYGEN` reader - Last Key Generation"]
pub type KeygenR = crate::BitReader<Keygenselect>;
impl KeygenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keygenselect {
        match self.bits {
            false => Keygenselect::None,
            true => Keygenselect::Last,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Keygenselect::None
    }
    #[doc = "Start Computation of the last NK words of the expanded key"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == Keygenselect::Last
    }
}
#[doc = "Field `KEYGEN` writer - Last Key Generation"]
pub type KeygenW<'a, REG> = crate::BitWriter<'a, REG, Keygenselect>;
impl<'a, REG> KeygenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Keygenselect::None)
    }
    #[doc = "Start Computation of the last NK words of the expanded key"]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(Keygenselect::Last)
    }
}
#[doc = "XOR Key Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xorkeyselect {
    #[doc = "0: No effect"]
    None = 0,
    #[doc = "1: The user keyword gets XORed with the previous keyword register content."]
    Xor = 1,
}
impl From<Xorkeyselect> for bool {
    #[inline(always)]
    fn from(variant: Xorkeyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XORKEY` reader - XOR Key Operation"]
pub type XorkeyR = crate::BitReader<Xorkeyselect>;
impl XorkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xorkeyselect {
        match self.bits {
            false => Xorkeyselect::None,
            true => Xorkeyselect::Xor,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Xorkeyselect::None
    }
    #[doc = "The user keyword gets XORed with the previous keyword register content."]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == Xorkeyselect::Xor
    }
}
#[doc = "Field `XORKEY` writer - XOR Key Operation"]
pub type XorkeyW<'a, REG> = crate::BitWriter<'a, REG, Xorkeyselect>;
impl<'a, REG> XorkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Xorkeyselect::None)
    }
    #[doc = "The user keyword gets XORed with the previous keyword register content."]
    #[inline(always)]
    pub fn xor(self) -> &'a mut crate::W<REG> {
        self.variant(Xorkeyselect::Xor)
    }
}
#[doc = "Field `CTYPE` reader - Counter Measure Type"]
pub type CtypeR = crate::FieldReader;
#[doc = "Field `CTYPE` writer - Counter Measure Type"]
pub type CtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - AES Modes of operation"]
    #[inline(always)]
    pub fn aesmode(&self) -> AesmodeR {
        AesmodeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Cipher Feedback Block Size"]
    #[inline(always)]
    pub fn cfbs(&self) -> CfbsR {
        CfbsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Encryption Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KeysizeR {
        KeysizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Cipher Mode"]
    #[inline(always)]
    pub fn cipher(&self) -> CipherR {
        CipherR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Start Mode Select"]
    #[inline(always)]
    pub fn startmode(&self) -> StartmodeR {
        StartmodeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&self) -> LodR {
        LodR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Last Key Generation"]
    #[inline(always)]
    pub fn keygen(&self) -> KeygenR {
        KeygenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XOR Key Operation"]
    #[inline(always)]
    pub fn xorkey(&self) -> XorkeyR {
        XorkeyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Counter Measure Type"]
    #[inline(always)]
    pub fn ctype(&self) -> CtypeR {
        CtypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:4 - AES Modes of operation"]
    #[inline(always)]
    #[must_use]
    pub fn aesmode(&mut self) -> AesmodeW<CtrlaSpec> {
        AesmodeW::new(self, 2)
    }
    #[doc = "Bits 5:7 - Cipher Feedback Block Size"]
    #[inline(always)]
    #[must_use]
    pub fn cfbs(&mut self) -> CfbsW<CtrlaSpec> {
        CfbsW::new(self, 5)
    }
    #[doc = "Bits 8:9 - Encryption Key Size"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KeysizeW<CtrlaSpec> {
        KeysizeW::new(self, 8)
    }
    #[doc = "Bit 10 - Cipher Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cipher(&mut self) -> CipherW<CtrlaSpec> {
        CipherW::new(self, 10)
    }
    #[doc = "Bit 11 - Start Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn startmode(&mut self) -> StartmodeW<CtrlaSpec> {
        StartmodeW::new(self, 11)
    }
    #[doc = "Bit 12 - Last Output Data Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lod(&mut self) -> LodW<CtrlaSpec> {
        LodW::new(self, 12)
    }
    #[doc = "Bit 13 - Last Key Generation"]
    #[inline(always)]
    #[must_use]
    pub fn keygen(&mut self) -> KeygenW<CtrlaSpec> {
        KeygenW::new(self, 13)
    }
    #[doc = "Bit 14 - XOR Key Operation"]
    #[inline(always)]
    #[must_use]
    pub fn xorkey(&mut self) -> XorkeyW<CtrlaSpec> {
        XorkeyW::new(self, 14)
    }
    #[doc = "Bits 16:19 - Counter Measure Type"]
    #[inline(always)]
    #[must_use]
    pub fn ctype(&mut self) -> CtypeW<CtrlaSpec> {
        CtypeW::new(self, 16)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0;
}
