#[doc = "Register `TSL` reader"]
pub type R = crate::R<TslSpec>;
#[doc = "Register `TSL` writer"]
pub type W = crate::W<TslSpec>;
#[doc = "Field `TCS` reader - Timer Count in Seconds"]
pub type TcsR = crate::FieldReader<u32>;
#[doc = "Field `TCS` writer - Timer Count in Seconds"]
pub type TcsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Count in Seconds"]
    #[inline(always)]
    pub fn tcs(&self) -> TcsR {
        TcsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Count in Seconds"]
    #[inline(always)]
    pub fn tcs(&mut self) -> TcsW<TslSpec> {
        TcsW::new(self, 0)
    }
}
#[doc = "1588 Timer Seconds \\[31:0\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TslSpec;
impl crate::RegisterSpec for TslSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsl::R`](R) reader structure"]
impl crate::Readable for TslSpec {}
#[doc = "`write(|w| ..)` method takes [`tsl::W`](W) writer structure"]
impl crate::Writable for TslSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSL to value 0"]
impl crate::Resettable for TslSpec {}
