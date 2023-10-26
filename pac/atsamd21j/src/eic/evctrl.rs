#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EVCTRL_SPEC>;
#[doc = "Field `EXTINTEO0` reader - External Interrupt 0 Event Output Enable"]
pub type EXTINTEO0_R = crate::BitReader;
#[doc = "Field `EXTINTEO0` writer - External Interrupt 0 Event Output Enable"]
pub type EXTINTEO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO1` reader - External Interrupt 1 Event Output Enable"]
pub type EXTINTEO1_R = crate::BitReader;
#[doc = "Field `EXTINTEO1` writer - External Interrupt 1 Event Output Enable"]
pub type EXTINTEO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO2` reader - External Interrupt 2 Event Output Enable"]
pub type EXTINTEO2_R = crate::BitReader;
#[doc = "Field `EXTINTEO2` writer - External Interrupt 2 Event Output Enable"]
pub type EXTINTEO2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO3` reader - External Interrupt 3 Event Output Enable"]
pub type EXTINTEO3_R = crate::BitReader;
#[doc = "Field `EXTINTEO3` writer - External Interrupt 3 Event Output Enable"]
pub type EXTINTEO3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO4` reader - External Interrupt 4 Event Output Enable"]
pub type EXTINTEO4_R = crate::BitReader;
#[doc = "Field `EXTINTEO4` writer - External Interrupt 4 Event Output Enable"]
pub type EXTINTEO4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO5` reader - External Interrupt 5 Event Output Enable"]
pub type EXTINTEO5_R = crate::BitReader;
#[doc = "Field `EXTINTEO5` writer - External Interrupt 5 Event Output Enable"]
pub type EXTINTEO5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO6` reader - External Interrupt 6 Event Output Enable"]
pub type EXTINTEO6_R = crate::BitReader;
#[doc = "Field `EXTINTEO6` writer - External Interrupt 6 Event Output Enable"]
pub type EXTINTEO6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO7` reader - External Interrupt 7 Event Output Enable"]
pub type EXTINTEO7_R = crate::BitReader;
#[doc = "Field `EXTINTEO7` writer - External Interrupt 7 Event Output Enable"]
pub type EXTINTEO7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO8` reader - External Interrupt 8 Event Output Enable"]
pub type EXTINTEO8_R = crate::BitReader;
#[doc = "Field `EXTINTEO8` writer - External Interrupt 8 Event Output Enable"]
pub type EXTINTEO8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO9` reader - External Interrupt 9 Event Output Enable"]
pub type EXTINTEO9_R = crate::BitReader;
#[doc = "Field `EXTINTEO9` writer - External Interrupt 9 Event Output Enable"]
pub type EXTINTEO9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO10` reader - External Interrupt 10 Event Output Enable"]
pub type EXTINTEO10_R = crate::BitReader;
#[doc = "Field `EXTINTEO10` writer - External Interrupt 10 Event Output Enable"]
pub type EXTINTEO10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO11` reader - External Interrupt 11 Event Output Enable"]
pub type EXTINTEO11_R = crate::BitReader;
#[doc = "Field `EXTINTEO11` writer - External Interrupt 11 Event Output Enable"]
pub type EXTINTEO11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO12` reader - External Interrupt 12 Event Output Enable"]
pub type EXTINTEO12_R = crate::BitReader;
#[doc = "Field `EXTINTEO12` writer - External Interrupt 12 Event Output Enable"]
pub type EXTINTEO12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO13` reader - External Interrupt 13 Event Output Enable"]
pub type EXTINTEO13_R = crate::BitReader;
#[doc = "Field `EXTINTEO13` writer - External Interrupt 13 Event Output Enable"]
pub type EXTINTEO13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO14` reader - External Interrupt 14 Event Output Enable"]
pub type EXTINTEO14_R = crate::BitReader;
#[doc = "Field `EXTINTEO14` writer - External Interrupt 14 Event Output Enable"]
pub type EXTINTEO14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTINTEO15` reader - External Interrupt 15 Event Output Enable"]
pub type EXTINTEO15_R = crate::BitReader;
#[doc = "Field `EXTINTEO15` writer - External Interrupt 15 Event Output Enable"]
pub type EXTINTEO15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo0(&self) -> EXTINTEO0_R {
        EXTINTEO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo1(&self) -> EXTINTEO1_R {
        EXTINTEO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo2(&self) -> EXTINTEO2_R {
        EXTINTEO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo3(&self) -> EXTINTEO3_R {
        EXTINTEO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo4(&self) -> EXTINTEO4_R {
        EXTINTEO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo5(&self) -> EXTINTEO5_R {
        EXTINTEO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo6(&self) -> EXTINTEO6_R {
        EXTINTEO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo7(&self) -> EXTINTEO7_R {
        EXTINTEO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo8(&self) -> EXTINTEO8_R {
        EXTINTEO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo9(&self) -> EXTINTEO9_R {
        EXTINTEO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo10(&self) -> EXTINTEO10_R {
        EXTINTEO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo11(&self) -> EXTINTEO11_R {
        EXTINTEO11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo12(&self) -> EXTINTEO12_R {
        EXTINTEO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo13(&self) -> EXTINTEO13_R {
        EXTINTEO13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo14(&self) -> EXTINTEO14_R {
        EXTINTEO14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo15(&self) -> EXTINTEO15_R {
        EXTINTEO15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo0(&mut self) -> EXTINTEO0_W<EVCTRL_SPEC, 0> {
        EXTINTEO0_W::new(self)
    }
    #[doc = "Bit 1 - External Interrupt 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo1(&mut self) -> EXTINTEO1_W<EVCTRL_SPEC, 1> {
        EXTINTEO1_W::new(self)
    }
    #[doc = "Bit 2 - External Interrupt 2 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo2(&mut self) -> EXTINTEO2_W<EVCTRL_SPEC, 2> {
        EXTINTEO2_W::new(self)
    }
    #[doc = "Bit 3 - External Interrupt 3 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo3(&mut self) -> EXTINTEO3_W<EVCTRL_SPEC, 3> {
        EXTINTEO3_W::new(self)
    }
    #[doc = "Bit 4 - External Interrupt 4 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo4(&mut self) -> EXTINTEO4_W<EVCTRL_SPEC, 4> {
        EXTINTEO4_W::new(self)
    }
    #[doc = "Bit 5 - External Interrupt 5 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo5(&mut self) -> EXTINTEO5_W<EVCTRL_SPEC, 5> {
        EXTINTEO5_W::new(self)
    }
    #[doc = "Bit 6 - External Interrupt 6 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo6(&mut self) -> EXTINTEO6_W<EVCTRL_SPEC, 6> {
        EXTINTEO6_W::new(self)
    }
    #[doc = "Bit 7 - External Interrupt 7 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo7(&mut self) -> EXTINTEO7_W<EVCTRL_SPEC, 7> {
        EXTINTEO7_W::new(self)
    }
    #[doc = "Bit 8 - External Interrupt 8 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo8(&mut self) -> EXTINTEO8_W<EVCTRL_SPEC, 8> {
        EXTINTEO8_W::new(self)
    }
    #[doc = "Bit 9 - External Interrupt 9 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo9(&mut self) -> EXTINTEO9_W<EVCTRL_SPEC, 9> {
        EXTINTEO9_W::new(self)
    }
    #[doc = "Bit 10 - External Interrupt 10 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo10(&mut self) -> EXTINTEO10_W<EVCTRL_SPEC, 10> {
        EXTINTEO10_W::new(self)
    }
    #[doc = "Bit 11 - External Interrupt 11 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo11(&mut self) -> EXTINTEO11_W<EVCTRL_SPEC, 11> {
        EXTINTEO11_W::new(self)
    }
    #[doc = "Bit 12 - External Interrupt 12 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo12(&mut self) -> EXTINTEO12_W<EVCTRL_SPEC, 12> {
        EXTINTEO12_W::new(self)
    }
    #[doc = "Bit 13 - External Interrupt 13 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo13(&mut self) -> EXTINTEO13_W<EVCTRL_SPEC, 13> {
        EXTINTEO13_W::new(self)
    }
    #[doc = "Bit 14 - External Interrupt 14 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo14(&mut self) -> EXTINTEO14_W<EVCTRL_SPEC, 14> {
        EXTINTEO14_W::new(self)
    }
    #[doc = "Bit 15 - External Interrupt 15 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo15(&mut self) -> EXTINTEO15_W<EVCTRL_SPEC, 15> {
        EXTINTEO15_W::new(self)
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
#[doc = "Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
