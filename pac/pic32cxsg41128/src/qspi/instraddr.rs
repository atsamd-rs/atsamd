#[doc = "Register `INSTRADDR` reader"]
pub type R = crate::R<InstraddrSpec>;
#[doc = "Register `INSTRADDR` writer"]
pub type W = crate::W<InstraddrSpec>;
#[doc = "Field `ADDR` reader - Instruction Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Instruction Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Instruction Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Instruction Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<InstraddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Instruction Address\n\nYou can [`read`](crate::Reg::read) this register and get [`instraddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instraddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InstraddrSpec;
impl crate::RegisterSpec for InstraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instraddr::R`](R) reader structure"]
impl crate::Readable for InstraddrSpec {}
#[doc = "`write(|w| ..)` method takes [`instraddr::W`](W) writer structure"]
impl crate::Writable for InstraddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INSTRADDR to value 0"]
impl crate::Resettable for InstraddrSpec {
    const RESET_VALUE: u32 = 0;
}
