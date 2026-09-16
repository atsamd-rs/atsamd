#[doc = "Register `RXF1A` reader"]
pub type R = crate::R<Rxf1aSpec>;
#[doc = "Register `RXF1A` writer"]
pub type W = crate::W<Rxf1aSpec>;
#[doc = "Field `F1AI` reader - Rx FIFO 1 Acknowledge Index"]
pub type F1aiR = crate::FieldReader;
#[doc = "Field `F1AI` writer - Rx FIFO 1 Acknowledge Index"]
pub type F1aiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 1 Acknowledge Index"]
    #[inline(always)]
    pub fn f1ai(&self) -> F1aiR {
        F1aiR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 1 Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn f1ai(&mut self) -> F1aiW<Rxf1aSpec> {
        F1aiW::new(self, 0)
    }
}
#[doc = "Rx FIFO 1 Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf1aSpec;
impl crate::RegisterSpec for Rxf1aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1a::R`](R) reader structure"]
impl crate::Readable for Rxf1aSpec {}
#[doc = "`write(|w| ..)` method takes [`rxf1a::W`](W) writer structure"]
impl crate::Writable for Rxf1aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF1A to value 0"]
impl crate::Resettable for Rxf1aSpec {
    const RESET_VALUE: u32 = 0;
}
