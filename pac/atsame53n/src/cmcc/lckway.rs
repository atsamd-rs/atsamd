#[doc = "Register `LCKWAY` reader"]
pub type R = crate::R<LckwaySpec>;
#[doc = "Register `LCKWAY` writer"]
pub type W = crate::W<LckwaySpec>;
#[doc = "Field `LCKWAY` reader - Lockdown way Register"]
pub type LckwayR = crate::FieldReader;
#[doc = "Field `LCKWAY` writer - Lockdown way Register"]
pub type LckwayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Lockdown way Register"]
    #[inline(always)]
    pub fn lckway(&self) -> LckwayR {
        LckwayR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Lockdown way Register"]
    #[inline(always)]
    pub fn lckway(&mut self) -> LckwayW<LckwaySpec> {
        LckwayW::new(self, 0)
    }
}
#[doc = "Cache Lock per Way Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lckway::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckway::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LckwaySpec;
impl crate::RegisterSpec for LckwaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckway::R`](R) reader structure"]
impl crate::Readable for LckwaySpec {}
#[doc = "`write(|w| ..)` method takes [`lckway::W`](W) writer structure"]
impl crate::Writable for LckwaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCKWAY to value 0"]
impl crate::Resettable for LckwaySpec {}
