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
