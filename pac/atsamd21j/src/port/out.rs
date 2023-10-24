#[doc = "Register `OUT%s` reader"]
pub type R = crate::R<OUT_SPEC>;
#[doc = "Register `OUT%s` writer"]
pub type W = crate::W<OUT_SPEC>;
#[doc = "Field `OUT` reader - Port Data Output Value"]
pub type OUT_R = crate::FieldReader<u32>;
#[doc = "Field `OUT` writer - Port Data Output Value"]
pub type OUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn out(&mut self) -> OUT_W<OUT_SPEC, 0> {
        OUT_W::new(self)
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
#[doc = "Data Output Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT%s to value 0"]
impl crate::Resettable for OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
