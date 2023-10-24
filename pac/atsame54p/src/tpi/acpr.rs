#[doc = "Register `ACPR` reader"]
pub type R = crate::R<ACPR_SPEC>;
#[doc = "Register `ACPR` writer"]
pub type W = crate::W<ACPR_SPEC>;
#[doc = "Field `PRESCALER` reader - "]
pub type PRESCALER_R = crate::FieldReader<u16>;
#[doc = "Field `PRESCALER` writer - "]
pub type PRESCALER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<ACPR_SPEC, 0> {
        PRESCALER_W::new(self)
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
#[doc = "Asynchronous Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACPR_SPEC;
impl crate::RegisterSpec for ACPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acpr::R`](R) reader structure"]
impl crate::Readable for ACPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acpr::W`](W) writer structure"]
impl crate::Writable for ACPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACPR to value 0"]
impl crate::Resettable for ACPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
