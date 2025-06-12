#[doc = "Register `TPFCP` reader"]
pub type R = crate::R<TpfcpSpec>;
#[doc = "Register `TPFCP` writer"]
pub type W = crate::W<TpfcpSpec>;
#[doc = "Field `PEV` reader - Priority Enable Vector"]
pub type PevR = crate::FieldReader;
#[doc = "Field `PEV` writer - Priority Enable Vector"]
pub type PevW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PQ` reader - Pause Quantum"]
pub type PqR = crate::FieldReader;
#[doc = "Field `PQ` writer - Pause Quantum"]
pub type PqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Priority Enable Vector"]
    #[inline(always)]
    pub fn pev(&self) -> PevR {
        PevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Pause Quantum"]
    #[inline(always)]
    pub fn pq(&self) -> PqR {
        PqR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority Enable Vector"]
    #[inline(always)]
    pub fn pev(&mut self) -> PevW<TpfcpSpec> {
        PevW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Pause Quantum"]
    #[inline(always)]
    pub fn pq(&mut self) -> PqW<TpfcpSpec> {
        PqW::new(self, 8)
    }
}
#[doc = "Transmit PFC Pause Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpfcp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpfcp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpfcpSpec;
impl crate::RegisterSpec for TpfcpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpfcp::R`](R) reader structure"]
impl crate::Readable for TpfcpSpec {}
#[doc = "`write(|w| ..)` method takes [`tpfcp::W`](W) writer structure"]
impl crate::Writable for TpfcpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPFCP to value 0"]
impl crate::Resettable for TpfcpSpec {}
