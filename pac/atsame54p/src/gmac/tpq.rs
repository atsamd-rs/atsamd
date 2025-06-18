#[doc = "Register `TPQ` reader"]
pub type R = crate::R<TpqSpec>;
#[doc = "Register `TPQ` writer"]
pub type W = crate::W<TpqSpec>;
#[doc = "Field `TPQ` reader - Transmit Pause Quantum"]
pub type TpqR = crate::FieldReader<u16>;
#[doc = "Field `TPQ` writer - Transmit Pause Quantum"]
pub type TpqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit Pause Quantum"]
    #[inline(always)]
    pub fn tpq(&self) -> TpqR {
        TpqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Pause Quantum"]
    #[inline(always)]
    pub fn tpq(&mut self) -> TpqW<TpqSpec> {
        TpqW::new(self, 0)
    }
}
#[doc = "Transmit Pause Quantum Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpqSpec;
impl crate::RegisterSpec for TpqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpq::R`](R) reader structure"]
impl crate::Readable for TpqSpec {}
#[doc = "`write(|w| ..)` method takes [`tpq::W`](W) writer structure"]
impl crate::Writable for TpqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPQ to value 0xffff"]
impl crate::Resettable for TpqSpec {
    const RESET_VALUE: u32 = 0xffff;
}
