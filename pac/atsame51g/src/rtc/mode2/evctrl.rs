#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `PEREO0` reader - Periodic Interval 0 Event Output Enable"]
pub type Pereo0R = crate::BitReader;
#[doc = "Field `PEREO0` writer - Periodic Interval 0 Event Output Enable"]
pub type Pereo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEREO1` reader - Periodic Interval 1 Event Output Enable"]
pub type Pereo1R = crate::BitReader;
#[doc = "Field `PEREO1` writer - Periodic Interval 1 Event Output Enable"]
pub type Pereo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEREO2` reader - Periodic Interval 2 Event Output Enable"]
pub type Pereo2R = crate::BitReader;
#[doc = "Field `PEREO2` writer - Periodic Interval 2 Event Output Enable"]
pub type Pereo2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEREO3` reader - Periodic Interval 3 Event Output Enable"]
pub type Pereo3R = crate::BitReader;
#[doc = "Field `PEREO3` writer - Periodic Interval 3 Event Output Enable"]
pub type Pereo3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEREO4` reader - Periodic Interval 4 Event Output Enable"]
pub type Pereo4R = crate::BitReader;
#[doc = "Field `PEREO4` writer - Periodic Interval 4 Event Output Enable"]
pub type Pereo4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEREO5` reader - Periodic Interval 5 Event Output Enable"]
pub type Pereo5R = crate::BitReader;
#[doc = "Field `PEREO5` writer - Periodic Interval 5 Event Output Enable"]
pub type Pereo5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEREO6` reader - Periodic Interval 6 Event Output Enable"]
pub type Pereo6R = crate::BitReader;
#[doc = "Field `PEREO6` writer - Periodic Interval 6 Event Output Enable"]
pub type Pereo6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEREO7` reader - Periodic Interval 7 Event Output Enable"]
pub type Pereo7R = crate::BitReader;
#[doc = "Field `PEREO7` writer - Periodic Interval 7 Event Output Enable"]
pub type Pereo7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARMEO0` reader - Alarm 0 Event Output Enable"]
pub type Alarmeo0R = crate::BitReader;
#[doc = "Field `ALARMEO0` writer - Alarm 0 Event Output Enable"]
pub type Alarmeo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARMEO1` reader - Alarm 1 Event Output Enable"]
pub type Alarmeo1R = crate::BitReader;
#[doc = "Field `ALARMEO1` writer - Alarm 1 Event Output Enable"]
pub type Alarmeo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPEREO` reader - Tamper Event Output Enable"]
pub type TampereoR = crate::BitReader;
#[doc = "Field `TAMPEREO` writer - Tamper Event Output Enable"]
pub type TampereoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFEO` reader - Overflow Event Output Enable"]
pub type OvfeoR = crate::BitReader;
#[doc = "Field `OVFEO` writer - Overflow Event Output Enable"]
pub type OvfeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPEVEI` reader - Tamper Event Input Enable"]
pub type TampeveiR = crate::BitReader;
#[doc = "Field `TAMPEVEI` writer - Tamper Event Input Enable"]
pub type TampeveiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline(always)]
    pub fn pereo0(&self) -> Pereo0R {
        Pereo0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline(always)]
    pub fn pereo1(&self) -> Pereo1R {
        Pereo1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline(always)]
    pub fn pereo2(&self) -> Pereo2R {
        Pereo2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline(always)]
    pub fn pereo3(&self) -> Pereo3R {
        Pereo3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline(always)]
    pub fn pereo4(&self) -> Pereo4R {
        Pereo4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline(always)]
    pub fn pereo5(&self) -> Pereo5R {
        Pereo5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline(always)]
    pub fn pereo6(&self) -> Pereo6R {
        Pereo6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline(always)]
    pub fn pereo7(&self) -> Pereo7R {
        Pereo7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm 0 Event Output Enable"]
    #[inline(always)]
    pub fn alarmeo0(&self) -> Alarmeo0R {
        Alarmeo0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm 1 Event Output Enable"]
    #[inline(always)]
    pub fn alarmeo1(&self) -> Alarmeo1R {
        Alarmeo1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper Event Output Enable"]
    #[inline(always)]
    pub fn tampereo(&self) -> TampereoR {
        TampereoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OvfeoR {
        OvfeoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper Event Input Enable"]
    #[inline(always)]
    pub fn tampevei(&self) -> TampeveiR {
        TampeveiR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interval 0 Event Output Enable"]
    #[inline(always)]
    pub fn pereo0(&mut self) -> Pereo0W<EvctrlSpec> {
        Pereo0W::new(self, 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Event Output Enable"]
    #[inline(always)]
    pub fn pereo1(&mut self) -> Pereo1W<EvctrlSpec> {
        Pereo1W::new(self, 1)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Event Output Enable"]
    #[inline(always)]
    pub fn pereo2(&mut self) -> Pereo2W<EvctrlSpec> {
        Pereo2W::new(self, 2)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Event Output Enable"]
    #[inline(always)]
    pub fn pereo3(&mut self) -> Pereo3W<EvctrlSpec> {
        Pereo3W::new(self, 3)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Event Output Enable"]
    #[inline(always)]
    pub fn pereo4(&mut self) -> Pereo4W<EvctrlSpec> {
        Pereo4W::new(self, 4)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Event Output Enable"]
    #[inline(always)]
    pub fn pereo5(&mut self) -> Pereo5W<EvctrlSpec> {
        Pereo5W::new(self, 5)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Event Output Enable"]
    #[inline(always)]
    pub fn pereo6(&mut self) -> Pereo6W<EvctrlSpec> {
        Pereo6W::new(self, 6)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Event Output Enable"]
    #[inline(always)]
    pub fn pereo7(&mut self) -> Pereo7W<EvctrlSpec> {
        Pereo7W::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm 0 Event Output Enable"]
    #[inline(always)]
    pub fn alarmeo0(&mut self) -> Alarmeo0W<EvctrlSpec> {
        Alarmeo0W::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm 1 Event Output Enable"]
    #[inline(always)]
    pub fn alarmeo1(&mut self) -> Alarmeo1W<EvctrlSpec> {
        Alarmeo1W::new(self, 9)
    }
    #[doc = "Bit 14 - Tamper Event Output Enable"]
    #[inline(always)]
    pub fn tampereo(&mut self) -> TampereoW<EvctrlSpec> {
        TampereoW::new(self, 14)
    }
    #[doc = "Bit 15 - Overflow Event Output Enable"]
    #[inline(always)]
    pub fn ovfeo(&mut self) -> OvfeoW<EvctrlSpec> {
        OvfeoW::new(self, 15)
    }
    #[doc = "Bit 16 - Tamper Event Input Enable"]
    #[inline(always)]
    pub fn tampevei(&mut self) -> TampeveiW<EvctrlSpec> {
        TampeveiW::new(self, 16)
    }
}
#[doc = "MODE2 Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {}
