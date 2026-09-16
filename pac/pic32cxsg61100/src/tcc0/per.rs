#[doc = "Register `PER` reader"]
pub type R = crate::R<PerSpec>;
#[doc = "Register `PER` writer"]
pub type W = crate::W<PerSpec>;
#[doc = "Field `PER` reader - Period Value"]
pub type PerR = crate::FieldReader<u32>;
#[doc = "Field `PER` writer - Period Value"]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Period Value"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Period Value"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PerW<PerSpec> {
        PerW::new(self, 0)
    }
}
#[doc = "Period\n\nYou can [`read`](crate::Reg::read) this register and get [`per::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerSpec;
impl crate::RegisterSpec for PerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`per::R`](R) reader structure"]
impl crate::Readable for PerSpec {}
#[doc = "`write(|w| ..)` method takes [`per::W`](W) writer structure"]
impl crate::Writable for PerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PER to value 0xffff_ffff"]
impl crate::Resettable for PerSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
