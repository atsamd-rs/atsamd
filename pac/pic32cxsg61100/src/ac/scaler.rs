#[doc = "Register `SCALER[%s]` reader"]
pub type R = crate::R<ScalerSpec>;
#[doc = "Register `SCALER[%s]` writer"]
pub type W = crate::W<ScalerSpec>;
#[doc = "Field `VALUE` reader - Scaler Value"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Scaler Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Scaler Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Scaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<ScalerSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Scaler n\n\nYou can [`read`](crate::Reg::read) this register and get [`scaler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scaler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScalerSpec;
impl crate::RegisterSpec for ScalerSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scaler::R`](R) reader structure"]
impl crate::Readable for ScalerSpec {}
#[doc = "`write(|w| ..)` method takes [`scaler::W`](W) writer structure"]
impl crate::Writable for ScalerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCALER[%s]
to value 0"]
impl crate::Resettable for ScalerSpec {
    const RESET_VALUE: u8 = 0;
}
