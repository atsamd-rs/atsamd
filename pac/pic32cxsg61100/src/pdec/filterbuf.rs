#[doc = "Register `FILTERBUF` reader"]
pub type R = crate::R<FilterbufSpec>;
#[doc = "Register `FILTERBUF` writer"]
pub type W = crate::W<FilterbufSpec>;
#[doc = "Field `FILTERBUF` reader - Filter Buffer Value"]
pub type FilterbufR = crate::FieldReader;
#[doc = "Field `FILTERBUF` writer - Filter Buffer Value"]
pub type FilterbufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Filter Buffer Value"]
    #[inline(always)]
    pub fn filterbuf(&self) -> FilterbufR {
        FilterbufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn filterbuf(&mut self) -> FilterbufW<FilterbufSpec> {
        FilterbufW::new(self, 0)
    }
}
#[doc = "Filter Buffer Value\n\nYou can [`read`](crate::Reg::read) this register and get [`filterbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filterbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterbufSpec;
impl crate::RegisterSpec for FilterbufSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`filterbuf::R`](R) reader structure"]
impl crate::Readable for FilterbufSpec {}
#[doc = "`write(|w| ..)` method takes [`filterbuf::W`](W) writer structure"]
impl crate::Writable for FilterbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FILTERBUF to value 0"]
impl crate::Resettable for FilterbufSpec {
    const RESET_VALUE: u8 = 0;
}
