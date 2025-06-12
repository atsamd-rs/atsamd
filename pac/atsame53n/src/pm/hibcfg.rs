#[doc = "Register `HIBCFG` reader"]
pub type R = crate::R<HibcfgSpec>;
#[doc = "Register `HIBCFG` writer"]
pub type W = crate::W<HibcfgSpec>;
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
#[doc = "Backup Ram Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bramcfgselect {
    #[doc = "0: All the backup RAM is retained"]
    Ret = 0,
    #[doc = "1: Only the first 4Kbytes of the backup RAM is retained"]
    Partial = 1,
    #[doc = "2: All the backup RAM is turned OFF"]
    Off = 2,
}
impl From<Bramcfgselect> for u8 {
    #[inline(always)]
    fn from(variant: Bramcfgselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bramcfgselect {
    type Ux = u8;
}
impl crate::IsEnum for Bramcfgselect {}
#[doc = "Field `BRAMCFG` reader - Backup Ram Configuration"]
pub type BramcfgR = crate::FieldReader<Bramcfgselect>;
impl BramcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bramcfgselect> {
        match self.bits {
            0 => Some(Bramcfgselect::Ret),
            1 => Some(Bramcfgselect::Partial),
            2 => Some(Bramcfgselect::Off),
            _ => None,
        }
    }
    #[doc = "All the backup RAM is retained"]
    #[inline(always)]
    pub fn is_ret(&self) -> bool {
        *self == Bramcfgselect::Ret
    }
    #[doc = "Only the first 4Kbytes of the backup RAM is retained"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == Bramcfgselect::Partial
    }
    #[doc = "All the backup RAM is turned OFF"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Bramcfgselect::Off
    }
}
#[doc = "Field `BRAMCFG` writer - Backup Ram Configuration"]
pub type BramcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bramcfgselect>;
impl<'a, REG> BramcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All the backup RAM is retained"]
    #[inline(always)]
    pub fn ret(self) -> &'a mut crate::W<REG> {
        self.variant(Bramcfgselect::Ret)
    }
    #[doc = "Only the first 4Kbytes of the backup RAM is retained"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut crate::W<REG> {
        self.variant(Bramcfgselect::Partial)
    }
    #[doc = "All the backup RAM is turned OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Bramcfgselect::Off)
    }
}
impl R {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&self) -> RamcfgR {
        RamcfgR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Backup Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&self) -> BramcfgR {
        BramcfgR::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ram Configuration"]
    #[inline(always)]
    pub fn ramcfg(&mut self) -> RamcfgW<HibcfgSpec> {
        RamcfgW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Backup Ram Configuration"]
    #[inline(always)]
    pub fn bramcfg(&mut self) -> BramcfgW<HibcfgSpec> {
        BramcfgW::new(self, 2)
    }
}
#[doc = "Hibernate Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hibcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hibcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HibcfgSpec;
impl crate::RegisterSpec for HibcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hibcfg::R`](R) reader structure"]
impl crate::Readable for HibcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hibcfg::W`](W) writer structure"]
impl crate::Writable for HibcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIBCFG to value 0"]
impl crate::Resettable for HibcfgSpec {}
