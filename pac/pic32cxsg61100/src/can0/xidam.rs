#[doc = "Register `XIDAM` reader"]
pub type R = crate::R<XidamSpec>;
#[doc = "Register `XIDAM` writer"]
pub type W = crate::W<XidamSpec>;
#[doc = "Field `EIDM` reader - Extended ID Mask"]
pub type EidmR = crate::FieldReader<u32>;
#[doc = "Field `EIDM` writer - Extended ID Mask"]
pub type EidmW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&self) -> EidmR {
        EidmR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eidm(&mut self) -> EidmW<XidamSpec> {
        EidmW::new(self, 0)
    }
}
#[doc = "Extended ID AND Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`xidam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XidamSpec;
impl crate::RegisterSpec for XidamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xidam::R`](R) reader structure"]
impl crate::Readable for XidamSpec {}
#[doc = "`write(|w| ..)` method takes [`xidam::W`](W) writer structure"]
impl crate::Writable for XidamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIDAM to value 0x1fff_ffff"]
impl crate::Resettable for XidamSpec {
    const RESET_VALUE: u32 = 0x1fff_ffff;
}
