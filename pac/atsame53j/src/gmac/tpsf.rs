#[doc = "Register `TPSF` reader"]
pub struct R(crate::R<TPSF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPSF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPSF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPSF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPSF` writer"]
pub struct W(crate::W<TPSF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPSF_SPEC>;
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
impl From<crate::W<TPSF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPSF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPB1ADR` reader - TX packet buffer address"]
pub type TPB1ADR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TPB1ADR` writer - TX packet buffer address"]
pub type TPB1ADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPSF_SPEC, u16, u16, 10, O>;
#[doc = "Field `ENTXP` reader - Enable TX partial store and forward operation"]
pub type ENTXP_R = crate::BitReader<bool>;
#[doc = "Field `ENTXP` writer - Enable TX partial store and forward operation"]
pub type ENTXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TPSF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - TX packet buffer address"]
    #[inline(always)]
    pub fn tpb1adr(&self) -> TPB1ADR_R {
        TPB1ADR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn entxp(&self) -> ENTXP_R {
        ENTXP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - TX packet buffer address"]
    #[inline(always)]
    #[must_use]
    pub fn tpb1adr(&mut self) -> TPB1ADR_W<0> {
        TPB1ADR_W::new(self)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    #[must_use]
    pub fn entxp(&mut self) -> ENTXP_W<31> {
        ENTXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX partial store and forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpsf](index.html) module"]
pub struct TPSF_SPEC;
impl crate::RegisterSpec for TPSF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpsf::R](R) reader structure"]
impl crate::Readable for TPSF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpsf::W](W) writer structure"]
impl crate::Writable for TPSF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPSF to value 0x03ff"]
impl crate::Resettable for TPSF_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
