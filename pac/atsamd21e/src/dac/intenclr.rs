#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<INTENCLR_SPEC>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<INTENCLR_SPEC>;
#[doc = "Field `UNDERRUN` reader - Underrun Interrupt Enable"]
pub type UNDERRUN_R = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - Underrun Interrupt Enable"]
pub type UNDERRUN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMPTY` reader - Data Buffer Empty Interrupt Enable"]
pub type EMPTY_R = crate::BitReader;
#[doc = "Field `EMPTY` writer - Data Buffer Empty Interrupt Enable"]
pub type EMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready Interrupt Enable"]
pub type SYNCRDY_R = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready Interrupt Enable"]
pub type SYNCRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SYNCRDY_R {
        SYNCRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Underrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn underrun(&mut self) -> UNDERRUN_W<INTENCLR_SPEC, 0> {
        UNDERRUN_W::new(self)
    }
    #[doc = "Bit 1 - Data Buffer Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EMPTY_W<INTENCLR_SPEC, 1> {
        EMPTY_W::new(self)
    }
    #[doc = "Bit 2 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncrdy(&mut self) -> SYNCRDY_W<INTENCLR_SPEC, 2> {
        SYNCRDY_W::new(self)
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
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
