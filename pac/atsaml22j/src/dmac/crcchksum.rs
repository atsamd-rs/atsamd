#[doc = "Register `CRCCHKSUM` reader"]
pub struct R(crate::R<CRCCHKSUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCCHKSUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCCHKSUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCCHKSUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCCHKSUM` writer"]
pub struct W(crate::W<CRCCHKSUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCCHKSUM_SPEC>;
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
impl From<crate::W<CRCCHKSUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCCHKSUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCCHKSUM` reader - CRC Checksum"]
pub type CRCCHKSUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRCCHKSUM` writer - CRC Checksum"]
pub type CRCCHKSUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CRCCHKSUM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CRC Checksum"]
    #[inline(always)]
    pub fn crcchksum(&self) -> CRCCHKSUM_R {
        CRCCHKSUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC Checksum"]
    #[inline(always)]
    #[must_use]
    pub fn crcchksum(&mut self) -> CRCCHKSUM_W<0> {
        CRCCHKSUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Checksum\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcchksum](index.html) module"]
pub struct CRCCHKSUM_SPEC;
impl crate::RegisterSpec for CRCCHKSUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcchksum::R](R) reader structure"]
impl crate::Readable for CRCCHKSUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcchksum::W](W) writer structure"]
impl crate::Writable for CRCCHKSUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCCHKSUM to value 0"]
impl crate::Resettable for CRCCHKSUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
