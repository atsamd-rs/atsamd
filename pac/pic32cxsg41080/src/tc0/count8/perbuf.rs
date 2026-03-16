#[doc = "Register `PERBUF` reader"]
pub type R = crate::R<PerbufSpec>;
#[doc = "Register `PERBUF` writer"]
pub type W = crate::W<PerbufSpec>;
#[doc = "Field `PERBUF` reader - Period Buffer Value"]
pub type PerbufR = crate::FieldReader;
#[doc = "Field `PERBUF` writer - Period Buffer Value"]
pub type PerbufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Period Buffer Value"]
    #[inline(always)]
    pub fn perbuf(&self) -> PerbufR {
        PerbufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Period Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn perbuf(&mut self) -> PerbufW<PerbufSpec> {
        PerbufW::new(self, 0)
    }
}
#[doc = "COUNT8 Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerbufSpec;
impl crate::RegisterSpec for PerbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`perbuf::R`](R) reader structure"]
impl crate::Readable for PerbufSpec {}
#[doc = "`write(|w| ..)` method takes [`perbuf::W`](W) writer structure"]
impl crate::Writable for PerbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PERBUF to value 0xff"]
impl crate::Resettable for PerbufSpec {
    const RESET_VALUE: u8 = 0xff;
}
