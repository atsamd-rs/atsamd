#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAENABLE` reader - DMA Enable"]
pub type DmaenableR = crate::BitReader;
#[doc = "Field `DMAENABLE` writer - DMA Enable"]
pub type DmaenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCENABLE` reader - CRC Enable"]
pub type CrcenableR = crate::BitReader;
#[doc = "Field `CRCENABLE` writer - CRC Enable"]
pub type CrcenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVLEN0` reader - Priority Level 0 Enable"]
pub type Lvlen0R = crate::BitReader;
#[doc = "Field `LVLEN0` writer - Priority Level 0 Enable"]
pub type Lvlen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVLEN1` reader - Priority Level 1 Enable"]
pub type Lvlen1R = crate::BitReader;
#[doc = "Field `LVLEN1` writer - Priority Level 1 Enable"]
pub type Lvlen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVLEN2` reader - Priority Level 2 Enable"]
pub type Lvlen2R = crate::BitReader;
#[doc = "Field `LVLEN2` writer - Priority Level 2 Enable"]
pub type Lvlen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVLEN3` reader - Priority Level 3 Enable"]
pub type Lvlen3R = crate::BitReader;
#[doc = "Field `LVLEN3` writer - Priority Level 3 Enable"]
pub type Lvlen3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    pub fn dmaenable(&self) -> DmaenableR {
        DmaenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC Enable"]
    #[inline(always)]
    pub fn crcenable(&self) -> CrcenableR {
        CrcenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Priority Level 0 Enable"]
    #[inline(always)]
    pub fn lvlen0(&self) -> Lvlen0R {
        Lvlen0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Priority Level 1 Enable"]
    #[inline(always)]
    pub fn lvlen1(&self) -> Lvlen1R {
        Lvlen1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Priority Level 2 Enable"]
    #[inline(always)]
    pub fn lvlen2(&self) -> Lvlen2R {
        Lvlen2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Priority Level 3 Enable"]
    #[inline(always)]
    pub fn lvlen3(&self) -> Lvlen3R {
        Lvlen3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaenable(&mut self) -> DmaenableW<CtrlSpec> {
        DmaenableW::new(self, 1)
    }
    #[doc = "Bit 2 - CRC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcenable(&mut self) -> CrcenableW<CtrlSpec> {
        CrcenableW::new(self, 2)
    }
    #[doc = "Bit 8 - Priority Level 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvlen0(&mut self) -> Lvlen0W<CtrlSpec> {
        Lvlen0W::new(self, 8)
    }
    #[doc = "Bit 9 - Priority Level 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvlen1(&mut self) -> Lvlen1W<CtrlSpec> {
        Lvlen1W::new(self, 9)
    }
    #[doc = "Bit 10 - Priority Level 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvlen2(&mut self) -> Lvlen2W<CtrlSpec> {
        Lvlen2W::new(self, 10)
    }
    #[doc = "Bit 11 - Priority Level 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvlen3(&mut self) -> Lvlen3W<CtrlSpec> {
        Lvlen3W::new(self, 11)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u16 = 0;
}
