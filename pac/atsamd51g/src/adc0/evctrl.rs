#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `FLUSHEI` reader - Flush Event Input Enable"]
pub type FlusheiR = crate::BitReader;
#[doc = "Field `FLUSHEI` writer - Flush Event Input Enable"]
pub type FlusheiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTEI` reader - Start Conversion Event Input Enable"]
pub type StarteiR = crate::BitReader;
#[doc = "Field `STARTEI` writer - Start Conversion Event Input Enable"]
pub type StarteiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHINV` reader - Flush Event Invert Enable"]
pub type FlushinvR = crate::BitReader;
#[doc = "Field `FLUSHINV` writer - Flush Event Invert Enable"]
pub type FlushinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTINV` reader - Start Conversion Event Invert Enable"]
pub type StartinvR = crate::BitReader;
#[doc = "Field `STARTINV` writer - Start Conversion Event Invert Enable"]
pub type StartinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRDYEO` reader - Result Ready Event Out"]
pub type ResrdyeoR = crate::BitReader;
#[doc = "Field `RESRDYEO` writer - Result Ready Event Out"]
pub type ResrdyeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINMONEO` reader - Window Monitor Event Out"]
pub type WinmoneoR = crate::BitReader;
#[doc = "Field `WINMONEO` writer - Window Monitor Event Out"]
pub type WinmoneoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flush Event Input Enable"]
    #[inline(always)]
    pub fn flushei(&self) -> FlusheiR {
        FlusheiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Input Enable"]
    #[inline(always)]
    pub fn startei(&self) -> StarteiR {
        StarteiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flush Event Invert Enable"]
    #[inline(always)]
    pub fn flushinv(&self) -> FlushinvR {
        FlushinvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start Conversion Event Invert Enable"]
    #[inline(always)]
    pub fn startinv(&self) -> StartinvR {
        StartinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    pub fn resrdyeo(&self) -> ResrdyeoR {
        ResrdyeoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn winmoneo(&self) -> WinmoneoR {
        WinmoneoR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Event Input Enable"]
    #[inline(always)]
    pub fn flushei(&mut self) -> FlusheiW<EvctrlSpec> {
        FlusheiW::new(self, 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Input Enable"]
    #[inline(always)]
    pub fn startei(&mut self) -> StarteiW<EvctrlSpec> {
        StarteiW::new(self, 1)
    }
    #[doc = "Bit 2 - Flush Event Invert Enable"]
    #[inline(always)]
    pub fn flushinv(&mut self) -> FlushinvW<EvctrlSpec> {
        FlushinvW::new(self, 2)
    }
    #[doc = "Bit 3 - Start Conversion Event Invert Enable"]
    #[inline(always)]
    pub fn startinv(&mut self) -> StartinvW<EvctrlSpec> {
        StartinvW::new(self, 3)
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    pub fn resrdyeo(&mut self) -> ResrdyeoW<EvctrlSpec> {
        ResrdyeoW::new(self, 4)
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn winmoneo(&mut self) -> WinmoneoW<EvctrlSpec> {
        WinmoneoW::new(self, 5)
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
