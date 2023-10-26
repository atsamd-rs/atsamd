#[doc = "Register `DRVCTRL` reader"]
pub type R = crate::R<DRVCTRL_SPEC>;
#[doc = "Register `DRVCTRL` writer"]
pub type W = crate::W<DRVCTRL_SPEC>;
#[doc = "Field `INVEN0` reader - Output Waveform Invert Enable 0"]
pub type INVEN0_R = crate::BitReader;
#[doc = "Field `INVEN0` writer - Output Waveform Invert Enable 0"]
pub type INVEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INVEN1` reader - Output Waveform Invert Enable 1"]
pub type INVEN1_R = crate::BitReader;
#[doc = "Field `INVEN1` writer - Output Waveform Invert Enable 1"]
pub type INVEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Output Waveform Invert Enable 0"]
    #[inline(always)]
    pub fn inven0(&self) -> INVEN0_R {
        INVEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Waveform Invert Enable 1"]
    #[inline(always)]
    pub fn inven1(&self) -> INVEN1_R {
        INVEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Waveform Invert Enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn inven0(&mut self) -> INVEN0_W<DRVCTRL_SPEC, 0> {
        INVEN0_W::new(self)
    }
    #[doc = "Bit 1 - Output Waveform Invert Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn inven1(&mut self) -> INVEN1_W<DRVCTRL_SPEC, 1> {
        INVEN1_W::new(self)
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
#[doc = "Control C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drvctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drvctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRVCTRL_SPEC;
impl crate::RegisterSpec for DRVCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`drvctrl::R`](R) reader structure"]
impl crate::Readable for DRVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`drvctrl::W`](W) writer structure"]
impl crate::Writable for DRVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRVCTRL to value 0"]
impl crate::Resettable for DRVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
