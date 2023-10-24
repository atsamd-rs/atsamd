#[doc = "Register `DEMCR` reader"]
pub type R = crate::R<DEMCR_SPEC>;
#[doc = "Register `DEMCR` writer"]
pub type W = crate::W<DEMCR_SPEC>;
#[doc = "Field `VC_CORERESET` reader - "]
pub type VC_CORERESET_R = crate::BitReader;
#[doc = "Field `VC_CORERESET` writer - "]
pub type VC_CORERESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VC_MMERR` reader - "]
pub type VC_MMERR_R = crate::BitReader;
#[doc = "Field `VC_MMERR` writer - "]
pub type VC_MMERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VC_NOCPERR` reader - "]
pub type VC_NOCPERR_R = crate::BitReader;
#[doc = "Field `VC_NOCPERR` writer - "]
pub type VC_NOCPERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VC_CHKERR` reader - "]
pub type VC_CHKERR_R = crate::BitReader;
#[doc = "Field `VC_CHKERR` writer - "]
pub type VC_CHKERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VC_STATERR` reader - "]
pub type VC_STATERR_R = crate::BitReader;
#[doc = "Field `VC_STATERR` writer - "]
pub type VC_STATERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VC_BUSERR` reader - "]
pub type VC_BUSERR_R = crate::BitReader;
#[doc = "Field `VC_BUSERR` writer - "]
pub type VC_BUSERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VC_INTERR` reader - "]
pub type VC_INTERR_R = crate::BitReader;
#[doc = "Field `VC_INTERR` writer - "]
pub type VC_INTERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VC_HARDERR` reader - "]
pub type VC_HARDERR_R = crate::BitReader;
#[doc = "Field `VC_HARDERR` writer - "]
pub type VC_HARDERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MON_EN` reader - "]
pub type MON_EN_R = crate::BitReader;
#[doc = "Field `MON_EN` writer - "]
pub type MON_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MON_PEND` reader - "]
pub type MON_PEND_R = crate::BitReader;
#[doc = "Field `MON_PEND` writer - "]
pub type MON_PEND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MON_STEP` reader - "]
pub type MON_STEP_R = crate::BitReader;
#[doc = "Field `MON_STEP` writer - "]
pub type MON_STEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MON_REQ` reader - "]
pub type MON_REQ_R = crate::BitReader;
#[doc = "Field `MON_REQ` writer - "]
pub type MON_REQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRCENA` reader - "]
pub type TRCENA_R = crate::BitReader;
#[doc = "Field `TRCENA` writer - "]
pub type TRCENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vc_corereset(&self) -> VC_CORERESET_R {
        VC_CORERESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn vc_mmerr(&self) -> VC_MMERR_R {
        VC_MMERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn vc_nocperr(&self) -> VC_NOCPERR_R {
        VC_NOCPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn vc_chkerr(&self) -> VC_CHKERR_R {
        VC_CHKERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn vc_staterr(&self) -> VC_STATERR_R {
        VC_STATERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn vc_buserr(&self) -> VC_BUSERR_R {
        VC_BUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vc_interr(&self) -> VC_INTERR_R {
        VC_INTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn vc_harderr(&self) -> VC_HARDERR_R {
        VC_HARDERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mon_en(&self) -> MON_EN_R {
        MON_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mon_pend(&self) -> MON_PEND_R {
        MON_PEND_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mon_step(&self) -> MON_STEP_R {
        MON_STEP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn mon_req(&self) -> MON_REQ_R {
        MON_REQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn trcena(&self) -> TRCENA_R {
        TRCENA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn vc_corereset(&mut self) -> VC_CORERESET_W<DEMCR_SPEC, 0> {
        VC_CORERESET_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn vc_mmerr(&mut self) -> VC_MMERR_W<DEMCR_SPEC, 4> {
        VC_MMERR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn vc_nocperr(&mut self) -> VC_NOCPERR_W<DEMCR_SPEC, 5> {
        VC_NOCPERR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn vc_chkerr(&mut self) -> VC_CHKERR_W<DEMCR_SPEC, 6> {
        VC_CHKERR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn vc_staterr(&mut self) -> VC_STATERR_W<DEMCR_SPEC, 7> {
        VC_STATERR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn vc_buserr(&mut self) -> VC_BUSERR_W<DEMCR_SPEC, 8> {
        VC_BUSERR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn vc_interr(&mut self) -> VC_INTERR_W<DEMCR_SPEC, 9> {
        VC_INTERR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn vc_harderr(&mut self) -> VC_HARDERR_W<DEMCR_SPEC, 10> {
        VC_HARDERR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn mon_en(&mut self) -> MON_EN_W<DEMCR_SPEC, 16> {
        MON_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn mon_pend(&mut self) -> MON_PEND_W<DEMCR_SPEC, 17> {
        MON_PEND_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn mon_step(&mut self) -> MON_STEP_W<DEMCR_SPEC, 18> {
        MON_STEP_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn mon_req(&mut self) -> MON_REQ_W<DEMCR_SPEC, 19> {
        MON_REQ_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn trcena(&mut self) -> TRCENA_W<DEMCR_SPEC, 24> {
        TRCENA_W::new(self)
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
#[doc = "Debug Exception and Monitor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`demcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEMCR_SPEC;
impl crate::RegisterSpec for DEMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`demcr::R`](R) reader structure"]
impl crate::Readable for DEMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`demcr::W`](W) writer structure"]
impl crate::Writable for DEMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEMCR to value 0"]
impl crate::Resettable for DEMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
