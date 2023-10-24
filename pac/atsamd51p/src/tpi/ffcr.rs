#[doc = "Register `FFCR` reader"]
pub type R = crate::R<FFCR_SPEC>;
#[doc = "Register `FFCR` writer"]
pub type W = crate::W<FFCR_SPEC>;
#[doc = "Field `EnFCont` reader - "]
pub type EN_FCONT_R = crate::BitReader;
#[doc = "Field `EnFCont` writer - "]
pub type EN_FCONT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TrigIn` reader - "]
pub type TRIG_IN_R = crate::BitReader;
#[doc = "Field `TrigIn` writer - "]
pub type TRIG_IN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_fcont(&self) -> EN_FCONT_R {
        EN_FCONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trig_in(&self) -> TRIG_IN_R {
        TRIG_IN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn en_fcont(&mut self) -> EN_FCONT_W<FFCR_SPEC, 1> {
        EN_FCONT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn trig_in(&mut self) -> TRIG_IN_W<FFCR_SPEC, 8> {
        TRIG_IN_W::new(self)
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
#[doc = "Formatter and Flush Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FFCR_SPEC;
impl crate::RegisterSpec for FFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ffcr::R`](R) reader structure"]
impl crate::Readable for FFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ffcr::W`](W) writer structure"]
impl crate::Writable for FFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FFCR to value 0"]
impl crate::Resettable for FFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
