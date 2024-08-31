#[doc = "Register `OUTCLR%s` reader"]
pub type R = crate::R<OutclrSpec>;
#[doc = "Register `OUTCLR%s` writer"]
pub type W = crate::W<OutclrSpec>;
#[doc = "Field `OUTCLR` reader - Port Data Output Value Clear"]
pub type OutclrR = crate::FieldReader<u32>;
#[doc = "Field `OUTCLR` writer - Port Data Output Value Clear"]
pub type OutclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Output Value Clear"]
    #[inline(always)]
    pub fn outclr(&self) -> OutclrR {
        OutclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output Value Clear"]
    #[inline(always)]
    #[must_use]
    pub fn outclr(&mut self) -> OutclrW<OutclrSpec> {
        OutclrW::new(self, 0)
    }
}
#[doc = "Data Output Value Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`outclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutclrSpec;
impl crate::RegisterSpec for OutclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outclr::R`](R) reader structure"]
impl crate::Readable for OutclrSpec {}
#[doc = "`write(|w| ..)` method takes [`outclr::W`](W) writer structure"]
impl crate::Writable for OutclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTCLR%s to value 0"]
impl crate::Resettable for OutclrSpec {
    const RESET_VALUE: u32 = 0;
}
