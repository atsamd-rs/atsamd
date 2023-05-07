#[doc = "Register `APBDMASK` reader"]
pub struct R(crate::R<APBDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBDMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBDMASK` writer"]
pub struct W(crate::W<APBDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APBDMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Clock Enable"]
pub type SERCOM4__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM4_` writer - SERCOM4 APB Clock Enable"]
pub type SERCOM4__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Clock Enable"]
pub type SERCOM5__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM5_` writer - SERCOM5 APB Clock Enable"]
pub type SERCOM5__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `TCC4_` reader - TCC4 APB Clock Enable"]
pub type TCC4__R = crate::BitReader<bool>;
#[doc = "Field `TCC4_` writer - TCC4 APB Clock Enable"]
pub type TCC4__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `ADC0_` reader - ADC0 APB Clock Enable"]
pub type ADC0__R = crate::BitReader<bool>;
#[doc = "Field `ADC0_` writer - ADC0 APB Clock Enable"]
pub type ADC0__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `ADC1_` reader - ADC1 APB Clock Enable"]
pub type ADC1__R = crate::BitReader<bool>;
#[doc = "Field `ADC1_` writer - ADC1 APB Clock Enable"]
pub type ADC1__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `DAC_` reader - DAC APB Clock Enable"]
pub type DAC__R = crate::BitReader<bool>;
#[doc = "Field `DAC_` writer - DAC APB Clock Enable"]
pub type DAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `I2S_` reader - I2S APB Clock Enable"]
pub type I2S__R = crate::BitReader<bool>;
#[doc = "Field `I2S_` writer - I2S APB Clock Enable"]
pub type I2S__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `PCC_` reader - PCC APB Clock Enable"]
pub type PCC__R = crate::BitReader<bool>;
#[doc = "Field `PCC_` writer - PCC APB Clock Enable"]
pub type PCC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SERCOM4 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TCC4 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc4_(&self) -> TCC4__R {
        TCC4__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 APB Clock Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> ADC0__R {
        ADC0__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 APB Clock Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> ADC1__R {
        ADC1__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2S APB Clock Enable"]
    #[inline(always)]
    pub fn i2s_(&self) -> I2S__R {
        I2S__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCC APB Clock Enable"]
    #[inline(always)]
    pub fn pcc_(&self) -> PCC__R {
        PCC__R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM4 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom4_(&mut self) -> SERCOM4__W<0> {
        SERCOM4__W::new(self)
    }
    #[doc = "Bit 1 - SERCOM5 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom5_(&mut self) -> SERCOM5__W<1> {
        SERCOM5__W::new(self)
    }
    #[doc = "Bit 4 - TCC4 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc4_(&mut self) -> TCC4__W<4> {
        TCC4__W::new(self)
    }
    #[doc = "Bit 7 - ADC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_(&mut self) -> ADC0__W<7> {
        ADC0__W::new(self)
    }
    #[doc = "Bit 8 - ADC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_(&mut self) -> ADC1__W<8> {
        ADC1__W::new(self)
    }
    #[doc = "Bit 9 - DAC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_(&mut self) -> DAC__W<9> {
        DAC__W::new(self)
    }
    #[doc = "Bit 10 - I2S APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_(&mut self) -> I2S__W<10> {
        I2S__W::new(self)
    }
    #[doc = "Bit 11 - PCC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcc_(&mut self) -> PCC__W<11> {
        PCC__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBD Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbdmask](index.html) module"]
pub struct APBDMASK_SPEC;
impl crate::RegisterSpec for APBDMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbdmask::R](R) reader structure"]
impl crate::Readable for APBDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbdmask::W](W) writer structure"]
impl crate::Writable for APBDMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBDMASK to value 0"]
impl crate::Resettable for APBDMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
