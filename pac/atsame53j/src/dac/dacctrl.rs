#[doc = "Register `DACCTRL[%s]` reader"]
pub type R = crate::R<DacctrlSpec>;
#[doc = "Register `DACCTRL[%s]` writer"]
pub type W = crate::W<DacctrlSpec>;
#[doc = "Field `LEFTADJ` reader - Left Adjusted Data"]
pub type LeftadjR = crate::BitReader;
#[doc = "Field `LEFTADJ` writer - Left Adjusted Data"]
pub type LeftadjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable DAC0"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable DAC0"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Current Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cctrlselect {
    #[doc = "0: 100kSPS"]
    Cc100k = 0,
    #[doc = "1: 500kSPS"]
    Cc1m = 1,
    #[doc = "2: 1MSPS"]
    Cc12m = 2,
}
impl From<Cctrlselect> for u8 {
    #[inline(always)]
    fn from(variant: Cctrlselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cctrlselect {
    type Ux = u8;
}
impl crate::IsEnum for Cctrlselect {}
#[doc = "Field `CCTRL` reader - Current Control"]
pub type CctrlR = crate::FieldReader<Cctrlselect>;
impl CctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cctrlselect> {
        match self.bits {
            0 => Some(Cctrlselect::Cc100k),
            1 => Some(Cctrlselect::Cc1m),
            2 => Some(Cctrlselect::Cc12m),
            _ => None,
        }
    }
    #[doc = "100kSPS"]
    #[inline(always)]
    pub fn is_cc100k(&self) -> bool {
        *self == Cctrlselect::Cc100k
    }
    #[doc = "500kSPS"]
    #[inline(always)]
    pub fn is_cc1m(&self) -> bool {
        *self == Cctrlselect::Cc1m
    }
    #[doc = "1MSPS"]
    #[inline(always)]
    pub fn is_cc12m(&self) -> bool {
        *self == Cctrlselect::Cc12m
    }
}
#[doc = "Field `CCTRL` writer - Current Control"]
pub type CctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cctrlselect>;
impl<'a, REG> CctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "100kSPS"]
    #[inline(always)]
    pub fn cc100k(self) -> &'a mut crate::W<REG> {
        self.variant(Cctrlselect::Cc100k)
    }
    #[doc = "500kSPS"]
    #[inline(always)]
    pub fn cc1m(self) -> &'a mut crate::W<REG> {
        self.variant(Cctrlselect::Cc1m)
    }
    #[doc = "1MSPS"]
    #[inline(always)]
    pub fn cc12m(self) -> &'a mut crate::W<REG> {
        self.variant(Cctrlselect::Cc12m)
    }
}
#[doc = "Field `FEXT` reader - Standalone Filter"]
pub type FextR = crate::BitReader;
#[doc = "Field `FEXT` writer - Standalone Filter"]
pub type FextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DITHER` reader - Dithering Mode"]
pub type DitherR = crate::BitReader;
#[doc = "Field `DITHER` writer - Dithering Mode"]
pub type DitherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Refresh period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refreshselect {
    #[doc = "0: Do not Refresh"]
    Refresh0 = 0,
    #[doc = "1: Refresh every 30 us"]
    Refresh1 = 1,
    #[doc = "2: Refresh every 60 us"]
    Refresh2 = 2,
    #[doc = "3: Refresh every 90 us"]
    Refresh3 = 3,
    #[doc = "4: Refresh every 120 us"]
    Refresh4 = 4,
    #[doc = "5: Refresh every 150 us"]
    Refresh5 = 5,
    #[doc = "6: Refresh every 180 us"]
    Refresh6 = 6,
    #[doc = "7: Refresh every 210 us"]
    Refresh7 = 7,
    #[doc = "8: Refresh every 240 us"]
    Refresh8 = 8,
    #[doc = "9: Refresh every 270 us"]
    Refresh9 = 9,
    #[doc = "10: Refresh every 300 us"]
    Refresh10 = 10,
    #[doc = "11: Refresh every 330 us"]
    Refresh11 = 11,
    #[doc = "12: Refresh every 360 us"]
    Refresh12 = 12,
    #[doc = "13: Refresh every 390 us"]
    Refresh13 = 13,
    #[doc = "14: Refresh every 420 us"]
    Refresh14 = 14,
    #[doc = "15: Refresh every 450 us"]
    Refresh15 = 15,
}
impl From<Refreshselect> for u8 {
    #[inline(always)]
    fn from(variant: Refreshselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refreshselect {
    type Ux = u8;
}
impl crate::IsEnum for Refreshselect {}
#[doc = "Field `REFRESH` reader - Refresh period"]
pub type RefreshR = crate::FieldReader<Refreshselect>;
impl RefreshR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refreshselect {
        match self.bits {
            0 => Refreshselect::Refresh0,
            1 => Refreshselect::Refresh1,
            2 => Refreshselect::Refresh2,
            3 => Refreshselect::Refresh3,
            4 => Refreshselect::Refresh4,
            5 => Refreshselect::Refresh5,
            6 => Refreshselect::Refresh6,
            7 => Refreshselect::Refresh7,
            8 => Refreshselect::Refresh8,
            9 => Refreshselect::Refresh9,
            10 => Refreshselect::Refresh10,
            11 => Refreshselect::Refresh11,
            12 => Refreshselect::Refresh12,
            13 => Refreshselect::Refresh13,
            14 => Refreshselect::Refresh14,
            15 => Refreshselect::Refresh15,
            _ => unreachable!(),
        }
    }
    #[doc = "Do not Refresh"]
    #[inline(always)]
    pub fn is_refresh_0(&self) -> bool {
        *self == Refreshselect::Refresh0
    }
    #[doc = "Refresh every 30 us"]
    #[inline(always)]
    pub fn is_refresh_1(&self) -> bool {
        *self == Refreshselect::Refresh1
    }
    #[doc = "Refresh every 60 us"]
    #[inline(always)]
    pub fn is_refresh_2(&self) -> bool {
        *self == Refreshselect::Refresh2
    }
    #[doc = "Refresh every 90 us"]
    #[inline(always)]
    pub fn is_refresh_3(&self) -> bool {
        *self == Refreshselect::Refresh3
    }
    #[doc = "Refresh every 120 us"]
    #[inline(always)]
    pub fn is_refresh_4(&self) -> bool {
        *self == Refreshselect::Refresh4
    }
    #[doc = "Refresh every 150 us"]
    #[inline(always)]
    pub fn is_refresh_5(&self) -> bool {
        *self == Refreshselect::Refresh5
    }
    #[doc = "Refresh every 180 us"]
    #[inline(always)]
    pub fn is_refresh_6(&self) -> bool {
        *self == Refreshselect::Refresh6
    }
    #[doc = "Refresh every 210 us"]
    #[inline(always)]
    pub fn is_refresh_7(&self) -> bool {
        *self == Refreshselect::Refresh7
    }
    #[doc = "Refresh every 240 us"]
    #[inline(always)]
    pub fn is_refresh_8(&self) -> bool {
        *self == Refreshselect::Refresh8
    }
    #[doc = "Refresh every 270 us"]
    #[inline(always)]
    pub fn is_refresh_9(&self) -> bool {
        *self == Refreshselect::Refresh9
    }
    #[doc = "Refresh every 300 us"]
    #[inline(always)]
    pub fn is_refresh_10(&self) -> bool {
        *self == Refreshselect::Refresh10
    }
    #[doc = "Refresh every 330 us"]
    #[inline(always)]
    pub fn is_refresh_11(&self) -> bool {
        *self == Refreshselect::Refresh11
    }
    #[doc = "Refresh every 360 us"]
    #[inline(always)]
    pub fn is_refresh_12(&self) -> bool {
        *self == Refreshselect::Refresh12
    }
    #[doc = "Refresh every 390 us"]
    #[inline(always)]
    pub fn is_refresh_13(&self) -> bool {
        *self == Refreshselect::Refresh13
    }
    #[doc = "Refresh every 420 us"]
    #[inline(always)]
    pub fn is_refresh_14(&self) -> bool {
        *self == Refreshselect::Refresh14
    }
    #[doc = "Refresh every 450 us"]
    #[inline(always)]
    pub fn is_refresh_15(&self) -> bool {
        *self == Refreshselect::Refresh15
    }
}
#[doc = "Field `REFRESH` writer - Refresh period"]
pub type RefreshW<'a, REG> = crate::FieldWriter<'a, REG, 4, Refreshselect, crate::Safe>;
impl<'a, REG> RefreshW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not Refresh"]
    #[inline(always)]
    pub fn refresh_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh0)
    }
    #[doc = "Refresh every 30 us"]
    #[inline(always)]
    pub fn refresh_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh1)
    }
    #[doc = "Refresh every 60 us"]
    #[inline(always)]
    pub fn refresh_2(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh2)
    }
    #[doc = "Refresh every 90 us"]
    #[inline(always)]
    pub fn refresh_3(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh3)
    }
    #[doc = "Refresh every 120 us"]
    #[inline(always)]
    pub fn refresh_4(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh4)
    }
    #[doc = "Refresh every 150 us"]
    #[inline(always)]
    pub fn refresh_5(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh5)
    }
    #[doc = "Refresh every 180 us"]
    #[inline(always)]
    pub fn refresh_6(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh6)
    }
    #[doc = "Refresh every 210 us"]
    #[inline(always)]
    pub fn refresh_7(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh7)
    }
    #[doc = "Refresh every 240 us"]
    #[inline(always)]
    pub fn refresh_8(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh8)
    }
    #[doc = "Refresh every 270 us"]
    #[inline(always)]
    pub fn refresh_9(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh9)
    }
    #[doc = "Refresh every 300 us"]
    #[inline(always)]
    pub fn refresh_10(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh10)
    }
    #[doc = "Refresh every 330 us"]
    #[inline(always)]
    pub fn refresh_11(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh11)
    }
    #[doc = "Refresh every 360 us"]
    #[inline(always)]
    pub fn refresh_12(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh12)
    }
    #[doc = "Refresh every 390 us"]
    #[inline(always)]
    pub fn refresh_13(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh13)
    }
    #[doc = "Refresh every 420 us"]
    #[inline(always)]
    pub fn refresh_14(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh14)
    }
    #[doc = "Refresh every 450 us"]
    #[inline(always)]
    pub fn refresh_15(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshselect::Refresh15)
    }
}
#[doc = "Sampling Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osrselect {
    #[doc = "0: No Over Sampling"]
    Osr1 = 0,
    #[doc = "1: 2x Over Sampling Ratio"]
    Osr2 = 1,
    #[doc = "2: 4x Over Sampling Ratio"]
    Osr4 = 2,
    #[doc = "3: 8x Over Sampling Ratio"]
    Osr8 = 3,
    #[doc = "4: 16x Over Sampling Ratio"]
    Osr16 = 4,
    #[doc = "5: 32x Over Sampling Ratio"]
    Osr32 = 5,
}
impl From<Osrselect> for u8 {
    #[inline(always)]
    fn from(variant: Osrselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Osrselect {
    type Ux = u8;
}
impl crate::IsEnum for Osrselect {}
#[doc = "Field `OSR` reader - Sampling Rate"]
pub type OsrR = crate::FieldReader<Osrselect>;
impl OsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Osrselect> {
        match self.bits {
            0 => Some(Osrselect::Osr1),
            1 => Some(Osrselect::Osr2),
            2 => Some(Osrselect::Osr4),
            3 => Some(Osrselect::Osr8),
            4 => Some(Osrselect::Osr16),
            5 => Some(Osrselect::Osr32),
            _ => None,
        }
    }
    #[doc = "No Over Sampling"]
    #[inline(always)]
    pub fn is_osr_1(&self) -> bool {
        *self == Osrselect::Osr1
    }
    #[doc = "2x Over Sampling Ratio"]
    #[inline(always)]
    pub fn is_osr_2(&self) -> bool {
        *self == Osrselect::Osr2
    }
    #[doc = "4x Over Sampling Ratio"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == Osrselect::Osr4
    }
    #[doc = "8x Over Sampling Ratio"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == Osrselect::Osr8
    }
    #[doc = "16x Over Sampling Ratio"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == Osrselect::Osr16
    }
    #[doc = "32x Over Sampling Ratio"]
    #[inline(always)]
    pub fn is_osr_32(&self) -> bool {
        *self == Osrselect::Osr32
    }
}
#[doc = "Field `OSR` writer - Sampling Rate"]
pub type OsrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Osrselect>;
impl<'a, REG> OsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Over Sampling"]
    #[inline(always)]
    pub fn osr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr1)
    }
    #[doc = "2x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_2(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr2)
    }
    #[doc = "4x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr4)
    }
    #[doc = "8x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr8)
    }
    #[doc = "16x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr16)
    }
    #[doc = "32x Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr_32(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr32)
    }
}
impl R {
    #[doc = "Bit 0 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&self) -> LeftadjR {
        LeftadjR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable DAC0"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Current Control"]
    #[inline(always)]
    pub fn cctrl(&self) -> CctrlR {
        CctrlR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Standalone Filter"]
    #[inline(always)]
    pub fn fext(&self) -> FextR {
        FextR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dithering Mode"]
    #[inline(always)]
    pub fn dither(&self) -> DitherR {
        DitherR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Refresh period"]
    #[inline(always)]
    pub fn refresh(&self) -> RefreshR {
        RefreshR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Sampling Rate"]
    #[inline(always)]
    pub fn osr(&self) -> OsrR {
        OsrR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Left Adjusted Data"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LeftadjW<DacctrlSpec> {
        LeftadjW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<DacctrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn cctrl(&mut self) -> CctrlW<DacctrlSpec> {
        CctrlW::new(self, 2)
    }
    #[doc = "Bit 5 - Standalone Filter"]
    #[inline(always)]
    #[must_use]
    pub fn fext(&mut self) -> FextW<DacctrlSpec> {
        FextW::new(self, 5)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<DacctrlSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - Dithering Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DitherW<DacctrlSpec> {
        DitherW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Refresh period"]
    #[inline(always)]
    #[must_use]
    pub fn refresh(&mut self) -> RefreshW<DacctrlSpec> {
        RefreshW::new(self, 8)
    }
    #[doc = "Bits 13:15 - Sampling Rate"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OsrW<DacctrlSpec> {
        OsrW::new(self, 13)
    }
}
#[doc = "DAC n Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dacctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacctrlSpec;
impl crate::RegisterSpec for DacctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dacctrl::R`](R) reader structure"]
impl crate::Readable for DacctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dacctrl::W`](W) writer structure"]
impl crate::Writable for DacctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DACCTRL[%s]
to value 0"]
impl crate::Resettable for DacctrlSpec {
    const RESET_VALUE: u16 = 0;
}
