#[doc = "Register `RXF0A` reader"]
pub type R = crate::R<Rxf0aSpec>;
#[doc = "Register `RXF0A` writer"]
pub type W = crate::W<Rxf0aSpec>;
#[doc = "Field `F0AI` reader - Rx FIFO 0 Acknowledge Index"]
pub type F0aiR = crate::FieldReader;
#[doc = "Field `F0AI` writer - Rx FIFO 0 Acknowledge Index"]
pub type F0aiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub fn f0ai(&self) -> F0aiR {
        F0aiR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0ai(&mut self) -> F0aiW<Rxf0aSpec> {
        F0aiW::new(self, 0)
    }
}
#[doc = "Rx FIFO 0 Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf0aSpec;
impl crate::RegisterSpec for Rxf0aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0a::R`](R) reader structure"]
impl crate::Readable for Rxf0aSpec {}
#[doc = "`write(|w| ..)` method takes [`rxf0a::W`](W) writer structure"]
impl crate::Writable for Rxf0aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF0A to value 0"]
impl crate::Resettable for Rxf0aSpec {
    const RESET_VALUE: u32 = 0;
}
