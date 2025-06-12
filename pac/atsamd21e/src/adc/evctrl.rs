#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `STARTEI` reader - Start Conversion Event In"]
pub type StarteiR = crate::BitReader;
#[doc = "Field `STARTEI` writer - Start Conversion Event In"]
pub type StarteiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEI` reader - Synchronization Event In"]
pub type SynceiR = crate::BitReader;
#[doc = "Field `SYNCEI` writer - Synchronization Event In"]
pub type SynceiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRDYEO` reader - Result Ready Event Out"]
pub type ResrdyeoR = crate::BitReader;
#[doc = "Field `RESRDYEO` writer - Result Ready Event Out"]
pub type ResrdyeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINMONEO` reader - Window Monitor Event Out"]
pub type WinmoneoR = crate::BitReader;
#[doc = "Field `WINMONEO` writer - Window Monitor Event Out"]
pub type WinmoneoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Conversion Event In"]
    #[inline(always)]
    pub fn startei(&self) -> StarteiR {
        StarteiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronization Event In"]
    #[inline(always)]
    pub fn syncei(&self) -> SynceiR {
        SynceiR::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bit 0 - Start Conversion Event In"]
    #[inline(always)]
    pub fn startei(&mut self) -> StarteiW<EvctrlSpec> {
        StarteiW::new(self, 0)
    }
    #[doc = "Bit 1 - Synchronization Event In"]
    #[inline(always)]
    pub fn syncei(&mut self) -> SynceiW<EvctrlSpec> {
        SynceiW::new(self, 1)
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
