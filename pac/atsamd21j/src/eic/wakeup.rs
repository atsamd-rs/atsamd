#[doc = "Register `WAKEUP` reader"]
pub type R = crate::R<WakeupSpec>;
#[doc = "Register `WAKEUP` writer"]
pub type W = crate::W<WakeupSpec>;
#[doc = "Field `WAKEUPEN0` reader - External Interrupt 0 Wake-up Enable"]
pub type Wakeupen0R = crate::BitReader;
#[doc = "Field `WAKEUPEN0` writer - External Interrupt 0 Wake-up Enable"]
pub type Wakeupen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN1` reader - External Interrupt 1 Wake-up Enable"]
pub type Wakeupen1R = crate::BitReader;
#[doc = "Field `WAKEUPEN1` writer - External Interrupt 1 Wake-up Enable"]
pub type Wakeupen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN2` reader - External Interrupt 2 Wake-up Enable"]
pub type Wakeupen2R = crate::BitReader;
#[doc = "Field `WAKEUPEN2` writer - External Interrupt 2 Wake-up Enable"]
pub type Wakeupen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN3` reader - External Interrupt 3 Wake-up Enable"]
pub type Wakeupen3R = crate::BitReader;
#[doc = "Field `WAKEUPEN3` writer - External Interrupt 3 Wake-up Enable"]
pub type Wakeupen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN4` reader - External Interrupt 4 Wake-up Enable"]
pub type Wakeupen4R = crate::BitReader;
#[doc = "Field `WAKEUPEN4` writer - External Interrupt 4 Wake-up Enable"]
pub type Wakeupen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN5` reader - External Interrupt 5 Wake-up Enable"]
pub type Wakeupen5R = crate::BitReader;
#[doc = "Field `WAKEUPEN5` writer - External Interrupt 5 Wake-up Enable"]
pub type Wakeupen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN6` reader - External Interrupt 6 Wake-up Enable"]
pub type Wakeupen6R = crate::BitReader;
#[doc = "Field `WAKEUPEN6` writer - External Interrupt 6 Wake-up Enable"]
pub type Wakeupen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN7` reader - External Interrupt 7 Wake-up Enable"]
pub type Wakeupen7R = crate::BitReader;
#[doc = "Field `WAKEUPEN7` writer - External Interrupt 7 Wake-up Enable"]
pub type Wakeupen7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN8` reader - External Interrupt 8 Wake-up Enable"]
pub type Wakeupen8R = crate::BitReader;
#[doc = "Field `WAKEUPEN8` writer - External Interrupt 8 Wake-up Enable"]
pub type Wakeupen8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN9` reader - External Interrupt 9 Wake-up Enable"]
pub type Wakeupen9R = crate::BitReader;
#[doc = "Field `WAKEUPEN9` writer - External Interrupt 9 Wake-up Enable"]
pub type Wakeupen9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN10` reader - External Interrupt 10 Wake-up Enable"]
pub type Wakeupen10R = crate::BitReader;
#[doc = "Field `WAKEUPEN10` writer - External Interrupt 10 Wake-up Enable"]
pub type Wakeupen10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN11` reader - External Interrupt 11 Wake-up Enable"]
pub type Wakeupen11R = crate::BitReader;
#[doc = "Field `WAKEUPEN11` writer - External Interrupt 11 Wake-up Enable"]
pub type Wakeupen11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN12` reader - External Interrupt 12 Wake-up Enable"]
pub type Wakeupen12R = crate::BitReader;
#[doc = "Field `WAKEUPEN12` writer - External Interrupt 12 Wake-up Enable"]
pub type Wakeupen12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN13` reader - External Interrupt 13 Wake-up Enable"]
pub type Wakeupen13R = crate::BitReader;
#[doc = "Field `WAKEUPEN13` writer - External Interrupt 13 Wake-up Enable"]
pub type Wakeupen13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN14` reader - External Interrupt 14 Wake-up Enable"]
pub type Wakeupen14R = crate::BitReader;
#[doc = "Field `WAKEUPEN14` writer - External Interrupt 14 Wake-up Enable"]
pub type Wakeupen14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPEN15` reader - External Interrupt 15 Wake-up Enable"]
pub type Wakeupen15R = crate::BitReader;
#[doc = "Field `WAKEUPEN15` writer - External Interrupt 15 Wake-up Enable"]
pub type Wakeupen15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen0(&self) -> Wakeupen0R {
        Wakeupen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen1(&self) -> Wakeupen1R {
        Wakeupen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen2(&self) -> Wakeupen2R {
        Wakeupen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen3(&self) -> Wakeupen3R {
        Wakeupen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen4(&self) -> Wakeupen4R {
        Wakeupen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen5(&self) -> Wakeupen5R {
        Wakeupen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen6(&self) -> Wakeupen6R {
        Wakeupen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen7(&self) -> Wakeupen7R {
        Wakeupen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen8(&self) -> Wakeupen8R {
        Wakeupen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen9(&self) -> Wakeupen9R {
        Wakeupen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen10(&self) -> Wakeupen10R {
        Wakeupen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen11(&self) -> Wakeupen11R {
        Wakeupen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen12(&self) -> Wakeupen12R {
        Wakeupen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen13(&self) -> Wakeupen13R {
        Wakeupen13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen14(&self) -> Wakeupen14R {
        Wakeupen14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen15(&self) -> Wakeupen15R {
        Wakeupen15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen0(&mut self) -> Wakeupen0W<WakeupSpec> {
        Wakeupen0W::new(self, 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen1(&mut self) -> Wakeupen1W<WakeupSpec> {
        Wakeupen1W::new(self, 1)
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen2(&mut self) -> Wakeupen2W<WakeupSpec> {
        Wakeupen2W::new(self, 2)
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen3(&mut self) -> Wakeupen3W<WakeupSpec> {
        Wakeupen3W::new(self, 3)
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen4(&mut self) -> Wakeupen4W<WakeupSpec> {
        Wakeupen4W::new(self, 4)
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen5(&mut self) -> Wakeupen5W<WakeupSpec> {
        Wakeupen5W::new(self, 5)
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen6(&mut self) -> Wakeupen6W<WakeupSpec> {
        Wakeupen6W::new(self, 6)
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen7(&mut self) -> Wakeupen7W<WakeupSpec> {
        Wakeupen7W::new(self, 7)
    }
    #[doc = "Bit 8 - External Interrupt 8 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen8(&mut self) -> Wakeupen8W<WakeupSpec> {
        Wakeupen8W::new(self, 8)
    }
    #[doc = "Bit 9 - External Interrupt 9 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen9(&mut self) -> Wakeupen9W<WakeupSpec> {
        Wakeupen9W::new(self, 9)
    }
    #[doc = "Bit 10 - External Interrupt 10 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen10(&mut self) -> Wakeupen10W<WakeupSpec> {
        Wakeupen10W::new(self, 10)
    }
    #[doc = "Bit 11 - External Interrupt 11 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen11(&mut self) -> Wakeupen11W<WakeupSpec> {
        Wakeupen11W::new(self, 11)
    }
    #[doc = "Bit 12 - External Interrupt 12 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen12(&mut self) -> Wakeupen12W<WakeupSpec> {
        Wakeupen12W::new(self, 12)
    }
    #[doc = "Bit 13 - External Interrupt 13 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen13(&mut self) -> Wakeupen13W<WakeupSpec> {
        Wakeupen13W::new(self, 13)
    }
    #[doc = "Bit 14 - External Interrupt 14 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen14(&mut self) -> Wakeupen14W<WakeupSpec> {
        Wakeupen14W::new(self, 14)
    }
    #[doc = "Bit 15 - External Interrupt 15 Wake-up Enable"]
    #[inline(always)]
    pub fn wakeupen15(&mut self) -> Wakeupen15W<WakeupSpec> {
        Wakeupen15W::new(self, 15)
    }
}
#[doc = "Wake-Up Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupSpec;
impl crate::RegisterSpec for WakeupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup::R`](R) reader structure"]
impl crate::Readable for WakeupSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeup::W`](W) writer structure"]
impl crate::Writable for WakeupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAKEUP to value 0"]
impl crate::Resettable for WakeupSpec {}
