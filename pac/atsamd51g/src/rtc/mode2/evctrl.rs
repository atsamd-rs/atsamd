#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EVCTRL_SPEC>;
#[doc = "Field `PEREO0` reader - Periodic Interval 0 Event Output Enable"]
pub type PEREO0_R = crate::BitReader;
#[doc = "Field `PEREO0` writer - Periodic Interval 0 Event Output Enable"]
pub type PEREO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEREO1` reader - Periodic Interval 1 Event Output Enable"]
pub type PEREO1_R = crate::BitReader;
#[doc = "Field `PEREO1` writer - Periodic Interval 1 Event Output Enable"]
pub type PEREO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEREO2` reader - Periodic Interval 2 Event Output Enable"]
pub type PEREO2_R = crate::BitReader;
#[doc = "Field `PEREO2` writer - Periodic Interval 2 Event Output Enable"]
pub type PEREO2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEREO3` reader - Periodic Interval 3 Event Output Enable"]
pub type PEREO3_R = crate::BitReader;
#[doc = "Field `PEREO3` writer - Periodic Interval 3 Event Output Enable"]
pub type PEREO3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEREO4` reader - Periodic Interval 4 Event Output Enable"]
pub type PEREO4_R = crate::BitReader;
#[doc = "Field `PEREO4` writer - Periodic Interval 4 Event Output Enable"]
pub type PEREO4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEREO5` reader - Periodic Interval 5 Event Output Enable"]
pub type PEREO5_R = crate::BitReader;
#[doc = "Field `PEREO5` writer - Periodic Interval 5 Event Output Enable"]
pub type PEREO5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEREO6` reader - Periodic Interval 6 Event Output Enable"]
pub type PEREO6_R = crate::BitReader;
#[doc = "Field `PEREO6` writer - Periodic Interval 6 Event Output Enable"]
pub type PEREO6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEREO7` reader - Periodic Interval 7 Event Output Enable"]
pub type PEREO7_R = crate::BitReader;
#[doc = "Field `PEREO7` writer - Periodic Interval 7 Event Output Enable"]
pub type PEREO7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALARMEO0` reader - Alarm 0 Event Output Enable"]
pub type ALARMEO0_R = crate::BitReader;
#[doc = "Field `ALARMEO0` writer - Alarm 0 Event Output Enable"]
pub type ALARMEO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALARMEO1` reader - Alarm 1 Event Output Enable"]
pub type ALARMEO1_R = crate::BitReader;
#[doc = "Field `ALARMEO1` writer - Alarm 1 Event Output Enable"]
pub type ALARMEO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAMPEREO` reader - Tamper Event Output Enable"]
pub type TAMPEREO_R = crate::BitReader;
#[doc = "Field `TAMPEREO` writer - Tamper Event Output Enable"]
pub type TAMPEREO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVFEO` reader - Overflow Event Output Enable"]
pub type OVFEO_R = crate::BitReader;
#[doc = "Field `OVFEO` writer - Overflow Event Output Enable"]
pub type OVFEO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAMPEVEI` reader - Tamper Event Input Enable"]
pub type TAMPEVEI_R = crate::BitReader;
#[doc = "Field `TAMPEVEI` writer - Tamper Event Input Enable"]
pub type TAMPEVEI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline(always)]
    pub fn pereo0(&self) -> PEREO0_R {
        PEREO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline(always)]
    pub fn pereo1(&self) -> PEREO1_R {
        PEREO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline(always)]
    pub fn pereo2(&self) -> PEREO2_R {
        PEREO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline(always)]
    pub fn pereo3(&self) -> PEREO3_R {
        PEREO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline(always)]
    pub fn pereo4(&self) -> PEREO4_R {
        PEREO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline(always)]
    pub fn pereo5(&self) -> PEREO5_R {
        PEREO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline(always)]
    pub fn pereo6(&self) -> PEREO6_R {
        PEREO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline(always)]
    pub fn pereo7(&self) -> PEREO7_R {
        PEREO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm 0 Event Output Enable"]
    #[inline(always)]
    pub fn alarmeo0(&self) -> ALARMEO0_R {
        ALARMEO0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm 1 Event Output Enable"]
    #[inline(always)]
    pub fn alarmeo1(&self) -> ALARMEO1_R {
        ALARMEO1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper Event Output Enable"]
    #[inline(always)]
    pub fn tampereo(&self) -> TAMPEREO_R {
        TAMPEREO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OVFEO_R {
        OVFEO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper Event Input Enable"]
    #[inline(always)]
    pub fn tampevei(&self) -> TAMPEVEI_R {
        TAMPEVEI_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pereo0(&mut self) -> PEREO0_W<EVCTRL_SPEC, 0> {
        PEREO0_W::new(self)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pereo1(&mut self) -> PEREO1_W<EVCTRL_SPEC, 1> {
        PEREO1_W::new(self)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pereo2(&mut self) -> PEREO2_W<EVCTRL_SPEC, 2> {
        PEREO2_W::new(self)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pereo3(&mut self) -> PEREO3_W<EVCTRL_SPEC, 3> {
        PEREO3_W::new(self)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pereo4(&mut self) -> PEREO4_W<EVCTRL_SPEC, 4> {
        PEREO4_W::new(self)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pereo5(&mut self) -> PEREO5_W<EVCTRL_SPEC, 5> {
        PEREO5_W::new(self)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pereo6(&mut self) -> PEREO6_W<EVCTRL_SPEC, 6> {
        PEREO6_W::new(self)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pereo7(&mut self) -> PEREO7_W<EVCTRL_SPEC, 7> {
        PEREO7_W::new(self)
    }
    #[doc = "Bit 8 - Alarm 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alarmeo0(&mut self) -> ALARMEO0_W<EVCTRL_SPEC, 8> {
        ALARMEO0_W::new(self)
    }
    #[doc = "Bit 9 - Alarm 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alarmeo1(&mut self) -> ALARMEO1_W<EVCTRL_SPEC, 9> {
        ALARMEO1_W::new(self)
    }
    #[doc = "Bit 14 - Tamper Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tampereo(&mut self) -> TAMPEREO_W<EVCTRL_SPEC, 14> {
        TAMPEREO_W::new(self)
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfeo(&mut self) -> OVFEO_W<EVCTRL_SPEC, 15> {
        OVFEO_W::new(self)
    }
    #[doc = "Bit 16 - Tamper Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tampevei(&mut self) -> TAMPEVEI_W<EVCTRL_SPEC, 16> {
        TAMPEVEI_W::new(self)
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
#[doc = "MODE2 Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
