#[doc = "Register `SAMT1` reader"]
pub type R = crate::R<Samt1Spec>;
#[doc = "Register `SAMT1` writer"]
pub type W = crate::W<Samt1Spec>;
#[doc = "Field `ADDR` reader - Specific Address 1 Mask"]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Specific Address 1 Mask"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Specific Address 1 Mask"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific Address 1 Mask"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<Samt1Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 1 Mask Top \\[47:32\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`samt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Samt1Spec;
impl crate::RegisterSpec for Samt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samt1::R`](R) reader structure"]
impl crate::Readable for Samt1Spec {}
#[doc = "`write(|w| ..)` method takes [`samt1::W`](W) writer structure"]
impl crate::Writable for Samt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMT1 to value 0"]
impl crate::Resettable for Samt1Spec {}
