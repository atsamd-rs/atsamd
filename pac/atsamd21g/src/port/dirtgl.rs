#[doc = "Register `DIRTGL%s` reader"]
pub type R = crate::R<DIRTGL_SPEC>;
#[doc = "Register `DIRTGL%s` writer"]
pub type W = crate::W<DIRTGL_SPEC>;
#[doc = "Field `DIRTGL` reader - Port Data Direction Toggle"]
pub type DIRTGL_R = crate::FieldReader<u32>;
#[doc = "Field `DIRTGL` writer - Port Data Direction Toggle"]
pub type DIRTGL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Toggle"]
    #[inline(always)]
    pub fn dirtgl(&self) -> DIRTGL_R {
        DIRTGL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn dirtgl(&mut self) -> DIRTGL_W<DIRTGL_SPEC, 0> {
        DIRTGL_W::new(self)
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
#[doc = "Data Direction Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dirtgl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dirtgl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRTGL_SPEC;
impl crate::RegisterSpec for DIRTGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirtgl::R`](R) reader structure"]
impl crate::Readable for DIRTGL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dirtgl::W`](W) writer structure"]
impl crate::Writable for DIRTGL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRTGL%s to value 0"]
impl crate::Resettable for DIRTGL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
