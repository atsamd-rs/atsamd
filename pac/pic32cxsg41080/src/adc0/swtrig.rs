#[doc = "Register `SWTRIG` reader"]
pub type R = crate::R<SwtrigSpec>;
#[doc = "Register `SWTRIG` writer"]
pub type W = crate::W<SwtrigSpec>;
#[doc = "Field `FLUSH` reader - ADC Conversion Flush"]
pub type FlushR = crate::BitReader;
#[doc = "Field `FLUSH` writer - ADC Conversion Flush"]
pub type FlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start ADC Conversion"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start ADC Conversion"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC Conversion Flush"]
    #[inline(always)]
    pub fn flush(&self) -> FlushR {
        FlushR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start ADC Conversion"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Conversion Flush"]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FlushW<SwtrigSpec> {
        FlushW::new(self, 0)
    }
    #[doc = "Bit 1 - Start ADC Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<SwtrigSpec> {
        StartW::new(self, 1)
    }
}
#[doc = "Software Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrigSpec;
impl crate::RegisterSpec for SwtrigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`swtrig::R`](R) reader structure"]
impl crate::Readable for SwtrigSpec {}
#[doc = "`write(|w| ..)` method takes [`swtrig::W`](W) writer structure"]
impl crate::Writable for SwtrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SWTRIG to value 0"]
impl crate::Resettable for SwtrigSpec {
    const RESET_VALUE: u8 = 0;
}
