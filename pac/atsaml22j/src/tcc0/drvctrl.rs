#[doc = "Register `DRVCTRL` reader"]
pub struct R(crate::R<DRVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRVCTRL` writer"]
pub struct W(crate::W<DRVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRVCTRL_SPEC>;
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
impl From<crate::W<DRVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NRE0` reader - Non-Recoverable State 0 Output Enable"]
pub type NRE0_R = crate::BitReader<bool>;
#[doc = "Field `NRE0` writer - Non-Recoverable State 0 Output Enable"]
pub type NRE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRE1` reader - Non-Recoverable State 1 Output Enable"]
pub type NRE1_R = crate::BitReader<bool>;
#[doc = "Field `NRE1` writer - Non-Recoverable State 1 Output Enable"]
pub type NRE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRE2` reader - Non-Recoverable State 2 Output Enable"]
pub type NRE2_R = crate::BitReader<bool>;
#[doc = "Field `NRE2` writer - Non-Recoverable State 2 Output Enable"]
pub type NRE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRE3` reader - Non-Recoverable State 3 Output Enable"]
pub type NRE3_R = crate::BitReader<bool>;
#[doc = "Field `NRE3` writer - Non-Recoverable State 3 Output Enable"]
pub type NRE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRE4` reader - Non-Recoverable State 4 Output Enable"]
pub type NRE4_R = crate::BitReader<bool>;
#[doc = "Field `NRE4` writer - Non-Recoverable State 4 Output Enable"]
pub type NRE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRE5` reader - Non-Recoverable State 5 Output Enable"]
pub type NRE5_R = crate::BitReader<bool>;
#[doc = "Field `NRE5` writer - Non-Recoverable State 5 Output Enable"]
pub type NRE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRE6` reader - Non-Recoverable State 6 Output Enable"]
pub type NRE6_R = crate::BitReader<bool>;
#[doc = "Field `NRE6` writer - Non-Recoverable State 6 Output Enable"]
pub type NRE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRE7` reader - Non-Recoverable State 7 Output Enable"]
pub type NRE7_R = crate::BitReader<bool>;
#[doc = "Field `NRE7` writer - Non-Recoverable State 7 Output Enable"]
pub type NRE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRV0` reader - Non-Recoverable State 0 Output Value"]
pub type NRV0_R = crate::BitReader<bool>;
#[doc = "Field `NRV0` writer - Non-Recoverable State 0 Output Value"]
pub type NRV0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRV1` reader - Non-Recoverable State 1 Output Value"]
pub type NRV1_R = crate::BitReader<bool>;
#[doc = "Field `NRV1` writer - Non-Recoverable State 1 Output Value"]
pub type NRV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRV2` reader - Non-Recoverable State 2 Output Value"]
pub type NRV2_R = crate::BitReader<bool>;
#[doc = "Field `NRV2` writer - Non-Recoverable State 2 Output Value"]
pub type NRV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRV3` reader - Non-Recoverable State 3 Output Value"]
pub type NRV3_R = crate::BitReader<bool>;
#[doc = "Field `NRV3` writer - Non-Recoverable State 3 Output Value"]
pub type NRV3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRV4` reader - Non-Recoverable State 4 Output Value"]
pub type NRV4_R = crate::BitReader<bool>;
#[doc = "Field `NRV4` writer - Non-Recoverable State 4 Output Value"]
pub type NRV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRV5` reader - Non-Recoverable State 5 Output Value"]
pub type NRV5_R = crate::BitReader<bool>;
#[doc = "Field `NRV5` writer - Non-Recoverable State 5 Output Value"]
pub type NRV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRV6` reader - Non-Recoverable State 6 Output Value"]
pub type NRV6_R = crate::BitReader<bool>;
#[doc = "Field `NRV6` writer - Non-Recoverable State 6 Output Value"]
pub type NRV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `NRV7` reader - Non-Recoverable State 7 Output Value"]
pub type NRV7_R = crate::BitReader<bool>;
#[doc = "Field `NRV7` writer - Non-Recoverable State 7 Output Value"]
pub type NRV7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEN0` reader - Output Waveform 0 Inversion"]
pub type INVEN0_R = crate::BitReader<bool>;
#[doc = "Field `INVEN0` writer - Output Waveform 0 Inversion"]
pub type INVEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEN1` reader - Output Waveform 1 Inversion"]
pub type INVEN1_R = crate::BitReader<bool>;
#[doc = "Field `INVEN1` writer - Output Waveform 1 Inversion"]
pub type INVEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEN2` reader - Output Waveform 2 Inversion"]
pub type INVEN2_R = crate::BitReader<bool>;
#[doc = "Field `INVEN2` writer - Output Waveform 2 Inversion"]
pub type INVEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEN3` reader - Output Waveform 3 Inversion"]
pub type INVEN3_R = crate::BitReader<bool>;
#[doc = "Field `INVEN3` writer - Output Waveform 3 Inversion"]
pub type INVEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEN4` reader - Output Waveform 4 Inversion"]
pub type INVEN4_R = crate::BitReader<bool>;
#[doc = "Field `INVEN4` writer - Output Waveform 4 Inversion"]
pub type INVEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEN5` reader - Output Waveform 5 Inversion"]
pub type INVEN5_R = crate::BitReader<bool>;
#[doc = "Field `INVEN5` writer - Output Waveform 5 Inversion"]
pub type INVEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEN6` reader - Output Waveform 6 Inversion"]
pub type INVEN6_R = crate::BitReader<bool>;
#[doc = "Field `INVEN6` writer - Output Waveform 6 Inversion"]
pub type INVEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEN7` reader - Output Waveform 7 Inversion"]
pub type INVEN7_R = crate::BitReader<bool>;
#[doc = "Field `INVEN7` writer - Output Waveform 7 Inversion"]
pub type INVEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `FILTERVAL0` reader - Non-Recoverable Fault Input 0 Filter Value"]
pub type FILTERVAL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTERVAL0` writer - Non-Recoverable Fault Input 0 Filter Value"]
pub type FILTERVAL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRVCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `FILTERVAL1` reader - Non-Recoverable Fault Input 1 Filter Value"]
pub type FILTERVAL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTERVAL1` writer - Non-Recoverable Fault Input 1 Filter Value"]
pub type FILTERVAL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRVCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Non-Recoverable State 0 Output Enable"]
    #[inline(always)]
    pub fn nre0(&self) -> NRE0_R {
        NRE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-Recoverable State 1 Output Enable"]
    #[inline(always)]
    pub fn nre1(&self) -> NRE1_R {
        NRE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-Recoverable State 2 Output Enable"]
    #[inline(always)]
    pub fn nre2(&self) -> NRE2_R {
        NRE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-Recoverable State 3 Output Enable"]
    #[inline(always)]
    pub fn nre3(&self) -> NRE3_R {
        NRE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Non-Recoverable State 4 Output Enable"]
    #[inline(always)]
    pub fn nre4(&self) -> NRE4_R {
        NRE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-Recoverable State 5 Output Enable"]
    #[inline(always)]
    pub fn nre5(&self) -> NRE5_R {
        NRE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Non-Recoverable State 6 Output Enable"]
    #[inline(always)]
    pub fn nre6(&self) -> NRE6_R {
        NRE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-Recoverable State 7 Output Enable"]
    #[inline(always)]
    pub fn nre7(&self) -> NRE7_R {
        NRE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Non-Recoverable State 0 Output Value"]
    #[inline(always)]
    pub fn nrv0(&self) -> NRV0_R {
        NRV0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Non-Recoverable State 1 Output Value"]
    #[inline(always)]
    pub fn nrv1(&self) -> NRV1_R {
        NRV1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non-Recoverable State 2 Output Value"]
    #[inline(always)]
    pub fn nrv2(&self) -> NRV2_R {
        NRV2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Non-Recoverable State 3 Output Value"]
    #[inline(always)]
    pub fn nrv3(&self) -> NRV3_R {
        NRV3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Non-Recoverable State 4 Output Value"]
    #[inline(always)]
    pub fn nrv4(&self) -> NRV4_R {
        NRV4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Non-Recoverable State 5 Output Value"]
    #[inline(always)]
    pub fn nrv5(&self) -> NRV5_R {
        NRV5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Non-Recoverable State 6 Output Value"]
    #[inline(always)]
    pub fn nrv6(&self) -> NRV6_R {
        NRV6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non-Recoverable State 7 Output Value"]
    #[inline(always)]
    pub fn nrv7(&self) -> NRV7_R {
        NRV7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Waveform 0 Inversion"]
    #[inline(always)]
    pub fn inven0(&self) -> INVEN0_R {
        INVEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output Waveform 1 Inversion"]
    #[inline(always)]
    pub fn inven1(&self) -> INVEN1_R {
        INVEN1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Waveform 2 Inversion"]
    #[inline(always)]
    pub fn inven2(&self) -> INVEN2_R {
        INVEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output Waveform 3 Inversion"]
    #[inline(always)]
    pub fn inven3(&self) -> INVEN3_R {
        INVEN3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output Waveform 4 Inversion"]
    #[inline(always)]
    pub fn inven4(&self) -> INVEN4_R {
        INVEN4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output Waveform 5 Inversion"]
    #[inline(always)]
    pub fn inven5(&self) -> INVEN5_R {
        INVEN5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output Waveform 6 Inversion"]
    #[inline(always)]
    pub fn inven6(&self) -> INVEN6_R {
        INVEN6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output Waveform 7 Inversion"]
    #[inline(always)]
    pub fn inven7(&self) -> INVEN7_R {
        INVEN7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Non-Recoverable Fault Input 0 Filter Value"]
    #[inline(always)]
    pub fn filterval0(&self) -> FILTERVAL0_R {
        FILTERVAL0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Non-Recoverable Fault Input 1 Filter Value"]
    #[inline(always)]
    pub fn filterval1(&self) -> FILTERVAL1_R {
        FILTERVAL1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Recoverable State 0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nre0(&mut self) -> NRE0_W<0> {
        NRE0_W::new(self)
    }
    #[doc = "Bit 1 - Non-Recoverable State 1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nre1(&mut self) -> NRE1_W<1> {
        NRE1_W::new(self)
    }
    #[doc = "Bit 2 - Non-Recoverable State 2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nre2(&mut self) -> NRE2_W<2> {
        NRE2_W::new(self)
    }
    #[doc = "Bit 3 - Non-Recoverable State 3 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nre3(&mut self) -> NRE3_W<3> {
        NRE3_W::new(self)
    }
    #[doc = "Bit 4 - Non-Recoverable State 4 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nre4(&mut self) -> NRE4_W<4> {
        NRE4_W::new(self)
    }
    #[doc = "Bit 5 - Non-Recoverable State 5 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nre5(&mut self) -> NRE5_W<5> {
        NRE5_W::new(self)
    }
    #[doc = "Bit 6 - Non-Recoverable State 6 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nre6(&mut self) -> NRE6_W<6> {
        NRE6_W::new(self)
    }
    #[doc = "Bit 7 - Non-Recoverable State 7 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nre7(&mut self) -> NRE7_W<7> {
        NRE7_W::new(self)
    }
    #[doc = "Bit 8 - Non-Recoverable State 0 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn nrv0(&mut self) -> NRV0_W<8> {
        NRV0_W::new(self)
    }
    #[doc = "Bit 9 - Non-Recoverable State 1 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn nrv1(&mut self) -> NRV1_W<9> {
        NRV1_W::new(self)
    }
    #[doc = "Bit 10 - Non-Recoverable State 2 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn nrv2(&mut self) -> NRV2_W<10> {
        NRV2_W::new(self)
    }
    #[doc = "Bit 11 - Non-Recoverable State 3 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn nrv3(&mut self) -> NRV3_W<11> {
        NRV3_W::new(self)
    }
    #[doc = "Bit 12 - Non-Recoverable State 4 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn nrv4(&mut self) -> NRV4_W<12> {
        NRV4_W::new(self)
    }
    #[doc = "Bit 13 - Non-Recoverable State 5 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn nrv5(&mut self) -> NRV5_W<13> {
        NRV5_W::new(self)
    }
    #[doc = "Bit 14 - Non-Recoverable State 6 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn nrv6(&mut self) -> NRV6_W<14> {
        NRV6_W::new(self)
    }
    #[doc = "Bit 15 - Non-Recoverable State 7 Output Value"]
    #[inline(always)]
    #[must_use]
    pub fn nrv7(&mut self) -> NRV7_W<15> {
        NRV7_W::new(self)
    }
    #[doc = "Bit 16 - Output Waveform 0 Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn inven0(&mut self) -> INVEN0_W<16> {
        INVEN0_W::new(self)
    }
    #[doc = "Bit 17 - Output Waveform 1 Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn inven1(&mut self) -> INVEN1_W<17> {
        INVEN1_W::new(self)
    }
    #[doc = "Bit 18 - Output Waveform 2 Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn inven2(&mut self) -> INVEN2_W<18> {
        INVEN2_W::new(self)
    }
    #[doc = "Bit 19 - Output Waveform 3 Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn inven3(&mut self) -> INVEN3_W<19> {
        INVEN3_W::new(self)
    }
    #[doc = "Bit 20 - Output Waveform 4 Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn inven4(&mut self) -> INVEN4_W<20> {
        INVEN4_W::new(self)
    }
    #[doc = "Bit 21 - Output Waveform 5 Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn inven5(&mut self) -> INVEN5_W<21> {
        INVEN5_W::new(self)
    }
    #[doc = "Bit 22 - Output Waveform 6 Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn inven6(&mut self) -> INVEN6_W<22> {
        INVEN6_W::new(self)
    }
    #[doc = "Bit 23 - Output Waveform 7 Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn inven7(&mut self) -> INVEN7_W<23> {
        INVEN7_W::new(self)
    }
    #[doc = "Bits 24:27 - Non-Recoverable Fault Input 0 Filter Value"]
    #[inline(always)]
    #[must_use]
    pub fn filterval0(&mut self) -> FILTERVAL0_W<24> {
        FILTERVAL0_W::new(self)
    }
    #[doc = "Bits 28:31 - Non-Recoverable Fault Input 1 Filter Value"]
    #[inline(always)]
    #[must_use]
    pub fn filterval1(&mut self) -> FILTERVAL1_W<28> {
        FILTERVAL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Driver Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drvctrl](index.html) module"]
pub struct DRVCTRL_SPEC;
impl crate::RegisterSpec for DRVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [drvctrl::R](R) reader structure"]
impl crate::Readable for DRVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drvctrl::W](W) writer structure"]
impl crate::Writable for DRVCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRVCTRL to value 0"]
impl crate::Resettable for DRVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
