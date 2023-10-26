#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EVCTRL_SPEC>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EVCTRL_SPEC>;
#[doc = "Field `STARTEI0` reader - Start Conversion Event Input DAC 0"]
pub type STARTEI0_R = crate::BitReader;
#[doc = "Field `STARTEI0` writer - Start Conversion Event Input DAC 0"]
pub type STARTEI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STARTEI1` reader - Start Conversion Event Input DAC 1"]
pub type STARTEI1_R = crate::BitReader;
#[doc = "Field `STARTEI1` writer - Start Conversion Event Input DAC 1"]
pub type STARTEI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMPTYEO0` reader - Data Buffer Empty Event Output DAC 0"]
pub type EMPTYEO0_R = crate::BitReader;
#[doc = "Field `EMPTYEO0` writer - Data Buffer Empty Event Output DAC 0"]
pub type EMPTYEO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMPTYEO1` reader - Data Buffer Empty Event Output DAC 1"]
pub type EMPTYEO1_R = crate::BitReader;
#[doc = "Field `EMPTYEO1` writer - Data Buffer Empty Event Output DAC 1"]
pub type EMPTYEO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INVEI0` reader - Enable Invertion of DAC 0 input event"]
pub type INVEI0_R = crate::BitReader;
#[doc = "Field `INVEI0` writer - Enable Invertion of DAC 0 input event"]
pub type INVEI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INVEI1` reader - Enable Invertion of DAC 1 input event"]
pub type INVEI1_R = crate::BitReader;
#[doc = "Field `INVEI1` writer - Enable Invertion of DAC 1 input event"]
pub type INVEI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESRDYEO0` reader - Result Ready Event Output 0"]
pub type RESRDYEO0_R = crate::BitReader;
#[doc = "Field `RESRDYEO0` writer - Result Ready Event Output 0"]
pub type RESRDYEO0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESRDYEO1` reader - Result Ready Event Output 1"]
pub type RESRDYEO1_R = crate::BitReader;
#[doc = "Field `RESRDYEO1` writer - Result Ready Event Output 1"]
pub type RESRDYEO1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Start Conversion Event Input DAC 0"]
    #[inline(always)]
    pub fn startei0(&self) -> STARTEI0_R {
        STARTEI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Input DAC 1"]
    #[inline(always)]
    pub fn startei1(&self) -> STARTEI1_R {
        STARTEI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Buffer Empty Event Output DAC 0"]
    #[inline(always)]
    pub fn emptyeo0(&self) -> EMPTYEO0_R {
        EMPTYEO0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Buffer Empty Event Output DAC 1"]
    #[inline(always)]
    pub fn emptyeo1(&self) -> EMPTYEO1_R {
        EMPTYEO1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Invertion of DAC 0 input event"]
    #[inline(always)]
    pub fn invei0(&self) -> INVEI0_R {
        INVEI0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Invertion of DAC 1 input event"]
    #[inline(always)]
    pub fn invei1(&self) -> INVEI1_R {
        INVEI1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Result Ready Event Output 0"]
    #[inline(always)]
    pub fn resrdyeo0(&self) -> RESRDYEO0_R {
        RESRDYEO0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Result Ready Event Output 1"]
    #[inline(always)]
    pub fn resrdyeo1(&self) -> RESRDYEO1_R {
        RESRDYEO1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event Input DAC 0"]
    #[inline(always)]
    #[must_use]
    pub fn startei0(&mut self) -> STARTEI0_W<EVCTRL_SPEC, 0> {
        STARTEI0_W::new(self)
    }
    #[doc = "Bit 1 - Start Conversion Event Input DAC 1"]
    #[inline(always)]
    #[must_use]
    pub fn startei1(&mut self) -> STARTEI1_W<EVCTRL_SPEC, 1> {
        STARTEI1_W::new(self)
    }
    #[doc = "Bit 2 - Data Buffer Empty Event Output DAC 0"]
    #[inline(always)]
    #[must_use]
    pub fn emptyeo0(&mut self) -> EMPTYEO0_W<EVCTRL_SPEC, 2> {
        EMPTYEO0_W::new(self)
    }
    #[doc = "Bit 3 - Data Buffer Empty Event Output DAC 1"]
    #[inline(always)]
    #[must_use]
    pub fn emptyeo1(&mut self) -> EMPTYEO1_W<EVCTRL_SPEC, 3> {
        EMPTYEO1_W::new(self)
    }
    #[doc = "Bit 4 - Enable Invertion of DAC 0 input event"]
    #[inline(always)]
    #[must_use]
    pub fn invei0(&mut self) -> INVEI0_W<EVCTRL_SPEC, 4> {
        INVEI0_W::new(self)
    }
    #[doc = "Bit 5 - Enable Invertion of DAC 1 input event"]
    #[inline(always)]
    #[must_use]
    pub fn invei1(&mut self) -> INVEI1_W<EVCTRL_SPEC, 5> {
        INVEI1_W::new(self)
    }
    #[doc = "Bit 6 - Result Ready Event Output 0"]
    #[inline(always)]
    #[must_use]
    pub fn resrdyeo0(&mut self) -> RESRDYEO0_W<EVCTRL_SPEC, 6> {
        RESRDYEO0_W::new(self)
    }
    #[doc = "Bit 7 - Result Ready Event Output 1"]
    #[inline(always)]
    #[must_use]
    pub fn resrdyeo1(&mut self) -> RESRDYEO1_W<EVCTRL_SPEC, 7> {
        RESRDYEO1_W::new(self)
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
