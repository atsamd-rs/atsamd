#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCCMP` reader - Encryption Complete"]
pub type ENCCMP_R = crate::BitReader<bool>;
#[doc = "Field `ENCCMP` writer - Encryption Complete"]
pub type ENCCMP_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
#[doc = "Field `GFMCMP` reader - GF Multiplication Complete"]
pub type GFMCMP_R = crate::BitReader<bool>;
#[doc = "Field `GFMCMP` writer - GF Multiplication Complete"]
pub type GFMCMP_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTENSET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Encryption Complete"]
    #[inline(always)]
    pub fn enccmp(&self) -> ENCCMP_R {
        ENCCMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GF Multiplication Complete"]
    #[inline(always)]
    pub fn gfmcmp(&self) -> GFMCMP_R {
        GFMCMP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Encryption Complete"]
    #[inline(always)]
    #[must_use]
    pub fn enccmp(&mut self) -> ENCCMP_W<0> {
        ENCCMP_W::new(self)
    }
    #[doc = "Bit 1 - GF Multiplication Complete"]
    #[inline(always)]
    #[must_use]
    pub fn gfmcmp(&mut self) -> GFMCMP_W<1> {
        GFMCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
