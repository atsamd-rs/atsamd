#[doc = "Register `WAVE` reader"]
pub type R = crate::R<WAVE_SPEC>;
#[doc = "Register `WAVE` writer"]
pub type W = crate::W<WAVE_SPEC>;
#[doc = "Field `WAVEGEN` reader - Waveform Generation Mode"]
pub type WAVEGEN_R = crate::FieldReader<WAVEGENSELECT_A>;
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
impl crate::FieldSpec for WAVEGENSELECT_A {
    type Ux = u8;
}
impl WAVEGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAVEGENSELECT_A {
        match self.bits {
            0 => WAVEGENSELECT_A::NFRQ,
            1 => WAVEGENSELECT_A::MFRQ,
            2 => WAVEGENSELECT_A::NPWM,
            3 => WAVEGENSELECT_A::MPWM,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == WAVEGENSELECT_A::NFRQ
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == WAVEGENSELECT_A::MFRQ
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == WAVEGENSELECT_A::NPWM
    }
    #[doc = "Match PWM"]
    #[inline(always)]
    pub fn is_mpwm(&self) -> bool {
        *self == WAVEGENSELECT_A::MPWM
    }
}
#[doc = "Field `WAVEGEN` writer - Waveform Generation Mode"]
pub type WAVEGEN_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, WAVEGENSELECT_A>;
impl<'a, REG, const O: u8> WAVEGEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::NFRQ)
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::MFRQ)
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::NPWM)
    }
    #[doc = "Match PWM"]
    #[inline(always)]
    pub fn mpwm(self) -> &'a mut crate::W<REG> {
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
    pub fn wavegen(&mut self) -> WAVEGEN_W<WAVE_SPEC, 0> {
        WAVEGEN_W::new(self)
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
#[doc = "Waveform Generation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAVE_SPEC;
impl crate::RegisterSpec for WAVE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wave::R`](R) reader structure"]
impl crate::Readable for WAVE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wave::W`](W) writer structure"]
impl crate::Writable for WAVE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAVE to value 0"]
impl crate::Resettable for WAVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
