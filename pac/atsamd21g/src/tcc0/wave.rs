#[doc = "Register `WAVE` reader"]
pub type R = crate::R<WAVE_SPEC>;
#[doc = "Register `WAVE` writer"]
pub type W = crate::W<WAVE_SPEC>;
#[doc = "Field `WAVEGEN` reader - Waveform Generation"]
pub type WAVEGEN_R = crate::FieldReader<WAVEGENSELECT_A>;
#[doc = "Waveform Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVEGENSELECT_A {
    #[doc = "0: Normal frequency"]
    NFRQ = 0,
    #[doc = "1: Match frequency"]
    MFRQ = 1,
    #[doc = "2: Normal PWM"]
    NPWM = 2,
    #[doc = "4: Dual-slope critical"]
    DSCRITICAL = 4,
    #[doc = "5: Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    DSBOTTOM = 5,
    #[doc = "6: Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    DSBOTH = 6,
    #[doc = "7: Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    DSTOP = 7,
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
    pub const fn variant(&self) -> Option<WAVEGENSELECT_A> {
        match self.bits {
            0 => Some(WAVEGENSELECT_A::NFRQ),
            1 => Some(WAVEGENSELECT_A::MFRQ),
            2 => Some(WAVEGENSELECT_A::NPWM),
            4 => Some(WAVEGENSELECT_A::DSCRITICAL),
            5 => Some(WAVEGENSELECT_A::DSBOTTOM),
            6 => Some(WAVEGENSELECT_A::DSBOTH),
            7 => Some(WAVEGENSELECT_A::DSTOP),
            _ => None,
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
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn is_dscritical(&self) -> bool {
        *self == WAVEGENSELECT_A::DSCRITICAL
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn is_dsbottom(&self) -> bool {
        *self == WAVEGENSELECT_A::DSBOTTOM
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn is_dsboth(&self) -> bool {
        *self == WAVEGENSELECT_A::DSBOTH
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn is_dstop(&self) -> bool {
        *self == WAVEGENSELECT_A::DSTOP
    }
}
#[doc = "Field `WAVEGEN` writer - Waveform Generation"]
pub type WAVEGEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, WAVEGENSELECT_A>;
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
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn dscritical(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::DSCRITICAL)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn dsbottom(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::DSBOTTOM)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn dsboth(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::DSBOTH)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn dstop(self) -> &'a mut crate::W<REG> {
        self.variant(WAVEGENSELECT_A::DSTOP)
    }
}
#[doc = "Field `RAMP` reader - Ramp Mode"]
pub type RAMP_R = crate::FieldReader<RAMPSELECT_A>;
#[doc = "Ramp Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMPSELECT_A {
    #[doc = "0: RAMP1 operation"]
    RAMP1 = 0,
    #[doc = "1: Alternative RAMP2 operation"]
    RAMP2A = 1,
    #[doc = "2: RAMP2 operation"]
    RAMP2 = 2,
}
impl From<RAMPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RAMPSELECT_A {
    type Ux = u8;
}
impl RAMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RAMPSELECT_A> {
        match self.bits {
            0 => Some(RAMPSELECT_A::RAMP1),
            1 => Some(RAMPSELECT_A::RAMP2A),
            2 => Some(RAMPSELECT_A::RAMP2),
            _ => None,
        }
    }
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn is_ramp1(&self) -> bool {
        *self == RAMPSELECT_A::RAMP1
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2a(&self) -> bool {
        *self == RAMPSELECT_A::RAMP2A
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2(&self) -> bool {
        *self == RAMPSELECT_A::RAMP2
    }
}
#[doc = "Field `RAMP` writer - Ramp Mode"]
pub type RAMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, RAMPSELECT_A>;
impl<'a, REG, const O: u8> RAMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn ramp1(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPSELECT_A::RAMP1)
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2a(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPSELECT_A::RAMP2A)
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2(self) -> &'a mut crate::W<REG> {
        self.variant(RAMPSELECT_A::RAMP2)
    }
}
#[doc = "Field `CIPEREN` reader - Circular period Enable"]
pub type CIPEREN_R = crate::BitReader;
#[doc = "Field `CIPEREN` writer - Circular period Enable"]
pub type CIPEREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CICCEN0` reader - Circular Channel 0 Enable"]
pub type CICCEN0_R = crate::BitReader;
#[doc = "Field `CICCEN0` writer - Circular Channel 0 Enable"]
pub type CICCEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CICCEN1` reader - Circular Channel 1 Enable"]
pub type CICCEN1_R = crate::BitReader;
#[doc = "Field `CICCEN1` writer - Circular Channel 1 Enable"]
pub type CICCEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CICCEN2` reader - Circular Channel 2 Enable"]
pub type CICCEN2_R = crate::BitReader;
#[doc = "Field `CICCEN2` writer - Circular Channel 2 Enable"]
pub type CICCEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CICCEN3` reader - Circular Channel 3 Enable"]
pub type CICCEN3_R = crate::BitReader;
#[doc = "Field `CICCEN3` writer - Circular Channel 3 Enable"]
pub type CICCEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POL0` reader - Channel 0 Polarity"]
pub type POL0_R = crate::BitReader;
#[doc = "Field `POL0` writer - Channel 0 Polarity"]
pub type POL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POL1` reader - Channel 1 Polarity"]
pub type POL1_R = crate::BitReader;
#[doc = "Field `POL1` writer - Channel 1 Polarity"]
pub type POL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POL2` reader - Channel 2 Polarity"]
pub type POL2_R = crate::BitReader;
#[doc = "Field `POL2` writer - Channel 2 Polarity"]
pub type POL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POL3` reader - Channel 3 Polarity"]
pub type POL3_R = crate::BitReader;
#[doc = "Field `POL3` writer - Channel 3 Polarity"]
pub type POL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWAP0` reader - Swap DTI Output Pair 0"]
pub type SWAP0_R = crate::BitReader;
#[doc = "Field `SWAP0` writer - Swap DTI Output Pair 0"]
pub type SWAP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWAP1` reader - Swap DTI Output Pair 1"]
pub type SWAP1_R = crate::BitReader;
#[doc = "Field `SWAP1` writer - Swap DTI Output Pair 1"]
pub type SWAP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWAP2` reader - Swap DTI Output Pair 2"]
pub type SWAP2_R = crate::BitReader;
#[doc = "Field `SWAP2` writer - Swap DTI Output Pair 2"]
pub type SWAP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWAP3` reader - Swap DTI Output Pair 3"]
pub type SWAP3_R = crate::BitReader;
#[doc = "Field `SWAP3` writer - Swap DTI Output Pair 3"]
pub type SWAP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Waveform Generation"]
    #[inline(always)]
    pub fn wavegen(&self) -> WAVEGEN_R {
        WAVEGEN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Ramp Mode"]
    #[inline(always)]
    pub fn ramp(&self) -> RAMP_R {
        RAMP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Circular period Enable"]
    #[inline(always)]
    pub fn ciperen(&self) -> CIPEREN_R {
        CIPEREN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable"]
    #[inline(always)]
    pub fn ciccen0(&self) -> CICCEN0_R {
        CICCEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable"]
    #[inline(always)]
    pub fn ciccen1(&self) -> CICCEN1_R {
        CICCEN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable"]
    #[inline(always)]
    pub fn ciccen2(&self) -> CICCEN2_R {
        CICCEN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable"]
    #[inline(always)]
    pub fn ciccen3(&self) -> CICCEN3_R {
        CICCEN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&self) -> POL0_R {
        POL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&self) -> POL3_R {
        POL3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0"]
    #[inline(always)]
    pub fn swap0(&self) -> SWAP0_R {
        SWAP0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1"]
    #[inline(always)]
    pub fn swap1(&self) -> SWAP1_R {
        SWAP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2"]
    #[inline(always)]
    pub fn swap2(&self) -> SWAP2_R {
        SWAP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3"]
    #[inline(always)]
    pub fn swap3(&self) -> SWAP3_R {
        SWAP3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Waveform Generation"]
    #[inline(always)]
    #[must_use]
    pub fn wavegen(&mut self) -> WAVEGEN_W<WAVE_SPEC, 0> {
        WAVEGEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Ramp Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ramp(&mut self) -> RAMP_W<WAVE_SPEC, 4> {
        RAMP_W::new(self)
    }
    #[doc = "Bit 7 - Circular period Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciperen(&mut self) -> CIPEREN_W<WAVE_SPEC, 7> {
        CIPEREN_W::new(self)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciccen0(&mut self) -> CICCEN0_W<WAVE_SPEC, 8> {
        CICCEN0_W::new(self)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciccen1(&mut self) -> CICCEN1_W<WAVE_SPEC, 9> {
        CICCEN1_W::new(self)
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciccen2(&mut self) -> CICCEN2_W<WAVE_SPEC, 10> {
        CICCEN2_W::new(self)
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciccen3(&mut self) -> CICCEN3_W<WAVE_SPEC, 11> {
        CICCEN3_W::new(self)
    }
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol0(&mut self) -> POL0_W<WAVE_SPEC, 16> {
        POL0_W::new(self)
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol1(&mut self) -> POL1_W<WAVE_SPEC, 17> {
        POL1_W::new(self)
    }
    #[doc = "Bit 18 - Channel 2 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol2(&mut self) -> POL2_W<WAVE_SPEC, 18> {
        POL2_W::new(self)
    }
    #[doc = "Bit 19 - Channel 3 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol3(&mut self) -> POL3_W<WAVE_SPEC, 19> {
        POL3_W::new(self)
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0"]
    #[inline(always)]
    #[must_use]
    pub fn swap0(&mut self) -> SWAP0_W<WAVE_SPEC, 24> {
        SWAP0_W::new(self)
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1"]
    #[inline(always)]
    #[must_use]
    pub fn swap1(&mut self) -> SWAP1_W<WAVE_SPEC, 25> {
        SWAP1_W::new(self)
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2"]
    #[inline(always)]
    #[must_use]
    pub fn swap2(&mut self) -> SWAP2_W<WAVE_SPEC, 26> {
        SWAP2_W::new(self)
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3"]
    #[inline(always)]
    #[must_use]
    pub fn swap3(&mut self) -> SWAP3_W<WAVE_SPEC, 27> {
        SWAP3_W::new(self)
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
#[doc = "Waveform Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAVE_SPEC;
impl crate::RegisterSpec for WAVE_SPEC {
    type Ux = u32;
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
