#[doc = "Register `DESCADD` reader"]
pub type R = crate::R<DESCADD_SPEC>;
#[doc = "Register `DESCADD` writer"]
pub type W = crate::W<DESCADD_SPEC>;
#[doc = "Field `DESCADD` reader - Descriptor Address Value"]
pub type DESCADD_R = crate::FieldReader<u32>;
#[doc = "Field `DESCADD` writer - Descriptor Address Value"]
pub type DESCADD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Descriptor Address Value"]
    #[inline(always)]
    pub fn descadd(&self) -> DESCADD_R {
        DESCADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Address Value"]
    #[inline(always)]
    #[must_use]
    pub fn descadd(&mut self) -> DESCADD_W<DESCADD_SPEC, 0> {
        DESCADD_W::new(self)
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
#[doc = "Descriptor Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descadd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descadd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESCADD_SPEC;
impl crate::RegisterSpec for DESCADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descadd::R`](R) reader structure"]
impl crate::Readable for DESCADD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`descadd::W`](W) writer structure"]
impl crate::Writable for DESCADD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESCADD to value 0"]
impl crate::Resettable for DESCADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
