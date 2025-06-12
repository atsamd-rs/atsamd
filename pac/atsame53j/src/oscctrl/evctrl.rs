#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `CFDEO0` reader - Clock 0 Failure Detector Event Output Enable"]
pub type Cfdeo0R = crate::BitReader;
#[doc = "Field `CFDEO0` writer - Clock 0 Failure Detector Event Output Enable"]
pub type Cfdeo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEO1` reader - Clock 1 Failure Detector Event Output Enable"]
pub type Cfdeo1R = crate::BitReader;
#[doc = "Field `CFDEO1` writer - Clock 1 Failure Detector Event Output Enable"]
pub type Cfdeo1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock 0 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo0(&self) -> Cfdeo0R {
        Cfdeo0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock 1 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo1(&self) -> Cfdeo1R {
        Cfdeo1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock 0 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo0(&mut self) -> Cfdeo0W<EvctrlSpec> {
        Cfdeo0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock 1 Failure Detector Event Output Enable"]
    #[inline(always)]
    pub fn cfdeo1(&mut self) -> Cfdeo1W<EvctrlSpec> {
        Cfdeo1W::new(self, 1)
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
