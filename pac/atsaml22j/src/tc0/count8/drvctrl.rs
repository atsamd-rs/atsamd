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
#[doc = "Field `INVEN0` reader - Output Waveform Invert Enable 0"]
pub type INVEN0_R = crate::BitReader<bool>;
#[doc = "Field `INVEN0` writer - Output Waveform Invert Enable 0"]
pub type INVEN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, DRVCTRL_SPEC, bool, O>;
#[doc = "Field `INVEN1` reader - Output Waveform Invert Enable 1"]
pub type INVEN1_R = crate::BitReader<bool>;
#[doc = "Field `INVEN1` writer - Output Waveform Invert Enable 1"]
pub type INVEN1_W<'a, const O: u8> = crate::BitWriter<'a, u8, DRVCTRL_SPEC, bool, O>;
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
    pub fn inven0(&mut self) -> INVEN0_W<0> {
        INVEN0_W::new(self)
    }
    #[doc = "Bit 1 - Output Waveform Invert Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn inven1(&mut self) -> INVEN1_W<1> {
        INVEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drvctrl](index.html) module"]
pub struct DRVCTRL_SPEC;
impl crate::RegisterSpec for DRVCTRL_SPEC {
    type Ux = u8;
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
