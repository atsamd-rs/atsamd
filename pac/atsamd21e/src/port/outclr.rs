#[doc = "Register `OUTCLR%s` reader"]
pub type R = crate::R<OUTCLR_SPEC>;
#[doc = "Register `OUTCLR%s` writer"]
pub type W = crate::W<OUTCLR_SPEC>;
#[doc = "Field `OUTCLR` reader - Port Data Output Value Clear"]
pub type OUTCLR_R = crate::FieldReader<u32>;
#[doc = "Field `OUTCLR` writer - Port Data Output Value Clear"]
pub type OUTCLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value Clear"]
    #[inline(always)]
    pub fn outclr(&self) -> OUTCLR_R {
        OUTCLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value Clear"]
    #[inline(always)]
    #[must_use]
    pub fn outclr(&mut self) -> OUTCLR_W<OUTCLR_SPEC, 0> {
        OUTCLR_W::new(self)
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
#[doc = "Data Output Value Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTCLR_SPEC;
impl crate::RegisterSpec for OUTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outclr::R`](R) reader structure"]
impl crate::Readable for OUTCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`outclr::W`](W) writer structure"]
impl crate::Writable for OUTCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTCLR%s to value 0"]
impl crate::Resettable for OUTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
