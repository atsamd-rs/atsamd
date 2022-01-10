#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WBDIS` reader - Write Back Disable"]
pub struct WBDIS_R(crate::FieldReader<bool, bool>);
impl WBDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WBDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WBDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WBDIS` writer - Write Back Disable"]
pub struct WBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WBDIS_W<'a> {
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
#[doc = "Field `EOMDIS` reader - End of Monitoring Disable"]
pub struct EOMDIS_R(crate::FieldReader<bool, bool>);
impl EOMDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOMDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOMDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOMDIS` writer - End of Monitoring Disable"]
pub struct EOMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOMDIS_W<'a> {
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
#[doc = "Field `SLBDIS` reader - Secondary List Branching Disable"]
pub struct SLBDIS_R(crate::FieldReader<bool, bool>);
impl SLBDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLBDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLBDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLBDIS` writer - Secondary List Branching Disable"]
pub struct SLBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLBDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `BBC` reader - Bus Burden Control"]
pub struct BBC_R(crate::FieldReader<u8, u8>);
impl BBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBC` writer - Bus Burden Control"]
pub struct BBC_W<'a> {
    w: &'a mut W,
}
impl<'a> BBC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `ASCD` reader - Automatic Switch To Compare Digest"]
pub struct ASCD_R(crate::FieldReader<bool, bool>);
impl ASCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ASCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASCD` writer - Automatic Switch To Compare Digest"]
pub struct ASCD_W<'a> {
    w: &'a mut W,
}
impl<'a> ASCD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DUALBUFF` reader - Dual Input Buffer"]
pub struct DUALBUFF_R(crate::FieldReader<bool, bool>);
impl DUALBUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUALBUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUALBUFF_R {
    type Target = crate::FieldReader<bool, bool>;
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `UIHASH` reader - User Initial Hash Value"]
pub struct UIHASH_R(crate::FieldReader<bool, bool>);
impl UIHASH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UIHASH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UIHASH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UIHASH` writer - User Initial Hash Value"]
pub struct UIHASH_W<'a> {
    w: &'a mut W,
}
impl<'a> UIHASH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "User SHA Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UALGO_A {
    #[doc = "0: SHA1 Algorithm"]
    SHA1 = 0,
    #[doc = "1: SHA256 Algorithm"]
    SHA256 = 1,
    #[doc = "4: SHA224 Algorithm"]
    SHA224 = 4,
}
impl From<UALGO_A> for u8 {
    #[inline(always)]
    fn from(variant: UALGO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UALGO` reader - User SHA Algorithm"]
pub struct UALGO_R(crate::FieldReader<u8, UALGO_A>);
impl UALGO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UALGO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UALGO_A> {
        match self.bits {
            0 => Some(UALGO_A::SHA1),
            1 => Some(UALGO_A::SHA256),
            4 => Some(UALGO_A::SHA224),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        **self == UALGO_A::SHA1
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        **self == UALGO_A::SHA256
    }
    #[doc = "Checks if the value of the field is `SHA224`"]
    #[inline(always)]
    pub fn is_sha224(&self) -> bool {
        **self == UALGO_A::SHA224
    }
}
impl core::ops::Deref for UALGO_R {
    type Target = crate::FieldReader<u8, UALGO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UALGO` writer - User SHA Algorithm"]
pub struct UALGO_W<'a> {
    w: &'a mut W,
}
impl<'a> UALGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UALGO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SHA1 Algorithm"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut W {
        self.variant(UALGO_A::SHA1)
    }
    #[doc = "SHA256 Algorithm"]
    #[inline(always)]
    pub fn sha256(self) -> &'a mut W {
        self.variant(UALGO_A::SHA256)
    }
    #[doc = "SHA224 Algorithm"]
    #[inline(always)]
    pub fn sha224(self) -> &'a mut W {
        self.variant(UALGO_A::SHA224)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `HAPROT` reader - Region Hash Area Protection"]
pub struct HAPROT_R(crate::FieldReader<u8, u8>);
impl HAPROT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HAPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HAPROT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HAPROT` writer - Region Hash Area Protection"]
pub struct HAPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> HAPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `DAPROT` reader - Region Descriptor Area Protection"]
pub struct DAPROT_R(crate::FieldReader<u8, u8>);
impl DAPROT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAPROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAPROT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAPROT` writer - Region Descriptor Area Protection"]
pub struct DAPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline(always)]
    pub fn wbdis(&self) -> WBDIS_R {
        WBDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline(always)]
    pub fn eomdis(&self) -> EOMDIS_R {
        EOMDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline(always)]
    pub fn slbdis(&self) -> SLBDIS_R {
        SLBDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline(always)]
    pub fn bbc(&self) -> BBC_R {
        BBC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline(always)]
    pub fn ascd(&self) -> ASCD_R {
        ASCD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DUALBUFF_R {
        DUALBUFF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline(always)]
    pub fn uihash(&self) -> UIHASH_R {
        UIHASH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline(always)]
    pub fn ualgo(&self) -> UALGO_R {
        UALGO_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:21 - Region Hash Area Protection"]
    #[inline(always)]
    pub fn haprot(&self) -> HAPROT_R {
        HAPROT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Region Descriptor Area Protection"]
    #[inline(always)]
    pub fn daprot(&self) -> DAPROT_R {
        DAPROT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline(always)]
    pub fn wbdis(&mut self) -> WBDIS_W {
        WBDIS_W { w: self }
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline(always)]
    pub fn eomdis(&mut self) -> EOMDIS_W {
        EOMDIS_W { w: self }
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline(always)]
    pub fn slbdis(&mut self) -> SLBDIS_W {
        SLBDIS_W { w: self }
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline(always)]
    pub fn bbc(&mut self) -> BBC_W {
        BBC_W { w: self }
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline(always)]
    pub fn ascd(&mut self) -> ASCD_W {
        ASCD_W { w: self }
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&mut self) -> DUALBUFF_W {
        DUALBUFF_W { w: self }
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline(always)]
    pub fn uihash(&mut self) -> UIHASH_W {
        UIHASH_W { w: self }
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline(always)]
    pub fn ualgo(&mut self) -> UALGO_W {
        UALGO_W { w: self }
    }
    #[doc = "Bits 16:21 - Region Hash Area Protection"]
    #[inline(always)]
    pub fn haprot(&mut self) -> HAPROT_W {
        HAPROT_W { w: self }
    }
    #[doc = "Bits 24:29 - Region Descriptor Area Protection"]
    #[inline(always)]
    pub fn daprot(&mut self) -> DAPROT_W {
        DAPROT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
