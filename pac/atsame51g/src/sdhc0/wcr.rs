#[doc = "Register `WCR` reader"]
pub type R = crate::R<WcrSpec>;
#[doc = "Register `WCR` writer"]
pub type W = crate::W<WcrSpec>;
#[doc = "Wakeup Event Enable on Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkencintselect {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Wkencintselect> for bool {
    #[inline(always)]
    fn from(variant: Wkencintselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKENCINT` reader - Wakeup Event Enable on Card Interrupt"]
pub type WkencintR = crate::BitReader<Wkencintselect>;
impl WkencintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkencintselect {
        match self.bits {
            false => Wkencintselect::Disable,
            true => Wkencintselect::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkencintselect::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkencintselect::Enable
    }
}
#[doc = "Field `WKENCINT` writer - Wakeup Event Enable on Card Interrupt"]
pub type WkencintW<'a, REG> = crate::BitWriter<'a, REG, Wkencintselect>;
impl<'a, REG> WkencintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkencintselect::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkencintselect::Enable)
    }
}
#[doc = "Wakeup Event Enable on Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkencinsselect {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Wkencinsselect> for bool {
    #[inline(always)]
    fn from(variant: Wkencinsselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKENCINS` reader - Wakeup Event Enable on Card Insertion"]
pub type WkencinsR = crate::BitReader<Wkencinsselect>;
impl WkencinsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkencinsselect {
        match self.bits {
            false => Wkencinsselect::Disable,
            true => Wkencinsselect::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkencinsselect::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkencinsselect::Enable
    }
}
#[doc = "Field `WKENCINS` writer - Wakeup Event Enable on Card Insertion"]
pub type WkencinsW<'a, REG> = crate::BitWriter<'a, REG, Wkencinsselect>;
impl<'a, REG> WkencinsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkencinsselect::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkencinsselect::Enable)
    }
}
#[doc = "Wakeup Event Enable on Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkencremselect {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Wkencremselect> for bool {
    #[inline(always)]
    fn from(variant: Wkencremselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKENCREM` reader - Wakeup Event Enable on Card Removal"]
pub type WkencremR = crate::BitReader<Wkencremselect>;
impl WkencremR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkencremselect {
        match self.bits {
            false => Wkencremselect::Disable,
            true => Wkencremselect::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wkencremselect::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wkencremselect::Enable
    }
}
#[doc = "Field `WKENCREM` writer - Wakeup Event Enable on Card Removal"]
pub type WkencremW<'a, REG> = crate::BitWriter<'a, REG, Wkencremselect>;
impl<'a, REG> WkencremW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkencremselect::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wkencremselect::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkencint(&self) -> WkencintR {
        WkencintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on Card Insertion"]
    #[inline(always)]
    pub fn wkencins(&self) -> WkencinsR {
        WkencinsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on Card Removal"]
    #[inline(always)]
    pub fn wkencrem(&self) -> WkencremR {
        WkencremR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable on Card Interrupt"]
    #[inline(always)]
    pub fn wkencint(&mut self) -> WkencintW<WcrSpec> {
        WkencintW::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable on Card Insertion"]
    #[inline(always)]
    pub fn wkencins(&mut self) -> WkencinsW<WcrSpec> {
        WkencinsW::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Event Enable on Card Removal"]
    #[inline(always)]
    pub fn wkencrem(&mut self) -> WkencremW<WcrSpec> {
        WkencremW::new(self, 2)
    }
}
#[doc = "Wakeup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`wcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcrSpec;
impl crate::RegisterSpec for WcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wcr::R`](R) reader structure"]
impl crate::Readable for WcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wcr::W`](W) writer structure"]
impl crate::Writable for WcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WcrSpec {}
