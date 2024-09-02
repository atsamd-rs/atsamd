#[doc = "Register `SWTRIGCTRL` reader"]
pub type R = crate::R<SwtrigctrlSpec>;
#[doc = "Register `SWTRIGCTRL` writer"]
pub type W = crate::W<SwtrigctrlSpec>;
#[doc = "Field `SWTRIG0` reader - Channel 0 Software Trigger"]
pub type Swtrig0R = crate::BitReader;
#[doc = "Field `SWTRIG0` writer - Channel 0 Software Trigger"]
pub type Swtrig0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG1` reader - Channel 1 Software Trigger"]
pub type Swtrig1R = crate::BitReader;
#[doc = "Field `SWTRIG1` writer - Channel 1 Software Trigger"]
pub type Swtrig1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG2` reader - Channel 2 Software Trigger"]
pub type Swtrig2R = crate::BitReader;
#[doc = "Field `SWTRIG2` writer - Channel 2 Software Trigger"]
pub type Swtrig2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG3` reader - Channel 3 Software Trigger"]
pub type Swtrig3R = crate::BitReader;
#[doc = "Field `SWTRIG3` writer - Channel 3 Software Trigger"]
pub type Swtrig3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG4` reader - Channel 4 Software Trigger"]
pub type Swtrig4R = crate::BitReader;
#[doc = "Field `SWTRIG4` writer - Channel 4 Software Trigger"]
pub type Swtrig4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG5` reader - Channel 5 Software Trigger"]
pub type Swtrig5R = crate::BitReader;
#[doc = "Field `SWTRIG5` writer - Channel 5 Software Trigger"]
pub type Swtrig5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG6` reader - Channel 6 Software Trigger"]
pub type Swtrig6R = crate::BitReader;
#[doc = "Field `SWTRIG6` writer - Channel 6 Software Trigger"]
pub type Swtrig6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG7` reader - Channel 7 Software Trigger"]
pub type Swtrig7R = crate::BitReader;
#[doc = "Field `SWTRIG7` writer - Channel 7 Software Trigger"]
pub type Swtrig7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG8` reader - Channel 8 Software Trigger"]
pub type Swtrig8R = crate::BitReader;
#[doc = "Field `SWTRIG8` writer - Channel 8 Software Trigger"]
pub type Swtrig8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG9` reader - Channel 9 Software Trigger"]
pub type Swtrig9R = crate::BitReader;
#[doc = "Field `SWTRIG9` writer - Channel 9 Software Trigger"]
pub type Swtrig9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG10` reader - Channel 10 Software Trigger"]
pub type Swtrig10R = crate::BitReader;
#[doc = "Field `SWTRIG10` writer - Channel 10 Software Trigger"]
pub type Swtrig10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG11` reader - Channel 11 Software Trigger"]
pub type Swtrig11R = crate::BitReader;
#[doc = "Field `SWTRIG11` writer - Channel 11 Software Trigger"]
pub type Swtrig11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    pub fn swtrig0(&self) -> Swtrig0R {
        Swtrig0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    pub fn swtrig1(&self) -> Swtrig1R {
        Swtrig1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    pub fn swtrig2(&self) -> Swtrig2R {
        Swtrig2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    pub fn swtrig3(&self) -> Swtrig3R {
        Swtrig3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    pub fn swtrig4(&self) -> Swtrig4R {
        Swtrig4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    pub fn swtrig5(&self) -> Swtrig5R {
        Swtrig5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    pub fn swtrig6(&self) -> Swtrig6R {
        Swtrig6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    pub fn swtrig7(&self) -> Swtrig7R {
        Swtrig7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    pub fn swtrig8(&self) -> Swtrig8R {
        Swtrig8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    pub fn swtrig9(&self) -> Swtrig9R {
        Swtrig9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    pub fn swtrig10(&self) -> Swtrig10R {
        Swtrig10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    pub fn swtrig11(&self) -> Swtrig11R {
        Swtrig11R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig0(&mut self) -> Swtrig0W<SwtrigctrlSpec> {
        Swtrig0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> Swtrig1W<SwtrigctrlSpec> {
        Swtrig1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig2(&mut self) -> Swtrig2W<SwtrigctrlSpec> {
        Swtrig2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig3(&mut self) -> Swtrig3W<SwtrigctrlSpec> {
        Swtrig3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig4(&mut self) -> Swtrig4W<SwtrigctrlSpec> {
        Swtrig4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig5(&mut self) -> Swtrig5W<SwtrigctrlSpec> {
        Swtrig5W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig6(&mut self) -> Swtrig6W<SwtrigctrlSpec> {
        Swtrig6W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig7(&mut self) -> Swtrig7W<SwtrigctrlSpec> {
        Swtrig7W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 8 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig8(&mut self) -> Swtrig8W<SwtrigctrlSpec> {
        Swtrig8W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 9 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig9(&mut self) -> Swtrig9W<SwtrigctrlSpec> {
        Swtrig9W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 10 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig10(&mut self) -> Swtrig10W<SwtrigctrlSpec> {
        Swtrig10W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 11 Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig11(&mut self) -> Swtrig11W<SwtrigctrlSpec> {
        Swtrig11W::new(self, 11)
    }
}
#[doc = "Software Trigger Control\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrigctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrigctrlSpec;
impl crate::RegisterSpec for SwtrigctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrigctrl::R`](R) reader structure"]
impl crate::Readable for SwtrigctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`swtrigctrl::W`](W) writer structure"]
impl crate::Writable for SwtrigctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWTRIGCTRL to value 0"]
impl crate::Resettable for SwtrigctrlSpec {
    const RESET_VALUE: u32 = 0;
}
