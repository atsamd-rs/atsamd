#[doc = "Register `FILTER` reader"]
pub type R = crate::R<FilterSpec>;
#[doc = "Register `FILTER` writer"]
pub type W = crate::W<FilterSpec>;
#[doc = "Field `FILTER` reader - Filter Value"]
pub type FilterR = crate::FieldReader;
#[doc = "Field `FILTER` writer - Filter Value"]
pub type FilterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Filter Value"]
    #[inline(always)]
    pub fn filter(&self) -> FilterR {
        FilterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter Value"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FilterW<FilterSpec> {
        FilterW::new(self, 0)
    }
}
#[doc = "Filter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterSpec;
impl crate::RegisterSpec for FilterSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`filter::R`](R) reader structure"]
impl crate::Readable for FilterSpec {}
#[doc = "`write(|w| ..)` method takes [`filter::W`](W) writer structure"]
impl crate::Writable for FilterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FilterSpec {
    const RESET_VALUE: u8 = 0;
}
