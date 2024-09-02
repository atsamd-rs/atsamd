#[doc = "Register `TN` reader"]
pub type R = crate::R<TnSpec>;
#[doc = "Register `TN` writer"]
pub type W = crate::W<TnSpec>;
#[doc = "Field `TNS` reader - Timer Count in Nanoseconds"]
pub type TnsR = crate::FieldReader<u32>;
#[doc = "Field `TNS` writer - Timer Count in Nanoseconds"]
pub type TnsW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Timer Count in Nanoseconds"]
    #[inline(always)]
    pub fn tns(&self) -> TnsR {
        TnsR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer Count in Nanoseconds"]
    #[inline(always)]
    #[must_use]
    pub fn tns(&mut self) -> TnsW<TnSpec> {
        TnsW::new(self, 0)
    }
}
#[doc = "1588 Timer Nanoseconds Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnSpec;
impl crate::RegisterSpec for TnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tn::R`](R) reader structure"]
impl crate::Readable for TnSpec {}
#[doc = "`write(|w| ..)` method takes [`tn::W`](W) writer structure"]
impl crate::Writable for TnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TN to value 0"]
impl crate::Resettable for TnSpec {
    const RESET_VALUE: u32 = 0;
}
