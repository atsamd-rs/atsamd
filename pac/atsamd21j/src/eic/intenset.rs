#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `EXTINT0` reader - External Interrupt 0 Enable"]
pub type Extint0R = crate::BitReader;
#[doc = "Field `EXTINT0` writer - External Interrupt 0 Enable"]
pub type Extint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT1` reader - External Interrupt 1 Enable"]
pub type Extint1R = crate::BitReader;
#[doc = "Field `EXTINT1` writer - External Interrupt 1 Enable"]
pub type Extint1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT2` reader - External Interrupt 2 Enable"]
pub type Extint2R = crate::BitReader;
#[doc = "Field `EXTINT2` writer - External Interrupt 2 Enable"]
pub type Extint2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT3` reader - External Interrupt 3 Enable"]
pub type Extint3R = crate::BitReader;
#[doc = "Field `EXTINT3` writer - External Interrupt 3 Enable"]
pub type Extint3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT4` reader - External Interrupt 4 Enable"]
pub type Extint4R = crate::BitReader;
#[doc = "Field `EXTINT4` writer - External Interrupt 4 Enable"]
pub type Extint4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT5` reader - External Interrupt 5 Enable"]
pub type Extint5R = crate::BitReader;
#[doc = "Field `EXTINT5` writer - External Interrupt 5 Enable"]
pub type Extint5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT6` reader - External Interrupt 6 Enable"]
pub type Extint6R = crate::BitReader;
#[doc = "Field `EXTINT6` writer - External Interrupt 6 Enable"]
pub type Extint6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT7` reader - External Interrupt 7 Enable"]
pub type Extint7R = crate::BitReader;
#[doc = "Field `EXTINT7` writer - External Interrupt 7 Enable"]
pub type Extint7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT8` reader - External Interrupt 8 Enable"]
pub type Extint8R = crate::BitReader;
#[doc = "Field `EXTINT8` writer - External Interrupt 8 Enable"]
pub type Extint8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT9` reader - External Interrupt 9 Enable"]
pub type Extint9R = crate::BitReader;
#[doc = "Field `EXTINT9` writer - External Interrupt 9 Enable"]
pub type Extint9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT10` reader - External Interrupt 10 Enable"]
pub type Extint10R = crate::BitReader;
#[doc = "Field `EXTINT10` writer - External Interrupt 10 Enable"]
pub type Extint10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT11` reader - External Interrupt 11 Enable"]
pub type Extint11R = crate::BitReader;
#[doc = "Field `EXTINT11` writer - External Interrupt 11 Enable"]
pub type Extint11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT12` reader - External Interrupt 12 Enable"]
pub type Extint12R = crate::BitReader;
#[doc = "Field `EXTINT12` writer - External Interrupt 12 Enable"]
pub type Extint12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT13` reader - External Interrupt 13 Enable"]
pub type Extint13R = crate::BitReader;
#[doc = "Field `EXTINT13` writer - External Interrupt 13 Enable"]
pub type Extint13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT14` reader - External Interrupt 14 Enable"]
pub type Extint14R = crate::BitReader;
#[doc = "Field `EXTINT14` writer - External Interrupt 14 Enable"]
pub type Extint14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINT15` reader - External Interrupt 15 Enable"]
pub type Extint15R = crate::BitReader;
#[doc = "Field `EXTINT15` writer - External Interrupt 15 Enable"]
pub type Extint15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Enable"]
    #[inline(always)]
    pub fn extint0(&self) -> Extint0R {
        Extint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Enable"]
    #[inline(always)]
    pub fn extint1(&self) -> Extint1R {
        Extint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Enable"]
    #[inline(always)]
    pub fn extint2(&self) -> Extint2R {
        Extint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Enable"]
    #[inline(always)]
    pub fn extint3(&self) -> Extint3R {
        Extint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Enable"]
    #[inline(always)]
    pub fn extint4(&self) -> Extint4R {
        Extint4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Enable"]
    #[inline(always)]
    pub fn extint5(&self) -> Extint5R {
        Extint5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Enable"]
    #[inline(always)]
    pub fn extint6(&self) -> Extint6R {
        Extint6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Enable"]
    #[inline(always)]
    pub fn extint7(&self) -> Extint7R {
        Extint7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8 Enable"]
    #[inline(always)]
    pub fn extint8(&self) -> Extint8R {
        Extint8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9 Enable"]
    #[inline(always)]
    pub fn extint9(&self) -> Extint9R {
        Extint9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10 Enable"]
    #[inline(always)]
    pub fn extint10(&self) -> Extint10R {
        Extint10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11 Enable"]
    #[inline(always)]
    pub fn extint11(&self) -> Extint11R {
        Extint11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12 Enable"]
    #[inline(always)]
    pub fn extint12(&self) -> Extint12R {
        Extint12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13 Enable"]
    #[inline(always)]
    pub fn extint13(&self) -> Extint13R {
        Extint13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14 Enable"]
    #[inline(always)]
    pub fn extint14(&self) -> Extint14R {
        Extint14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15 Enable"]
    #[inline(always)]
    pub fn extint15(&self) -> Extint15R {
        Extint15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint0(&mut self) -> Extint0W<IntensetSpec> {
        Extint0W::new(self, 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint1(&mut self) -> Extint1W<IntensetSpec> {
        Extint1W::new(self, 1)
    }
    #[doc = "Bit 2 - External Interrupt 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint2(&mut self) -> Extint2W<IntensetSpec> {
        Extint2W::new(self, 2)
    }
    #[doc = "Bit 3 - External Interrupt 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint3(&mut self) -> Extint3W<IntensetSpec> {
        Extint3W::new(self, 3)
    }
    #[doc = "Bit 4 - External Interrupt 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint4(&mut self) -> Extint4W<IntensetSpec> {
        Extint4W::new(self, 4)
    }
    #[doc = "Bit 5 - External Interrupt 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint5(&mut self) -> Extint5W<IntensetSpec> {
        Extint5W::new(self, 5)
    }
    #[doc = "Bit 6 - External Interrupt 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint6(&mut self) -> Extint6W<IntensetSpec> {
        Extint6W::new(self, 6)
    }
    #[doc = "Bit 7 - External Interrupt 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint7(&mut self) -> Extint7W<IntensetSpec> {
        Extint7W::new(self, 7)
    }
    #[doc = "Bit 8 - External Interrupt 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint8(&mut self) -> Extint8W<IntensetSpec> {
        Extint8W::new(self, 8)
    }
    #[doc = "Bit 9 - External Interrupt 9 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint9(&mut self) -> Extint9W<IntensetSpec> {
        Extint9W::new(self, 9)
    }
    #[doc = "Bit 10 - External Interrupt 10 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint10(&mut self) -> Extint10W<IntensetSpec> {
        Extint10W::new(self, 10)
    }
    #[doc = "Bit 11 - External Interrupt 11 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint11(&mut self) -> Extint11W<IntensetSpec> {
        Extint11W::new(self, 11)
    }
    #[doc = "Bit 12 - External Interrupt 12 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint12(&mut self) -> Extint12W<IntensetSpec> {
        Extint12W::new(self, 12)
    }
    #[doc = "Bit 13 - External Interrupt 13 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint13(&mut self) -> Extint13W<IntensetSpec> {
        Extint13W::new(self, 13)
    }
    #[doc = "Bit 14 - External Interrupt 14 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint14(&mut self) -> Extint14W<IntensetSpec> {
        Extint14W::new(self, 14)
    }
    #[doc = "Bit 15 - External Interrupt 15 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint15(&mut self) -> Extint15W<IntensetSpec> {
        Extint15W::new(self, 15)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
