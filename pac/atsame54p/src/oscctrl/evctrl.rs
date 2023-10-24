#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EVCTRL_SPEC>;
#[doc = "Field `CFDEO0` reader - Clock 0 Failure Detector Event Output Enable"]
pub type CFDEO0_R = crate::BitReader;
#[doc = "Field `CFDEO0` writer - Clock 0 Failure Detector Event Output Enable"]
pub type CFDEO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFDEO1` reader - Clock 1 Failure Detector Event Output Enable"]
pub type CFDEO1_R = crate::BitReader;
#[doc = "Field `CFDEO1` writer - Clock 1 Failure Detector Event Output Enable"]
pub type CFDEO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Clock 0 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo0(&self) -> CFDEO0_R {
        CFDEO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock 1 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo1(&self) -> CFDEO1_R {
        CFDEO1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock 0 Failure Detector Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdeo0(&mut self) -> CFDEO0_W<EVCTRL_SPEC, 0> {
        CFDEO0_W::new(self)
    }
    #[doc = "Bit 1 - Clock 1 Failure Detector Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdeo1(&mut self) -> CFDEO1_W<EVCTRL_SPEC, 1> {
        CFDEO1_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u8;
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
