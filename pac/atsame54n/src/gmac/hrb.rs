#[doc = "Register `HRB` reader"]
pub type R = crate::R<HrbSpec>;
#[doc = "Register `HRB` writer"]
pub type W = crate::W<HrbSpec>;
#[doc = "Field `ADDR` reader - Hash Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Hash Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<HrbSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Hash Register Bottom \\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`hrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrbSpec;
impl crate::RegisterSpec for HrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrb::R`](R) reader structure"]
impl crate::Readable for HrbSpec {}
#[doc = "`write(|w| ..)` method takes [`hrb::W`](W) writer structure"]
impl crate::Writable for HrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HRB to value 0"]
impl crate::Resettable for HrbSpec {}
