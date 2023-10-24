#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<INTENSET_SPEC>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<INTENSET_SPEC>;
#[doc = "Field `READY` reader - NVM Ready Interrupt Enable"]
pub type READY_R = crate::BitReader;
#[doc = "Field `READY` writer - NVM Ready Interrupt Enable"]
pub type READY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERROR` reader - Error Interrupt Enable"]
pub type ERROR_R = crate::BitReader;
#[doc = "Field `ERROR` writer - Error Interrupt Enable"]
pub type ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - NVM Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVM Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<INTENSET_SPEC, 0> {
        READY_W::new(self)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<INTENSET_SPEC, 1> {
        ERROR_W::new(self)
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
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
