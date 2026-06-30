#[doc = "Register `DATABUFPTR` reader"]
pub type R = crate::R<DatabufptrSpec>;
#[doc = "Register `DATABUFPTR` writer"]
pub type W = crate::W<DatabufptrSpec>;
#[doc = "Field `INDATAPTR` reader - Input Data Pointer"]
pub type IndataptrR = crate::FieldReader;
#[doc = "Field `INDATAPTR` writer - Input Data Pointer"]
pub type IndataptrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Input Data Pointer"]
    #[inline(always)]
    pub fn indataptr(&self) -> IndataptrR {
        IndataptrR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Data Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn indataptr(&mut self) -> IndataptrW<DatabufptrSpec> {
        IndataptrW::new(self, 0)
    }
}
#[doc = "Data buffer pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`databufptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databufptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatabufptrSpec;
impl crate::RegisterSpec for DatabufptrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`databufptr::R`](R) reader structure"]
impl crate::Readable for DatabufptrSpec {}
#[doc = "`write(|w| ..)` method takes [`databufptr::W`](W) writer structure"]
impl crate::Writable for DatabufptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DATABUFPTR to value 0"]
impl crate::Resettable for DatabufptrSpec {
    const RESET_VALUE: u8 = 0;
}
