#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `OVR0` reader - Channel 0 Overrun Interrupt Enable"]
pub type Ovr0R = crate::BitReader;
#[doc = "Field `OVR0` writer - Channel 0 Overrun Interrupt Enable"]
pub type Ovr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR1` reader - Channel 1 Overrun Interrupt Enable"]
pub type Ovr1R = crate::BitReader;
#[doc = "Field `OVR1` writer - Channel 1 Overrun Interrupt Enable"]
pub type Ovr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR2` reader - Channel 2 Overrun Interrupt Enable"]
pub type Ovr2R = crate::BitReader;
#[doc = "Field `OVR2` writer - Channel 2 Overrun Interrupt Enable"]
pub type Ovr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR3` reader - Channel 3 Overrun Interrupt Enable"]
pub type Ovr3R = crate::BitReader;
#[doc = "Field `OVR3` writer - Channel 3 Overrun Interrupt Enable"]
pub type Ovr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR4` reader - Channel 4 Overrun Interrupt Enable"]
pub type Ovr4R = crate::BitReader;
#[doc = "Field `OVR4` writer - Channel 4 Overrun Interrupt Enable"]
pub type Ovr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR5` reader - Channel 5 Overrun Interrupt Enable"]
pub type Ovr5R = crate::BitReader;
#[doc = "Field `OVR5` writer - Channel 5 Overrun Interrupt Enable"]
pub type Ovr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD0` reader - Channel 0 Event Detection Interrupt Enable"]
pub type Evd0R = crate::BitReader;
#[doc = "Field `EVD0` writer - Channel 0 Event Detection Interrupt Enable"]
pub type Evd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD1` reader - Channel 1 Event Detection Interrupt Enable"]
pub type Evd1R = crate::BitReader;
#[doc = "Field `EVD1` writer - Channel 1 Event Detection Interrupt Enable"]
pub type Evd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD2` reader - Channel 2 Event Detection Interrupt Enable"]
pub type Evd2R = crate::BitReader;
#[doc = "Field `EVD2` writer - Channel 2 Event Detection Interrupt Enable"]
pub type Evd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD3` reader - Channel 3 Event Detection Interrupt Enable"]
pub type Evd3R = crate::BitReader;
#[doc = "Field `EVD3` writer - Channel 3 Event Detection Interrupt Enable"]
pub type Evd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD4` reader - Channel 4 Event Detection Interrupt Enable"]
pub type Evd4R = crate::BitReader;
#[doc = "Field `EVD4` writer - Channel 4 Event Detection Interrupt Enable"]
pub type Evd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVD5` reader - Channel 5 Event Detection Interrupt Enable"]
pub type Evd5R = crate::BitReader;
#[doc = "Field `EVD5` writer - Channel 5 Event Detection Interrupt Enable"]
pub type Evd5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr0(&self) -> Ovr0R {
        Ovr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr1(&self) -> Ovr1R {
        Ovr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr2(&self) -> Ovr2R {
        Ovr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr3(&self) -> Ovr3R {
        Ovr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr4(&self) -> Ovr4R {
        Ovr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr5(&self) -> Ovr5R {
        Ovr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd0(&self) -> Evd0R {
        Evd0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd1(&self) -> Evd1R {
        Evd1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd2(&self) -> Evd2R {
        Evd2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd3(&self) -> Evd3R {
        Evd3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd4(&self) -> Evd4R {
        Evd4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd5(&self) -> Evd5R {
        Evd5R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr0(&mut self) -> Ovr0W<IntenclrSpec> {
        Ovr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr1(&mut self) -> Ovr1W<IntenclrSpec> {
        Ovr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr2(&mut self) -> Ovr2W<IntenclrSpec> {
        Ovr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr3(&mut self) -> Ovr3W<IntenclrSpec> {
        Ovr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr4(&mut self) -> Ovr4W<IntenclrSpec> {
        Ovr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ovr5(&mut self) -> Ovr5W<IntenclrSpec> {
        Ovr5W::new(self, 5)
    }
    #[doc = "Bit 8 - Channel 0 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd0(&mut self) -> Evd0W<IntenclrSpec> {
        Evd0W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd1(&mut self) -> Evd1W<IntenclrSpec> {
        Evd1W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd2(&mut self) -> Evd2W<IntenclrSpec> {
        Evd2W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd3(&mut self) -> Evd3W<IntenclrSpec> {
        Evd3W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd4(&mut self) -> Evd4W<IntenclrSpec> {
        Evd4W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 5 Event Detection Interrupt Enable"]
    #[inline(always)]
    pub fn evd5(&mut self) -> Evd5W<IntenclrSpec> {
        Evd5W::new(self, 13)
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
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
