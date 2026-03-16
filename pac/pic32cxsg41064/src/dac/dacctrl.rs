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
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DITHER` reader - Dithering Mode"]
pub type DitherR = crate::BitReader;
#[doc = "Field `DITHER` writer - Dithering Mode"]
pub type DitherW<'a, REG> = crate::BitWriter<'a, REG>;
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
