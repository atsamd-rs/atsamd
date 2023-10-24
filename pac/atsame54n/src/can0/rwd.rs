#[doc = "Register `RWD` reader"]
pub type R = crate::R<RWD_SPEC>;
#[doc = "Register `RWD` writer"]
pub type W = crate::W<RWD_SPEC>;
#[doc = "Field `WDC` reader - Watchdog Configuration"]
pub type WDC_R = crate::FieldReader;
#[doc = "Field `WDC` writer - Watchdog Configuration"]
pub type WDC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WDV` reader - Watchdog Value"]
pub type WDV_R = crate::FieldReader;
#[doc = "Field `WDV` writer - Watchdog Value"]
pub type WDV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Watchdog Configuration"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog Value"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WDC_W<RWD_SPEC, 0> {
        WDC_W::new(self)
    }
    #[doc = "Bits 8:15 - Watchdog Value"]
    #[inline(always)]
    #[must_use]
    pub fn wdv(&mut self) -> WDV_W<RWD_SPEC, 8> {
        WDV_W::new(self)
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
#[doc = "RAM Watchdog\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RWD_SPEC;
impl crate::RegisterSpec for RWD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rwd::R`](R) reader structure"]
impl crate::Readable for RWD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rwd::W`](W) writer structure"]
impl crate::Writable for RWD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RWD to value 0"]
impl crate::Resettable for RWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
