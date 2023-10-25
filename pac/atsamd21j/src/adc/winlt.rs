#[doc = "Register `WINLT` reader"]
pub type R = crate::R<WINLT_SPEC>;
#[doc = "Register `WINLT` writer"]
pub type W = crate::W<WINLT_SPEC>;
#[doc = "Field `WINLT` reader - Window Lower Threshold"]
pub type WINLT_R = crate::FieldReader<u16>;
#[doc = "Field `WINLT` writer - Window Lower Threshold"]
pub type WINLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Window Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Lower Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn winlt(&mut self) -> WINLT_W<WINLT_SPEC, 0> {
        WINLT_W::new(self)
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
#[doc = "Window Monitor Lower Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winlt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winlt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WINLT_SPEC;
impl crate::RegisterSpec for WINLT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`winlt::R`](R) reader structure"]
impl crate::Readable for WINLT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`winlt::W`](W) writer structure"]
impl crate::Writable for WINLT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WINLT to value 0"]
impl crate::Resettable for WINLT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
