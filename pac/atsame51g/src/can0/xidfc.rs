#[doc = "Register `XIDFC` reader"]
pub type R = crate::R<XidfcSpec>;
#[doc = "Register `XIDFC` writer"]
pub type W = crate::W<XidfcSpec>;
#[doc = "Field `FLESA` reader - Filter List Extended Start Address"]
pub type FlesaR = crate::FieldReader<u16>;
#[doc = "Field `FLESA` writer - Filter List Extended Start Address"]
pub type FlesaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LSE` reader - List Size Extended"]
pub type LseR = crate::FieldReader;
#[doc = "Field `LSE` writer - List Size Extended"]
pub type LseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:15 - Filter List Extended Start Address"]
    #[inline(always)]
    pub fn flesa(&self) -> FlesaR {
        FlesaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - List Size Extended"]
    #[inline(always)]
    pub fn lse(&self) -> LseR {
        LseR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Filter List Extended Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flesa(&mut self) -> FlesaW<XidfcSpec> {
        FlesaW::new(self, 0)
    }
    #[doc = "Bits 16:22 - List Size Extended"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LseW<XidfcSpec> {
        LseW::new(self, 16)
    }
}
#[doc = "Extended ID Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`xidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XidfcSpec;
impl crate::RegisterSpec for XidfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xidfc::R`](R) reader structure"]
impl crate::Readable for XidfcSpec {}
#[doc = "`write(|w| ..)` method takes [`xidfc::W`](W) writer structure"]
impl crate::Writable for XidfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIDFC to value 0"]
impl crate::Resettable for XidfcSpec {
    const RESET_VALUE: u32 = 0;
}
