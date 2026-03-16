#[doc = "Register `WRBADDR` reader"]
pub type R = crate::R<WrbaddrSpec>;
#[doc = "Register `WRBADDR` writer"]
pub type W = crate::W<WrbaddrSpec>;
#[doc = "Field `WRBADDR` reader - Write-Back Memory Base Address"]
pub type WrbaddrR = crate::FieldReader<u32>;
#[doc = "Field `WRBADDR` writer - Write-Back Memory Base Address"]
pub type WrbaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write-Back Memory Base Address"]
    #[inline(always)]
    pub fn wrbaddr(&self) -> WrbaddrR {
        WrbaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write-Back Memory Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn wrbaddr(&mut self) -> WrbaddrW<WrbaddrSpec> {
        WrbaddrW::new(self, 0)
    }
}
#[doc = "Write-Back Memory Section Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`wrbaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrbaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrbaddrSpec;
impl crate::RegisterSpec for WrbaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrbaddr::R`](R) reader structure"]
impl crate::Readable for WrbaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`wrbaddr::W`](W) writer structure"]
impl crate::Writable for WrbaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRBADDR to value 0"]
impl crate::Resettable for WrbaddrSpec {
    const RESET_VALUE: u32 = 0;
}
