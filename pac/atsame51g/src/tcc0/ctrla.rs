#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enhanced Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resolutionselect {
    #[doc = "0: Dithering is disabled"]
    None = 0,
    #[doc = "1: Dithering is done every 16 PWM frames"]
    Dith4 = 1,
    #[doc = "2: Dithering is done every 32 PWM frames"]
    Dith5 = 2,
    #[doc = "3: Dithering is done every 64 PWM frames"]
    Dith6 = 3,
}
impl From<Resolutionselect> for u8 {
    #[inline(always)]
    fn from(variant: Resolutionselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resolutionselect {
    type Ux = u8;
}
impl crate::IsEnum for Resolutionselect {}
#[doc = "Field `RESOLUTION` reader - Enhanced Resolution"]
pub type ResolutionR = crate::FieldReader<Resolutionselect>;
impl ResolutionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resolutionselect {
        match self.bits {
            0 => Resolutionselect::None,
            1 => Resolutionselect::Dith4,
            2 => Resolutionselect::Dith5,
            3 => Resolutionselect::Dith6,
            _ => unreachable!(),
        }
    }
    #[doc = "Dithering is disabled"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Resolutionselect::None
    }
    #[doc = "Dithering is done every 16 PWM frames"]
    #[inline(always)]
    pub fn is_dith4(&self) -> bool {
        *self == Resolutionselect::Dith4
    }
    #[doc = "Dithering is done every 32 PWM frames"]
    #[inline(always)]
    pub fn is_dith5(&self) -> bool {
        *self == Resolutionselect::Dith5
    }
    #[doc = "Dithering is done every 64 PWM frames"]
    #[inline(always)]
    pub fn is_dith6(&self) -> bool {
        *self == Resolutionselect::Dith6
    }
}
#[doc = "Field `RESOLUTION` writer - Enhanced Resolution"]
pub type ResolutionW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resolutionselect, crate::Safe>;
impl<'a, REG> ResolutionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Dithering is disabled"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Resolutionselect::None)
    }
    #[doc = "Dithering is done every 16 PWM frames"]
    #[inline(always)]
    pub fn dith4(self) -> &'a mut crate::W<REG> {
        self.variant(Resolutionselect::Dith4)
    }
    #[doc = "Dithering is done every 32 PWM frames"]
    #[inline(always)]
    pub fn dith5(self) -> &'a mut crate::W<REG> {
        self.variant(Resolutionselect::Dith5)
    }
    #[doc = "Dithering is done every 64 PWM frames"]
    #[inline(always)]
    pub fn dith6(self) -> &'a mut crate::W<REG> {
        self.variant(Resolutionselect::Dith6)
    }
}
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescalerselect {
    #[doc = "0: No division"]
    Div1 = 0,
    #[doc = "1: Divide by 2"]
    Div2 = 1,
    #[doc = "2: Divide by 4"]
    Div4 = 2,
    #[doc = "3: Divide by 8"]
    Div8 = 3,
    #[doc = "4: Divide by 16"]
    Div16 = 4,
    #[doc = "5: Divide by 64"]
    Div64 = 5,
    #[doc = "6: Divide by 256"]
    Div256 = 6,
    #[doc = "7: Divide by 1024"]
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
    #[doc = "No division"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Prescalerselect::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescalerselect::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescalerselect::Div4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescalerselect::Div8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescalerselect::Div16
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescalerselect::Div64
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Prescalerselect::Div256
    }
    #[doc = "Divide by 1024"]
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
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div16)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div64)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div256)
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div1024)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Prescaler and Counter Synchronization Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescsyncselect {
    #[doc = "0: Reload or reset counter on next GCLK"]
    Gclk = 0,
    #[doc = "1: Reload or reset counter on next prescaler clock"]
    Presc = 1,
    #[doc = "2: Reload or reset counter on next GCLK and reset prescaler counter"]
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
#[doc = "Field `PRESCSYNC` reader - Prescaler and Counter Synchronization Selection"]
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
    #[doc = "Reload or reset counter on next GCLK"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == Prescsyncselect::Gclk
    }
    #[doc = "Reload or reset counter on next prescaler clock"]
    #[inline(always)]
    pub fn is_presc(&self) -> bool {
        *self == Prescsyncselect::Presc
    }
    #[doc = "Reload or reset counter on next GCLK and reset prescaler counter"]
    #[inline(always)]
    pub fn is_resync(&self) -> bool {
        *self == Prescsyncselect::Resync
    }
}
#[doc = "Field `PRESCSYNC` writer - Prescaler and Counter Synchronization Selection"]
pub type PrescsyncW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prescsyncselect>;
impl<'a, REG> PrescsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reload or reset counter on next GCLK"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Prescsyncselect::Gclk)
    }
    #[doc = "Reload or reset counter on next prescaler clock"]
    #[inline(always)]
    pub fn presc(self) -> &'a mut crate::W<REG> {
        self.variant(Prescsyncselect::Presc)
    }
    #[doc = "Reload or reset counter on next GCLK and reset prescaler counter"]
    #[inline(always)]
    pub fn resync(self) -> &'a mut crate::W<REG> {
        self.variant(Prescsyncselect::Resync)
    }
}
#[doc = "Field `ALOCK` reader - Auto Lock"]
pub type AlockR = crate::BitReader;
#[doc = "Field `ALOCK` writer - Auto Lock"]
pub type AlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSYNC` reader - Master Synchronization (only for TCC Slave Instance)"]
pub type MsyncR = crate::BitReader;
#[doc = "Field `MSYNC` writer - Master Synchronization (only for TCC Slave Instance)"]
pub type MsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAOS` reader - DMA One-shot Trigger Mode"]
pub type DmaosR = crate::BitReader;
#[doc = "Field `DMAOS` writer - DMA One-shot Trigger Mode"]
pub type DmaosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPTEN0` reader - Capture Channel 0 Enable"]
pub type Cpten0R = crate::BitReader;
#[doc = "Field `CPTEN0` writer - Capture Channel 0 Enable"]
pub type Cpten0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPTEN1` reader - Capture Channel 1 Enable"]
pub type Cpten1R = crate::BitReader;
#[doc = "Field `CPTEN1` writer - Capture Channel 1 Enable"]
pub type Cpten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPTEN2` reader - Capture Channel 2 Enable"]
pub type Cpten2R = crate::BitReader;
#[doc = "Field `CPTEN2` writer - Capture Channel 2 Enable"]
pub type Cpten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPTEN3` reader - Capture Channel 3 Enable"]
pub type Cpten3R = crate::BitReader;
#[doc = "Field `CPTEN3` writer - Capture Channel 3 Enable"]
pub type Cpten3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPTEN4` reader - Capture Channel 4 Enable"]
pub type Cpten4R = crate::BitReader;
#[doc = "Field `CPTEN4` writer - Capture Channel 4 Enable"]
pub type Cpten4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPTEN5` reader - Capture Channel 5 Enable"]
pub type Cpten5R = crate::BitReader;
#[doc = "Field `CPTEN5` writer - Capture Channel 5 Enable"]
pub type Cpten5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Enhanced Resolution"]
    #[inline(always)]
    pub fn resolution(&self) -> ResolutionR {
        ResolutionR::new(((self.bits >> 5) & 3) as u8)
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
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization Selection"]
    #[inline(always)]
    pub fn prescsync(&self) -> PrescsyncR {
        PrescsyncR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Auto Lock"]
    #[inline(always)]
    pub fn alock(&self) -> AlockR {
        AlockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Master Synchronization (only for TCC Slave Instance)"]
    #[inline(always)]
    pub fn msync(&self) -> MsyncR {
        MsyncR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 23 - DMA One-shot Trigger Mode"]
    #[inline(always)]
    pub fn dmaos(&self) -> DmaosR {
        DmaosR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Capture Channel 0 Enable"]
    #[inline(always)]
    pub fn cpten0(&self) -> Cpten0R {
        Cpten0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Capture Channel 1 Enable"]
    #[inline(always)]
    pub fn cpten1(&self) -> Cpten1R {
        Cpten1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Capture Channel 2 Enable"]
    #[inline(always)]
    pub fn cpten2(&self) -> Cpten2R {
        Cpten2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Capture Channel 3 Enable"]
    #[inline(always)]
    pub fn cpten3(&self) -> Cpten3R {
        Cpten3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Capture Channel 4 Enable"]
    #[inline(always)]
    pub fn cpten4(&self) -> Cpten4R {
        Cpten4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Capture Channel 5 Enable"]
    #[inline(always)]
    pub fn cpten5(&self) -> Cpten5R {
        Cpten5R::new(((self.bits >> 29) & 1) != 0)
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
    #[doc = "Bits 5:6 - Enhanced Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn resolution(&mut self) -> ResolutionW<CtrlaSpec> {
        ResolutionW::new(self, 5)
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
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization Selection"]
    #[inline(always)]
    #[must_use]
    pub fn prescsync(&mut self) -> PrescsyncW<CtrlaSpec> {
        PrescsyncW::new(self, 12)
    }
    #[doc = "Bit 14 - Auto Lock"]
    #[inline(always)]
    #[must_use]
    pub fn alock(&mut self) -> AlockW<CtrlaSpec> {
        AlockW::new(self, 14)
    }
    #[doc = "Bit 15 - Master Synchronization (only for TCC Slave Instance)"]
    #[inline(always)]
    #[must_use]
    pub fn msync(&mut self) -> MsyncW<CtrlaSpec> {
        MsyncW::new(self, 15)
    }
    #[doc = "Bit 23 - DMA One-shot Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmaos(&mut self) -> DmaosW<CtrlaSpec> {
        DmaosW::new(self, 23)
    }
    #[doc = "Bit 24 - Capture Channel 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten0(&mut self) -> Cpten0W<CtrlaSpec> {
        Cpten0W::new(self, 24)
    }
    #[doc = "Bit 25 - Capture Channel 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten1(&mut self) -> Cpten1W<CtrlaSpec> {
        Cpten1W::new(self, 25)
    }
    #[doc = "Bit 26 - Capture Channel 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten2(&mut self) -> Cpten2W<CtrlaSpec> {
        Cpten2W::new(self, 26)
    }
    #[doc = "Bit 27 - Capture Channel 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten3(&mut self) -> Cpten3W<CtrlaSpec> {
        Cpten3W::new(self, 27)
    }
    #[doc = "Bit 28 - Capture Channel 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten4(&mut self) -> Cpten4W<CtrlaSpec> {
        Cpten4W::new(self, 28)
    }
    #[doc = "Bit 29 - Capture Channel 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpten5(&mut self) -> Cpten5W<CtrlaSpec> {
        Cpten5W::new(self, 29)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u32 = 0;
}
