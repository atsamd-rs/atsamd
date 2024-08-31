#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `DATARDY` reader - Data Ready Interrupt Flag"]
pub type DatardyR = crate::BitReader;
#[doc = "Field `DATARDY` writer - Data Ready Interrupt Flag"]
pub type DatardyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Ready Interrupt Flag"]
    #[inline(always)]
    pub fn datardy(&self) -> DatardyR {
        DatardyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn datardy(&mut self) -> DatardyW<IntflagSpec> {
        DatardyW::new(self, 0)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u8 = 0;
}
