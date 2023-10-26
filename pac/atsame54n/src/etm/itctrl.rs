#[doc = "Register `ITCTRL` reader"]
pub type R = crate::R<ITCTRL_SPEC>;
#[doc = "Register `ITCTRL` writer"]
pub type W = crate::W<ITCTRL_SPEC>;
#[doc = "Field `INTEGRATION` reader - "]
pub type INTEGRATION_R = crate::BitReader;
#[doc = "Field `INTEGRATION` writer - "]
pub type INTEGRATION_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn integration(&self) -> INTEGRATION_R {
        INTEGRATION_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn integration(&mut self) -> INTEGRATION_W<ITCTRL_SPEC, 0> {
        INTEGRATION_W::new(self)
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
#[doc = "ETM Integration Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITCTRL_SPEC;
impl crate::RegisterSpec for ITCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itctrl::R`](R) reader structure"]
impl crate::Readable for ITCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itctrl::W`](W) writer structure"]
impl crate::Writable for ITCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITCTRL to value 0"]
impl crate::Resettable for ITCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
