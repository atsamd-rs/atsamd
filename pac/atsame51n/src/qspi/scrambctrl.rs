#[doc = "Register `SCRAMBCTRL` reader"]
pub type R = crate::R<SCRAMBCTRL_SPEC>;
#[doc = "Register `SCRAMBCTRL` writer"]
pub type W = crate::W<SCRAMBCTRL_SPEC>;
#[doc = "Field `ENABLE` reader - Scrambling/Unscrambling Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Scrambling/Unscrambling Enable"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RANDOMDIS` reader - Scrambling/Unscrambling Random Value Disable"]
pub type RANDOMDIS_R = crate::BitReader;
#[doc = "Field `RANDOMDIS` writer - Scrambling/Unscrambling Random Value Disable"]
pub type RANDOMDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn randomdis(&self) -> RANDOMDIS_R {
        RANDOMDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<SCRAMBCTRL_SPEC, 0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    #[must_use]
    pub fn randomdis(&mut self) -> RANDOMDIS_W<SCRAMBCTRL_SPEC, 1> {
        RANDOMDIS_W::new(self)
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
#[doc = "Scrambling Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scrambctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scrambctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRAMBCTRL_SPEC;
impl crate::RegisterSpec for SCRAMBCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scrambctrl::R`](R) reader structure"]
impl crate::Readable for SCRAMBCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scrambctrl::W`](W) writer structure"]
impl crate::Writable for SCRAMBCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCRAMBCTRL to value 0"]
impl crate::Resettable for SCRAMBCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
