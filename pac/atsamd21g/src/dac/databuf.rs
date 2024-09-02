#[doc = "Register `DATABUF` reader"]
pub type R = crate::R<DatabufSpec>;
#[doc = "Register `DATABUF` writer"]
pub type W = crate::W<DatabufSpec>;
#[doc = "Field `DATABUF` reader - Data Buffer"]
pub type DatabufR = crate::FieldReader<u16>;
#[doc = "Field `DATABUF` writer - Data Buffer"]
pub type DatabufW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Buffer"]
    #[inline(always)]
    pub fn databuf(&self) -> DatabufR {
        DatabufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn databuf(&mut self) -> DatabufW<DatabufSpec> {
        DatabufW::new(self, 0)
    }
}
#[doc = "Data Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`databuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatabufSpec;
impl crate::RegisterSpec for DatabufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`databuf::R`](R) reader structure"]
impl crate::Readable for DatabufSpec {}
#[doc = "`write(|w| ..)` method takes [`databuf::W`](W) writer structure"]
impl crate::Writable for DatabufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DATABUF to value 0"]
impl crate::Resettable for DatabufSpec {
    const RESET_VALUE: u16 = 0;
}
