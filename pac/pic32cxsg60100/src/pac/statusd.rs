#[doc = "Register `STATUSD` reader"]
pub type R = crate::R<StatusdSpec>;
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Protect Enable"]
pub type Sercom4_R = crate::BitReader;
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Protect Enable"]
pub type Sercom5_R = crate::BitReader;
#[doc = "Field `SERCOM6_` reader - SERCOM6 APB Protect Enable"]
pub type Sercom6_R = crate::BitReader;
#[doc = "Field `SERCOM7_` reader - SERCOM7 APB Protect Enable"]
pub type Sercom7_R = crate::BitReader;
#[doc = "Field `TCC4_` reader - TCC4 APB Protect Enable"]
pub type Tcc4_R = crate::BitReader;
#[doc = "Field `TC6_` reader - TC6 APB Protect Enable"]
pub type Tc6_R = crate::BitReader;
#[doc = "Field `TC7_` reader - TC7 APB Protect Enable"]
pub type Tc7_R = crate::BitReader;
#[doc = "Field `ADC0_` reader - ADC0 APB Protect Enable"]
pub type Adc0_R = crate::BitReader;
#[doc = "Field `ADC1_` reader - ADC1 APB Protect Enable"]
pub type Adc1_R = crate::BitReader;
#[doc = "Field `DAC_` reader - DAC APB Protect Enable"]
pub type Dac_R = crate::BitReader;
#[doc = "Field `I2S_` reader - I2S APB Protect Enable"]
pub type I2s_R = crate::BitReader;
#[doc = "Field `PCC_` reader - PCC APB Protect Enable"]
pub type Pcc_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SERCOM4 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> Sercom4_R {
        Sercom4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> Sercom5_R {
        Sercom5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM6 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom6_(&self) -> Sercom6_R {
        Sercom6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM7 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom7_(&self) -> Sercom7_R {
        Sercom7_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCC4 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc4_(&self) -> Tcc4_R {
        Tcc4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC6 APB Protect Enable"]
    #[inline(always)]
    pub fn tc6_(&self) -> Tc6_R {
        Tc6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC7 APB Protect Enable"]
    #[inline(always)]
    pub fn tc7_(&self) -> Tc7_R {
        Tc7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Protect Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> Adc0_R {
        Adc0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 APB Protect Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> Adc1_R {
        Adc1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC APB Protect Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> Dac_R {
        Dac_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2S APB Protect Enable"]
    #[inline(always)]
    pub fn i2s_(&self) -> I2s_R {
        I2s_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCC APB Protect Enable"]
    #[inline(always)]
    pub fn pcc_(&self) -> Pcc_R {
        Pcc_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge D\n\nYou can [`read`](crate::Reg::read) this register and get [`statusd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusdSpec;
impl crate::RegisterSpec for StatusdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusd::R`](R) reader structure"]
impl crate::Readable for StatusdSpec {}
#[doc = "`reset()` method sets STATUSD to value 0"]
impl crate::Resettable for StatusdSpec {
    const RESET_VALUE: u32 = 0;
}
