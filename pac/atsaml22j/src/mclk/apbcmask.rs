#[doc = "Register `APBCMASK` reader"]
pub struct R(crate::R<APBCMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBCMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBCMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBCMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBCMASK` writer"]
pub struct W(crate::W<APBCMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBCMASK_SPEC>;
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
impl From<crate::W<APBCMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBCMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS APB Clock Enable"]
pub type EVSYS__R = crate::BitReader<bool>;
#[doc = "Field `EVSYS_` writer - EVSYS APB Clock Enable"]
pub type EVSYS__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Clock Enable"]
pub type SERCOM0__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM0_` writer - SERCOM0 APB Clock Enable"]
pub type SERCOM0__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Clock Enable"]
pub type SERCOM1__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM1_` writer - SERCOM1 APB Clock Enable"]
pub type SERCOM1__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Clock Enable"]
pub type SERCOM2__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM2_` writer - SERCOM2 APB Clock Enable"]
pub type SERCOM2__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Clock Enable"]
pub type SERCOM3__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM3_` writer - SERCOM3 APB Clock Enable"]
pub type SERCOM3__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TCC0_` reader - TCC0 APB Clock Enable"]
pub type TCC0__R = crate::BitReader<bool>;
#[doc = "Field `TCC0_` writer - TCC0 APB Clock Enable"]
pub type TCC0__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TC0_` reader - TC0 APB Clock Enable"]
pub type TC0__R = crate::BitReader<bool>;
#[doc = "Field `TC0_` writer - TC0 APB Clock Enable"]
pub type TC0__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TC1_` reader - TC1 APB Clock Enable"]
pub type TC1__R = crate::BitReader<bool>;
#[doc = "Field `TC1_` writer - TC1 APB Clock Enable"]
pub type TC1__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TC2_` reader - TC2 APB Clock Enable"]
pub type TC2__R = crate::BitReader<bool>;
#[doc = "Field `TC2_` writer - TC2 APB Clock Enable"]
pub type TC2__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TC3_` reader - TC3 APB Clock Enable"]
pub type TC3__R = crate::BitReader<bool>;
#[doc = "Field `TC3_` writer - TC3 APB Clock Enable"]
pub type TC3__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `ADC_` reader - ADC APB Clock Enable"]
pub type ADC__R = crate::BitReader<bool>;
#[doc = "Field `ADC_` writer - ADC APB Clock Enable"]
pub type ADC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub type AC__R = crate::BitReader<bool>;
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub type AC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `PTC_` reader - PTC APB Clock Enable"]
pub type PTC__R = crate::BitReader<bool>;
#[doc = "Field `PTC_` writer - PTC APB Clock Enable"]
pub type PTC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `SLCD_` reader - SLCD APB Clock Enable"]
pub type SLCD__R = crate::BitReader<bool>;
#[doc = "Field `SLCD_` writer - SLCD APB Clock Enable"]
pub type SLCD__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `AES_` reader - AES APB Clock Enable"]
pub type AES__R = crate::BitReader<bool>;
#[doc = "Field `AES_` writer - AES APB Clock Enable"]
pub type AES__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TRNG_` reader - TRNG APB Clock Enable"]
pub type TRNG__R = crate::BitReader<bool>;
#[doc = "Field `TRNG_` writer - TRNG APB Clock Enable"]
pub type TRNG__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `CCL_` reader - CCL APB Clock Enable"]
pub type CCL__R = crate::BitReader<bool>;
#[doc = "Field `CCL_` writer - CCL APB Clock Enable"]
pub type CCL__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - TCC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC APB Clock Enable"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SLCD APB Clock Enable"]
    #[inline(always)]
    pub fn slcd_(&self) -> SLCD__R {
        SLCD__R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AES APB Clock Enable"]
    #[inline(always)]
    pub fn aes_(&self) -> AES__R {
        AES__R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EVSYS APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evsys_(&mut self) -> EVSYS__W<0> {
        EVSYS__W::new(self)
    }
    #[doc = "Bit 1 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom0_(&mut self) -> SERCOM0__W<1> {
        SERCOM0__W::new(self)
    }
    #[doc = "Bit 2 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom1_(&mut self) -> SERCOM1__W<2> {
        SERCOM1__W::new(self)
    }
    #[doc = "Bit 3 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> SERCOM2__W<3> {
        SERCOM2__W::new(self)
    }
    #[doc = "Bit 4 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom3_(&mut self) -> SERCOM3__W<4> {
        SERCOM3__W::new(self)
    }
    #[doc = "Bit 7 - TCC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc0_(&mut self) -> TCC0__W<7> {
        TCC0__W::new(self)
    }
    #[doc = "Bit 8 - TC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc0_(&mut self) -> TC0__W<8> {
        TC0__W::new(self)
    }
    #[doc = "Bit 9 - TC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc1_(&mut self) -> TC1__W<9> {
        TC1__W::new(self)
    }
    #[doc = "Bit 10 - TC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc2_(&mut self) -> TC2__W<10> {
        TC2__W::new(self)
    }
    #[doc = "Bit 11 - TC3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc3_(&mut self) -> TC3__W<11> {
        TC3__W::new(self)
    }
    #[doc = "Bit 12 - ADC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_(&mut self) -> ADC__W<12> {
        ADC__W::new(self)
    }
    #[doc = "Bit 13 - AC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac_(&mut self) -> AC__W<13> {
        AC__W::new(self)
    }
    #[doc = "Bit 14 - PTC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ptc_(&mut self) -> PTC__W<14> {
        PTC__W::new(self)
    }
    #[doc = "Bit 15 - SLCD APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn slcd_(&mut self) -> SLCD__W<15> {
        SLCD__W::new(self)
    }
    #[doc = "Bit 16 - AES APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aes_(&mut self) -> AES__W<16> {
        AES__W::new(self)
    }
    #[doc = "Bit 17 - TRNG APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trng_(&mut self) -> TRNG__W<17> {
        TRNG__W::new(self)
    }
    #[doc = "Bit 18 - CCL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccl_(&mut self) -> CCL__W<18> {
        CCL__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBC Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbcmask](index.html) module"]
pub struct APBCMASK_SPEC;
impl crate::RegisterSpec for APBCMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbcmask::R](R) reader structure"]
impl crate::Readable for APBCMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbcmask::W](W) writer structure"]
impl crate::Writable for APBCMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBCMASK to value 0x0007_ffff"]
impl crate::Resettable for APBCMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_ffff;
}
