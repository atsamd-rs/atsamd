#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<INTENSET_SPEC>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<INTENSET_SPEC>;
#[doc = "Field `ALARM0` reader - Alarm 0 Interrupt Enable"]
pub type ALARM0_R = crate::BitReader;
#[doc = "Field `ALARM0` writer - Alarm 0 Interrupt Enable"]
pub type ALARM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYNCRDY` reader - Synchronization Ready Interrupt Enable"]
pub type SYNCRDY_R = crate::BitReader;
#[doc = "Field `SYNCRDY` writer - Synchronization Ready Interrupt Enable"]
pub type SYNCRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub type OVF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Alarm 0 Interrupt Enable"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    pub fn syncrdy(&self) -> SYNCRDY_R {
        SYNCRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0(&mut self) -> ALARM0_W<INTENSET_SPEC, 0> {
        ALARM0_W::new(self)
    }
    #[doc = "Bit 6 - Synchronization Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncrdy(&mut self) -> SYNCRDY_W<INTENSET_SPEC, 6> {
        SYNCRDY_W::new(self)
    }
    #[doc = "Bit 7 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<INTENSET_SPEC, 7> {
        OVF_W::new(self)
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
#[doc = "MODE2 Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
