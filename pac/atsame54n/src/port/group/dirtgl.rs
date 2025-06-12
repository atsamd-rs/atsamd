#[doc = "Register `DIRTGL` reader"]
pub type R = crate::R<DirtglSpec>;
#[doc = "Register `DIRTGL` writer"]
pub type W = crate::W<DirtglSpec>;
#[doc = "Field `DIRTGL` reader - Port Data Direction Toggle"]
pub type DirtglR = crate::FieldReader<u32>;
#[doc = "Field `DIRTGL` writer - Port Data Direction Toggle"]
pub type DirtglW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Direction Toggle"]
    #[inline(always)]
    pub fn dirtgl(&self) -> DirtglR {
        DirtglR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction Toggle"]
    #[inline(always)]
    pub fn dirtgl(&mut self) -> DirtglW<DirtglSpec> {
        DirtglW::new(self, 0)
    }
}
#[doc = "Data Direction Toggle\n\nYou can [`read`](crate::Reg::read) this register and get [`dirtgl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirtgl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirtglSpec;
impl crate::RegisterSpec for DirtglSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirtgl::R`](R) reader structure"]
impl crate::Readable for DirtglSpec {}
#[doc = "`write(|w| ..)` method takes [`dirtgl::W`](W) writer structure"]
impl crate::Writable for DirtglSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIRTGL to value 0"]
impl crate::Resettable for DirtglSpec {}
