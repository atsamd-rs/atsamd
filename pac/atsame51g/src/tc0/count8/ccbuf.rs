#[doc = "Register `CCBUF[%s]` reader"]
pub type R = crate::R<CcbufSpec>;
#[doc = "Register `CCBUF[%s]` writer"]
pub type W = crate::W<CcbufSpec>;
#[doc = "Field `CCBUF` reader - Counter/Compare Buffer Value"]
pub type CcbufR = crate::FieldReader;
#[doc = "Field `CCBUF` writer - Counter/Compare Buffer Value"]
pub type CcbufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter/Compare Buffer Value"]
    #[inline(always)]
    pub fn ccbuf(&self) -> CcbufR {
        CcbufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter/Compare Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccbuf(&mut self) -> CcbufW<CcbufSpec> {
        CcbufW::new(self, 0)
    }
}
#[doc = "COUNT8 Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcbufSpec;
impl crate::RegisterSpec for CcbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ccbuf::R`](R) reader structure"]
impl crate::Readable for CcbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ccbuf::W`](W) writer structure"]
impl crate::Writable for CcbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CCBUF[%s]
to value 0"]
impl crate::Resettable for CcbufSpec {
    const RESET_VALUE: u8 = 0;
}
