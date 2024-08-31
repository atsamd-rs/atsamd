#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
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
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint0(&mut self) -> Extint0W<IntenclrSpec> {
        Extint0W::new(self, 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint1(&mut self) -> Extint1W<IntenclrSpec> {
        Extint1W::new(self, 1)
    }
    #[doc = "Bit 2 - External Interrupt 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint2(&mut self) -> Extint2W<IntenclrSpec> {
        Extint2W::new(self, 2)
    }
    #[doc = "Bit 3 - External Interrupt 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint3(&mut self) -> Extint3W<IntenclrSpec> {
        Extint3W::new(self, 3)
    }
    #[doc = "Bit 4 - External Interrupt 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint4(&mut self) -> Extint4W<IntenclrSpec> {
        Extint4W::new(self, 4)
    }
    #[doc = "Bit 5 - External Interrupt 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint5(&mut self) -> Extint5W<IntenclrSpec> {
        Extint5W::new(self, 5)
    }
    #[doc = "Bit 6 - External Interrupt 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint6(&mut self) -> Extint6W<IntenclrSpec> {
        Extint6W::new(self, 6)
    }
    #[doc = "Bit 7 - External Interrupt 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint7(&mut self) -> Extint7W<IntenclrSpec> {
        Extint7W::new(self, 7)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
