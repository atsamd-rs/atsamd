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
#[doc = "Waveform Generation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVEGEN_A {
    #[doc = "0: Normal frequency"]
    NFRQ = 0,
    #[doc = "1: Match frequency"]
    MFRQ = 1,
    #[doc = "2: Normal PWM"]
    NPWM = 2,
    #[doc = "3: Match PWM"]
    MPWM = 3,
}
impl From<WAVEGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVEGEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAVEGEN` reader - Waveform Generation Mode"]
pub struct WAVEGEN_R(crate::FieldReader<u8, WAVEGEN_A>);
impl WAVEGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAVEGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVEGEN_A {
        match self.bits {
            0 => WAVEGEN_A::NFRQ,
            1 => WAVEGEN_A::MFRQ,
            2 => WAVEGEN_A::NPWM,
            3 => WAVEGEN_A::MPWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NFRQ`"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        **self == WAVEGEN_A::NFRQ
    }
    #[doc = "Checks if the value of the field is `MFRQ`"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        **self == WAVEGEN_A::MFRQ
    }
    #[doc = "Checks if the value of the field is `NPWM`"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        **self == WAVEGEN_A::NPWM
    }
    #[doc = "Checks if the value of the field is `MPWM`"]
    #[inline(always)]
    pub fn is_mpwm(&self) -> bool {
        **self == WAVEGEN_A::MPWM
    }
}
impl core::ops::Deref for WAVEGEN_R {
    type Target = crate::FieldReader<u8, WAVEGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVEGEN` writer - Waveform Generation Mode"]
pub struct WAVEGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVEGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVEGEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut W {
        self.variant(WAVEGEN_A::NFRQ)
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut W {
        self.variant(WAVEGEN_A::MFRQ)
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut W {
        self.variant(WAVEGEN_A::NPWM)
    }
    #[doc = "Match PWM"]
    #[inline(always)]
    pub fn mpwm(self) -> &'a mut W {
        self.variant(WAVEGEN_A::MPWM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u8 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wavegen(&self) -> WAVEGEN_R {
        WAVEGEN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Waveform Generation Mode"]
    #[inline(always)]
    pub fn wavegen(&mut self) -> WAVEGEN_W {
        WAVEGEN_W { w: self }
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
}
#[doc = "`reset()` method sets WAVE to value 0"]
impl crate::Resettable for WAVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
