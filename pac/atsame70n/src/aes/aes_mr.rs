#[doc = "Register `AES_MR` reader"]
pub struct R(crate::R<AES_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_MR` writer"]
pub struct W(crate::W<AES_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_MR_SPEC>;
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
impl From<crate::W<AES_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CIPHER` reader - Processing Mode"]
pub struct CIPHER_R(crate::FieldReader<bool, bool>);
impl CIPHER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CIPHER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIPHER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIPHER` writer - Processing Mode"]
pub struct CIPHER_W<'a> {
    w: &'a mut W,
}
impl<'a> CIPHER_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `GTAGEN` reader - GCM Automatic Tag Generation Enable"]
pub struct GTAGEN_R(crate::FieldReader<bool, bool>);
impl GTAGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GTAGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTAGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTAGEN` writer - GCM Automatic Tag Generation Enable"]
pub struct GTAGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GTAGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Dual Input Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUALBUFF_A {
    #[doc = "0: AES_IDATARx cannot be written during processing of previous block."]
    INACTIVE = 0,
    #[doc = "1: AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    ACTIVE = 1,
}
impl From<DUALBUFF_A> for bool {
    #[inline(always)]
    fn from(variant: DUALBUFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUALBUFF` reader - Dual Input Buffer"]
pub struct DUALBUFF_R(crate::FieldReader<bool, DUALBUFF_A>);
impl DUALBUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUALBUFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUALBUFF_A {
        match self.bits {
            false => DUALBUFF_A::INACTIVE,
            true => DUALBUFF_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == DUALBUFF_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == DUALBUFF_A::ACTIVE
    }
}
impl core::ops::Deref for DUALBUFF_R {
    type Target = crate::FieldReader<bool, DUALBUFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUALBUFF` writer - Dual Input Buffer"]
pub struct DUALBUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALBUFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DUALBUFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "AES_IDATARx cannot be written during processing of previous block."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DUALBUFF_A::INACTIVE)
    }
    #[doc = "AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(DUALBUFF_A::ACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PROCDLY` reader - Processing Delay"]
pub struct PROCDLY_R(crate::FieldReader<u8, u8>);
impl PROCDLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PROCDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROCDLY` writer - Processing Delay"]
pub struct PROCDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Manual Mode"]
    MANUAL_START = 0,
    #[doc = "1: Auto Mode"]
    AUTO_START = 1,
    #[doc = "2: AES_IDATAR0 access only Auto Mode (DMA)"]
    IDATAR0_START = 2,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMOD` reader - Start Mode"]
pub struct SMOD_R(crate::FieldReader<u8, SMOD_A>);
impl SMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMOD_A> {
        match self.bits {
            0 => Some(SMOD_A::MANUAL_START),
            1 => Some(SMOD_A::AUTO_START),
            2 => Some(SMOD_A::IDATAR0_START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL_START`"]
    #[inline(always)]
    pub fn is_manual_start(&self) -> bool {
        **self == SMOD_A::MANUAL_START
    }
    #[doc = "Checks if the value of the field is `AUTO_START`"]
    #[inline(always)]
    pub fn is_auto_start(&self) -> bool {
        **self == SMOD_A::AUTO_START
    }
    #[doc = "Checks if the value of the field is `IDATAR0_START`"]
    #[inline(always)]
    pub fn is_idatar0_start(&self) -> bool {
        **self == SMOD_A::IDATAR0_START
    }
}
impl core::ops::Deref for SMOD_R {
    type Target = crate::FieldReader<u8, SMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMOD` writer - Start Mode"]
pub struct SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Manual Mode"]
    #[inline(always)]
    pub fn manual_start(self) -> &'a mut W {
        self.variant(SMOD_A::MANUAL_START)
    }
    #[doc = "Auto Mode"]
    #[inline(always)]
    pub fn auto_start(self) -> &'a mut W {
        self.variant(SMOD_A::AUTO_START)
    }
    #[doc = "AES_IDATAR0 access only Auto Mode (DMA)"]
    #[inline(always)]
    pub fn idatar0_start(self) -> &'a mut W {
        self.variant(SMOD_A::IDATAR0_START)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEYSIZE_A {
    #[doc = "0: AES Key Size is 128 bits"]
    AES128 = 0,
    #[doc = "1: AES Key Size is 192 bits"]
    AES192 = 1,
    #[doc = "2: AES Key Size is 256 bits"]
    AES256 = 2,
}
impl From<KEYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEYSIZE` reader - Key Size"]
pub struct KEYSIZE_R(crate::FieldReader<u8, KEYSIZE_A>);
impl KEYSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEYSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEYSIZE_A> {
        match self.bits {
            0 => Some(KEYSIZE_A::AES128),
            1 => Some(KEYSIZE_A::AES192),
            2 => Some(KEYSIZE_A::AES256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        **self == KEYSIZE_A::AES128
    }
    #[doc = "Checks if the value of the field is `AES192`"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        **self == KEYSIZE_A::AES192
    }
    #[doc = "Checks if the value of the field is `AES256`"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        **self == KEYSIZE_A::AES256
    }
}
impl core::ops::Deref for KEYSIZE_R {
    type Target = crate::FieldReader<u8, KEYSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYSIZE` writer - Key Size"]
pub struct KEYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AES Key Size is 128 bits"]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut W {
        self.variant(KEYSIZE_A::AES128)
    }
    #[doc = "AES Key Size is 192 bits"]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut W {
        self.variant(KEYSIZE_A::AES192)
    }
    #[doc = "AES Key Size is 256 bits"]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut W {
        self.variant(KEYSIZE_A::AES256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPMOD_A {
    #[doc = "0: ECB: Electronic Code Book mode"]
    ECB = 0,
    #[doc = "1: CBC: Cipher Block Chaining mode"]
    CBC = 1,
    #[doc = "2: OFB: Output Feedback mode"]
    OFB = 2,
    #[doc = "3: CFB: Cipher Feedback mode"]
    CFB = 3,
    #[doc = "4: CTR: Counter mode (16-bit internal counter)"]
    CTR = 4,
    #[doc = "5: GCM: Galois/Counter mode"]
    GCM = 5,
}
impl From<OPMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPMOD` reader - Operating Mode"]
pub struct OPMOD_R(crate::FieldReader<u8, OPMOD_A>);
impl OPMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPMOD_A> {
        match self.bits {
            0 => Some(OPMOD_A::ECB),
            1 => Some(OPMOD_A::CBC),
            2 => Some(OPMOD_A::OFB),
            3 => Some(OPMOD_A::CFB),
            4 => Some(OPMOD_A::CTR),
            5 => Some(OPMOD_A::GCM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        **self == OPMOD_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        **self == OPMOD_A::CBC
    }
    #[doc = "Checks if the value of the field is `OFB`"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        **self == OPMOD_A::OFB
    }
    #[doc = "Checks if the value of the field is `CFB`"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        **self == OPMOD_A::CFB
    }
    #[doc = "Checks if the value of the field is `CTR`"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        **self == OPMOD_A::CTR
    }
    #[doc = "Checks if the value of the field is `GCM`"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        **self == OPMOD_A::GCM
    }
}
impl core::ops::Deref for OPMOD_R {
    type Target = crate::FieldReader<u8, OPMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMOD` writer - Operating Mode"]
pub struct OPMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ECB: Electronic Code Book mode"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(OPMOD_A::ECB)
    }
    #[doc = "CBC: Cipher Block Chaining mode"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(OPMOD_A::CBC)
    }
    #[doc = "OFB: Output Feedback mode"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut W {
        self.variant(OPMOD_A::OFB)
    }
    #[doc = "CFB: Cipher Feedback mode"]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut W {
        self.variant(OPMOD_A::CFB)
    }
    #[doc = "CTR: Counter mode (16-bit internal counter)"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut W {
        self.variant(OPMOD_A::CTR)
    }
    #[doc = "GCM: Galois/Counter mode"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut W {
        self.variant(OPMOD_A::GCM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `LOD` reader - Last Output Data Mode"]
pub struct LOD_R(crate::FieldReader<bool, bool>);
impl LOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOD` writer - Last Output Data Mode"]
pub struct LOD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Cipher Feedback Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFBS_A {
    #[doc = "0: 128-bit"]
    SIZE_128BIT = 0,
    #[doc = "1: 64-bit"]
    SIZE_64BIT = 1,
    #[doc = "2: 32-bit"]
    SIZE_32BIT = 2,
    #[doc = "3: 16-bit"]
    SIZE_16BIT = 3,
    #[doc = "4: 8-bit"]
    SIZE_8BIT = 4,
}
impl From<CFBS_A> for u8 {
    #[inline(always)]
    fn from(variant: CFBS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFBS` reader - Cipher Feedback Data Size"]
pub struct CFBS_R(crate::FieldReader<u8, CFBS_A>);
impl CFBS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFBS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFBS_A> {
        match self.bits {
            0 => Some(CFBS_A::SIZE_128BIT),
            1 => Some(CFBS_A::SIZE_64BIT),
            2 => Some(CFBS_A::SIZE_32BIT),
            3 => Some(CFBS_A::SIZE_16BIT),
            4 => Some(CFBS_A::SIZE_8BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SIZE_128BIT`"]
    #[inline(always)]
    pub fn is_size_128bit(&self) -> bool {
        **self == CFBS_A::SIZE_128BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_64BIT`"]
    #[inline(always)]
    pub fn is_size_64bit(&self) -> bool {
        **self == CFBS_A::SIZE_64BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_32BIT`"]
    #[inline(always)]
    pub fn is_size_32bit(&self) -> bool {
        **self == CFBS_A::SIZE_32BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_16BIT`"]
    #[inline(always)]
    pub fn is_size_16bit(&self) -> bool {
        **self == CFBS_A::SIZE_16BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_8BIT`"]
    #[inline(always)]
    pub fn is_size_8bit(&self) -> bool {
        **self == CFBS_A::SIZE_8BIT
    }
}
impl core::ops::Deref for CFBS_R {
    type Target = crate::FieldReader<u8, CFBS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFBS` writer - Cipher Feedback Data Size"]
pub struct CFBS_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFBS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "128-bit"]
    #[inline(always)]
    pub fn size_128bit(self) -> &'a mut W {
        self.variant(CFBS_A::SIZE_128BIT)
    }
    #[doc = "64-bit"]
    #[inline(always)]
    pub fn size_64bit(self) -> &'a mut W {
        self.variant(CFBS_A::SIZE_64BIT)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn size_32bit(self) -> &'a mut W {
        self.variant(CFBS_A::SIZE_32BIT)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn size_16bit(self) -> &'a mut W {
        self.variant(CFBS_A::SIZE_16BIT)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn size_8bit(self) -> &'a mut W {
        self.variant(CFBS_A::SIZE_8BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Countermeasure Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKEY_A {
    #[doc = "14: This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    PASSWD = 14,
}
impl From<CKEY_A> for u8 {
    #[inline(always)]
    fn from(variant: CKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKEY` reader - Countermeasure Key"]
pub struct CKEY_R(crate::FieldReader<u8, CKEY_A>);
impl CKEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CKEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKEY_A> {
        match self.bits {
            14 => Some(CKEY_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        **self == CKEY_A::PASSWD
    }
}
impl core::ops::Deref for CKEY_R {
    type Target = crate::FieldReader<u8, CKEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKEY` writer - Countermeasure Key"]
pub struct CKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(CKEY_A::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Processing Mode"]
    #[inline(always)]
    pub fn cipher(&self) -> CIPHER_R {
        CIPHER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline(always)]
    pub fn gtagen(&self) -> GTAGEN_R {
        GTAGEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DUALBUFF_R {
        DUALBUFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline(always)]
    pub fn procdly(&self) -> PROCDLY_R {
        PROCDLY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline(always)]
    pub fn opmod(&self) -> OPMOD_R {
        OPMOD_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&self) -> LOD_R {
        LOD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline(always)]
    pub fn cfbs(&self) -> CFBS_R {
        CFBS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline(always)]
    pub fn ckey(&self) -> CKEY_R {
        CKEY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Processing Mode"]
    #[inline(always)]
    pub fn cipher(&mut self) -> CIPHER_W {
        CIPHER_W { w: self }
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline(always)]
    pub fn gtagen(&mut self) -> GTAGEN_W {
        GTAGEN_W { w: self }
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&mut self) -> DUALBUFF_W {
        DUALBUFF_W { w: self }
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline(always)]
    pub fn procdly(&mut self) -> PROCDLY_W {
        PROCDLY_W { w: self }
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline(always)]
    pub fn smod(&mut self) -> SMOD_W {
        SMOD_W { w: self }
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W {
        KEYSIZE_W { w: self }
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline(always)]
    pub fn opmod(&mut self) -> OPMOD_W {
        OPMOD_W { w: self }
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&mut self) -> LOD_W {
        LOD_W { w: self }
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline(always)]
    pub fn cfbs(&mut self) -> CFBS_W {
        CFBS_W { w: self }
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline(always)]
    pub fn ckey(&mut self) -> CKEY_W {
        CKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_mr](index.html) module"]
pub struct AES_MR_SPEC;
impl crate::RegisterSpec for AES_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_mr::R](R) reader structure"]
impl crate::Readable for AES_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_mr::W](W) writer structure"]
impl crate::Writable for AES_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AES_MR to value 0"]
impl crate::Resettable for AES_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
