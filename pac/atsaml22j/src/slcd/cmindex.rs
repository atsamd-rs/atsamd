#[doc = "Register `CMINDEX` reader"]
pub struct R(crate::R<CMINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMINDEX` writer"]
pub struct W(crate::W<CMINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMINDEX_SPEC>;
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
impl From<crate::W<CMINDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINDEX` reader - SEG Line Index"]
pub type SINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINDEX` writer - SEG Line Index"]
pub type SINDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CMINDEX_SPEC, u8, u8, 6, O>;
#[doc = "Field `CINDEX` reader - COM Line Index"]
pub type CINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CINDEX` writer - COM Line Index"]
pub type CINDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CMINDEX_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:5 - SEG Line Index"]
    #[inline(always)]
    pub fn sindex(&self) -> SINDEX_R {
        SINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - COM Line Index"]
    #[inline(always)]
    pub fn cindex(&self) -> CINDEX_R {
        CINDEX_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SEG Line Index"]
    #[inline(always)]
    #[must_use]
    pub fn sindex(&mut self) -> SINDEX_W<0> {
        SINDEX_W::new(self)
    }
    #[doc = "Bits 8:10 - COM Line Index"]
    #[inline(always)]
    #[must_use]
    pub fn cindex(&mut self) -> CINDEX_W<8> {
        CINDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Character Mapping SEG/COM Index\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmindex](index.html) module"]
pub struct CMINDEX_SPEC;
impl crate::RegisterSpec for CMINDEX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cmindex::R](R) reader structure"]
impl crate::Readable for CMINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmindex::W](W) writer structure"]
impl crate::Writable for CMINDEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMINDEX to value 0"]
impl crate::Resettable for CMINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
