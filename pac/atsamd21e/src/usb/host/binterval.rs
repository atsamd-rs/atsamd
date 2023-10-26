#[doc = "Register `BINTERVAL%s` reader"]
pub type R = crate::R<BINTERVAL_SPEC>;
#[doc = "Register `BINTERVAL%s` writer"]
pub type W = crate::W<BINTERVAL_SPEC>;
#[doc = "Field `BITINTERVAL` reader - Bit Interval"]
pub type BITINTERVAL_R = crate::FieldReader;
#[doc = "Field `BITINTERVAL` writer - Bit Interval"]
pub type BITINTERVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bit Interval"]
    #[inline(always)]
    pub fn bitinterval(&self) -> BITINTERVAL_R {
        BITINTERVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit Interval"]
    #[inline(always)]
    #[must_use]
    pub fn bitinterval(&mut self) -> BITINTERVAL_W<BINTERVAL_SPEC, 0> {
        BITINTERVAL_W::new(self)
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
#[doc = "HOST Bus Access Period of Pipe\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`binterval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`binterval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BINTERVAL_SPEC;
impl crate::RegisterSpec for BINTERVAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`binterval::R`](R) reader structure"]
impl crate::Readable for BINTERVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`binterval::W`](W) writer structure"]
impl crate::Writable for BINTERVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BINTERVAL%s to value 0"]
impl crate::Resettable for BINTERVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
