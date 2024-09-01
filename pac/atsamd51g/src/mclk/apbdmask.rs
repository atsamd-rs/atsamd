#[doc = "Register `APBDMASK` reader"]
pub type R = crate::R<ApbdmaskSpec>;
#[doc = "Register `APBDMASK` writer"]
pub type W = crate::W<ApbdmaskSpec>;
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Clock Enable"]
pub type Sercom4_R = crate::BitReader;
#[doc = "Field `SERCOM4_` writer - SERCOM4 APB Clock Enable"]
pub type Sercom4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Clock Enable"]
pub type Sercom5_R = crate::BitReader;
#[doc = "Field `SERCOM5_` writer - SERCOM5 APB Clock Enable"]
pub type Sercom5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0_` reader - ADC0 APB Clock Enable"]
pub type Adc0_R = crate::BitReader;
#[doc = "Field `ADC0_` writer - ADC0 APB Clock Enable"]
pub type Adc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_` reader - ADC1 APB Clock Enable"]
pub type Adc1_R = crate::BitReader;
#[doc = "Field `ADC1_` writer - ADC1 APB Clock Enable"]
pub type Adc1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_` reader - DAC APB Clock Enable"]
pub type Dac_R = crate::BitReader;
#[doc = "Field `DAC_` writer - DAC APB Clock Enable"]
pub type Dac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCC_` reader - PCC APB Clock Enable"]
pub type Pcc_R = crate::BitReader;
#[doc = "Field `PCC_` writer - PCC APB Clock Enable"]
pub type Pcc_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SERCOM4 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> Sercom4_R {
        Sercom4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> Sercom5_R {
        Sercom5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Clock Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> Adc0_R {
        Adc0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 APB Clock Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> Adc1_R {
        Adc1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> Dac_R {
        Dac_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - PCC APB Clock Enable"]
    #[inline(always)]
    pub fn pcc_(&self) -> Pcc_R {
        Pcc_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM4 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom4_(&mut self) -> Sercom4_W<ApbdmaskSpec> {
        Sercom4_W::new(self, 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom5_(&mut self) -> Sercom5_W<ApbdmaskSpec> {
        Sercom5_W::new(self, 1)
    }
    #[doc = "Bit 7 - ADC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_(&mut self) -> Adc0_W<ApbdmaskSpec> {
        Adc0_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_(&mut self) -> Adc1_W<ApbdmaskSpec> {
        Adc1_W::new(self, 8)
    }
    #[doc = "Bit 9 - DAC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_(&mut self) -> Dac_W<ApbdmaskSpec> {
        Dac_W::new(self, 9)
    }
    #[doc = "Bit 11 - PCC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcc_(&mut self) -> Pcc_W<ApbdmaskSpec> {
        Pcc_W::new(self, 11)
    }
}
#[doc = "APBD Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbdmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbdmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbdmaskSpec;
impl crate::RegisterSpec for ApbdmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbdmask::R`](R) reader structure"]
impl crate::Readable for ApbdmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`apbdmask::W`](W) writer structure"]
impl crate::Writable for ApbdmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBDMASK to value 0"]
impl crate::Resettable for ApbdmaskSpec {
    const RESET_VALUE: u32 = 0;
}
