#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `PREC` reader - Stop Received Interrupt"]
pub type PrecR = crate::BitReader;
#[doc = "Field `PREC` writer - Stop Received Interrupt"]
pub type PrecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMATCH` reader - Address Match Interrupt"]
pub type AmatchR = crate::BitReader;
#[doc = "Field `AMATCH` writer - Address Match Interrupt"]
pub type AmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRDY` reader - Data Interrupt"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `DRDY` writer - Data Interrupt"]
pub type DrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Combined Error Interrupt"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop Received Interrupt"]
    #[inline(always)]
    pub fn prec(&self) -> PrecR {
        PrecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Match Interrupt"]
    #[inline(always)]
    pub fn amatch(&self) -> AmatchR {
        AmatchR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Interrupt"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Received Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn prec(&mut self) -> PrecW<IntflagSpec> {
        PrecW::new(self, 0)
    }
    #[doc = "Bit 1 - Address Match Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn amatch(&mut self) -> AmatchW<IntflagSpec> {
        AmatchW::new(self, 1)
    }
    #[doc = "Bit 2 - Data Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DrdyW<IntflagSpec> {
        DrdyW::new(self, 2)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IntflagSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "I2CS Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
