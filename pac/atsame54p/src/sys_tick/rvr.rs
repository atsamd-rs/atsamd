#[doc = "Register `RVR` reader"]
pub type R = crate::R<RVR_SPEC>;
#[doc = "Register `RVR` writer"]
pub type W = crate::W<RVR_SPEC>;
#[doc = "Field `RELOAD` reader - Value to load into the SysTick Current Value Register when the counter reaches 0"]
pub type RELOAD_R = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - Value to load into the SysTick Current Value Register when the counter reaches 0"]
pub type RELOAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<RVR_SPEC, 0> {
        RELOAD_W::new(self)
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
#[doc = "SysTick Reload Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RVR_SPEC;
impl crate::RegisterSpec for RVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rvr::R`](R) reader structure"]
impl crate::Readable for RVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rvr::W`](W) writer structure"]
impl crate::Writable for RVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RVR to value 0"]
impl crate::Resettable for RVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
