#[doc = "Register `DIRCLR%s` reader"]
pub type R = crate::R<DIRCLR_SPEC>;
#[doc = "Register `DIRCLR%s` writer"]
pub type W = crate::W<DIRCLR_SPEC>;
#[doc = "Field `DIRCLR` reader - Port Data Direction Clear"]
pub type DIRCLR_R = crate::FieldReader<u32>;
#[doc = "Field `DIRCLR` writer - Port Data Direction Clear"]
pub type DIRCLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Clear"]
    #[inline(always)]
    pub fn dirclr(&self) -> DIRCLR_R {
        DIRCLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dirclr(&mut self) -> DIRCLR_W<DIRCLR_SPEC, 0> {
        DIRCLR_W::new(self)
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
#[doc = "Data Direction Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRCLR_SPEC;
impl crate::RegisterSpec for DIRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirclr::R`](R) reader structure"]
impl crate::Readable for DIRCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dirclr::W`](W) writer structure"]
impl crate::Writable for DIRCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRCLR%s to value 0"]
impl crate::Resettable for DIRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
