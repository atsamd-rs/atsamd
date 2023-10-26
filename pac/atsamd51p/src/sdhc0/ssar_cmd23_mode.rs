#[doc = "Register `SSAR_CMD23_MODE` reader"]
pub type R = crate::R<SSAR_CMD23_MODE_SPEC>;
#[doc = "Register `SSAR_CMD23_MODE` writer"]
pub type W = crate::W<SSAR_CMD23_MODE_SPEC>;
#[doc = "Field `ARG2` reader - Argument 2"]
pub type ARG2_R = crate::FieldReader<u32>;
#[doc = "Field `ARG2` writer - Argument 2"]
pub type ARG2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Argument 2"]
    #[inline(always)]
    pub fn arg2(&self) -> ARG2_R {
        ARG2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Argument 2"]
    #[inline(always)]
    #[must_use]
    pub fn arg2(&mut self) -> ARG2_W<SSAR_CMD23_MODE_SPEC, 0> {
        ARG2_W::new(self)
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
#[doc = "SDMA System Address / Argument 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssar_cmd23_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssar_cmd23_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSAR_CMD23_MODE_SPEC;
impl crate::RegisterSpec for SSAR_CMD23_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssar_cmd23_mode::R`](R) reader structure"]
impl crate::Readable for SSAR_CMD23_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssar_cmd23_mode::W`](W) writer structure"]
impl crate::Writable for SSAR_CMD23_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSAR_CMD23_MODE to value 0"]
impl crate::Resettable for SSAR_CMD23_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
