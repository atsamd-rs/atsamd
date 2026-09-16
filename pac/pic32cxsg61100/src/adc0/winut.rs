#[doc = "Register `WINUT` reader"]
pub type R = crate::R<WinutSpec>;
#[doc = "Register `WINUT` writer"]
pub type W = crate::W<WinutSpec>;
#[doc = "Field `WINUT` reader - Window Upper Threshold"]
pub type WinutR = crate::FieldReader<u16>;
#[doc = "Field `WINUT` writer - Window Upper Threshold"]
pub type WinutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Window Upper Threshold"]
    #[inline(always)]
    pub fn winut(&self) -> WinutR {
        WinutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Window Upper Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn winut(&mut self) -> WinutW<WinutSpec> {
        WinutW::new(self, 0)
    }
}
#[doc = "Window Monitor Upper Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`winut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinutSpec;
impl crate::RegisterSpec for WinutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`winut::R`](R) reader structure"]
impl crate::Readable for WinutSpec {}
#[doc = "`write(|w| ..)` method takes [`winut::W`](W) writer structure"]
impl crate::Writable for WinutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WINUT to value 0"]
impl crate::Resettable for WinutSpec {
    const RESET_VALUE: u16 = 0;
}
