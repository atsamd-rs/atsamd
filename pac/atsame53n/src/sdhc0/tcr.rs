#[doc = "Register `TCR` reader"]
pub type R = crate::R<TCR_SPEC>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TCR_SPEC>;
#[doc = "Field `DTCVAL` reader - Data Timeout Counter Value"]
pub type DTCVAL_R = crate::FieldReader;
#[doc = "Field `DTCVAL` writer - Data Timeout Counter Value"]
pub type DTCVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dtcval(&self) -> DTCVAL_R {
        DTCVAL_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Timeout Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn dtcval(&mut self) -> DTCVAL_W<TCR_SPEC, 0> {
        DTCVAL_W::new(self)
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
#[doc = "Timeout Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
