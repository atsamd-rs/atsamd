#[doc = "Register `TPQ` reader"]
pub type R = crate::R<TPQ_SPEC>;
#[doc = "Register `TPQ` writer"]
pub type W = crate::W<TPQ_SPEC>;
#[doc = "Field `TPQ` reader - Transmit Pause Quantum"]
pub type TPQ_R = crate::FieldReader<u16>;
#[doc = "Field `TPQ` writer - Transmit Pause Quantum"]
pub type TPQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit Pause Quantum"]
    #[inline(always)]
    pub fn tpq(&self) -> TPQ_R {
        TPQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Pause Quantum"]
    #[inline(always)]
    #[must_use]
    pub fn tpq(&mut self) -> TPQ_W<TPQ_SPEC, 0> {
        TPQ_W::new(self)
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
#[doc = "Transmit Pause Quantum Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPQ_SPEC;
impl crate::RegisterSpec for TPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpq::R`](R) reader structure"]
impl crate::Readable for TPQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpq::W`](W) writer structure"]
impl crate::Writable for TPQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPQ to value 0xffff"]
impl crate::Resettable for TPQ_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
