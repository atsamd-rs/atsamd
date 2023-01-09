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
pub type AESMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AESMODE` writer - AES Modes of operation"]
pub type AESMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 3, O>;
#[doc = "Field `CFBS` reader - CFB Types"]
pub type CFBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFBS` writer - CFB Types"]
pub type CFBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 3, O>;
#[doc = "Field `KEYSIZE` reader - Keysize"]
pub type KEYSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEYSIZE` writer - Keysize"]
pub type KEYSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLA_SPEC, u8, u8, 2, O>;
#[doc = "Field `CIPHER` reader - Cipher mode"]
pub type CIPHER_R = crate::BitReader<bool>;
#[doc = "Field `CIPHER` writer - Cipher mode"]
pub type CIPHER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `STARTMODE` reader - Start mode"]
pub type STARTMODE_R = crate::BitReader<bool>;
#[doc = "Field `STARTMODE` writer - Start mode"]
pub type STARTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `LOD` reader - LOD Enable"]
pub type LOD_R = crate::BitReader<bool>;
#[doc = "Field `LOD` writer - LOD Enable"]
pub type LOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `KEYGEN` reader - Last key generation"]
pub type KEYGEN_R = crate::BitReader<bool>;
#[doc = "Field `KEYGEN` writer - Last key generation"]
pub type KEYGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `XORKEY` reader - Xor Key operation"]
pub type XORKEY_R = crate::BitReader<bool>;
#[doc = "Field `XORKEY` writer - Xor Key operation"]
pub type XORKEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLA_SPEC, bool, O>;
#[doc = "Field `CTYPE` reader - Counter measure types"]
pub type CTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTYPE` writer - Counter measure types"]
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
    #[doc = "Bits 5:7 - CFB Types"]
    #[inline(always)]
    pub fn cfbs(&self) -> CFBS_R {
        CFBS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Keysize"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Cipher mode"]
    #[inline(always)]
    pub fn cipher(&self) -> CIPHER_R {
        CIPHER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Start mode"]
    #[inline(always)]
    pub fn startmode(&self) -> STARTMODE_R {
        STARTMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LOD Enable"]
    #[inline(always)]
    pub fn lod(&self) -> LOD_R {
        LOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Last key generation"]
    #[inline(always)]
    pub fn keygen(&self) -> KEYGEN_R {
        KEYGEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Xor Key operation"]
    #[inline(always)]
    pub fn xorkey(&self) -> XORKEY_R {
        XORKEY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Counter measure types"]
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
    #[doc = "Bits 5:7 - CFB Types"]
    #[inline(always)]
    #[must_use]
    pub fn cfbs(&mut self) -> CFBS_W<5> {
        CFBS_W::new(self)
    }
    #[doc = "Bits 8:9 - Keysize"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<8> {
        KEYSIZE_W::new(self)
    }
    #[doc = "Bit 10 - Cipher mode"]
    #[inline(always)]
    #[must_use]
    pub fn cipher(&mut self) -> CIPHER_W<10> {
        CIPHER_W::new(self)
    }
    #[doc = "Bit 11 - Start mode"]
    #[inline(always)]
    #[must_use]
    pub fn startmode(&mut self) -> STARTMODE_W<11> {
        STARTMODE_W::new(self)
    }
    #[doc = "Bit 12 - LOD Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lod(&mut self) -> LOD_W<12> {
        LOD_W::new(self)
    }
    #[doc = "Bit 13 - Last key generation"]
    #[inline(always)]
    #[must_use]
    pub fn keygen(&mut self) -> KEYGEN_W<13> {
        KEYGEN_W::new(self)
    }
    #[doc = "Bit 14 - Xor Key operation"]
    #[inline(always)]
    #[must_use]
    pub fn xorkey(&mut self) -> XORKEY_W<14> {
        XORKEY_W::new(self)
    }
    #[doc = "Bits 16:19 - Counter measure types"]
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
