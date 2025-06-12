#[doc = "Register `SAMB1` reader"]
pub type R = crate::R<Samb1Spec>;
#[doc = "Register `SAMB1` writer"]
pub type W = crate::W<Samb1Spec>;
#[doc = "Field `ADDR` reader - Specific Address 1 Mask"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Specific Address 1 Mask"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specific Address 1 Mask"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specific Address 1 Mask"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<Samb1Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Specific Address 1 Mask Bottom \\[31:0\\] Register\n\nYou can [`read`](crate::Reg::read) this register and get [`samb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Samb1Spec;
impl crate::RegisterSpec for Samb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samb1::R`](R) reader structure"]
impl crate::Readable for Samb1Spec {}
#[doc = "`write(|w| ..)` method takes [`samb1::W`](W) writer structure"]
impl crate::Writable for Samb1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMB1 to value 0"]
impl crate::Resettable for Samb1Spec {}
