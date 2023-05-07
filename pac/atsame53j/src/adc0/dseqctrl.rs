#[doc = "Register `DSEQCTRL` reader"]
pub struct R(crate::R<DSEQCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSEQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSEQCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSEQCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSEQCTRL` writer"]
pub struct W(crate::W<DSEQCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSEQCTRL_SPEC>;
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
impl From<crate::W<DSEQCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSEQCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTCTRL` reader - Input Control"]
pub type INPUTCTRL_R = crate::BitReader<bool>;
#[doc = "Field `INPUTCTRL` writer - Input Control"]
pub type INPUTCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSEQCTRL_SPEC, bool, O>;
#[doc = "Field `CTRLB` reader - Control B"]
pub type CTRLB_R = crate::BitReader<bool>;
#[doc = "Field `CTRLB` writer - Control B"]
pub type CTRLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSEQCTRL_SPEC, bool, O>;
#[doc = "Field `REFCTRL` reader - Reference Control"]
pub type REFCTRL_R = crate::BitReader<bool>;
#[doc = "Field `REFCTRL` writer - Reference Control"]
pub type REFCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSEQCTRL_SPEC, bool, O>;
#[doc = "Field `AVGCTRL` reader - Average Control"]
pub type AVGCTRL_R = crate::BitReader<bool>;
#[doc = "Field `AVGCTRL` writer - Average Control"]
pub type AVGCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSEQCTRL_SPEC, bool, O>;
#[doc = "Field `SAMPCTRL` reader - Sampling Time Control"]
pub type SAMPCTRL_R = crate::BitReader<bool>;
#[doc = "Field `SAMPCTRL` writer - Sampling Time Control"]
pub type SAMPCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSEQCTRL_SPEC, bool, O>;
#[doc = "Field `WINLT` reader - Window Monitor Lower Threshold"]
pub type WINLT_R = crate::BitReader<bool>;
#[doc = "Field `WINLT` writer - Window Monitor Lower Threshold"]
pub type WINLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSEQCTRL_SPEC, bool, O>;
#[doc = "Field `WINUT` reader - Window Monitor Upper Threshold"]
pub type WINUT_R = crate::BitReader<bool>;
#[doc = "Field `WINUT` writer - Window Monitor Upper Threshold"]
pub type WINUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSEQCTRL_SPEC, bool, O>;
#[doc = "Field `GAINCORR` reader - Gain Correction"]
pub type GAINCORR_R = crate::BitReader<bool>;
#[doc = "Field `GAINCORR` writer - Gain Correction"]
pub type GAINCORR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSEQCTRL_SPEC, bool, O>;
#[doc = "Field `OFFSETCORR` reader - Offset Correction"]
pub type OFFSETCORR_R = crate::BitReader<bool>;
#[doc = "Field `OFFSETCORR` writer - Offset Correction"]
pub type OFFSETCORR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSEQCTRL_SPEC, bool, O>;
#[doc = "Field `AUTOSTART` reader - ADC Auto-Start Conversion"]
pub type AUTOSTART_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSTART` writer - ADC Auto-Start Conversion"]
pub type AUTOSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSEQCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Input Control"]
    #[inline(always)]
    pub fn inputctrl(&self) -> INPUTCTRL_R {
        INPUTCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control B"]
    #[inline(always)]
    pub fn ctrlb(&self) -> CTRLB_R {
        CTRLB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reference Control"]
    #[inline(always)]
    pub fn refctrl(&self) -> REFCTRL_R {
        REFCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Average Control"]
    #[inline(always)]
    pub fn avgctrl(&self) -> AVGCTRL_R {
        AVGCTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sampling Time Control"]
    #[inline(always)]
    pub fn sampctrl(&self) -> SAMPCTRL_R {
        SAMPCTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WINLT_R {
        WINLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Window Monitor Upper Threshold"]
    #[inline(always)]
    pub fn winut(&self) -> WINUT_R {
        WINUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC Auto-Start Conversion"]
    #[inline(always)]
    pub fn autostart(&self) -> AUTOSTART_R {
        AUTOSTART_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Control"]
    #[inline(always)]
    #[must_use]
    pub fn inputctrl(&mut self) -> INPUTCTRL_W<0> {
        INPUTCTRL_W::new(self)
    }
    #[doc = "Bit 1 - Control B"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlb(&mut self) -> CTRLB_W<1> {
        CTRLB_W::new(self)
    }
    #[doc = "Bit 2 - Reference Control"]
    #[inline(always)]
    #[must_use]
    pub fn refctrl(&mut self) -> REFCTRL_W<2> {
        REFCTRL_W::new(self)
    }
    #[doc = "Bit 3 - Average Control"]
    #[inline(always)]
    #[must_use]
    pub fn avgctrl(&mut self) -> AVGCTRL_W<3> {
        AVGCTRL_W::new(self)
    }
    #[doc = "Bit 4 - Sampling Time Control"]
    #[inline(always)]
    #[must_use]
    pub fn sampctrl(&mut self) -> SAMPCTRL_W<4> {
        SAMPCTRL_W::new(self)
    }
    #[doc = "Bit 5 - Window Monitor Lower Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn winlt(&mut self) -> WINLT_W<5> {
        WINLT_W::new(self)
    }
    #[doc = "Bit 6 - Window Monitor Upper Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn winut(&mut self) -> WINUT_W<6> {
        WINUT_W::new(self)
    }
    #[doc = "Bit 7 - Gain Correction"]
    #[inline(always)]
    #[must_use]
    pub fn gaincorr(&mut self) -> GAINCORR_W<7> {
        GAINCORR_W::new(self)
    }
    #[doc = "Bit 8 - Offset Correction"]
    #[inline(always)]
    #[must_use]
    pub fn offsetcorr(&mut self) -> OFFSETCORR_W<8> {
        OFFSETCORR_W::new(self)
    }
    #[doc = "Bit 31 - ADC Auto-Start Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn autostart(&mut self) -> AUTOSTART_W<31> {
        AUTOSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Sequential Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dseqctrl](index.html) module"]
pub struct DSEQCTRL_SPEC;
impl crate::RegisterSpec for DSEQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dseqctrl::R](R) reader structure"]
impl crate::Readable for DSEQCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dseqctrl::W](W) writer structure"]
impl crate::Writable for DSEQCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSEQCTRL to value 0"]
impl crate::Resettable for DSEQCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
