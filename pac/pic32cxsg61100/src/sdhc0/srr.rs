#[doc = "Register `SRR` reader"]
pub type R = crate::R<SrrSpec>;
#[doc = "Register `SRR` writer"]
pub type W = crate::W<SrrSpec>;
#[doc = "Software Reset For All\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstallselect {
    #[doc = "0: Work"]
    Work = 0,
    #[doc = "1: Reset"]
    Reset = 1,
}
impl From<Swrstallselect> for bool {
    #[inline(always)]
    fn from(variant: Swrstallselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTALL` reader - Software Reset For All"]
pub type SwrstallR = crate::BitReader<Swrstallselect>;
impl SwrstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstallselect {
        match self.bits {
            false => Swrstallselect::Work,
            true => Swrstallselect::Reset,
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == Swrstallselect::Work
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Swrstallselect::Reset
    }
}
#[doc = "Field `SWRSTALL` writer - Software Reset For All"]
pub type SwrstallW<'a, REG> = crate::BitWriter<'a, REG, Swrstallselect>;
impl<'a, REG> SwrstallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstallselect::Work)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstallselect::Reset)
    }
}
#[doc = "Software Reset For CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstcmdselect {
    #[doc = "0: Work"]
    Work = 0,
    #[doc = "1: Reset"]
    Reset = 1,
}
impl From<Swrstcmdselect> for bool {
    #[inline(always)]
    fn from(variant: Swrstcmdselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTCMD` reader - Software Reset For CMD Line"]
pub type SwrstcmdR = crate::BitReader<Swrstcmdselect>;
impl SwrstcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstcmdselect {
        match self.bits {
            false => Swrstcmdselect::Work,
            true => Swrstcmdselect::Reset,
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == Swrstcmdselect::Work
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Swrstcmdselect::Reset
    }
}
#[doc = "Field `SWRSTCMD` writer - Software Reset For CMD Line"]
pub type SwrstcmdW<'a, REG> = crate::BitWriter<'a, REG, Swrstcmdselect>;
impl<'a, REG> SwrstcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstcmdselect::Work)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstcmdselect::Reset)
    }
}
#[doc = "Software Reset For DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrstdatselect {
    #[doc = "0: Work"]
    Work = 0,
    #[doc = "1: Reset"]
    Reset = 1,
}
impl From<Swrstdatselect> for bool {
    #[inline(always)]
    fn from(variant: Swrstdatselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRSTDAT` reader - Software Reset For DAT Line"]
pub type SwrstdatR = crate::BitReader<Swrstdatselect>;
impl SwrstdatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrstdatselect {
        match self.bits {
            false => Swrstdatselect::Work,
            true => Swrstdatselect::Reset,
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn is_work(&self) -> bool {
        *self == Swrstdatselect::Work
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Swrstdatselect::Reset
    }
}
#[doc = "Field `SWRSTDAT` writer - Software Reset For DAT Line"]
pub type SwrstdatW<'a, REG> = crate::BitWriter<'a, REG, Swrstdatselect>;
impl<'a, REG> SwrstdatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Work"]
    #[inline(always)]
    pub fn work(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstdatselect::Work)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Swrstdatselect::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline(always)]
    pub fn swrstall(&self) -> SwrstallR {
        SwrstallR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline(always)]
    pub fn swrstcmd(&self) -> SwrstcmdR {
        SwrstcmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline(always)]
    pub fn swrstdat(&self) -> SwrstdatR {
        SwrstdatR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All"]
    #[inline(always)]
    #[must_use]
    pub fn swrstall(&mut self) -> SwrstallW<SrrSpec> {
        SwrstallW::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line"]
    #[inline(always)]
    #[must_use]
    pub fn swrstcmd(&mut self) -> SwrstcmdW<SrrSpec> {
        SwrstcmdW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line"]
    #[inline(always)]
    #[must_use]
    pub fn swrstdat(&mut self) -> SwrstdatW<SrrSpec> {
        SwrstdatW::new(self, 2)
    }
}
#[doc = "Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`srr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrrSpec;
impl crate::RegisterSpec for SrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`srr::R`](R) reader structure"]
impl crate::Readable for SrrSpec {}
#[doc = "`write(|w| ..)` method takes [`srr::W`](W) writer structure"]
impl crate::Writable for SrrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SRR to value 0"]
impl crate::Resettable for SrrSpec {
    const RESET_VALUE: u8 = 0;
}
