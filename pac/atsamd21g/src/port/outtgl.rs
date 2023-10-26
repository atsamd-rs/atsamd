#[doc = "Register `OUTTGL%s` reader"]
pub type R = crate::R<OUTTGL_SPEC>;
#[doc = "Register `OUTTGL%s` writer"]
pub type W = crate::W<OUTTGL_SPEC>;
#[doc = "Field `OUTTGL` reader - Port Data Output Value Toggle"]
pub type OUTTGL_R = crate::FieldReader<u32>;
#[doc = "Field `OUTTGL` writer - Port Data Output Value Toggle"]
pub type OUTTGL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value Toggle"]
    #[inline(always)]
    pub fn outtgl(&self) -> OUTTGL_R {
        OUTTGL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn outtgl(&mut self) -> OUTTGL_W<OUTTGL_SPEC, 0> {
        OUTTGL_W::new(self)
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
#[doc = "Data Output Value Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outtgl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outtgl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTTGL_SPEC;
impl crate::RegisterSpec for OUTTGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outtgl::R`](R) reader structure"]
impl crate::Readable for OUTTGL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`outtgl::W`](W) writer structure"]
impl crate::Writable for OUTTGL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTTGL%s to value 0"]
impl crate::Resettable for OUTTGL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
