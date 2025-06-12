#[doc = "Register `CCB%s` reader"]
pub type R = crate::R<CcbSpec>;
#[doc = "Register `CCB%s` writer"]
pub type W = crate::W<CcbSpec>;
#[doc = "Field `CCB` reader - Channel Compare/Capture Buffer Value"]
pub type CcbR = crate::FieldReader<u32>;
#[doc = "Field `CCB` writer - Channel Compare/Capture Buffer Value"]
pub type CcbW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&self) -> CcbR {
        CcbR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Compare/Capture Buffer Value"]
    #[inline(always)]
    pub fn ccb(&mut self) -> CcbW<CcbSpec> {
        CcbW::new(self, 0)
    }
}
#[doc = "Compare and Capture Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`ccb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcbSpec;
impl crate::RegisterSpec for CcbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccb::R`](R) reader structure"]
impl crate::Readable for CcbSpec {}
#[doc = "`write(|w| ..)` method takes [`ccb::W`](W) writer structure"]
impl crate::Writable for CcbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCB%s to value 0"]
impl crate::Resettable for CcbSpec {}
