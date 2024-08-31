#[doc = "Register `ITCTRL` reader"]
pub type R = crate::R<ItctrlSpec>;
#[doc = "Register `ITCTRL` writer"]
pub type W = crate::W<ItctrlSpec>;
#[doc = "Field `Mode` reader - "]
pub type ModeR = crate::BitReader;
#[doc = "Field `Mode` writer - "]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ItctrlSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Integration Mode Control\n\nYou can [`read`](crate::Reg::read) this register and get [`itctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ItctrlSpec;
impl crate::RegisterSpec for ItctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itctrl::R`](R) reader structure"]
impl crate::Readable for ItctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`itctrl::W`](W) writer structure"]
impl crate::Writable for ItctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITCTRL to value 0"]
impl crate::Resettable for ItctrlSpec {
    const RESET_VALUE: u32 = 0;
}
