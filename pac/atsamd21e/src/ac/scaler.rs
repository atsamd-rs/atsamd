#[doc = "Register `SCALER%s` reader"]
pub type R = crate::R<SCALER_SPEC>;
#[doc = "Register `SCALER%s` writer"]
pub type W = crate::W<SCALER_SPEC>;
#[doc = "Field `VALUE` reader - Scaler Value"]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `VALUE` writer - Scaler Value"]
pub type VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Scaler Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Scaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<SCALER_SPEC, 0> {
        VALUE_W::new(self)
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
#[doc = "Scaler n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scaler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scaler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCALER_SPEC;
impl crate::RegisterSpec for SCALER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scaler::R`](R) reader structure"]
impl crate::Readable for SCALER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scaler::W`](W) writer structure"]
impl crate::Writable for SCALER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCALER%s to value 0"]
impl crate::Resettable for SCALER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
