#[doc = "Register `APBCMASK` reader"]
pub type R = crate::R<APBCMASK_SPEC>;
#[doc = "Register `APBCMASK` writer"]
pub type W = crate::W<APBCMASK_SPEC>;
#[doc = "Field `PAC2_` reader - PAC2 APB Clock Enable"]
pub type PAC2__R = crate::BitReader;
#[doc = "Field `PAC2_` writer - PAC2 APB Clock Enable"]
pub type PAC2__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVSYS_` reader - EVSYS APB Clock Enable"]
pub type EVSYS__R = crate::BitReader;
#[doc = "Field `EVSYS_` writer - EVSYS APB Clock Enable"]
pub type EVSYS__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Clock Enable"]
pub type SERCOM0__R = crate::BitReader;
#[doc = "Field `SERCOM0_` writer - SERCOM0 APB Clock Enable"]
pub type SERCOM0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Clock Enable"]
pub type SERCOM1__R = crate::BitReader;
#[doc = "Field `SERCOM1_` writer - SERCOM1 APB Clock Enable"]
pub type SERCOM1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Clock Enable"]
pub type SERCOM2__R = crate::BitReader;
#[doc = "Field `SERCOM2_` writer - SERCOM2 APB Clock Enable"]
pub type SERCOM2__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCC0_` reader - TCC0 APB Clock Enable"]
pub type TCC0__R = crate::BitReader;
#[doc = "Field `TCC0_` writer - TCC0 APB Clock Enable"]
pub type TCC0__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC1_` reader - TC1 APB Clock Enable"]
pub type TC1__R = crate::BitReader;
#[doc = "Field `TC1_` writer - TC1 APB Clock Enable"]
pub type TC1__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC2_` reader - TC2 APB Clock Enable"]
pub type TC2__R = crate::BitReader;
#[doc = "Field `TC2_` writer - TC2 APB Clock Enable"]
pub type TC2__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_` reader - ADC APB Clock Enable"]
pub type ADC__R = crate::BitReader;
#[doc = "Field `ADC_` writer - ADC APB Clock Enable"]
pub type ADC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub type AC__R = crate::BitReader;
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub type AC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAC_` reader - DAC APB Clock Enable"]
pub type DAC__R = crate::BitReader;
#[doc = "Field `DAC_` writer - DAC APB Clock Enable"]
pub type DAC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTC_` reader - PTC APB Clock Enable"]
pub type PTC__R = crate::BitReader;
#[doc = "Field `PTC_` writer - PTC APB Clock Enable"]
pub type PTC__W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PAC2 APB Clock Enable"]
    #[inline(always)]
    pub fn pac2_(&self) -> PAC2__R {
        PAC2__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TCC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC APB Clock Enable"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pac2_(&mut self) -> PAC2__W<APBCMASK_SPEC, 0> {
        PAC2__W::new(self)
    }
    #[doc = "Bit 1 - EVSYS APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evsys_(&mut self) -> EVSYS__W<APBCMASK_SPEC, 1> {
        EVSYS__W::new(self)
    }
    #[doc = "Bit 2 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom0_(&mut self) -> SERCOM0__W<APBCMASK_SPEC, 2> {
        SERCOM0__W::new(self)
    }
    #[doc = "Bit 3 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom1_(&mut self) -> SERCOM1__W<APBCMASK_SPEC, 3> {
        SERCOM1__W::new(self)
    }
    #[doc = "Bit 4 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> SERCOM2__W<APBCMASK_SPEC, 4> {
        SERCOM2__W::new(self)
    }
    #[doc = "Bit 5 - TCC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc0_(&mut self) -> TCC0__W<APBCMASK_SPEC, 5> {
        TCC0__W::new(self)
    }
    #[doc = "Bit 6 - TC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc1_(&mut self) -> TC1__W<APBCMASK_SPEC, 6> {
        TC1__W::new(self)
    }
    #[doc = "Bit 7 - TC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc2_(&mut self) -> TC2__W<APBCMASK_SPEC, 7> {
        TC2__W::new(self)
    }
    #[doc = "Bit 8 - ADC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_(&mut self) -> ADC__W<APBCMASK_SPEC, 8> {
        ADC__W::new(self)
    }
    #[doc = "Bit 9 - AC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac_(&mut self) -> AC__W<APBCMASK_SPEC, 9> {
        AC__W::new(self)
    }
    #[doc = "Bit 10 - DAC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_(&mut self) -> DAC__W<APBCMASK_SPEC, 10> {
        DAC__W::new(self)
    }
    #[doc = "Bit 11 - PTC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ptc_(&mut self) -> PTC__W<APBCMASK_SPEC, 11> {
        PTC__W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APBC Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbcmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbcmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBCMASK_SPEC;
impl crate::RegisterSpec for APBCMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbcmask::R`](R) reader structure"]
impl crate::Readable for APBCMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbcmask::W`](W) writer structure"]
impl crate::Writable for APBCMASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBCMASK to value 0x0100"]
impl crate::Resettable for APBCMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
