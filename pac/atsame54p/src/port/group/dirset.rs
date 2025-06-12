#[doc = "Register `DIRSET` reader"]
pub type R = crate::R<DirsetSpec>;
#[doc = "Register `DIRSET` writer"]
pub type W = crate::W<DirsetSpec>;
#[doc = "Field `DIRSET` reader - Port Data Direction Set"]
pub type DirsetR = crate::FieldReader<u32>;
#[doc = "Field `DIRSET` writer - Port Data Direction Set"]
pub type DirsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Set"]
    #[inline(always)]
    pub fn dirset(&self) -> DirsetR {
        DirsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Set"]
    #[inline(always)]
    pub fn dirset(&mut self) -> DirsetW<DirsetSpec> {
        DirsetW::new(self, 0)
    }
}
#[doc = "Data Direction Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dirset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirsetSpec;
impl crate::RegisterSpec for DirsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirset::R`](R) reader structure"]
impl crate::Readable for DirsetSpec {}
#[doc = "`write(|w| ..)` method takes [`dirset::W`](W) writer structure"]
impl crate::Writable for DirsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIRSET to value 0"]
impl crate::Resettable for DirsetSpec {}
