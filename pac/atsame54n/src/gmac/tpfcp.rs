#[doc = "Register `TPFCP` reader"]
pub struct R(crate::R<TPFCP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPFCP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPFCP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPFCP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPFCP` writer"]
pub struct W(crate::W<TPFCP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPFCP_SPEC>;
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
impl From<crate::W<TPFCP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPFCP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEV` reader - Priority Enable Vector"]
pub type PEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PEV` writer - Priority Enable Vector"]
pub type PEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPFCP_SPEC, u8, u8, 8, O>;
#[doc = "Field `PQ` reader - Pause Quantum"]
pub type PQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PQ` writer - Pause Quantum"]
pub type PQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPFCP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Priority Enable Vector"]
    #[inline(always)]
    pub fn pev(&self) -> PEV_R {
        PEV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pause Quantum"]
    #[inline(always)]
    pub fn pq(&self) -> PQ_R {
        PQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority Enable Vector"]
    #[inline(always)]
    #[must_use]
    pub fn pev(&mut self) -> PEV_W<0> {
        PEV_W::new(self)
    }
    #[doc = "Bits 8:15 - Pause Quantum"]
    #[inline(always)]
    #[must_use]
    pub fn pq(&mut self) -> PQ_W<8> {
        PQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit PFC Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpfcp](index.html) module"]
pub struct TPFCP_SPEC;
impl crate::RegisterSpec for TPFCP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpfcp::R](R) reader structure"]
impl crate::Readable for TPFCP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpfcp::W](W) writer structure"]
impl crate::Writable for TPFCP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPFCP to value 0"]
impl crate::Resettable for TPFCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
