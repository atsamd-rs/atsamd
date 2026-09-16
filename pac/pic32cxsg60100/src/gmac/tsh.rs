#[doc = "Register `TSH` reader"]
pub type R = crate::R<TshSpec>;
#[doc = "Register `TSH` writer"]
pub type W = crate::W<TshSpec>;
#[doc = "Field `TCS` reader - Timer Count in Seconds"]
pub type TcsR = crate::FieldReader<u16>;
#[doc = "Field `TCS` writer - Timer Count in Seconds"]
pub type TcsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer Count in Seconds"]
    #[inline(always)]
    pub fn tcs(&self) -> TcsR {
        TcsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Count in Seconds"]
    #[inline(always)]
    #[must_use]
    pub fn tcs(&mut self) -> TcsW<TshSpec> {
        TcsW::new(self, 0)
    }
}
#[doc = "1588 Timer Seconds High \\[15:0\\]
Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TshSpec;
impl crate::RegisterSpec for TshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsh::R`](R) reader structure"]
impl crate::Readable for TshSpec {}
#[doc = "`write(|w| ..)` method takes [`tsh::W`](W) writer structure"]
impl crate::Writable for TshSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSH to value 0"]
impl crate::Resettable for TshSpec {
    const RESET_VALUE: u32 = 0;
}
