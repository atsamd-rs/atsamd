#[doc = "Register `HRT` reader"]
pub type R = crate::R<HrtSpec>;
#[doc = "Register `HRT` writer"]
pub type W = crate::W<HrtSpec>;
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
    pub fn addr(&mut self) -> AddrW<HrtSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Hash Register Top \\[63:32\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`hrt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrtSpec;
impl crate::RegisterSpec for HrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrt::R`](R) reader structure"]
impl crate::Readable for HrtSpec {}
#[doc = "`write(|w| ..)` method takes [`hrt::W`](W) writer structure"]
impl crate::Writable for HrtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HRT to value 0"]
impl crate::Resettable for HrtSpec {}
