#[doc = "Register `WINLT` reader"]
pub type R = crate::R<WinltSpec>;
#[doc = "Register `WINLT` writer"]
pub type W = crate::W<WinltSpec>;
#[doc = "Field `WINLT` reader - Window Lower Threshold"]
pub type WinltR = crate::FieldReader<u16>;
#[doc = "Field `WINLT` writer - Window Lower Threshold"]
pub type WinltW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Window Lower Threshold"]
    #[inline(always)]
    pub fn winlt(&self) -> WinltR {
        WinltR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Lower Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn winlt(&mut self) -> WinltW<WinltSpec> {
        WinltW::new(self, 0)
    }
}
#[doc = "Window Monitor Lower Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`winlt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winlt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinltSpec;
impl crate::RegisterSpec for WinltSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`winlt::R`](R) reader structure"]
impl crate::Readable for WinltSpec {}
#[doc = "`write(|w| ..)` method takes [`winlt::W`](W) writer structure"]
impl crate::Writable for WinltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WINLT to value 0"]
impl crate::Resettable for WinltSpec {
    const RESET_VALUE: u16 = 0;
}
