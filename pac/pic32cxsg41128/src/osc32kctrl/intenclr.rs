#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready Interrupt Enable"]
pub type Xosc32krdyR = crate::BitReader;
#[doc = "Field `XOSC32KRDY` writer - XOSC32K Ready Interrupt Enable"]
pub type Xosc32krdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC32KFAIL` reader - XOSC32K Clock Failure Detector Interrupt Enable"]
pub type Xosc32kfailR = crate::BitReader;
#[doc = "Field `XOSC32KFAIL` writer - XOSC32K Clock Failure Detector Interrupt Enable"]
pub type Xosc32kfailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> Xosc32krdyR {
        Xosc32krdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32kfail(&self) -> Xosc32kfailR {
        Xosc32kfailR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xosc32krdy(&mut self) -> Xosc32krdyW<IntenclrSpec> {
        Xosc32krdyW::new(self, 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xosc32kfail(&mut self) -> Xosc32kfailW<IntenclrSpec> {
        Xosc32kfailW::new(self, 2)
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
