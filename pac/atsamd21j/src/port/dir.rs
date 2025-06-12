#[doc = "Register `DIR%s` reader"]
pub type R = crate::R<DirSpec>;
#[doc = "Register `DIR%s` writer"]
pub type W = crate::W<DirSpec>;
#[doc = "Field `DIR` reader - Port Data Direction"]
pub type DirR = crate::FieldReader<u32>;
#[doc = "Field `DIR` writer - Port Data Direction"]
pub type DirW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<DirSpec> {
        DirW::new(self, 0)
    }
}
#[doc = "Data Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirSpec;
impl crate::RegisterSpec for DirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dir::R`](R) reader structure"]
impl crate::Readable for DirSpec {}
#[doc = "`write(|w| ..)` method takes [`dir::W`](W) writer structure"]
impl crate::Writable for DirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIR%s to value 0"]
impl crate::Resettable for DirSpec {}
