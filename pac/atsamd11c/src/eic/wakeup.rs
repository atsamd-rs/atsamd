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
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen0(&mut self) -> Wakeupen0W<WakeupSpec> {
        Wakeupen0W::new(self, 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen1(&mut self) -> Wakeupen1W<WakeupSpec> {
        Wakeupen1W::new(self, 1)
    }
    #[doc = "Bit 2 - External Interrupt 2 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen2(&mut self) -> Wakeupen2W<WakeupSpec> {
        Wakeupen2W::new(self, 2)
    }
    #[doc = "Bit 3 - External Interrupt 3 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen3(&mut self) -> Wakeupen3W<WakeupSpec> {
        Wakeupen3W::new(self, 3)
    }
    #[doc = "Bit 4 - External Interrupt 4 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen4(&mut self) -> Wakeupen4W<WakeupSpec> {
        Wakeupen4W::new(self, 4)
    }
    #[doc = "Bit 5 - External Interrupt 5 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen5(&mut self) -> Wakeupen5W<WakeupSpec> {
        Wakeupen5W::new(self, 5)
    }
    #[doc = "Bit 6 - External Interrupt 6 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen6(&mut self) -> Wakeupen6W<WakeupSpec> {
        Wakeupen6W::new(self, 6)
    }
    #[doc = "Bit 7 - External Interrupt 7 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupen7(&mut self) -> Wakeupen7W<WakeupSpec> {
        Wakeupen7W::new(self, 7)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKEUP to value 0"]
impl crate::Resettable for WakeupSpec {
    const RESET_VALUE: u32 = 0;
}
