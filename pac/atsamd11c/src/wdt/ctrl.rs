#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEN` reader - Watchdog Timer Window Mode Enable"]
pub type WenR = crate::BitReader;
#[doc = "Field `WEN` writer - Watchdog Timer Window Mode Enable"]
pub type WenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALWAYSON` reader - Always-On"]
pub type AlwaysonR = crate::BitReader;
#[doc = "Field `ALWAYSON` writer - Always-On"]
pub type AlwaysonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog Timer Window Mode Enable"]
    #[inline(always)]
    pub fn wen(&self) -> WenR {
        WenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Always-On"]
    #[inline(always)]
    pub fn alwayson(&self) -> AlwaysonR {
        AlwaysonR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Watchdog Timer Window Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WenW<CtrlSpec> {
        WenW::new(self, 2)
    }
    #[doc = "Bit 7 - Always-On"]
    #[inline(always)]
    #[must_use]
    pub fn alwayson(&mut self) -> AlwaysonW<CtrlSpec> {
        AlwaysonW::new(self, 7)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u8 = 0;
}
