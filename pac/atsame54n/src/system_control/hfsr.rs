#[doc = "Register `HFSR` reader"]
pub type R = crate::R<HFSR_SPEC>;
#[doc = "Register `HFSR` writer"]
pub type W = crate::W<HFSR_SPEC>;
#[doc = "Field `VECTTBL` reader - BusFault on a Vector Table read during exception processing"]
pub type VECTTBL_R = crate::BitReader;
#[doc = "Field `VECTTBL` writer - BusFault on a Vector Table read during exception processing"]
pub type VECTTBL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCED` reader - Forced Hard Fault"]
pub type FORCED_R = crate::BitReader;
#[doc = "Field `FORCED` writer - Forced Hard Fault"]
pub type FORCED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEBUGEVT` reader - Debug: always write 0"]
pub type DEBUGEVT_R = crate::BitReader;
#[doc = "Field `DEBUGEVT` writer - Debug: always write 0"]
pub type DEBUGEVT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - BusFault on a Vector Table read during exception processing"]
    #[inline(always)]
    pub fn vecttbl(&self) -> VECTTBL_R {
        VECTTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 30 - Forced Hard Fault"]
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Debug: always write 0"]
    #[inline(always)]
    pub fn debugevt(&self) -> DEBUGEVT_R {
        DEBUGEVT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BusFault on a Vector Table read during exception processing"]
    #[inline(always)]
    #[must_use]
    pub fn vecttbl(&mut self) -> VECTTBL_W<HFSR_SPEC, 1> {
        VECTTBL_W::new(self)
    }
    #[doc = "Bit 30 - Forced Hard Fault"]
    #[inline(always)]
    #[must_use]
    pub fn forced(&mut self) -> FORCED_W<HFSR_SPEC, 30> {
        FORCED_W::new(self)
    }
    #[doc = "Bit 31 - Debug: always write 0"]
    #[inline(always)]
    #[must_use]
    pub fn debugevt(&mut self) -> DEBUGEVT_W<HFSR_SPEC, 31> {
        DEBUGEVT_W::new(self)
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
#[doc = "HardFault Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFSR_SPEC;
impl crate::RegisterSpec for HFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfsr::R`](R) reader structure"]
impl crate::Readable for HFSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfsr::W`](W) writer structure"]
impl crate::Writable for HFSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFSR to value 0"]
impl crate::Resettable for HFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
