#[doc = "Register `TI` reader"]
pub type R = crate::R<TiSpec>;
#[doc = "Register `TI` writer"]
pub type W = crate::W<TiSpec>;
#[doc = "Field `CNS` reader - Count Nanoseconds"]
pub type CnsR = crate::FieldReader;
#[doc = "Field `CNS` writer - Count Nanoseconds"]
pub type CnsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ACNS` reader - Alternative Count Nanoseconds"]
pub type AcnsR = crate::FieldReader;
#[doc = "Field `ACNS` writer - Alternative Count Nanoseconds"]
pub type AcnsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NIT` reader - Number of Increments"]
pub type NitR = crate::FieldReader;
#[doc = "Field `NIT` writer - Number of Increments"]
pub type NitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Count Nanoseconds"]
    #[inline(always)]
    pub fn cns(&self) -> CnsR {
        CnsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Alternative Count Nanoseconds"]
    #[inline(always)]
    pub fn acns(&self) -> AcnsR {
        AcnsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of Increments"]
    #[inline(always)]
    pub fn nit(&self) -> NitR {
        NitR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Count Nanoseconds"]
    #[inline(always)]
    pub fn cns(&mut self) -> CnsW<TiSpec> {
        CnsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Alternative Count Nanoseconds"]
    #[inline(always)]
    pub fn acns(&mut self) -> AcnsW<TiSpec> {
        AcnsW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Number of Increments"]
    #[inline(always)]
    pub fn nit(&mut self) -> NitW<TiSpec> {
        NitW::new(self, 16)
    }
}
#[doc = "1588 Timer Increment Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ti::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TiSpec;
impl crate::RegisterSpec for TiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ti::R`](R) reader structure"]
impl crate::Readable for TiSpec {}
#[doc = "`write(|w| ..)` method takes [`ti::W`](W) writer structure"]
impl crate::Writable for TiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TI to value 0"]
impl crate::Resettable for TiSpec {}
