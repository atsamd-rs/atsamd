#[doc = "Register `WAVE` reader"]
pub type R = crate::R<WaveSpec>;
#[doc = "Register `WAVE` writer"]
pub type W = crate::W<WaveSpec>;
#[doc = "Waveform Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wavegenselect {
    #[doc = "0: Normal frequency"]
    Nfrq = 0,
    #[doc = "1: Match frequency"]
    Mfrq = 1,
    #[doc = "2: Normal PWM"]
    Npwm = 2,
    #[doc = "4: Dual-slope critical"]
    Dscritical = 4,
    #[doc = "5: Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    Dsbottom = 5,
    #[doc = "6: Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    Dsboth = 6,
    #[doc = "7: Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    Dstop = 7,
}
impl From<Wavegenselect> for u8 {
    #[inline(always)]
    fn from(variant: Wavegenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wavegenselect {
    type Ux = u8;
}
impl crate::IsEnum for Wavegenselect {}
#[doc = "Field `WAVEGEN` reader - Waveform Generation"]
pub type WavegenR = crate::FieldReader<Wavegenselect>;
impl WavegenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wavegenselect> {
        match self.bits {
            0 => Some(Wavegenselect::Nfrq),
            1 => Some(Wavegenselect::Mfrq),
            2 => Some(Wavegenselect::Npwm),
            4 => Some(Wavegenselect::Dscritical),
            5 => Some(Wavegenselect::Dsbottom),
            6 => Some(Wavegenselect::Dsboth),
            7 => Some(Wavegenselect::Dstop),
            _ => None,
        }
    }
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == Wavegenselect::Nfrq
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == Wavegenselect::Mfrq
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == Wavegenselect::Npwm
    }
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn is_dscritical(&self) -> bool {
        *self == Wavegenselect::Dscritical
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn is_dsbottom(&self) -> bool {
        *self == Wavegenselect::Dsbottom
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn is_dsboth(&self) -> bool {
        *self == Wavegenselect::Dsboth
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn is_dstop(&self) -> bool {
        *self == Wavegenselect::Dstop
    }
}
#[doc = "Field `WAVEGEN` writer - Waveform Generation"]
pub type WavegenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wavegenselect>;
impl<'a, REG> WavegenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Nfrq)
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Mfrq)
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Npwm)
    }
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn dscritical(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Dscritical)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn dsbottom(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Dsbottom)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn dsboth(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Dsboth)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn dstop(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Dstop)
    }
}
#[doc = "Ramp Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rampselect {
    #[doc = "0: RAMP1 operation"]
    Ramp1 = 0,
    #[doc = "1: Alternative RAMP2 operation"]
    Ramp2a = 1,
    #[doc = "2: RAMP2 operation"]
    Ramp2 = 2,
    #[doc = "3: Critical RAMP2 operation"]
    Ramp2c = 3,
}
impl From<Rampselect> for u8 {
    #[inline(always)]
    fn from(variant: Rampselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rampselect {
    type Ux = u8;
}
impl crate::IsEnum for Rampselect {}
#[doc = "Field `RAMP` reader - Ramp Mode"]
pub type RampR = crate::FieldReader<Rampselect>;
impl RampR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rampselect {
        match self.bits {
            0 => Rampselect::Ramp1,
            1 => Rampselect::Ramp2a,
            2 => Rampselect::Ramp2,
            3 => Rampselect::Ramp2c,
            _ => unreachable!(),
        }
    }
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn is_ramp1(&self) -> bool {
        *self == Rampselect::Ramp1
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2a(&self) -> bool {
        *self == Rampselect::Ramp2a
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2(&self) -> bool {
        *self == Rampselect::Ramp2
    }
    #[doc = "Critical RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2c(&self) -> bool {
        *self == Rampselect::Ramp2c
    }
}
#[doc = "Field `RAMP` writer - Ramp Mode"]
pub type RampW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rampselect, crate::Safe>;
impl<'a, REG> RampW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn ramp1(self) -> &'a mut crate::W<REG> {
        self.variant(Rampselect::Ramp1)
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2a(self) -> &'a mut crate::W<REG> {
        self.variant(Rampselect::Ramp2a)
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2(self) -> &'a mut crate::W<REG> {
        self.variant(Rampselect::Ramp2)
    }
    #[doc = "Critical RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2c(self) -> &'a mut crate::W<REG> {
        self.variant(Rampselect::Ramp2c)
    }
}
#[doc = "Field `CIPEREN` reader - Circular period Enable"]
pub type CiperenR = crate::BitReader;
#[doc = "Field `CIPEREN` writer - Circular period Enable"]
pub type CiperenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICCEN0` reader - Circular Channel 0 Enable"]
pub type Ciccen0R = crate::BitReader;
#[doc = "Field `CICCEN0` writer - Circular Channel 0 Enable"]
pub type Ciccen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICCEN1` reader - Circular Channel 1 Enable"]
pub type Ciccen1R = crate::BitReader;
#[doc = "Field `CICCEN1` writer - Circular Channel 1 Enable"]
pub type Ciccen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICCEN2` reader - Circular Channel 2 Enable"]
pub type Ciccen2R = crate::BitReader;
#[doc = "Field `CICCEN2` writer - Circular Channel 2 Enable"]
pub type Ciccen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICCEN3` reader - Circular Channel 3 Enable"]
pub type Ciccen3R = crate::BitReader;
#[doc = "Field `CICCEN3` writer - Circular Channel 3 Enable"]
pub type Ciccen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL0` reader - Channel 0 Polarity"]
pub type Pol0R = crate::BitReader;
#[doc = "Field `POL0` writer - Channel 0 Polarity"]
pub type Pol0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL1` reader - Channel 1 Polarity"]
pub type Pol1R = crate::BitReader;
#[doc = "Field `POL1` writer - Channel 1 Polarity"]
pub type Pol1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL2` reader - Channel 2 Polarity"]
pub type Pol2R = crate::BitReader;
#[doc = "Field `POL2` writer - Channel 2 Polarity"]
pub type Pol2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL3` reader - Channel 3 Polarity"]
pub type Pol3R = crate::BitReader;
#[doc = "Field `POL3` writer - Channel 3 Polarity"]
pub type Pol3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP0` reader - Swap DTI Output Pair 0"]
pub type Swap0R = crate::BitReader;
#[doc = "Field `SWAP0` writer - Swap DTI Output Pair 0"]
pub type Swap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP1` reader - Swap DTI Output Pair 1"]
pub type Swap1R = crate::BitReader;
#[doc = "Field `SWAP1` writer - Swap DTI Output Pair 1"]
pub type Swap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP2` reader - Swap DTI Output Pair 2"]
pub type Swap2R = crate::BitReader;
#[doc = "Field `SWAP2` writer - Swap DTI Output Pair 2"]
pub type Swap2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP3` reader - Swap DTI Output Pair 3"]
pub type Swap3R = crate::BitReader;
#[doc = "Field `SWAP3` writer - Swap DTI Output Pair 3"]
pub type Swap3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Waveform Generation"]
    #[inline(always)]
    pub fn wavegen(&self) -> WavegenR {
        WavegenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Ramp Mode"]
    #[inline(always)]
    pub fn ramp(&self) -> RampR {
        RampR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Circular period Enable"]
    #[inline(always)]
    pub fn ciperen(&self) -> CiperenR {
        CiperenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable"]
    #[inline(always)]
    pub fn ciccen0(&self) -> Ciccen0R {
        Ciccen0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable"]
    #[inline(always)]
    pub fn ciccen1(&self) -> Ciccen1R {
        Ciccen1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable"]
    #[inline(always)]
    pub fn ciccen2(&self) -> Ciccen2R {
        Ciccen2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable"]
    #[inline(always)]
    pub fn ciccen3(&self) -> Ciccen3R {
        Ciccen3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&self) -> Pol0R {
        Pol0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> Pol1R {
        Pol1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> Pol2R {
        Pol2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&self) -> Pol3R {
        Pol3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0"]
    #[inline(always)]
    pub fn swap0(&self) -> Swap0R {
        Swap0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1"]
    #[inline(always)]
    pub fn swap1(&self) -> Swap1R {
        Swap1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2"]
    #[inline(always)]
    pub fn swap2(&self) -> Swap2R {
        Swap2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3"]
    #[inline(always)]
    pub fn swap3(&self) -> Swap3R {
        Swap3R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Waveform Generation"]
    #[inline(always)]
    #[must_use]
    pub fn wavegen(&mut self) -> WavegenW<WaveSpec> {
        WavegenW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Ramp Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ramp(&mut self) -> RampW<WaveSpec> {
        RampW::new(self, 4)
    }
    #[doc = "Bit 7 - Circular period Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciperen(&mut self) -> CiperenW<WaveSpec> {
        CiperenW::new(self, 7)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciccen0(&mut self) -> Ciccen0W<WaveSpec> {
        Ciccen0W::new(self, 8)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciccen1(&mut self) -> Ciccen1W<WaveSpec> {
        Ciccen1W::new(self, 9)
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciccen2(&mut self) -> Ciccen2W<WaveSpec> {
        Ciccen2W::new(self, 10)
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciccen3(&mut self) -> Ciccen3W<WaveSpec> {
        Ciccen3W::new(self, 11)
    }
    #[doc = "Bit 16 - Channel 0 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol0(&mut self) -> Pol0W<WaveSpec> {
        Pol0W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol1(&mut self) -> Pol1W<WaveSpec> {
        Pol1W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel 2 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol2(&mut self) -> Pol2W<WaveSpec> {
        Pol2W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel 3 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol3(&mut self) -> Pol3W<WaveSpec> {
        Pol3W::new(self, 19)
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0"]
    #[inline(always)]
    #[must_use]
    pub fn swap0(&mut self) -> Swap0W<WaveSpec> {
        Swap0W::new(self, 24)
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1"]
    #[inline(always)]
    #[must_use]
    pub fn swap1(&mut self) -> Swap1W<WaveSpec> {
        Swap1W::new(self, 25)
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2"]
    #[inline(always)]
    #[must_use]
    pub fn swap2(&mut self) -> Swap2W<WaveSpec> {
        Swap2W::new(self, 26)
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3"]
    #[inline(always)]
    #[must_use]
    pub fn swap3(&mut self) -> Swap3W<WaveSpec> {
        Swap3W::new(self, 27)
    }
}
#[doc = "Waveform Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wave::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wave::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaveSpec;
impl crate::RegisterSpec for WaveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wave::R`](R) reader structure"]
impl crate::Readable for WaveSpec {}
#[doc = "`write(|w| ..)` method takes [`wave::W`](W) writer structure"]
impl crate::Writable for WaveSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAVE to value 0"]
impl crate::Resettable for WaveSpec {
    const RESET_VALUE: u32 = 0;
}
