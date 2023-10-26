#[doc = "Register `COMP[%s]` reader"]
pub type R = crate::R<COMP_SPEC>;
#[doc = "Register `COMP[%s]` writer"]
pub type W = crate::W<COMP_SPEC>;
#[doc = "Field `COMP` reader - Compare Value"]
pub type COMP_R = crate::FieldReader<u32>;
#[doc = "Field `COMP` writer - Compare Value"]
pub type COMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<COMP_SPEC, 0> {
        COMP_W::new(self)
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
#[doc = "MODE0 Compare n Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP_SPEC;
impl crate::RegisterSpec for COMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp::R`](R) reader structure"]
impl crate::Readable for COMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`comp::W`](W) writer structure"]
impl crate::Writable for COMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP[%s]
to value 0"]
impl crate::Resettable for COMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
