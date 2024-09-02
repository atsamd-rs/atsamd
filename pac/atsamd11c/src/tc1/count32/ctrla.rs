#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TC Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: Counter in 16-bit mode"]
    Count16 = 0,
    #[doc = "1: Counter in 8-bit mode"]
    Count8 = 1,
    #[doc = "2: Counter in 32-bit mode"]
    Count32 = 2,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - TC Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::Count16),
            1 => Some(Modeselect::Count8),
            2 => Some(Modeselect::Count32),
            _ => None,
        }
    }
    #[doc = "Counter in 16-bit mode"]
    #[inline(always)]
    pub fn is_count16(&self) -> bool {
        *self == Modeselect::Count16
    }
    #[doc = "Counter in 8-bit mode"]
    #[inline(always)]
    pub fn is_count8(&self) -> bool {
        *self == Modeselect::Count8
    }
    #[doc = "Counter in 32-bit mode"]
    #[inline(always)]
    pub fn is_count32(&self) -> bool {
        *self == Modeselect::Count32
    }
}
#[doc = "Field `MODE` writer - TC Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counter in 16-bit mode"]
    #[inline(always)]
    pub fn count16(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Count16)
    }
    #[doc = "Counter in 8-bit mode"]
    #[inline(always)]
    pub fn count8(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Count8)
    }
    #[doc = "Counter in 32-bit mode"]
    #[inline(always)]
    pub fn count32(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Count32)
    }
}
#[doc = "Waveform Generation Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wavegenselect {
    #[doc = "0: `0`"]
    Nfrq = 0,
    #[doc = "1: `1`"]
    Mfrq = 1,
    #[doc = "2: `10`"]
    Npwm = 2,
    #[doc = "3: `11`"]
    Mpwm = 3,
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
#[doc = "Field `WAVEGEN` reader - Waveform Generation Operation"]
pub type WavegenR = crate::FieldReader<Wavegenselect>;
impl WavegenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wavegenselect {
        match self.bits {
            0 => Wavegenselect::Nfrq,
            1 => Wavegenselect::Mfrq,
            2 => Wavegenselect::Npwm,
            3 => Wavegenselect::Mpwm,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nfrq(&self) -> bool {
        *self == Wavegenselect::Nfrq
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_mfrq(&self) -> bool {
        *self == Wavegenselect::Mfrq
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_npwm(&self) -> bool {
        *self == Wavegenselect::Npwm
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_mpwm(&self) -> bool {
        *self == Wavegenselect::Mpwm
    }
}
#[doc = "Field `WAVEGEN` writer - Waveform Generation Operation"]
pub type WavegenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wavegenselect, crate::Safe>;
impl<'a, REG> WavegenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nfrq(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Nfrq)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mfrq(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Mfrq)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn npwm(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Npwm)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn mpwm(self) -> &'a mut crate::W<REG> {
        self.variant(Wavegenselect::Mpwm)
    }
}
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescalerselect {
    #[doc = "0: Prescaler: GCLK_TC"]
    Div1 = 0,
    #[doc = "1: Prescaler: GCLK_TC/2"]
    Div2 = 1,
    #[doc = "2: Prescaler: GCLK_TC/4"]
    Div4 = 2,
    #[doc = "3: Prescaler: GCLK_TC/8"]
    Div8 = 3,
    #[doc = "4: Prescaler: GCLK_TC/16"]
    Div16 = 4,
    #[doc = "5: Prescaler: GCLK_TC/64"]
    Div64 = 5,
    #[doc = "6: Prescaler: GCLK_TC/256"]
    Div256 = 6,
    #[doc = "7: Prescaler: GCLK_TC/1024"]
    Div1024 = 7,
}
impl From<Prescalerselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescalerselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescalerselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescalerselect {}
#[doc = "Field `PRESCALER` reader - Prescaler"]
pub type PrescalerR = crate::FieldReader<Prescalerselect>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescalerselect {
        match self.bits {
            0 => Prescalerselect::Div1,
            1 => Prescalerselect::Div2,
            2 => Prescalerselect::Div4,
            3 => Prescalerselect::Div8,
            4 => Prescalerselect::Div16,
            5 => Prescalerselect::Div64,
            6 => Prescalerselect::Div256,
            7 => Prescalerselect::Div1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler: GCLK_TC"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Prescalerselect::Div1
    }
    #[doc = "Prescaler: GCLK_TC/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescalerselect::Div2
    }
    #[doc = "Prescaler: GCLK_TC/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescalerselect::Div4
    }
    #[doc = "Prescaler: GCLK_TC/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescalerselect::Div8
    }
    #[doc = "Prescaler: GCLK_TC/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescalerselect::Div16
    }
    #[doc = "Prescaler: GCLK_TC/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescalerselect::Div64
    }
    #[doc = "Prescaler: GCLK_TC/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Prescalerselect::Div256
    }
    #[doc = "Prescaler: GCLK_TC/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Prescalerselect::Div1024
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prescalerselect, crate::Safe>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler: GCLK_TC"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div1)
    }
    #[doc = "Prescaler: GCLK_TC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div2)
    }
    #[doc = "Prescaler: GCLK_TC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div4)
    }
    #[doc = "Prescaler: GCLK_TC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div8)
    }
    #[doc = "Prescaler: GCLK_TC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div16)
    }
    #[doc = "Prescaler: GCLK_TC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div64)
    }
    #[doc = "Prescaler: GCLK_TC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div256)
    }
    #[doc = "Prescaler: GCLK_TC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div1024)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Prescaler and Counter Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescsyncselect {
    #[doc = "0: Reload or reset the counter on next generic clock"]
    Gclk = 0,
    #[doc = "1: Reload or reset the counter on next prescaler clock"]
    Presc = 1,
    #[doc = "2: Reload or reset the counter on next generic clock. Reset the prescaler counter"]
    Resync = 2,
}
impl From<Prescsyncselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescsyncselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescsyncselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescsyncselect {}
#[doc = "Field `PRESCSYNC` reader - Prescaler and Counter Synchronization"]
pub type PrescsyncR = crate::FieldReader<Prescsyncselect>;
impl PrescsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prescsyncselect> {
        match self.bits {
            0 => Some(Prescsyncselect::Gclk),
            1 => Some(Prescsyncselect::Presc),
            2 => Some(Prescsyncselect::Resync),
            _ => None,
        }
    }
    #[doc = "Reload or reset the counter on next generic clock"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == Prescsyncselect::Gclk
    }
    #[doc = "Reload or reset the counter on next prescaler clock"]
    #[inline(always)]
    pub fn is_presc(&self) -> bool {
        *self == Prescsyncselect::Presc
    }
    #[doc = "Reload or reset the counter on next generic clock. Reset the prescaler counter"]
    #[inline(always)]
    pub fn is_resync(&self) -> bool {
        *self == Prescsyncselect::Resync
    }
}
#[doc = "Field `PRESCSYNC` writer - Prescaler and Counter Synchronization"]
pub type PrescsyncW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prescsyncselect>;
impl<'a, REG> PrescsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reload or reset the counter on next generic clock"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Prescsyncselect::Gclk)
    }
    #[doc = "Reload or reset the counter on next prescaler clock"]
    #[inline(always)]
    pub fn presc(self) -> &'a mut crate::W<REG> {
        self.variant(Prescsyncselect::Presc)
    }
    #[doc = "Reload or reset the counter on next generic clock. Reset the prescaler counter"]
    #[inline(always)]
    pub fn resync(self) -> &'a mut crate::W<REG> {
        self.variant(Prescsyncselect::Resync)
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - TC Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Waveform Generation Operation"]
    #[inline(always)]
    pub fn wavegen(&self) -> WavegenR {
        WavegenR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    pub fn prescsync(&self) -> PrescsyncR {
        PrescsyncR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:3 - TC Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtrlaSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bits 5:6 - Waveform Generation Operation"]
    #[inline(always)]
    #[must_use]
    pub fn wavegen(&mut self) -> WavegenW<CtrlaSpec> {
        WavegenW::new(self, 5)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<CtrlaSpec> {
        PrescalerW::new(self, 8)
    }
    #[doc = "Bit 11 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<CtrlaSpec> {
        RunstdbyW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn prescsync(&mut self) -> PrescsyncW<CtrlaSpec> {
        PrescsyncW::new(self, 12)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u16 = 0;
}
