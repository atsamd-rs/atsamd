#[doc = "Register `GAINCORR` reader"]
pub type R = crate::R<GAINCORR_SPEC>;
#[doc = "Register `GAINCORR` writer"]
pub type W = crate::W<GAINCORR_SPEC>;
#[doc = "Field `GAINCORR` reader - Gain Correction Value"]
pub type GAINCORR_R = crate::FieldReader<u16>;
#[doc = "Field `GAINCORR` writer - Gain Correction Value"]
pub type GAINCORR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Gain Correction Value"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - Gain Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn gaincorr(&mut self) -> GAINCORR_W<GAINCORR_SPEC, 0> {
        GAINCORR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Gain Correction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gaincorr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gaincorr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAINCORR_SPEC;
impl crate::RegisterSpec for GAINCORR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`gaincorr::R`](R) reader structure"]
impl crate::Readable for GAINCORR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gaincorr::W`](W) writer structure"]
impl crate::Writable for GAINCORR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAINCORR to value 0"]
impl crate::Resettable for GAINCORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
