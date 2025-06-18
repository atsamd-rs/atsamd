#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `STARTEI0` reader - Start Conversion Event Input DAC 0"]
pub type Startei0R = crate::BitReader;
#[doc = "Field `STARTEI0` writer - Start Conversion Event Input DAC 0"]
pub type Startei0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTEI1` reader - Start Conversion Event Input DAC 1"]
pub type Startei1R = crate::BitReader;
#[doc = "Field `STARTEI1` writer - Start Conversion Event Input DAC 1"]
pub type Startei1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTYEO0` reader - Data Buffer Empty Event Output DAC 0"]
pub type Emptyeo0R = crate::BitReader;
#[doc = "Field `EMPTYEO0` writer - Data Buffer Empty Event Output DAC 0"]
pub type Emptyeo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTYEO1` reader - Data Buffer Empty Event Output DAC 1"]
pub type Emptyeo1R = crate::BitReader;
#[doc = "Field `EMPTYEO1` writer - Data Buffer Empty Event Output DAC 1"]
pub type Emptyeo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEI0` reader - Enable Invertion of DAC 0 input event"]
pub type Invei0R = crate::BitReader;
#[doc = "Field `INVEI0` writer - Enable Invertion of DAC 0 input event"]
pub type Invei0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEI1` reader - Enable Invertion of DAC 1 input event"]
pub type Invei1R = crate::BitReader;
#[doc = "Field `INVEI1` writer - Enable Invertion of DAC 1 input event"]
pub type Invei1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRDYEO0` reader - Result Ready Event Output 0"]
pub type Resrdyeo0R = crate::BitReader;
#[doc = "Field `RESRDYEO0` writer - Result Ready Event Output 0"]
pub type Resrdyeo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRDYEO1` reader - Result Ready Event Output 1"]
pub type Resrdyeo1R = crate::BitReader;
#[doc = "Field `RESRDYEO1` writer - Result Ready Event Output 1"]
pub type Resrdyeo1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Conversion Event Input DAC 0"]
    #[inline(always)]
    pub fn startei0(&self) -> Startei0R {
        Startei0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Input DAC 1"]
    #[inline(always)]
    pub fn startei1(&self) -> Startei1R {
        Startei1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Buffer Empty Event Output DAC 0"]
    #[inline(always)]
    pub fn emptyeo0(&self) -> Emptyeo0R {
        Emptyeo0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Buffer Empty Event Output DAC 1"]
    #[inline(always)]
    pub fn emptyeo1(&self) -> Emptyeo1R {
        Emptyeo1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Invertion of DAC 0 input event"]
    #[inline(always)]
    pub fn invei0(&self) -> Invei0R {
        Invei0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Invertion of DAC 1 input event"]
    #[inline(always)]
    pub fn invei1(&self) -> Invei1R {
        Invei1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Result Ready Event Output 0"]
    #[inline(always)]
    pub fn resrdyeo0(&self) -> Resrdyeo0R {
        Resrdyeo0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Result Ready Event Output 1"]
    #[inline(always)]
    pub fn resrdyeo1(&self) -> Resrdyeo1R {
        Resrdyeo1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event Input DAC 0"]
    #[inline(always)]
    pub fn startei0(&mut self) -> Startei0W<EvctrlSpec> {
        Startei0W::new(self, 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Input DAC 1"]
    #[inline(always)]
    pub fn startei1(&mut self) -> Startei1W<EvctrlSpec> {
        Startei1W::new(self, 1)
    }
    #[doc = "Bit 2 - Data Buffer Empty Event Output DAC 0"]
    #[inline(always)]
    pub fn emptyeo0(&mut self) -> Emptyeo0W<EvctrlSpec> {
        Emptyeo0W::new(self, 2)
    }
    #[doc = "Bit 3 - Data Buffer Empty Event Output DAC 1"]
    #[inline(always)]
    pub fn emptyeo1(&mut self) -> Emptyeo1W<EvctrlSpec> {
        Emptyeo1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Invertion of DAC 0 input event"]
    #[inline(always)]
    pub fn invei0(&mut self) -> Invei0W<EvctrlSpec> {
        Invei0W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Invertion of DAC 1 input event"]
    #[inline(always)]
    pub fn invei1(&mut self) -> Invei1W<EvctrlSpec> {
        Invei1W::new(self, 5)
    }
    #[doc = "Bit 6 - Result Ready Event Output 0"]
    #[inline(always)]
    pub fn resrdyeo0(&mut self) -> Resrdyeo0W<EvctrlSpec> {
        Resrdyeo0W::new(self, 6)
    }
    #[doc = "Bit 7 - Result Ready Event Output 1"]
    #[inline(always)]
    pub fn resrdyeo1(&mut self) -> Resrdyeo1W<EvctrlSpec> {
        Resrdyeo1W::new(self, 7)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {}
