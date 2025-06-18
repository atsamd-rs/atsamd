#[doc = "Register `ASAR[%s]` reader"]
pub type R = crate::R<AsarSpec>;
#[doc = "Register `ASAR[%s]` writer"]
pub type W = crate::W<AsarSpec>;
#[doc = "Field `ADMASA` reader - ADMA System Address"]
pub type AdmasaR = crate::FieldReader<u32>;
#[doc = "Field `ADMASA` writer - ADMA System Address"]
pub type AdmasaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    pub fn admasa(&self) -> AdmasaR {
        AdmasaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address"]
    #[inline(always)]
    pub fn admasa(&mut self) -> AdmasaW<AsarSpec> {
        AdmasaW::new(self, 0)
    }
}
#[doc = "ADMA System Address\n\nYou can [`read`](crate::Reg::read) this register and get [`asar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsarSpec;
impl crate::RegisterSpec for AsarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asar::R`](R) reader structure"]
impl crate::Readable for AsarSpec {}
#[doc = "`write(|w| ..)` method takes [`asar::W`](W) writer structure"]
impl crate::Writable for AsarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASAR[%s] to value 0"]
impl crate::Resettable for AsarSpec {}
