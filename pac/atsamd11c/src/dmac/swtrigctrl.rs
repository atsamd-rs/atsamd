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
