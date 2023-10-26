#[doc = "Register `OFFSETCORR` reader"]
pub type R = crate::R<OFFSETCORR_SPEC>;
#[doc = "Register `OFFSETCORR` writer"]
pub type W = crate::W<OFFSETCORR_SPEC>;
#[doc = "Field `OFFSETCORR` reader - Offset Correction Value"]
pub type OFFSETCORR_R = crate::FieldReader<u16>;
#[doc = "Field `OFFSETCORR` writer - Offset Correction Value"]
pub type OFFSETCORR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Offset Correction Value"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(self.bits & 0x0fff)
    }
}
impl W {
    #[doc = "Bits 0:11 - Offset Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn offsetcorr(&mut self) -> OFFSETCORR_W<OFFSETCORR_SPEC, 0> {
        OFFSETCORR_W::new(self)
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
#[doc = "Offset Correction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`offsetcorr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`offsetcorr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OFFSETCORR_SPEC;
impl crate::RegisterSpec for OFFSETCORR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`offsetcorr::R`](R) reader structure"]
impl crate::Readable for OFFSETCORR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`offsetcorr::W`](W) writer structure"]
impl crate::Writable for OFFSETCORR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFFSETCORR to value 0"]
impl crate::Resettable for OFFSETCORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
