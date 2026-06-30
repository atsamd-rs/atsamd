#[doc = "Register `BASEADDR` reader"]
pub type R = crate::R<BaseaddrSpec>;
#[doc = "Register `BASEADDR` writer"]
pub type W = crate::W<BaseaddrSpec>;
#[doc = "Field `BASEADDR` reader - Descriptor Memory Base Address"]
pub type BaseaddrR = crate::FieldReader<u32>;
#[doc = "Field `BASEADDR` writer - Descriptor Memory Base Address"]
pub type BaseaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BaseaddrR {
        BaseaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Memory Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn baseaddr(&mut self) -> BaseaddrW<BaseaddrSpec> {
        BaseaddrW::new(self, 0)
    }
}
#[doc = "Descriptor Memory Section Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`baseaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baseaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseaddrSpec;
impl crate::RegisterSpec for BaseaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baseaddr::R`](R) reader structure"]
impl crate::Readable for BaseaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`baseaddr::W`](W) writer structure"]
impl crate::Writable for BaseaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BASEADDR to value 0"]
impl crate::Resettable for BaseaddrSpec {
    const RESET_VALUE: u32 = 0;
}
