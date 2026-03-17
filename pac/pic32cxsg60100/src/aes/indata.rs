#[doc = "Register `INDATA` reader"]
pub type R = crate::R<IndataSpec>;
#[doc = "Register `INDATA` writer"]
pub type W = crate::W<IndataSpec>;
#[doc = "Field `INDATA` reader - Data Value"]
pub type IndataR = crate::FieldReader<u32>;
#[doc = "Field `INDATA` writer - Data Value"]
pub type IndataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Value"]
    #[inline(always)]
    pub fn indata(&self) -> IndataR {
        IndataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn indata(&mut self) -> IndataW<IndataSpec> {
        IndataW::new(self, 0)
    }
}
#[doc = "Indata\n\nYou can [`read`](crate::Reg::read) this register and get [`indata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndataSpec;
impl crate::RegisterSpec for IndataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indata::R`](R) reader structure"]
impl crate::Readable for IndataSpec {}
#[doc = "`write(|w| ..)` method takes [`indata::W`](W) writer structure"]
impl crate::Writable for IndataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INDATA to value 0"]
impl crate::Resettable for IndataSpec {
    const RESET_VALUE: u32 = 0;
}
