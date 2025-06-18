#[doc = "Register `IPGS` reader"]
pub type R = crate::R<IpgsSpec>;
#[doc = "Register `IPGS` writer"]
pub type W = crate::W<IpgsSpec>;
#[doc = "Field `FL` reader - Frame Length"]
pub type FlR = crate::FieldReader<u16>;
#[doc = "Field `FL` writer - Frame Length"]
pub type FlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame Length"]
    #[inline(always)]
    pub fn fl(&self) -> FlR {
        FlR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Length"]
    #[inline(always)]
    pub fn fl(&mut self) -> FlW<IpgsSpec> {
        FlW::new(self, 0)
    }
}
#[doc = "IPG Stretch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipgs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpgsSpec;
impl crate::RegisterSpec for IpgsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipgs::R`](R) reader structure"]
impl crate::Readable for IpgsSpec {}
#[doc = "`write(|w| ..)` method takes [`ipgs::W`](W) writer structure"]
impl crate::Writable for IpgsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPGS to value 0"]
impl crate::Resettable for IpgsSpec {}
