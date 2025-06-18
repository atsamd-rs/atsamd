#[doc = "Register `BDPR` reader"]
pub type R = crate::R<BdprSpec>;
#[doc = "Register `BDPR` writer"]
pub type W = crate::W<BdprSpec>;
#[doc = "Field `BUFDATA` reader - Buffer Data"]
pub type BufdataR = crate::FieldReader<u32>;
#[doc = "Field `BUFDATA` writer - Buffer Data"]
pub type BufdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdata(&self) -> BufdataR {
        BufdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdata(&mut self) -> BufdataW<BdprSpec> {
        BufdataW::new(self, 0)
    }
}
#[doc = "Buffer Data Port\n\nYou can [`read`](crate::Reg::read) this register and get [`bdpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdprSpec;
impl crate::RegisterSpec for BdprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdpr::R`](R) reader structure"]
impl crate::Readable for BdprSpec {}
#[doc = "`write(|w| ..)` method takes [`bdpr::W`](W) writer structure"]
impl crate::Writable for BdprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDPR to value 0"]
impl crate::Resettable for BdprSpec {}
