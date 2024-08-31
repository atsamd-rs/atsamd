#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `READY` reader - NVM Ready Interrupt Enable"]
pub type ReadyR = crate::BitReader;
#[doc = "Field `READY` writer - NVM Ready Interrupt Enable"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Error Interrupt Enable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error Interrupt Enable"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NVM Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NVM Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> ReadyW<IntensetSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntensetSpec> {
        ErrorW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u8 = 0;
}
