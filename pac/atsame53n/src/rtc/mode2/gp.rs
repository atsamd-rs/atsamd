#[doc = "Register `GP[%s]` reader"]
pub type R = crate::R<GP_SPEC>;
#[doc = "Register `GP[%s]` writer"]
pub type W = crate::W<GP_SPEC>;
#[doc = "Field `GP` reader - General Purpose"]
pub type GP_R = crate::FieldReader<u32>;
#[doc = "Field `GP` writer - General Purpose"]
pub type GP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - General Purpose"]
    #[inline(always)]
    pub fn gp(&self) -> GP_R {
        GP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General Purpose"]
    #[inline(always)]
    #[must_use]
    pub fn gp(&mut self) -> GP_W<GP_SPEC, 0> {
        GP_W::new(self)
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
#[doc = "General Purpose\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_SPEC;
impl crate::RegisterSpec for GP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp::R`](R) reader structure"]
impl crate::Readable for GP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp::W`](W) writer structure"]
impl crate::Writable for GP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GP[%s]
to value 0"]
impl crate::Resettable for GP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
