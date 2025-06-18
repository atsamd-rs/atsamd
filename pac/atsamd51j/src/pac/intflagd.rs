#[doc = "Register `INTFLAGD` reader"]
pub type R = crate::R<IntflagdSpec>;
#[doc = "Register `INTFLAGD` writer"]
pub type W = crate::W<IntflagdSpec>;
#[doc = "Field `SERCOM4_` reader - SERCOM4"]
pub type Sercom4_R = crate::BitReader;
#[doc = "Field `SERCOM4_` writer - SERCOM4"]
pub type Sercom4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM5_` reader - SERCOM5"]
pub type Sercom5_R = crate::BitReader;
#[doc = "Field `SERCOM5_` writer - SERCOM5"]
pub type Sercom5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC4_` reader - TCC4"]
pub type Tcc4_R = crate::BitReader;
#[doc = "Field `TCC4_` writer - TCC4"]
pub type Tcc4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0_` reader - ADC0"]
pub type Adc0_R = crate::BitReader;
#[doc = "Field `ADC0_` writer - ADC0"]
pub type Adc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_` reader - ADC1"]
pub type Adc1_R = crate::BitReader;
#[doc = "Field `ADC1_` writer - ADC1"]
pub type Adc1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_` reader - DAC"]
pub type Dac_R = crate::BitReader;
#[doc = "Field `DAC_` writer - DAC"]
pub type Dac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_` reader - I2S"]
pub type I2s_R = crate::BitReader;
#[doc = "Field `I2S_` writer - I2S"]
pub type I2s_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCC_` reader - PCC"]
pub type Pcc_R = crate::BitReader;
#[doc = "Field `PCC_` writer - PCC"]
pub type Pcc_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SERCOM4"]
    #[inline(always)]
    pub fn sercom4_(&self) -> Sercom4_R {
        Sercom4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM5"]
    #[inline(always)]
    pub fn sercom5_(&self) -> Sercom5_R {
        Sercom5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TCC4"]
    #[inline(always)]
    pub fn tcc4_(&self) -> Tcc4_R {
        Tcc4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0"]
    #[inline(always)]
    pub fn adc0_(&self) -> Adc0_R {
        Adc0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1"]
    #[inline(always)]
    pub fn adc1_(&self) -> Adc1_R {
        Adc1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC"]
    #[inline(always)]
    pub fn dac_(&self) -> Dac_R {
        Dac_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2S"]
    #[inline(always)]
    pub fn i2s_(&self) -> I2s_R {
        I2s_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCC"]
    #[inline(always)]
    pub fn pcc_(&self) -> Pcc_R {
        Pcc_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM4"]
    #[inline(always)]
    pub fn sercom4_(&mut self) -> Sercom4_W<IntflagdSpec> {
        Sercom4_W::new(self, 0)
    }
    #[doc = "Bit 1 - SERCOM5"]
    #[inline(always)]
    pub fn sercom5_(&mut self) -> Sercom5_W<IntflagdSpec> {
        Sercom5_W::new(self, 1)
    }
    #[doc = "Bit 4 - TCC4"]
    #[inline(always)]
    pub fn tcc4_(&mut self) -> Tcc4_W<IntflagdSpec> {
        Tcc4_W::new(self, 4)
    }
    #[doc = "Bit 7 - ADC0"]
    #[inline(always)]
    pub fn adc0_(&mut self) -> Adc0_W<IntflagdSpec> {
        Adc0_W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC1"]
    #[inline(always)]
    pub fn adc1_(&mut self) -> Adc1_W<IntflagdSpec> {
        Adc1_W::new(self, 8)
    }
    #[doc = "Bit 9 - DAC"]
    #[inline(always)]
    pub fn dac_(&mut self) -> Dac_W<IntflagdSpec> {
        Dac_W::new(self, 9)
    }
    #[doc = "Bit 10 - I2S"]
    #[inline(always)]
    pub fn i2s_(&mut self) -> I2s_W<IntflagdSpec> {
        I2s_W::new(self, 10)
    }
    #[doc = "Bit 11 - PCC"]
    #[inline(always)]
    pub fn pcc_(&mut self) -> Pcc_W<IntflagdSpec> {
        Pcc_W::new(self, 11)
    }
}
#[doc = "Peripheral interrupt flag status - Bridge D\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagdSpec;
impl crate::RegisterSpec for IntflagdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagd::R`](R) reader structure"]
impl crate::Readable for IntflagdSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagd::W`](W) writer structure"]
impl crate::Writable for IntflagdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFLAGD to value 0"]
impl crate::Resettable for IntflagdSpec {}
