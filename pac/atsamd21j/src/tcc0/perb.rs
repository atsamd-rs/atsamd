#[doc = "Register `PERB` reader"]
pub type R = crate::R<PerbSpec>;
#[doc = "Register `PERB` writer"]
pub type W = crate::W<PerbSpec>;
#[doc = "Field `PERB` reader - Period Buffer Value"]
pub type PerbR = crate::FieldReader<u32>;
#[doc = "Field `PERB` writer - Period Buffer Value"]
pub type PerbW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    pub fn perb(&self) -> PerbR {
        PerbR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Period Buffer Value"]
    #[inline(always)]
    #[must_use]
    pub fn perb(&mut self) -> PerbW<PerbSpec> {
        PerbW::new(self, 0)
    }
}
#[doc = "Period Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`perb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerbSpec;
impl crate::RegisterSpec for PerbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perb::R`](R) reader structure"]
impl crate::Readable for PerbSpec {}
#[doc = "`write(|w| ..)` method takes [`perb::W`](W) writer structure"]
impl crate::Writable for PerbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERB to value 0xffff_ffff"]
impl crate::Resettable for PerbSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
