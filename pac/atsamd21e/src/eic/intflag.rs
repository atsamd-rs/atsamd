#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `EXTINT0` reader - External Interrupt 0"]
pub type Extint0R = crate::BitReader;
#[doc = "Field `EXTINT0` writer - External Interrupt 0"]
pub type Extint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT1` reader - External Interrupt 1"]
pub type Extint1R = crate::BitReader;
#[doc = "Field `EXTINT1` writer - External Interrupt 1"]
pub type Extint1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT2` reader - External Interrupt 2"]
pub type Extint2R = crate::BitReader;
#[doc = "Field `EXTINT2` writer - External Interrupt 2"]
pub type Extint2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT3` reader - External Interrupt 3"]
pub type Extint3R = crate::BitReader;
#[doc = "Field `EXTINT3` writer - External Interrupt 3"]
pub type Extint3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT4` reader - External Interrupt 4"]
pub type Extint4R = crate::BitReader;
#[doc = "Field `EXTINT4` writer - External Interrupt 4"]
pub type Extint4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT5` reader - External Interrupt 5"]
pub type Extint5R = crate::BitReader;
#[doc = "Field `EXTINT5` writer - External Interrupt 5"]
pub type Extint5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT6` reader - External Interrupt 6"]
pub type Extint6R = crate::BitReader;
#[doc = "Field `EXTINT6` writer - External Interrupt 6"]
pub type Extint6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT7` reader - External Interrupt 7"]
pub type Extint7R = crate::BitReader;
#[doc = "Field `EXTINT7` writer - External Interrupt 7"]
pub type Extint7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT8` reader - External Interrupt 8"]
pub type Extint8R = crate::BitReader;
#[doc = "Field `EXTINT8` writer - External Interrupt 8"]
pub type Extint8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT9` reader - External Interrupt 9"]
pub type Extint9R = crate::BitReader;
#[doc = "Field `EXTINT9` writer - External Interrupt 9"]
pub type Extint9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT10` reader - External Interrupt 10"]
pub type Extint10R = crate::BitReader;
#[doc = "Field `EXTINT10` writer - External Interrupt 10"]
pub type Extint10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT11` reader - External Interrupt 11"]
pub type Extint11R = crate::BitReader;
#[doc = "Field `EXTINT11` writer - External Interrupt 11"]
pub type Extint11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT12` reader - External Interrupt 12"]
pub type Extint12R = crate::BitReader;
#[doc = "Field `EXTINT12` writer - External Interrupt 12"]
pub type Extint12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT13` reader - External Interrupt 13"]
pub type Extint13R = crate::BitReader;
#[doc = "Field `EXTINT13` writer - External Interrupt 13"]
pub type Extint13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT14` reader - External Interrupt 14"]
pub type Extint14R = crate::BitReader;
#[doc = "Field `EXTINT14` writer - External Interrupt 14"]
pub type Extint14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT15` reader - External Interrupt 15"]
pub type Extint15R = crate::BitReader;
#[doc = "Field `EXTINT15` writer - External Interrupt 15"]
pub type Extint15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0"]
    #[inline(always)]
    pub fn extint0(&self) -> Extint0R {
        Extint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn extint1(&self) -> Extint1R {
        Extint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn extint2(&self) -> Extint2R {
        Extint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn extint3(&self) -> Extint3R {
        Extint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn extint4(&self) -> Extint4R {
        Extint4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn extint5(&self) -> Extint5R {
        Extint5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn extint6(&self) -> Extint6R {
        Extint6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn extint7(&self) -> Extint7R {
        Extint7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8"]
    #[inline(always)]
    pub fn extint8(&self) -> Extint8R {
        Extint8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9"]
    #[inline(always)]
    pub fn extint9(&self) -> Extint9R {
        Extint9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10"]
    #[inline(always)]
    pub fn extint10(&self) -> Extint10R {
        Extint10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11"]
    #[inline(always)]
    pub fn extint11(&self) -> Extint11R {
        Extint11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12"]
    #[inline(always)]
    pub fn extint12(&self) -> Extint12R {
        Extint12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13"]
    #[inline(always)]
    pub fn extint13(&self) -> Extint13R {
        Extint13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14"]
    #[inline(always)]
    pub fn extint14(&self) -> Extint14R {
        Extint14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15"]
    #[inline(always)]
    pub fn extint15(&self) -> Extint15R {
        Extint15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn extint0(&mut self) -> Extint0W<IntflagSpec> {
        Extint0W::new(self, 0)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn extint1(&mut self) -> Extint1W<IntflagSpec> {
        Extint1W::new(self, 1)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn extint2(&mut self) -> Extint2W<IntflagSpec> {
        Extint2W::new(self, 2)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn extint3(&mut self) -> Extint3W<IntflagSpec> {
        Extint3W::new(self, 3)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn extint4(&mut self) -> Extint4W<IntflagSpec> {
        Extint4W::new(self, 4)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn extint5(&mut self) -> Extint5W<IntflagSpec> {
        Extint5W::new(self, 5)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn extint6(&mut self) -> Extint6W<IntflagSpec> {
        Extint6W::new(self, 6)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn extint7(&mut self) -> Extint7W<IntflagSpec> {
        Extint7W::new(self, 7)
    }
    #[doc = "Bit 8 - External Interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn extint8(&mut self) -> Extint8W<IntflagSpec> {
        Extint8W::new(self, 8)
    }
    #[doc = "Bit 9 - External Interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn extint9(&mut self) -> Extint9W<IntflagSpec> {
        Extint9W::new(self, 9)
    }
    #[doc = "Bit 10 - External Interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn extint10(&mut self) -> Extint10W<IntflagSpec> {
        Extint10W::new(self, 10)
    }
    #[doc = "Bit 11 - External Interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn extint11(&mut self) -> Extint11W<IntflagSpec> {
        Extint11W::new(self, 11)
    }
    #[doc = "Bit 12 - External Interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn extint12(&mut self) -> Extint12W<IntflagSpec> {
        Extint12W::new(self, 12)
    }
    #[doc = "Bit 13 - External Interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn extint13(&mut self) -> Extint13W<IntflagSpec> {
        Extint13W::new(self, 13)
    }
    #[doc = "Bit 14 - External Interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn extint14(&mut self) -> Extint14W<IntflagSpec> {
        Extint14W::new(self, 14)
    }
    #[doc = "Bit 15 - External Interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn extint15(&mut self) -> Extint15W<IntflagSpec> {
        Extint15W::new(self, 15)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u32 = 0;
}
