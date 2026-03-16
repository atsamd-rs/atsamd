#[doc = "Register `DIRCLR` reader"]
pub type R = crate::R<DirclrSpec>;
#[doc = "Register `DIRCLR` writer"]
pub type W = crate::W<DirclrSpec>;
#[doc = "Field `DIRCLR` reader - Port Data Direction Clear"]
pub type DirclrR = crate::FieldReader<u32>;
#[doc = "Field `DIRCLR` writer - Port Data Direction Clear"]
pub type DirclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Clear"]
    #[inline(always)]
    pub fn dirclr(&self) -> DirclrR {
        DirclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dirclr(&mut self) -> DirclrW<DirclrSpec> {
        DirclrW::new(self, 0)
    }
}
#[doc = "Data Direction Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dirclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirclrSpec;
impl crate::RegisterSpec for DirclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirclr::R`](R) reader structure"]
impl crate::Readable for DirclrSpec {}
#[doc = "`write(|w| ..)` method takes [`dirclr::W`](W) writer structure"]
impl crate::Writable for DirclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRCLR to value 0"]
impl crate::Resettable for DirclrSpec {
    const RESET_VALUE: u32 = 0;
}
