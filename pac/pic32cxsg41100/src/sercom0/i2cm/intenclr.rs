#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `MB` reader - Master On Bus Interrupt Disable"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - Master On Bus Interrupt Disable"]
pub type MbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SB` reader - Slave On Bus Interrupt Disable"]
pub type SbR = crate::BitReader;
#[doc = "Field `SB` writer - Slave On Bus Interrupt Disable"]
pub type SbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt Disable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Combined Error Interrupt Disable"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Master On Bus Interrupt Disable"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave On Bus Interrupt Disable"]
    #[inline(always)]
    pub fn sb(&self) -> SbR {
        SbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master On Bus Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MbW<IntenclrSpec> {
        MbW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave On Bus Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sb(&mut self) -> SbW<IntenclrSpec> {
        SbW::new(self, 1)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntenclrSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "I2CM Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u8 = 0;
}
