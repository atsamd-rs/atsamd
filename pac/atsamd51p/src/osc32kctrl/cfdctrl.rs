#[doc = "Register `CFDCTRL` reader"]
pub type R = crate::R<CFDCTRL_SPEC>;
#[doc = "Register `CFDCTRL` writer"]
pub type W = crate::W<CFDCTRL_SPEC>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CFDEN_R = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CFDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWBACK` reader - Clock Switch Back"]
pub type SWBACK_R = crate::BitReader;
#[doc = "Field `SWBACK` writer - Clock Switch Back"]
pub type SWBACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFDPRESC` reader - Clock Failure Detector Prescaler"]
pub type CFDPRESC_R = crate::BitReader;
#[doc = "Field `CFDPRESC` writer - Clock Failure Detector Prescaler"]
pub type CFDPRESC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Switch Back"]
    #[inline(always)]
    pub fn swback(&self) -> SWBACK_R {
        SWBACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CFDPRESC_R {
        CFDPRESC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<CFDCTRL_SPEC, 0> {
        CFDEN_W::new(self)
    }
    #[doc = "Bit 1 - Clock Switch Back"]
    #[inline(always)]
    #[must_use]
    pub fn swback(&mut self) -> SWBACK_W<CFDCTRL_SPEC, 1> {
        SWBACK_W::new(self)
    }
    #[doc = "Bit 2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn cfdpresc(&mut self) -> CFDPRESC_W<CFDCTRL_SPEC, 2> {
        CFDPRESC_W::new(self)
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
#[doc = "Clock Failure Detector Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfdctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfdctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFDCTRL_SPEC;
impl crate::RegisterSpec for CFDCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cfdctrl::R`](R) reader structure"]
impl crate::Readable for CFDCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfdctrl::W`](W) writer structure"]
impl crate::Writable for CFDCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDCTRL to value 0"]
impl crate::Resettable for CFDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
