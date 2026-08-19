#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `EVINV` reader - Inverted Event Input Enable"]
pub type EvinvR = crate::FieldReader;
#[doc = "Field `EVINV` writer - Inverted Event Input Enable"]
pub type EvinvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EVEI` reader - Event Input Enable"]
pub type EveiR = crate::FieldReader;
#[doc = "Field `EVEI` writer - Event Input Enable"]
pub type EveiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OVFEO` reader - Overflow/Underflow Output Event Enable"]
pub type OvfeoR = crate::BitReader;
#[doc = "Field `OVFEO` writer - Overflow/Underflow Output Event Enable"]
pub type OvfeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERREO` reader - Error Output Event Enable"]
pub type ErreoR = crate::BitReader;
#[doc = "Field `ERREO` writer - Error Output Event Enable"]
pub type ErreoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIREO` reader - Direction Output Event Enable"]
pub type DireoR = crate::BitReader;
#[doc = "Field `DIREO` writer - Direction Output Event Enable"]
pub type DireoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLCEO` reader - Velocity Output Event Enable"]
pub type VlceoR = crate::BitReader;
#[doc = "Field `VLCEO` writer - Velocity Output Event Enable"]
pub type VlceoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEO0` reader - Match Channel 0 Event Output Enable"]
pub type Mceo0R = crate::BitReader;
#[doc = "Field `MCEO0` writer - Match Channel 0 Event Output Enable"]
pub type Mceo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEO1` reader - Match Channel 1 Event Output Enable"]
pub type Mceo1R = crate::BitReader;
#[doc = "Field `MCEO1` writer - Match Channel 1 Event Output Enable"]
pub type Mceo1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:4 - Inverted Event Input Enable"]
    #[inline(always)]
    pub fn evinv(&self) -> EvinvR {
        EvinvR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Event Input Enable"]
    #[inline(always)]
    pub fn evei(&self) -> EveiR {
        EveiR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    pub fn ovfeo(&self) -> OvfeoR {
        OvfeoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error Output Event Enable"]
    #[inline(always)]
    pub fn erreo(&self) -> ErreoR {
        ErreoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Direction Output Event Enable"]
    #[inline(always)]
    pub fn direo(&self) -> DireoR {
        DireoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Velocity Output Event Enable"]
    #[inline(always)]
    pub fn vlceo(&self) -> VlceoR {
        VlceoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Match Channel 0 Event Output Enable"]
    #[inline(always)]
    pub fn mceo0(&self) -> Mceo0R {
        Mceo0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Match Channel 1 Event Output Enable"]
    #[inline(always)]
    pub fn mceo1(&self) -> Mceo1R {
        Mceo1R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:4 - Inverted Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evinv(&mut self) -> EvinvW<EvctrlSpec> {
        EvinvW::new(self, 2)
    }
    #[doc = "Bits 5:7 - Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evei(&mut self) -> EveiW<EvctrlSpec> {
        EveiW::new(self, 5)
    }
    #[doc = "Bit 8 - Overflow/Underflow Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovfeo(&mut self) -> OvfeoW<EvctrlSpec> {
        OvfeoW::new(self, 8)
    }
    #[doc = "Bit 9 - Error Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erreo(&mut self) -> ErreoW<EvctrlSpec> {
        ErreoW::new(self, 9)
    }
    #[doc = "Bit 10 - Direction Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn direo(&mut self) -> DireoW<EvctrlSpec> {
        DireoW::new(self, 10)
    }
    #[doc = "Bit 11 - Velocity Output Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vlceo(&mut self) -> VlceoW<EvctrlSpec> {
        VlceoW::new(self, 11)
    }
    #[doc = "Bit 12 - Match Channel 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo0(&mut self) -> Mceo0W<EvctrlSpec> {
        Mceo0W::new(self, 12)
    }
    #[doc = "Bit 13 - Match Channel 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mceo1(&mut self) -> Mceo1W<EvctrlSpec> {
        Mceo1W::new(self, 13)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {
    const RESET_VALUE: u16 = 0;
}
