#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `EXTINTEO0` reader - External Interrupt 0 Event Output Enable"]
pub type Extinteo0R = crate::BitReader;
#[doc = "Field `EXTINTEO0` writer - External Interrupt 0 Event Output Enable"]
pub type Extinteo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINTEO1` reader - External Interrupt 1 Event Output Enable"]
pub type Extinteo1R = crate::BitReader;
#[doc = "Field `EXTINTEO1` writer - External Interrupt 1 Event Output Enable"]
pub type Extinteo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINTEO2` reader - External Interrupt 2 Event Output Enable"]
pub type Extinteo2R = crate::BitReader;
#[doc = "Field `EXTINTEO2` writer - External Interrupt 2 Event Output Enable"]
pub type Extinteo2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINTEO3` reader - External Interrupt 3 Event Output Enable"]
pub type Extinteo3R = crate::BitReader;
#[doc = "Field `EXTINTEO3` writer - External Interrupt 3 Event Output Enable"]
pub type Extinteo3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINTEO4` reader - External Interrupt 4 Event Output Enable"]
pub type Extinteo4R = crate::BitReader;
#[doc = "Field `EXTINTEO4` writer - External Interrupt 4 Event Output Enable"]
pub type Extinteo4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINTEO5` reader - External Interrupt 5 Event Output Enable"]
pub type Extinteo5R = crate::BitReader;
#[doc = "Field `EXTINTEO5` writer - External Interrupt 5 Event Output Enable"]
pub type Extinteo5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINTEO6` reader - External Interrupt 6 Event Output Enable"]
pub type Extinteo6R = crate::BitReader;
#[doc = "Field `EXTINTEO6` writer - External Interrupt 6 Event Output Enable"]
pub type Extinteo6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTINTEO7` reader - External Interrupt 7 Event Output Enable"]
pub type Extinteo7R = crate::BitReader;
#[doc = "Field `EXTINTEO7` writer - External Interrupt 7 Event Output Enable"]
pub type Extinteo7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Interrupt 0 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo0(&self) -> Extinteo0R {
        Extinteo0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo1(&self) -> Extinteo1R {
        Extinteo1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo2(&self) -> Extinteo2R {
        Extinteo2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo3(&self) -> Extinteo3R {
        Extinteo3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo4(&self) -> Extinteo4R {
        Extinteo4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo5(&self) -> Extinteo5R {
        Extinteo5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo6(&self) -> Extinteo6R {
        Extinteo6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7 Event Output Enable"]
    #[inline(always)]
    pub fn extinteo7(&self) -> Extinteo7R {
        Extinteo7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Interrupt 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo0(&mut self) -> Extinteo0W<EvctrlSpec> {
        Extinteo0W::new(self, 0)
    }
    #[doc = "Bit 1 - External Interrupt 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo1(&mut self) -> Extinteo1W<EvctrlSpec> {
        Extinteo1W::new(self, 1)
    }
    #[doc = "Bit 2 - External Interrupt 2 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo2(&mut self) -> Extinteo2W<EvctrlSpec> {
        Extinteo2W::new(self, 2)
    }
    #[doc = "Bit 3 - External Interrupt 3 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo3(&mut self) -> Extinteo3W<EvctrlSpec> {
        Extinteo3W::new(self, 3)
    }
    #[doc = "Bit 4 - External Interrupt 4 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo4(&mut self) -> Extinteo4W<EvctrlSpec> {
        Extinteo4W::new(self, 4)
    }
    #[doc = "Bit 5 - External Interrupt 5 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo5(&mut self) -> Extinteo5W<EvctrlSpec> {
        Extinteo5W::new(self, 5)
    }
    #[doc = "Bit 6 - External Interrupt 6 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo6(&mut self) -> Extinteo6W<EvctrlSpec> {
        Extinteo6W::new(self, 6)
    }
    #[doc = "Bit 7 - External Interrupt 7 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extinteo7(&mut self) -> Extinteo7W<EvctrlSpec> {
        Extinteo7W::new(self, 7)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {
    const RESET_VALUE: u32 = 0;
}
