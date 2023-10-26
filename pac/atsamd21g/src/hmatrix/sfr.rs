#[doc = "Register `SFR%s` reader"]
pub type R = crate::R<SFR_SPEC>;
#[doc = "Register `SFR%s` writer"]
pub type W = crate::W<SFR_SPEC>;
#[doc = "Field `SFR` reader - Special Function Register"]
pub type SFR_R = crate::FieldReader<u32>;
#[doc = "Field `SFR` writer - Special Function Register"]
pub type SFR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Special Function Register"]
    #[inline(always)]
    pub fn sfr(&self) -> SFR_R {
        SFR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Special Function Register"]
    #[inline(always)]
    #[must_use]
    pub fn sfr(&mut self) -> SFR_W<SFR_SPEC, 0> {
        SFR_W::new(self)
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
#[doc = "Special Function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SFR_SPEC;
impl crate::RegisterSpec for SFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr::R`](R) reader structure"]
impl crate::Readable for SFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sfr::W`](W) writer structure"]
impl crate::Writable for SFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFR%s to value 0"]
impl crate::Resettable for SFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
