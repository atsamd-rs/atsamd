#[doc = "Register `CFDCTRL` reader"]
pub type R = crate::R<CfdctrlSpec>;
#[doc = "Register `CFDCTRL` writer"]
pub type W = crate::W<CfdctrlSpec>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CfdenR = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CfdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWBACK` reader - Clock Switch Back"]
pub type SwbackR = crate::BitReader;
#[doc = "Field `SWBACK` writer - Clock Switch Back"]
pub type SwbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDPRESC` reader - Clock Failure Detector Prescaler"]
pub type CfdprescR = crate::BitReader;
#[doc = "Field `CFDPRESC` writer - Clock Failure Detector Prescaler"]
pub type CfdprescW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CfdenR {
        CfdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Switch Back"]
    #[inline(always)]
    pub fn swback(&self) -> SwbackR {
        SwbackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CfdprescR {
        CfdprescR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CfdenW<CfdctrlSpec> {
        CfdenW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Switch Back"]
    #[inline(always)]
    #[must_use]
    pub fn swback(&mut self) -> SwbackW<CfdctrlSpec> {
        SwbackW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn cfdpresc(&mut self) -> CfdprescW<CfdctrlSpec> {
        CfdprescW::new(self, 2)
    }
}
#[doc = "Clock Failure Detector Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cfdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfdctrlSpec;
impl crate::RegisterSpec for CfdctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cfdctrl::R`](R) reader structure"]
impl crate::Readable for CfdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfdctrl::W`](W) writer structure"]
impl crate::Writable for CfdctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CFDCTRL to value 0"]
impl crate::Resettable for CfdctrlSpec {
    const RESET_VALUE: u8 = 0;
}
