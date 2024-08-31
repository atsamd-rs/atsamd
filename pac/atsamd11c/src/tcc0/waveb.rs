#[doc = "Register `WAVEB` reader"]
pub type R = crate::R<WavebSpec>;
#[doc = "Register `WAVEB` writer"]
pub type W = crate::W<WavebSpec>;
#[doc = "Waveform Generation Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wavegenbselect {
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
impl From<Wavegenbselect> for u8 {
    #[inline(always)]
    fn from(variant: Wavegenbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wavegenbselect {
    type Ux = u8;
}
impl crate::IsEnum for Wavegenbselect {}
#[doc = "Field `WAVEGENB` reader - Waveform Generation Buffer"]
pub type WavegenbR = crate::FieldReader<Wavegenbselect>;
impl WavegenbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wavegenbselect> {
        match self.bits {
            0 => Some(Wavegenbselect::Nfrq),
            1 => Some(Wavegenbselect::Mfrq),
            2 => Some(Wavegenbselect::Npwm),
            4 => Some(Wavegenbselect::Dscritical),
            5 => Some(Wavegenbselect::Dsbottom),
            6 => Some(Wavegenbselect::Dsboth),
            7 => Some(Wavegenbselect::Dstop),
            _ => None,
        }
    }
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == Wavegenbselect::Nfrq
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == Wavegenbselect::Mfrq
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == Wavegenbselect::Npwm
    }
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn is_dscritical(&self) -> bool {
        *self == Wavegenbselect::Dscritical
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn is_dsbottom(&self) -> bool {
        *self == Wavegenbselect::Dsbottom
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn is_dsboth(&self) -> bool {
        *self == Wavegenbselect::Dsboth
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn is_dstop(&self) -> bool {
        *self == Wavegenbselect::Dstop
    }
}
#[doc = "Field `WAVEGENB` writer - Waveform Generation Buffer"]
pub type WavegenbW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wavegenbselect>;
impl<'a, REG> WavegenbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal frequency"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenbselect::Nfrq)
    }
    #[doc = "Match frequency"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenbselect::Mfrq)
    }
    #[doc = "Normal PWM"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenbselect::Npwm)
    }
    #[doc = "Dual-slope critical"]
    #[inline(always)]
    pub fn dscritical(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenbselect::Dscritical)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO"]
    #[inline(always)]
    pub fn dsbottom(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenbselect::Dsbottom)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches ZERO or TOP"]
    #[inline(always)]
    pub fn dsboth(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenbselect::Dsboth)
    }
    #[doc = "Dual-slope with interrupt/event condition when COUNT reaches TOP"]
    #[inline(always)]
    pub fn dstop(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenbselect::Dstop)
    }
}
#[doc = "Ramp Mode Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rampbselect {
    #[doc = "0: RAMP1 operation"]
    Ramp1 = 0,
    #[doc = "1: Alternative RAMP2 operation"]
    Ramp2a = 1,
    #[doc = "2: RAMP2 operation"]
    Ramp2 = 2,
}
impl From<Rampbselect> for u8 {
    #[inline(always)]
    fn from(variant: Rampbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rampbselect {
    type Ux = u8;
}
impl crate::IsEnum for Rampbselect {}
#[doc = "Field `RAMPB` reader - Ramp Mode Buffer"]
pub type RampbR = crate::FieldReader<Rampbselect>;
impl RampbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rampbselect> {
        match self.bits {
            0 => Some(Rampbselect::Ramp1),
            1 => Some(Rampbselect::Ramp2a),
            2 => Some(Rampbselect::Ramp2),
            _ => None,
        }
    }
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn is_ramp1(&self) -> bool {
        *self == Rampbselect::Ramp1
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2a(&self) -> bool {
        *self == Rampbselect::Ramp2a
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn is_ramp2(&self) -> bool {
        *self == Rampbselect::Ramp2
    }
}
#[doc = "Field `RAMPB` writer - Ramp Mode Buffer"]
pub type RampbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rampbselect>;
impl<'a, REG> RampbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RAMP1 operation"]
    #[inline(always)]
    pub fn ramp1(self) -> &'a mut crate::W<REG> {
        self.variant(Rampbselect::Ramp1)
    }
    #[doc = "Alternative RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2a(self) -> &'a mut crate::W<REG> {
        self.variant(Rampbselect::Ramp2a)
    }
    #[doc = "RAMP2 operation"]
    #[inline(always)]
    pub fn ramp2(self) -> &'a mut crate::W<REG> {
        self.variant(Rampbselect::Ramp2)
    }
}
#[doc = "Field `CIPERENB` reader - Circular Period Enable Buffer"]
pub type CiperenbR = crate::BitReader;
#[doc = "Field `CIPERENB` writer - Circular Period Enable Buffer"]
pub type CiperenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICCENB0` reader - Circular Channel 0 Enable Buffer"]
pub type Ciccenb0R = crate::BitReader;
#[doc = "Field `CICCENB0` writer - Circular Channel 0 Enable Buffer"]
pub type Ciccenb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICCENB1` reader - Circular Channel 1 Enable Buffer"]
pub type Ciccenb1R = crate::BitReader;
#[doc = "Field `CICCENB1` writer - Circular Channel 1 Enable Buffer"]
pub type Ciccenb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICCENB2` reader - Circular Channel 2 Enable Buffer"]
pub type Ciccenb2R = crate::BitReader;
#[doc = "Field `CICCENB2` writer - Circular Channel 2 Enable Buffer"]
pub type Ciccenb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CICCENB3` reader - Circular Channel 3 Enable Buffer"]
pub type Ciccenb3R = crate::BitReader;
#[doc = "Field `CICCENB3` writer - Circular Channel 3 Enable Buffer"]
pub type Ciccenb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLB0` reader - Channel 0 Polarity Buffer"]
pub type Polb0R = crate::BitReader;
#[doc = "Field `POLB0` writer - Channel 0 Polarity Buffer"]
pub type Polb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLB1` reader - Channel 1 Polarity Buffer"]
pub type Polb1R = crate::BitReader;
#[doc = "Field `POLB1` writer - Channel 1 Polarity Buffer"]
pub type Polb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLB2` reader - Channel 2 Polarity Buffer"]
pub type Polb2R = crate::BitReader;
#[doc = "Field `POLB2` writer - Channel 2 Polarity Buffer"]
pub type Polb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLB3` reader - Channel 3 Polarity Buffer"]
pub type Polb3R = crate::BitReader;
#[doc = "Field `POLB3` writer - Channel 3 Polarity Buffer"]
pub type Polb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAPB0` reader - Swap DTI Output Pair 0 Buffer"]
pub type Swapb0R = crate::BitReader;
#[doc = "Field `SWAPB0` writer - Swap DTI Output Pair 0 Buffer"]
pub type Swapb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAPB1` reader - Swap DTI Output Pair 1 Buffer"]
pub type Swapb1R = crate::BitReader;
#[doc = "Field `SWAPB1` writer - Swap DTI Output Pair 1 Buffer"]
pub type Swapb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAPB2` reader - Swap DTI Output Pair 2 Buffer"]
pub type Swapb2R = crate::BitReader;
#[doc = "Field `SWAPB2` writer - Swap DTI Output Pair 2 Buffer"]
pub type Swapb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAPB3` reader - Swap DTI Output Pair 3 Buffer"]
pub type Swapb3R = crate::BitReader;
#[doc = "Field `SWAPB3` writer - Swap DTI Output Pair 3 Buffer"]
pub type Swapb3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Waveform Generation Buffer"]
    #[inline(always)]
    pub fn wavegenb(&self) -> WavegenbR {
        WavegenbR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Ramp Mode Buffer"]
    #[inline(always)]
    pub fn rampb(&self) -> RampbR {
        RampbR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Circular Period Enable Buffer"]
    #[inline(always)]
    pub fn ciperenb(&self) -> CiperenbR {
        CiperenbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb0(&self) -> Ciccenb0R {
        Ciccenb0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb1(&self) -> Ciccenb1R {
        Ciccenb1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb2(&self) -> Ciccenb2R {
        Ciccenb2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable Buffer"]
    #[inline(always)]
    pub fn ciccenb3(&self) -> Ciccenb3R {
        Ciccenb3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 0 Polarity Buffer"]
    #[inline(always)]
    pub fn polb0(&self) -> Polb0R {
        Polb0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 1 Polarity Buffer"]
    #[inline(always)]
    pub fn polb1(&self) -> Polb1R {
        Polb1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 2 Polarity Buffer"]
    #[inline(always)]
    pub fn polb2(&self) -> Polb2R {
        Polb2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 3 Polarity Buffer"]
    #[inline(always)]
    pub fn polb3(&self) -> Polb3R {
        Polb3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0 Buffer"]
    #[inline(always)]
    pub fn swapb0(&self) -> Swapb0R {
        Swapb0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1 Buffer"]
    #[inline(always)]
    pub fn swapb1(&self) -> Swapb1R {
        Swapb1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2 Buffer"]
    #[inline(always)]
    pub fn swapb2(&self) -> Swapb2R {
        Swapb2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3 Buffer"]
    #[inline(always)]
    pub fn swapb3(&self) -> Swapb3R {
        Swapb3R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Waveform Generation Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn wavegenb(&mut self) -> WavegenbW<WavebSpec> {
        WavegenbW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Ramp Mode Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn rampb(&mut self) -> RampbW<WavebSpec> {
        RampbW::new(self, 4)
    }
    #[doc = "Bit 7 - Circular Period Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ciperenb(&mut self) -> CiperenbW<WavebSpec> {
        CiperenbW::new(self, 7)
    }
    #[doc = "Bit 8 - Circular Channel 0 Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ciccenb0(&mut self) -> Ciccenb0W<WavebSpec> {
        Ciccenb0W::new(self, 8)
    }
    #[doc = "Bit 9 - Circular Channel 1 Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ciccenb1(&mut self) -> Ciccenb1W<WavebSpec> {
        Ciccenb1W::new(self, 9)
    }
    #[doc = "Bit 10 - Circular Channel 2 Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ciccenb2(&mut self) -> Ciccenb2W<WavebSpec> {
        Ciccenb2W::new(self, 10)
    }
    #[doc = "Bit 11 - Circular Channel 3 Enable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ciccenb3(&mut self) -> Ciccenb3W<WavebSpec> {
        Ciccenb3W::new(self, 11)
    }
    #[doc = "Bit 16 - Channel 0 Polarity Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn polb0(&mut self) -> Polb0W<WavebSpec> {
        Polb0W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 1 Polarity Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn polb1(&mut self) -> Polb1W<WavebSpec> {
        Polb1W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel 2 Polarity Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn polb2(&mut self) -> Polb2W<WavebSpec> {
        Polb2W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel 3 Polarity Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn polb3(&mut self) -> Polb3W<WavebSpec> {
        Polb3W::new(self, 19)
    }
    #[doc = "Bit 24 - Swap DTI Output Pair 0 Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn swapb0(&mut self) -> Swapb0W<WavebSpec> {
        Swapb0W::new(self, 24)
    }
    #[doc = "Bit 25 - Swap DTI Output Pair 1 Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn swapb1(&mut self) -> Swapb1W<WavebSpec> {
        Swapb1W::new(self, 25)
    }
    #[doc = "Bit 26 - Swap DTI Output Pair 2 Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn swapb2(&mut self) -> Swapb2W<WavebSpec> {
        Swapb2W::new(self, 26)
    }
    #[doc = "Bit 27 - Swap DTI Output Pair 3 Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn swapb3(&mut self) -> Swapb3W<WavebSpec> {
        Swapb3W::new(self, 27)
    }
}
#[doc = "Waveform Control Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`waveb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`waveb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WavebSpec;
impl crate::RegisterSpec for WavebSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`waveb::R`](R) reader structure"]
impl crate::Readable for WavebSpec {}
#[doc = "`write(|w| ..)` method takes [`waveb::W`](W) writer structure"]
impl crate::Writable for WavebSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAVEB to value 0"]
impl crate::Resettable for WavebSpec {
    const RESET_VALUE: u32 = 0;
}
