#[doc = "Register `EXTCTRL` reader"]
pub type R = crate::R<EXTCTRL_SPEC>;
#[doc = "Register `EXTCTRL` writer"]
pub type W = crate::W<EXTCTRL_SPEC>;
#[doc = "Field `SETDIS` reader - External Reset Disable"]
pub type SETDIS_R = crate::BitReader;
#[doc = "Field `SETDIS` writer - External Reset Disable"]
pub type SETDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - External Reset Disable"]
    #[inline(always)]
    pub fn setdis(&self) -> SETDIS_R {
        SETDIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Reset Disable"]
    #[inline(always)]
    #[must_use]
    pub fn setdis(&mut self) -> SETDIS_W<EXTCTRL_SPEC, 0> {
        SETDIS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "External Reset Controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTCTRL_SPEC;
impl crate::RegisterSpec for EXTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`extctrl::R`](R) reader structure"]
impl crate::Readable for EXTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extctrl::W`](W) writer structure"]
impl crate::Writable for EXTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTCTRL to value 0"]
impl crate::Resettable for EXTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
