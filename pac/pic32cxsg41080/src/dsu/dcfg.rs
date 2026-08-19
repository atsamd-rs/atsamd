#[doc = "Register `DCFG[%s]` reader"]
pub type R = crate::R<DcfgSpec>;
#[doc = "Register `DCFG[%s]` writer"]
pub type W = crate::W<DcfgSpec>;
#[doc = "Field `DCFG` reader - Device Configuration"]
pub type DcfgR = crate::FieldReader<u32>;
#[doc = "Field `DCFG` writer - Device Configuration"]
pub type DcfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device Configuration"]
    #[inline(always)]
    pub fn dcfg(&self) -> DcfgR {
        DcfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dcfg(&mut self) -> DcfgW<DcfgSpec> {
        DcfgW::new(self, 0)
    }
}
#[doc = "Device Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfgSpec;
impl crate::RegisterSpec for DcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcfg::R`](R) reader structure"]
impl crate::Readable for DcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dcfg::W`](W) writer structure"]
impl crate::Writable for DcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCFG[%s]
to value 0"]
impl crate::Resettable for DcfgSpec {
    const RESET_VALUE: u32 = 0;
}
