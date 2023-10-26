#[doc = "Register `TPFCP` reader"]
pub type R = crate::R<TPFCP_SPEC>;
#[doc = "Register `TPFCP` writer"]
pub type W = crate::W<TPFCP_SPEC>;
#[doc = "Field `PEV` reader - Priority Enable Vector"]
pub type PEV_R = crate::FieldReader;
#[doc = "Field `PEV` writer - Priority Enable Vector"]
pub type PEV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PQ` reader - Pause Quantum"]
pub type PQ_R = crate::FieldReader;
#[doc = "Field `PQ` writer - Pause Quantum"]
pub type PQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
    pub fn pev(&mut self) -> PEV_W<TPFCP_SPEC, 0> {
        PEV_W::new(self)
    }
    #[doc = "Bits 8:15 - Pause Quantum"]
    #[inline(always)]
    #[must_use]
    pub fn pq(&mut self) -> PQ_W<TPFCP_SPEC, 8> {
        PQ_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit PFC Pause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpfcp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpfcp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPFCP_SPEC;
impl crate::RegisterSpec for TPFCP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpfcp::R`](R) reader structure"]
impl crate::Readable for TPFCP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpfcp::W`](W) writer structure"]
impl crate::Writable for TPFCP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPFCP to value 0"]
impl crate::Resettable for TPFCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
