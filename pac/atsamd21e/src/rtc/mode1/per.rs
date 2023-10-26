#[doc = "Register `PER` reader"]
pub type R = crate::R<PER_SPEC>;
#[doc = "Register `PER` writer"]
pub type W = crate::W<PER_SPEC>;
#[doc = "Field `PER` reader - Counter Period"]
pub type PER_R = crate::FieldReader<u16>;
#[doc = "Field `PER` writer - Counter Period"]
pub type PER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Period"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<PER_SPEC, 0> {
        PER_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MODE1 Counter Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PER_SPEC;
impl crate::RegisterSpec for PER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`per::R`](R) reader structure"]
impl crate::Readable for PER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`per::W`](W) writer structure"]
impl crate::Writable for PER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PER to value 0"]
impl crate::Resettable for PER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
