#[doc = "Register `WAVE` reader"]
pub struct R(crate::R<WAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAVE` writer"]
pub struct W(crate::W<WAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAVE_SPEC>;
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
impl From<crate::W<WAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAVEGEN` reader - Waveform Generation Mode"]
pub type WAVEGEN_R = crate::FieldReader<u8, WAVEGENSELECT_A>;
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVEGENSELECT_A {
    #[doc = "0: Normal frequency"]
    NFRQ = 0,
    #[doc = "1: Match frequency"]
    MFRQ = 1,
    #[doc = "2: Normal PWM"]
    NPWM = 2,
    #[doc = "3: Match PWM"]
    MPWM = 3,
}
impl From<WAVEGENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVEGENSELECT_A) -> Self {
        variant as _
    }
}
impl WAVEGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVEGENSELECT_A {
        match self.bits {
            0 => WAVEGENSELECT_A::NFRQ,
            1 => WAVEGENSELECT_A::MFRQ,
            2 => WAVEGENSELECT_A::NPWM,
            3 => WAVEGENSELECT_A::MPWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == WAVEGENSELECT_A::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == WAVEGENSELECT_A::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == WAVEGENSELECT_A::NPWM
    }
    #[doc = "Checks if the value of the field is `MPWM`"]
    #[inline(always)]
    pub fn is_mpwm(&self) -> bool {
        *self == WAVEGENSELECT_A::MPWM
    }
}
#[doc = "Field `WAVEGEN` writer - Waveform Generation Mode"]
pub type WAVEGEN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, WAVE_SPEC, u8, WAVEGENSELECT_A, 2, O>;
impl<'a, const O: u8> WAVEGEN_W<'a, O> {
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut W {
        self.variant(WAVEGENSELECT_A::NFRQ)
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut W {
        self.variant(WAVEGENSELECT_A::MFRQ)
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut W {
        self.variant(WAVEGENSELECT_A::NPWM)
    }
    #[doc = "Match PWM"]
    #[inline(always)]
    pub fn mpwm(self) -> &'a mut W {
        self.variant(WAVEGENSELECT_A::MPWM)
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wavegen(&self) -> WAVEGEN_R {
        WAVEGEN_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wavegen(&mut self) -> WAVEGEN_W<0> {
        WAVEGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Waveform Generation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wave](index.html) module"]
pub struct WAVE_SPEC;
impl crate::RegisterSpec for WAVE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wave::R](R) reader structure"]
impl crate::Readable for WAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wave::W](W) writer structure"]
impl crate::Writable for WAVE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAVE to value 0"]
impl crate::Resettable for WAVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
