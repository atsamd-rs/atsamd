#[doc = "Register `DCC[%s]` reader"]
pub type R = crate::R<DccSpec>;
#[doc = "Register `DCC[%s]` writer"]
pub type W = crate::W<DccSpec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<DccSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Debug Communication Channel n\n\nYou can [`read`](crate::Reg::read) this register and get [`dcc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DccSpec;
impl crate::RegisterSpec for DccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcc::R`](R) reader structure"]
impl crate::Readable for DccSpec {}
#[doc = "`write(|w| ..)` method takes [`dcc::W`](W) writer structure"]
impl crate::Writable for DccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCC[%s]
to value 0"]
impl crate::Resettable for DccSpec {
    const RESET_VALUE: u32 = 0;
}
