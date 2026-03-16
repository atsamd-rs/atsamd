#[doc = "Register `STDBYCFG` reader"]
pub type R = crate::R<StdbycfgSpec>;
#[doc = "Register `STDBYCFG` writer"]
pub type W = crate::W<StdbycfgSpec>;
#[doc = "Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ramcfgselect {
    #[doc = "0: All the system RAM is retained"]
    Ret = 0,
    #[doc = "1: Only the first 32Kbytes of the system RAM is retained"]
    Partial = 1,
    #[doc = "2: All the system RAM is turned OFF"]
    Off = 2,
}
impl From<Ramcfgselect> for u8 {
    #[inline(always)]
    fn from(variant: Ramcfgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ramcfgselect {
    type Ux = u8;
}
impl crate::IsEnum for Ramcfgselect {}
#[doc = "Field `RAMCFG` reader - Ram Configuration"]
pub type RamcfgR = crate::FieldReader<Ramcfgselect>;
impl RamcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ramcfgselect> {
        match self.bits {
            0 => Some(Ramcfgselect::Ret),
            1 => Some(Ramcfgselect::Partial),
            2 => Some(Ramcfgselect::Off),
            _ => None,
        }
    }
    #[doc = "All the system RAM is retained"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == Ramcfgselect::Ret
    }
    #[doc = "Only the first 32Kbytes of the system RAM is retained"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == Ramcfgselect::Partial
    }
    #[doc = "All the system RAM is turned OFF"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ramcfgselect::Off
    }
}
#[doc = "Field `RAMCFG` writer - Ram Configuration"]
pub type RamcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ramcfgselect>;
impl<'a, REG> RamcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All the system RAM is retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut crate::W<REG> {
        self.variant(Ramcfgselect::Ret)
    }
    #[doc = "Only the first 32Kbytes of the system RAM is retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut crate::W<REG> {
        self.variant(Ramcfgselect::Partial)
    }
    #[doc = "All the system RAM is turned OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Ramcfgselect::Off)
    }
}
#[doc = "Fast Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fastwkupselect {
    #[doc = "0: Fast Wakeup is disabled"]
    No = 0,
    #[doc = "1: Fast Wakeup is enabled on NVM"]
    Nvm = 1,
    #[doc = "2: Fast Wakeup is enabled on the main voltage regulator (MAINVREG)"]
    Mainvreg = 2,
    #[doc = "3: Fast Wakeup is enabled on both NVM and MAINVREG"]
    Both = 3,
}
impl From<Fastwkupselect> for u8 {
    #[inline(always)]
    fn from(variant: Fastwkupselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fastwkupselect {
    type Ux = u8;
}
impl crate::IsEnum for Fastwkupselect {}
#[doc = "Field `FASTWKUP` reader - Fast Wakeup"]
pub type FastwkupR = crate::FieldReader<Fastwkupselect>;
impl FastwkupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fastwkupselect {
        match self.bits {
            0 => Fastwkupselect::No,
            1 => Fastwkupselect::Nvm,
            2 => Fastwkupselect::Mainvreg,
            3 => Fastwkupselect::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Fast Wakeup is disabled"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Fastwkupselect::No
    }
    #[doc = "Fast Wakeup is enabled on NVM"]
    #[inline(always)]
    pub fn is_nvm(&self) -> bool {
        *self == Fastwkupselect::Nvm
    }
    #[doc = "Fast Wakeup is enabled on the main voltage regulator (MAINVREG)"]
    #[inline(always)]
    pub fn is_mainvreg(&self) -> bool {
        *self == Fastwkupselect::Mainvreg
    }
    #[doc = "Fast Wakeup is enabled on both NVM and MAINVREG"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Fastwkupselect::Both
    }
}
#[doc = "Field `FASTWKUP` writer - Fast Wakeup"]
pub type FastwkupW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fastwkupselect, crate::Safe>;
impl<'a, REG> FastwkupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fast Wakeup is disabled"]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(Fastwkupselect::No)
    }
    #[doc = "Fast Wakeup is enabled on NVM"]
    #[inline(always)]
    pub fn nvm(self) -> &'a mut crate::W<REG> {
        self.variant(Fastwkupselect::Nvm)
    }
    #[doc = "Fast Wakeup is enabled on the main voltage regulator (MAINVREG)"]
    #[inline(always)]
    pub fn mainvreg(self) -> &'a mut crate::W<REG> {
        self.variant(Fastwkupselect::Mainvreg)
    }
    #[doc = "Fast Wakeup is enabled on both NVM and MAINVREG"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Fastwkupselect::Both)
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&self) -> RamcfgR {
        RamcfgR::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&self) -> FastwkupR {
        FastwkupR::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ramcfg(&mut self) -> RamcfgW<StdbycfgSpec> {
        RamcfgW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Fast Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fastwkup(&mut self) -> FastwkupW<StdbycfgSpec> {
        FastwkupW::new(self, 4)
    }
}
#[doc = "Standby Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`stdbycfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stdbycfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StdbycfgSpec;
impl crate::RegisterSpec for StdbycfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`stdbycfg::R`](R) reader structure"]
impl crate::Readable for StdbycfgSpec {}
#[doc = "`write(|w| ..)` method takes [`stdbycfg::W`](W) writer structure"]
impl crate::Writable for StdbycfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets STDBYCFG to value 0"]
impl crate::Resettable for StdbycfgSpec {
    const RESET_VALUE: u8 = 0;
}
